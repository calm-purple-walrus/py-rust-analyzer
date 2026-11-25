"""Reading the crate `written` from Python: where its definitions are written, and how its code is written."""

from types import ModuleType
from typing import Any

import pytest
from helpers import Loaded, crate, declared, load, parse

LIB = "src/lib.rs"


@pytest.fixture(scope="module")
def db(written_loaded: Loaded) -> Any:
    return written_loaded.db


@pytest.fixture(scope="module")
def sema(written_loaded: Loaded) -> Any:
    return written_loaded.semantics.deref()


@pytest.fixture(scope="module")
def root(written_loaded: Loaded) -> Any:
    return parse(written_loaded, LIB).syntax()


@pytest.fixture(scope="module")
def text(written_loaded: Loaded) -> str:
    return (written_loaded.root / LIB).read_text()


def node(root: Any, text: str) -> Any:
    """The first node, in preorder, whose text is exactly `text`."""
    return next(n for n in root.descendants() if str(n.text()) == text)


def casts(ra: ModuleType, root: Any, name: str) -> list[Any]:
    return [cast for n in root.descendants() if (cast := getattr(ra, name).cast(n)) is not None]


def adt(ra: ModuleType, loaded: Loaded, name: str) -> Any:
    return declared(ra, loaded, "Adt", name)


def function(ra: ModuleType, loaded: Loaded, name: str) -> Any:
    return declared(ra, loaded, "Function", name)


def implementations(ra: ModuleType, loaded: Loaded) -> list[Any]:
    return crate(ra, loaded, LIB).root_module(loaded.db).impl_defs(loaded.db)


# ---------------------------------------------------------------- modules and definitions


def test_a_crate_has_its_root_module_and_a_module_its_children(ra: ModuleType, written_loaded: Loaded, db: Any) -> None:
    root_module = crate(ra, written_loaded, LIB).root_module(db)
    assert root_module.name(db) is None
    children = root_module.children(db)
    assert sorted(child.name(db).as_str() for child in children) == ["inline", "outline"]
    assert {child.name(db).as_str(): child.is_inline(db) for child in children} == {"inline": True, "outline": False}
    assert [child.children(db) for child in children] == [[], []]


def test_definitions_are_equal_and_hash_alike_when_they_are_the_same(ra: ModuleType, written_loaded: Loaded) -> None:
    for variant, name, other in [
        ("Adt", "Record", "Pair"),
        ("Function", "body", "switched"),
        ("Trait", "Convert", None),
    ]:
        first, again = declared(ra, written_loaded, variant, name), declared(ra, written_loaded, variant, name)
        assert first is not again and first == again and hash(first) == hash(again)
        assert {first: 1}[again] == 1
        if other is not None:
            assert first != declared(ra, written_loaded, variant, other)
    assert first != adt(ra, written_loaded, "Record")


def test_modules_are_equal_when_they_are_the_same(ra: ModuleType, written_loaded: Loaded, db: Any) -> None:
    record, pair = adt(ra, written_loaded, "Record"), adt(ra, written_loaded, "Pair")
    assert record.module(db) == pair.module(db)
    root_module = crate(ra, written_loaded, LIB).root_module(db)
    assert record.module(db) == root_module
    assert all(child != root_module for child in root_module.children(db))


def test_a_union_is_read_with_its_fields(ra: ModuleType, written_loaded: Loaded, db: Any) -> None:
    union = adt(ra, written_loaded, "Bits").as_Union()
    assert [field.name(db).as_str() for field in union.fields(db)] == ["whole", "parts"]
    assert adt(ra, written_loaded, "Record").as_Union() is None


def test_a_trait_knows_its_module_and_its_parameters(ra: ModuleType, written_loaded: Loaded, db: Any) -> None:
    convert = declared(ra, written_loaded, "Trait", "Convert")
    assert convert.module(db) == adt(ra, written_loaded, "Record").module(db)
    # `Self` and `T`
    assert len(ra.GenericDef.from_Trait(convert).params(db)) == 2


def test_a_field_is_seen_by_everything_by_a_module_or_by_the_crate(
    ra: ModuleType, written_loaded: Loaded, db: Any
) -> None:
    record = adt(ra, written_loaded, "Record")
    count, shared, hidden = record.as_struct().fields(db)
    assert count.visibility(db) == ra.Visibility.Public() and count.visibility(db).as_Module() is None
    # `pub(crate)` names the crate, not a module.
    assert shared.visibility(db) != ra.Visibility.Public() and shared.visibility(db).as_Module() is None
    assert hidden.visibility(db).as_Module() == record.module(db)
    assert hidden.visibility(db).as_Module() not in record.module(db).children(db)


