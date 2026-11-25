"""Const arguments, crate versions, field accesses and the type aliases of an implementation."""

from types import ModuleType
from typing import Any

import pytest
from helpers import Loaded, crate, declared, load, parse

LIB = """\
pub struct Buffer<const N: usize>(pub [u8; N]);

pub struct Holder {
    pub small: Buffer<16>,
    pub either: Result<u8, Buffer<32>>,
    pub plain: u8,
}

pub struct Shape {
    pub sides: u8,
}

pub union Bits {
    pub whole: u32,
    pub half: u16,
}

pub enum Choice {
    Named { inner: u8 },
}

pub struct Pair(pub u8, pub u8);

pub struct Problem;

impl TryFrom<&Shape> for Pair {
    type Error = Problem;

    fn try_from(shape: &Shape) -> Result<Self, Self::Error> {
        Ok(Pair(shape.sides, shape.sides))
    }
}

pub fn reads(shape: &Shape, bits: Bits, pair: (u8, u16)) -> u16 {
    let half = unsafe { bits.half };
    half + pair.1 + shape.sides as u16
}
"""


@pytest.fixture(scope="module")
def loaded(ra: ModuleType, tmp_path_factory: pytest.TempPathFactory) -> Loaded:
    root = tmp_path_factory.mktemp("brief")
    (root / "Cargo.toml").write_text('[package]\nname = "brief"\nversion = "1.2.3"\nedition = "2021"\n\n[workspace]\n')
    (root / "src").mkdir()
    (root / "src" / "lib.rs").write_text(LIB)
    return load(ra, root)


def fields(ra: ModuleType, loaded: Loaded, name: str) -> dict[str, Any]:
    found = declared(ra, loaded, "Adt", name)
    holder = found.as_struct() or found.as_Union()
    return {field.name(loaded.db).as_str(): field for field in holder.fields(loaded.db)}


def test_a_type_lists_its_type_and_const_arguments_as_text(ra: ModuleType, loaded: Loaded) -> None:
    db = loaded.db
    target = crate(ra, loaded, "src/lib.rs").to_display_target(db)
    types = {name: field.ty(db) for name, field in fields(ra, loaded, "Holder").items()}
    assert list(types["small"].type_and_const_arguments(db, target)) == ["16"]
    assert list(types["either"].type_and_const_arguments(db, target)) == ["u8", "Buffer<32>"]
    assert list(types["plain"].type_and_const_arguments(db, target)) == []


def test_a_crate_has_the_version_of_its_package(ra: ModuleType, loaded: Loaded) -> None:
    assert crate(ra, loaded, "src/lib.rs").version(loaded.db) == "1.2.3"


def test_a_field_expression_resolves_to_the_field_it_reads(ra: ModuleType, loaded: Loaded) -> None:
    db = loaded.db
    semantics = loaded.semantics.deref()
    read: list[tuple[str, str | None]] = []
    for node in parse(loaded, "src/lib.rs").syntax().descendants():
        if (expression := ra.AstFieldExpr.cast(node)) is not None:
            resolved = semantics.resolve_field(expression)
            field = None if resolved is None else resolved.as_Left()
            read.append((str(node.text()), None if field is None else field.name(db).as_str()))
    # A field of a tuple is no `Field`.
    assert read == [
        ("shape.sides", "sides"),
        ("shape.sides", "sides"),
        ("bits.half", "half"),
        ("pair.1", None),
        ("shape.sides", "sides"),
    ]


def test_a_field_knows_the_struct_union_or_variant_that_declares_it(ra: ModuleType, loaded: Loaded) -> None:
    db = loaded.db
    of_struct = fields(ra, loaded, "Shape")["sides"].parent_def(db)
    of_union = fields(ra, loaded, "Bits")["half"].parent_def(db)
    [named] = declared(ra, loaded, "Adt", "Choice").as_enum().variants(db)
    [inner] = named.fields(db)
    of_variant = inner.parent_def(db)
    assert ra.Adt.Struct(of_struct.as_Struct()).name(db).as_str() == "Shape"
    assert ra.Adt.Union(of_union.as_Union()).name(db).as_str() == "Bits"
    assert of_variant.as_EnumVariant().name(db).as_str() == "Named"
    assert (of_struct.as_Union(), of_struct.as_EnumVariant()) == (None, None)
    assert (of_union.as_Struct(), of_union.as_EnumVariant()) == (None, None)
    assert (of_variant.as_Struct(), of_variant.as_Union()) == (None, None)


def test_an_implementation_gives_the_type_its_alias_stands_for(ra: ModuleType, loaded: Loaded) -> None:
    db = loaded.db
    library = crate(ra, loaded, "src/lib.rs")
    target = library.to_display_target(db)
    [implementation] = library.root_module(db).impl_defs(db)
    aliases = [item.as_type_alias() for item in implementation.items(db)]
    [error] = [alias for alias in aliases if alias is not None]
    assert len(aliases) == 2  # the alias and the function, which is no alias
    assert (error.name(db).as_str(), str(error.ty(db).display(db, target))) == ("Error", "Problem")
