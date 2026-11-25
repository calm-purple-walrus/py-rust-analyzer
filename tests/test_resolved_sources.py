"""Read an item's declared type and the source chosen by rust-analyzer's resolver."""

from pathlib import Path
from shutil import copytree
from types import ModuleType
from typing import Any

import pytest
from helpers import Loaded, crate, load, parse

LIB = """\
pub mod declarations;
pub mod documented;
use declarations::{LIMIT, STATE, Alias};
pub struct With<T>(pub T);
impl<T> With<T> { pub fn get(&self) -> &T { &self.0 } }
#[derive(Clone)]
pub struct Point { pub x: u16 }
macro_rules! make { () => { 15u16 } }
macro_rules! other { () => { 2u16 } }
#[mutants::skip]
pub fn decorated() -> u16 { let unique = other!(); unique + 8 }
pub fn example<T, const N: usize>(x: u16, _: [T; N]) -> u16 {
    let x = x + 1;
    let _ = missing_name;
    let named = Point { x: LIMIT };
    let shorthand = Point { x };
    match x { LIMIT => make!(), _ => named.x + shorthand.x }
}
"""
DEFS = """\
pub const LIMIT: u16 = 3;
pub static STATE: u32 = 5;
pub type Alias<T> = Option<T>;
"""


DOCUMENTED = """\
#[cfg_attr(all(), doc = r#"ADT says "yes""#)]
#[cfg_attr(any(), doc = "disabled")]
pub(in crate::documented) struct Shape;
/// Trait docs
pub(crate) trait Role {}
#[doc = "Impl docs"]
impl Role for Shape {}
#[doc = "Function docs"]
pub(in crate::documented) fn action() {}
#[doc = "Const docs"]
pub const LIMIT: u8 = 0;
#[doc = "Static docs"]
pub(crate) static VALUE: u8 = 0;
#[doc = "Alias docs"]
pub(in crate::documented) type Alias = u8;
"""


@pytest.fixture(scope="module")
def loaded(ra: ModuleType, tmp_path_factory: pytest.TempPathFactory) -> Loaded:
    root = tmp_path_factory.mktemp("resolved-sources")
    copytree(Path(__file__).parent / "fixtures/harbor/vendor/mutants", root / "mutants")
    (root / "Cargo.toml").write_text(
        '[package]\nname="resolved"\nversion="0.1.0"\nedition="2021"\n[dependencies]\nmutants={path="mutants"}\n[workspace]\n'
    )
    (root / "src").mkdir()
    (root / "src/lib.rs").write_text(LIB)
    (root / "src/declarations.rs").write_text(DEFS)
    (root / "src/documented.rs").write_text(DOCUMENTED)
    return load(ra, root, with_build_scripts=True)


def nodes(loaded: Loaded, cls: Any, spelling: str, filename: str = "src/lib.rs") -> list[Any]:
    return [
        value
        for node in parse(loaded, filename).syntax().descendants()
        if str(node.text()) == spelling and (value := cls.cast(node)) is not None
    ]


def path_name(loaded: Loaded, file_id: Any) -> str:
    return Path(str(loaded.vfs.file_path(file_id).as_path())).name


def display(ra: ModuleType, loaded: Loaded, ty: Any) -> str:
    return str(ty.display(loaded.db, crate(ra, loaded, "src/lib.rs").to_display_target(loaded.db)))


def test_shadowed_expression_references_and_types(ra: ModuleType, loaded: Loaded) -> None:
    semantics = loaded.semantics.deref()
    db = loaded.db
    declarations: list[int] = []
    for path in nodes(loaded, ra.AstPath, "x"):
        res = semantics.resolve_path(path)
        source = res.as_Local().primary_source(db)
        declarations.append(int(source.syntax().text_range().start()))
        name = source.name().syntax()
        assert str(name.value.text()) == "x"
        assert int(name.value.text_range().start()) == declarations[-1]
        assert path_name(loaded, name.file_id.file_id().file_id(db)) == "lib.rs"
        assert path_name(loaded, source.file().file_id().file_id(db)) == "lib.rs"
        assert res.as_TypeParam() is None
    assert declarations == [LIB.index("x: u16, _"), LIB.index("x = x"), LIB.index("x = x")]
    expr = nodes(loaded, ra.AstExpr, "x + 1")[0]
    inferred = semantics.type_of_expr(expr).original
    assert display(ra, loaded, inferred) == "u16"
    assert not inferred.is_unknown()
    unresolved = nodes(loaded, ra.AstExpr, "missing_name")[0]
    assert semantics.type_of_expr(unresolved).original.is_unknown()