def test_documentation_is_read_from_a_field_and_a_variant(ra: ModuleType, written_loaded: Loaded, db: Any) -> None:
    count, shared, _ = adt(ra, written_loaded, "Record").as_struct().fields(db)
    assert count.hir_docs(db).docs() == "The count."
    assert shared.hir_docs(db) is None
    first, second = adt(ra, written_loaded, "Choice").as_enum().variants(db)
    assert first.hir_docs(db).docs() == "The first."
    assert second.hir_docs(db) is None


def test_a_type_is_displayed_truncated_without_its_default_arguments(
    ra: ModuleType, written_loaded: Loaded, db: Any
) -> None:
    [implementation] = [i for i in implementations(ra, written_loaded) if i.trait_(db) is None]
    [method] = [item.as_Function() for item in implementation.items(db)]
    returned = method.ret_type(db)
    target = crate(ra, written_loaded, LIB).to_display_target(db)
    assert str(returned.display_truncated(db, None, target)) == "Vec<u32>"
    assert str(returned.display(db, target)) == "Vec<u32, Global>"
    assert str(returned.display_truncated(db, 3, target)) == "Vec<…>"


# ---------------------------------------------------------------- where definitions are written


def test_the_source_of_a_definition_is_its_node(ra: ModuleType, written_loaded: Loaded, db: Any, sema: Any) -> None:
    record = adt(ra, written_loaded, "Record")
    assert sema.source(record).value.name().text() == "Record"
    assert sema.source(function(ra, written_loaded, "body")).value.name().text() == "body"
    convert = declared(ra, written_loaded, "Trait", "Convert")
    written_trait = sema.source(convert)
    assert str(written_trait.value.syntax().text()).startswith("pub trait Convert<T>")
    assert written_trait.file_id.as_MacroFile() is None
    count = record.as_struct().fields(db)[0]
    field = sema.source(count)
    assert field.file_id.as_MacroFile() is None
    assert str(field.value.as_Named().syntax().text()).endswith("pub count: u32")
    assert field.value.as_Pos() is None
    positional = sema.source(adt(ra, written_loaded, "Pair").as_struct().fields(db)[1]).value
    assert positional.as_Named() is None and str(positional.as_Pos().syntax().text()) == "u16"
    variant = sema.source(adt(ra, written_loaded, "Choice").as_enum().variants(db)[1])
    assert str(variant.value.syntax().text()) == "Second(u8)" and variant.file_id.as_MacroFile() is None


def test_the_source_of_an_implementation_is_its_node(
    ra: ModuleType, written_loaded: Loaded, db: Any, sema: Any
) -> None:
    found = implementations(ra, written_loaded)
    written = sorted(str(sema.source(i).value.syntax().text()).splitlines()[0] for i in found)
    assert written == ["impl Convert<u8> for Pair {", "impl Record {"]
    assert sema.source(found[0]).file_id.as_MacroFile() is None


def test_the_original_node_is_the_node_of_the_file_or_none_for_what_a_macro_wrote(
    ra: ModuleType, written_loaded: Loaded, db: Any, sema: Any, root: Any
) -> None:
    record = sema.source(adt(ra, written_loaded, "Record")).value
    assert sema.original_ast_node(record).syntax() == node(root, str(record.syntax().text()))
    made = adt(ra, written_loaded, "Made")
    source = sema.source(made)
    assert source.file_id.as_MacroFile() is not None and sema.original_ast_node(source.value) is None
    made_field = sema.source(made.as_struct().fields(db)[0]).value.as_Named()
    assert sema.original_ast_node(made_field) is None
    for name in ("AstFn", "AstImpl", "AstTrait", "AstModule"):
        written = casts(ra, root, name)
        assert written
        assert all(sema.original_ast_node(each).syntax() == each.syntax() for each in written)
    positional = sema.source(adt(ra, written_loaded, "Pair").as_struct().fields(db)[1]).value.as_Pos()
    assert sema.original_ast_node(positional).syntax() == node(root, "u16")
    named = sema.source(adt(ra, written_loaded, "Record").as_struct().fields(db)[2]).value.as_Named()
    assert sema.original_ast_node(named).syntax() == node(root, "hidden: bool")
    variant = sema.source(adt(ra, written_loaded, "Choice").as_enum().variants(db)[1]).value
    assert sema.original_ast_node(variant).syntax() == node(root, "Second(u8)")


