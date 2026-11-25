"""How a trait reference is written, and the trait of an associated type."""

from types import ModuleType
from typing import Any

import pytest
from helpers import Loaded, crate, load

LIB = """\
use std::ops::Add;

pub struct Pair;
pub struct Problem;

pub trait Parse<'a, const N: usize, T> {
    type Error;
    fn parse(&self, from: &'a T) -> Result<[u8; N], Self::Error>;
}

impl<'a> Parse<'a, 4, str> for Pair {
    type Error = Problem;

    fn parse(&self, _: &'a str) -> Result<[u8; 4], Self::Error> {
        Err(Problem)
    }
}

impl Add for Pair {
    type Output = Pair;

    fn add(self, _: Pair) -> Self::Output {
        Pair
    }
}
"""


@pytest.fixture(scope="module")
def loaded(ra: ModuleType, tmp_path_factory: pytest.TempPathFactory) -> Loaded:
    root = tmp_path_factory.mktemp("traits")
    (root / "Cargo.toml").write_text('[package]\nname = "traits"\nversion = "0.1.0"\nedition = "2021"\n\n[workspace]\n')
    (root / "src").mkdir()
    (root / "src" / "lib.rs").write_text(LIB)
    return load(ra, root)


def implementations(ra: ModuleType, loaded: Loaded) -> dict[str, Any]:
    """The implementations of the library, by the name of their trait."""
    found = crate(ra, loaded, "src/lib.rs").root_module(loaded.db).impl_defs(loaded.db)
    return {each.trait_(loaded.db).name(loaded.db).as_str(): each for each in found}


def test_a_trait_reference_is_written_with_its_lifetime_const_and_type_arguments(
    ra: ModuleType, loaded: Loaded
) -> None:
    db = loaded.db
    target = crate(ra, loaded, "src/lib.rs").to_display_target(db)
    written = {name: str(each.trait_ref(db).display(db, target)) for name, each in implementations(ra, loaded).items()}
    assert written == {"Parse": "Parse<'a, 4, str>", "Add": "Add<Self>"}


def test_an_associated_type_knows_its_trait_and_another_type_has_none(ra: ModuleType, loaded: Loaded) -> None:
    db = loaded.db
    for name, implementation in implementations(ra, loaded).items():
        functions = [item.as_Function() for item in implementation.items(db)]
        [function] = [each for each in functions if each is not None]
        returned = function.ret_type(db)
        named = [each for each in (returned, *returned.type_arguments()) if each.as_associated_type_parent_trait(db)]
        [associated] = named
        assert associated.as_associated_type_parent_trait(db) == implementation.trait_(db), name
    assert implementations(ra, loaded)["Add"].self_ty(db).as_associated_type_parent_trait(db) is None