def test_shorthand_resolves_to_its_field_local_and_type(ra: ModuleType, loaded: Loaded) -> None:
    semantics = loaded.semantics.deref()
    for spelling, expected in [("x: LIMIT", None), ("x", LIB.index("x = x"))]:
        record = nodes(loaded, ra.AstRecordExprField, spelling)[0]
        assert str(record.field_name().syntax().text()) == "x"
        field, local, ty = semantics.resolve_record_field(record)
        assert field.name(loaded.db).as_str() == "x"
        assert display(ra, loaded, ty) == "u16"
        assert (
            None if local is None else int(local.primary_source(loaded.db).syntax().text_range().start())
        ) == expected
    pattern = nodes(loaded, ra.AstPat, "LIMIT")[0].as_IdentPat()
    assert semantics.resolve_bind_pat_to_const(pattern).as_Const() is not None
    binding = nodes(loaded, ra.AstPat, "named")[0].as_IdentPat()
    assert semantics.resolve_bind_pat_to_const(binding) is None


def test_definition_sources_retain_file_and_declaration(ra: ModuleType, loaded: Loaded) -> None:
    db = loaded.db
    sem = loaded.semantics.deref()
    root = crate(ra, loaded, "src/lib.rs").root_module(db)
    defs = root.declarations(db)
    module = next(m for d in defs if (m := d.as_Module()) is not None and m.name(db).as_str() == "declarations")
    assert module.name(db).as_str() == "declarations"
    items = module.declarations(db)
    for kind, declaration in [
        ("Const", DEFS.splitlines()[0]),
        ("Static", DEFS.splitlines()[1]),
        ("TypeAlias", DEFS.splitlines()[2]),
    ]:
        item = next(value for d in items if (value := getattr(d, "as_" + kind)()) is not None)
        source = sem.source(item).syntax()
        assert str(source.value.text()) == declaration
        assert path_name(loaded, source.file_id.file_id().file_id(db)) == "declarations.rs"
    macro = sem.resolve_path(nodes(loaded, ra.AstPath, "make")[0]).as_Def().as_Macro()
    src = sem.source(macro).syntax()
    assert str(src.value.text()).startswith("macro_rules! make")
    assert path_name(loaded, src.file_id.file_id().file_id(db)) == "lib.rs"
    for name, kind, expected in [("T", "TypeParam", "T"), ("N", "ConstParam", "const N: usize")]:
        path = nodes(loaded, ra.AstPath, name)[-1]
        resolved = sem.resolve_path(path)
        assert resolved.as_Local() is None
        param = getattr(resolved, "as_" + kind)()
        source = sem.source(param.merge()).syntax()
        assert str(source.value.text()) == expected
        assert path_name(loaded, source.file_id.file_id().file_id(db)) == "lib.rs"