def test_a_node_of_a_file_gives_its_definition(
    ra: ModuleType, written_loaded: Loaded, db: Any, sema: Any, root: Any
) -> None:
    assert [sema.to_def(each).name(db).as_str() for each in casts(ra, root, "AstAdt")] == [
        "Record",
        "Pair",
        "Bits",
        "Choice",
    ]
    assert [sema.to_def(each).name(db).as_str() for each in casts(ra, root, "AstTrait")] == ["Convert"]
    assert sema.to_def(casts(ra, root, "AstFn")[0]).name(db).as_str() == "inside"
    implementations = [sema.to_def(each) for each in casts(ra, root, "AstImpl")]
    assert [i.trait_(db) is None for i in implementations] == [False, True]


# ---------------------------------------------------------------- how code is written


@pytest.mark.parametrize(
    ("name", "kinds"),
    [
        ("AstImpl", {"IMPL"}),
        ("AstTrait", {"TRAIT"}),
        ("AstLetExpr", {"LET_EXPR"}),
        ("AstPat", {"IDENT_PAT", "TUPLE_STRUCT_PAT", "WILDCARD_PAT", "LITERAL_PAT"}),
    ],
)
def test_cast_accepts_exactly_its_kinds(ra: ModuleType, root: Any, name: str, kinds: set[str]) -> None:
    nodes = list(root.descendants())
    accepted = [getattr(ra, name).cast(n) is not None for n in nodes]
    assert accepted == [repr(n.kind()) in kinds for n in nodes]
    assert {repr(n.kind()) for n, cast in zip(nodes, accepted, strict=True) if cast} == kinds


def test_can_cast_tells_an_item_and_a_documentation_comment(ra: ModuleType, root: Any) -> None:
    nodes = list(root.descendants())
    items = {repr(n.kind()) for n in nodes if ra.AstItem.can_cast(n.kind())}
    assert items == {"MODULE", "FN", "STRUCT", "UNION", "ENUM", "TRAIT", "IMPL", "MACRO_RULES", "MACRO_CALL"}
    documentation = [str(n.text()) for n in nodes if ra.AstDocComment.can_cast(n.kind())]
    assert documentation == [
        "//! Code that the tests read as it is written.",
        "/// A record.",
        "/// The count.",
        "/// The first.",
    ]


def test_an_implementation_and_a_trait_list_their_items(ra: ModuleType, root: Any) -> None:
    [convert] = casts(ra, root, "AstTrait")
    assert len(list(convert.assoc_item_list().assoc_items())) == 1
    assert str(convert.syntax().text()).startswith("pub trait Convert<T> {")
    first, second = casts(ra, root, "AstImpl")
    assert str(first.syntax().text()).startswith("impl Convert<u8> for Pair {")
    assert [len(list(each.assoc_item_list().assoc_items())) for each in (first, second)] == [1, 1]
    assert casts(ra, node(root, "pub struct Pair(pub u8, u16);"), "AstImpl") == []


def test_children_are_the_nodes_directly_inside_and_with_tokens_also_the_tokens(ra: ModuleType, root: Any) -> None:
    pair = node(root, "pub struct Pair(pub u8, u16);")
    assert [repr(child.kind()) for child in pair.children()] == ["VISIBILITY", "NAME", "TUPLE_FIELD_LIST"]
    elements = list(pair.children_with_tokens())
    written = [token.text() if (token := e.as_Token()) is not None else repr(e.into_node().kind()) for e in elements]
    assert written == ["VISIBILITY", " ", "struct", " ", "NAME", "TUPLE_FIELD_LIST", ";"]
    assert [e.into_node() is None for e in elements] == [False, True, True, True, False, False, True]


def test_a_token_gives_what_stands_before_it(ra: ModuleType, root: Any) -> None:
    pair = node(root, "pub struct Pair(pub u8, u16);")
    semicolon = list(pair.children_with_tokens())[-1].as_Token()
    before = semicolon.prev_sibling_or_token()
    assert repr(before.into_node().kind()) == "TUPLE_FIELD_LIST"
    keyword = list(pair.children_with_tokens())[2].as_Token()
    assert keyword.prev_sibling_or_token().as_Token().text() == " "
    first = next(e.as_Token() for e in root.descendants_with_tokens() if e.as_Token() is not None)
    assert first.prev_sibling_or_token() is None