def test_signatures_and_declared_types(ra: ModuleType, loaded: Loaded) -> None:
    db = loaded.db
    sem = loaded.semantics.deref()
    root = crate(ra, loaded, "src/lib.rs").root_module(db)
    fn = next(
        f for d in root.declarations(db) if (f := d.as_Function()) is not None and f.name(db).as_str() == "example"
    )
    assert ra.ModuleDef.Function(fn).as_Function() == fn
    assert [display(ra, loaded, p.ty()) for p in fn.assoc_fn_params(db)] == ["u16", "[T; N]"]
    alias = sem.to_def(nodes(loaded, ra.AstTypeAlias, DEFS.splitlines()[2], "src/declarations.rs")[0])
    impl_ = sem.to_def(nodes(loaded, ra.AstImpl, "impl<T> With<T> { pub fn get(&self) -> &T { &self.0 } }")[0])
    get = next(f for item in impl_.items(db) if (f := item.as_Function()) is not None)
    assert [display(ra, loaded, p.ty()) for p in get.assoc_fn_params(db)] == ["&With<T>"]
    for owner in [ra.GenericDef.from_Function(fn), ra.GenericDef.from_TypeAlias(alias), ra.GenericDef.from_Impl(impl_)]:
        assert owner.type_or_const_params(db)[0].as_type_param(db).name(db).as_str() == "T"
    for cls, declaration, spelling in zip(
        [ra.AstConst, ra.AstStatic, ra.AstTypeAlias], DEFS.splitlines(), ["u16", "u32", "Option<T>"], strict=True
    ):
        ast_ty = nodes(loaded, ra.AstType, spelling, "src/declarations.rs")[0]
        assert str(ast_ty.syntax().text()) == spelling
        ast_item = nodes(loaded, cls, declaration, "src/declarations.rs")[0]
        assert display(ra, loaded, sem.to_def(src=ast_item).ty(db)) == spelling


def test_expanded_ranges_and_derived_impls(ra: ModuleType, loaded: Loaded) -> None:
    db = loaded.db
    sem = loaded.semantics.deref()
    root = crate(ra, loaded, "src/lib.rs").root_module(db)
    call = nodes(loaded, ra.AstMacroCall, "make!()")[0]
    expanded = sem.expand_macro_call(call).value
    macro_file = loaded.semantics.hir_file_for(expanded)
    assert macro_file.file_id() is None
    outer = macro_file.original_call_node(db)
    assert str(outer.value.text()) == "make!()"
    assert path_name(loaded, outer.file_id.file_id(db)) == "lib.rs"
    assert loaded.semantics.hir_file_for(call.syntax()).original_call_node(db) is None
    assert loaded.semantics.hir_file_for(call.syntax()).file_id() is not None
    origins = [sem.impl_generated_from_derive(i) for i in root.impl_defs(db)]
    assert [adt.name(db).as_str() for adt in origins if adt is not None] == ["Point"]
    assert None in origins


def test_descend_into_attribute_returns_native_nodes(ra: ModuleType, loaded: Loaded) -> None:
    sem = loaded.semantics.deref()
    for cls, spelling in [
        (ra.AstMacroCall, "other!()"),
        (ra.AstExpr, "unique + 8"),
        (ra.AstPat, "unique"),
        (ra.AstPath, "unique"),
    ]:
        written = nodes(loaded, cls, spelling)[0]
        candidates = sem.descend_node_into_attributes(node=written)
        assert all(type(node) is cls for node in candidates)
        assert [str(node.syntax().text()) for node in candidates] == [spelling.replace(" ", "")]
        assert loaded.semantics.hir_file_for(candidates[0].syntax()).file_id() is None
        assert loaded.semantics.hir_file_for(written.syntax()).file_id() is not None


def test_documentation_and_visibility_on_hir_definitions(ra: ModuleType, loaded: Loaded) -> None:
    db = loaded.db
    root = crate(ra, loaded, "src/lib.rs").root_module(db)
    module = next(m for m in root.children(db) if m.name(db).as_str() == "documented")
    definitions = module.declarations(db)
    for kind, docs, visible in [
        ("Adt", 'ADT says "yes"', module),
        ("Trait", "Trait docs", "crate"),
        ("Function", "Function docs", module),
        ("Const", "Const docs", None),
        ("Static", "Static docs", "crate"),
        ("TypeAlias", "Alias docs", module),
    ]:
        definition = next(x for d in definitions if (x := getattr(d, "as_" + kind)()) is not None)
        assert definition.hir_docs(db).docs() == docs
        visibility = definition.visibility(db)
        if visible is None:
            assert visibility == ra.Visibility.Public()
        elif visible == "crate":
            assert visibility != ra.Visibility.Public() and visibility.as_Module() is None, kind
        else:
            assert visibility.as_Module() == visible, kind
    assert [i.hir_docs(db).docs() for i in module.impl_defs(db)] == ["Impl docs"]