def test_trivia_is_space_and_comments_and_a_comma_is_a_kind(ra: ModuleType, root: Any) -> None:
    tokens = [
        t for e in node(root, "_ => first,").parent().descendants_with_tokens() if (t := e.as_Token()) is not None
    ]
    trivia = {repr(t.kind()) for t in tokens if t.kind().is_trivia()}
    assert trivia == {"WHITESPACE", "COMMENT"}
    assert [t.text() for t in tokens if t.kind() == ra.SyntaxKind.COMMA()] == [",", ","]
    assert repr(ra.SyntaxKind.COMMA()) == "COMMA"


def test_names_give_their_nodes(ra: ModuleType, root: Any) -> None:
    [call] = casts(ra, root, "AstMethodCallExpr")
    assert str(call.name_ref().syntax().text()) == "convert"
    assert repr(call.name_ref().syntax().kind()) == "NAME_REF"
    body = next(f for f in casts(ra, root, "AstFn") if f.name().text() == "body")
    assert str(body.name().syntax().text()) == "body" and repr(body.name().syntax().kind()) == "NAME"


def test_a_let_and_a_match_give_what_they_take_apart(ra: ModuleType, root: Any) -> None:
    kept, taken_apart = casts(ra, root, "AstLetStmt")
    assert str(kept.initializer().syntax().text()) == "pair.convert()"
    assert str(taken_apart.initializer().syntax().text()) == "pair"
    [condition] = casts(ra, root, "AstLetExpr")
    assert condition.pat().as_IdentPat() is None and condition.pat().as_WildcardPat() is None
    assert str(condition.expr().syntax().text()) == "choice"
    [match] = [
        m for n in root.descendants() if (m := ra.AstExpr.cast(n)) is not None and repr(n.kind()) == "MATCH_EXPR"
    ]
    assert str(match.as_MatchExpr().expr().syntax().text()) == "first"


def test_an_identifier_pattern_gives_its_subpattern(ra: ModuleType, root: Any) -> None:
    kept = casts(ra, root, "AstLetStmt")[0].pat().as_IdentPat()
    assert kept.pat() is None


def test_the_type_of_a_pattern_is_what_it_takes_apart(
    ra: ModuleType, written_loaded: Loaded, db: Any, sema: Any
) -> None:
    body = sema.source(function(ra, written_loaded, "body")).value.syntax()
    target = crate(ra, written_loaded, LIB).to_display_target(db)
    shown = [
        (str(n.text()), str(sema.type_of_pat(pattern).original.display(db, target)))
        for n in body.descendants()
        if (pattern := ra.AstPat.cast(n)) is not None
    ]
    assert shown == [
        ("pair", "Pair"),
        ("choice", "Choice"),
        ("kept", "u8"),
        ("Pair(first, _)", "Pair"),
        ("first", "u8"),
        ("_", "u16"),
        ("Choice::Second(inner)", "Choice"),
        ("inner", "u8"),
        ("0", "u8"),
        ("_", "u8"),
    ]
    [condition] = casts(ra, body, "AstLetExpr")
    assert str(sema.type_of_pat(condition.pat()).original.display(db, target)) == "Choice"


def test_a_place_counts_characters_in_another_encoding(ra: ModuleType, text: str) -> None:
    index = ra.LineIndex.new(text)
    offset = text.encode().index(b"a comment")
    at = index.line_col(ra.TextSize.from_u32(offset))
    line_number: int = at.line
    before = text.splitlines()[line_number].partition("a comment")[0]
    wide = index.to_wide(ra.WideEncoding.Utf32(), at)
    assert at.col == len(before.encode())
    assert wide.col == len(before) == at.col - 3  # `ä` is two bytes and `→` is three
    assert int(ra.TextSize.from_u32(offset)) == offset


def test_a_file_that_parses_has_no_errors_and_one_that_does_not_lists_them(
    ra: ModuleType, written_loaded: Loaded, db: Any, sema: Any, tmp_path_factory: pytest.TempPathFactory
) -> None:
    source = sema.source(adt(ra, written_loaded, "Record"))
    assert source.file_id.file_id().parse_errors(db) is None
    broken = tmp_path_factory.mktemp("broken")
    (broken / "src").mkdir()
    (broken / "Cargo.toml").write_text('[package]\nname = "broken"\nversion = "0.1.0"\nedition = "2024"\n')
    (broken / "src/lib.rs").write_text("pub fn fine() {}\n\npub fn broken( {\n")
    loaded = load(ra, broken)
    fine = loaded.semantics.deref().source(declared(ra, loaded, "Function", "fine"))
    errors = fine.file_id.file_id().parse_errors(loaded.db)
    assert errors and "expected" in str(errors[0])
    assert int(errors[0].range().start()) >= len("pub fn fine() {}\n\npub fn broken(")
