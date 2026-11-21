"""Reading the syntax tree of the build-script crate's `shapes` module from Python."""

from types import ModuleType
from typing import Any

import pytest
from helpers import Loaded, parse

SHAPES = "src/shapes.rs"

# Each class with `can_cast`, and the one kind it accepts.
CAST = [
    ("AstArgList", "ARG_LIST"), ("AstAwaitExpr", "AWAIT_EXPR"), ("AstBlockExpr", "BLOCK_EXPR"),
    ("AstExprStmt", "EXPR_STMT"), ("AstExternCrate", "EXTERN_CRATE"), ("AstItemList", "ITEM_LIST"),
    ("AstLetExpr", "LET_EXPR"), ("AstMatchArmList", "MATCH_ARM_LIST"), ("AstMatchExpr", "MATCH_EXPR"),
    ("AstMethodCallExpr", "METHOD_CALL_EXPR"), ("AstParenExpr", "PAREN_EXPR"), ("AstRefExpr", "REF_EXPR"),
    ("AstReturnExpr", "RETURN_EXPR"), ("AstSourceFile", "SOURCE_FILE"), ("AstTryExpr", "TRY_EXPR"), ("AstUse", "USE"),
]  # fmt: skip


@pytest.fixture(scope="module")
def root(built_loaded: Loaded) -> Any:
    return parse(built_loaded, SHAPES).syntax()


@pytest.fixture(scope="module")
def text(built_loaded: Loaded) -> str:
    return (built_loaded.root / SHAPES).read_text()


def node(root: Any, text: str) -> Any:
    """The first node, in preorder, whose text is exactly `text`."""
    return next(n for n in root.descendants() if str(n.text()) == text)


def test_a_node_has_its_kind(root: Any) -> None:
    kinds = [repr(n.kind()) for n in (root, node(root, "pub struct Unit;"), node(root, "Unit"))]
    assert kinds == ["SOURCE_FILE", "STRUCT", "NAME"]


def test_a_node_knows_its_parent_and_ancestors(root: Any) -> None:
    name = node(root, "Unit")
    assert repr(name.parent().kind()) == "STRUCT"
    assert root.parent() is None
    assert [repr(a.kind()) for a in name.ancestors()] == ["NAME", "STRUCT", "SOURCE_FILE"]


def test_descendants_with_tokens_give_the_nodes_and_every_token_in_order(root: Any) -> None:
    elements = node(root, "pub struct Unit;").descendants_with_tokens()
    written = [token.text() if (token := e.as_Token()) is not None else "<node>" for e in elements]
    # STRUCT, then VISIBILITY with `pub`, then NAME with `Unit`, in preorder
    assert written == ["<node>", "<node>", "pub", " ", "struct", " ", "<node>", "Unit", ";"]


def test_a_token_knows_where_it_is(ra: ModuleType, root: Any, text: str) -> None:
    elements = node(root, "pub struct Unit;").descendants_with_tokens()
    [keyword] = [t for e in elements if (t := e.as_Token()) is not None and t.text() == "struct"]
    start = text.index("pub struct Unit;") + len("pub ")  # the STRUCT node itself starts 4 earlier
    assert repr(keyword.text_range()) == f"{start}..{start + 6}"
    assert repr(keyword.text_range().end()) == str(start + 6)
    assert repr(ra.TextRange.empty(ra.TextSize.from_u32(7))) == "7..7"


@pytest.mark.parametrize(("name", "kind"), CAST)
def test_can_cast_accepts_exactly_its_kind(ra: ModuleType, root: Any, name: str, kind: str) -> None:
    nodes = list(root.descendants())
    accepted = [getattr(ra, name).can_cast(n.kind()) for n in nodes]
    assert accepted == [repr(n.kind()) == kind for n in nodes]
    assert any(accepted)


# Each class with `cast`, and the one kind it accepts.
CAST_ONE = [
    ("AstAssocItemList", "ASSOC_ITEM_LIST"), ("AstAttr", "ATTR"), ("AstCallExpr", "CALL_EXPR"),
    ("AstExprStmt", "EXPR_STMT"), ("AstFieldExpr", "FIELD_EXPR"), ("AstItemList", "ITEM_LIST"),
    ("AstLetStmt", "LET_STMT"), ("AstMethodCallExpr", "METHOD_CALL_EXPR"), ("AstModule", "MODULE"),
    ("AstRecordFieldList", "RECORD_FIELD_LIST"), ("AstRecordPat", "RECORD_PAT"), ("AstSourceFile", "SOURCE_FILE"),
    ("AstStmtList", "STMT_LIST"), ("AstVariantList", "VARIANT_LIST"),
]  # fmt: skip


@pytest.mark.parametrize(("name", "kind"), CAST_ONE)
def test_cast_accepts_exactly_its_kind(ra: ModuleType, root: Any, name: str, kind: str) -> None:
    nodes = list(root.descendants())
    accepted = [getattr(ra, name).cast(n) is not None for n in nodes]
    assert accepted == [repr(n.kind()) == kind for n in nodes]
    assert any(accepted)


def test_an_expression_is_any_expression_kind_or_a_literal(ra: ModuleType, root: Any) -> None:
    nodes = list(root.descendants())
    accepted = [ra.AstExpr.cast(n) is not None for n in nodes]
    assert accepted == [repr(n.kind()).endswith("_EXPR") or repr(n.kind()) == "LITERAL" for n in nodes]
    assert any(accepted)


def test_a_type_definition_has_its_name(ra: ModuleType, root: Any) -> None:
    assert [adt.name().text() for _, adt in casts(ra, root, "AstAdt")] == ["Record", "Pair", "Unit", "Choice"]


def test_a_type_definition_is_a_struct_an_enum_or_a_union(ra: ModuleType, root: Any) -> None:
    nodes = list(root.descendants())
    accepted = [ra.AstAdt.cast(n) is not None for n in nodes]
    assert accepted == [repr(n.kind()) in ("STRUCT", "ENUM", "UNION") for n in nodes]
    assert sum(accepted) == 4  # Record, Pair, Unit, Choice


@pytest.mark.parametrize("name", ["AstAdt", "AstAttr", "AstExpr", "AstFn", "AstMacroCall", "AstModule"])
def test_a_cast_node_gives_back_the_node_it_was_cast_from(ra: ModuleType, root: Any, name: str) -> None:
    pairs = [(n, cast) for n in root.descendants() if (cast := getattr(ra, name).cast(n)) is not None]
    assert pairs
    assert all(cast.syntax() == n for n, cast in pairs)


def test_an_item_gives_its_node(built_loaded: Loaded, root: Any) -> None:
    items = list(parse(built_loaded, SHAPES).items())
    assert [repr(item.syntax().kind()) for item in items] == [
        "EXTERN_CRATE", "USE", "STRUCT", "STRUCT", "STRUCT", "ENUM", "MODULE", "TRAIT", "IMPL", "IMPL", "IMPL", "FN",
        "FN", "FN",
    ]  # fmt: skip
    assert all(item.syntax().parent() == root for item in items)


EXPR_VARIANTS = [
    ("AwaitExpr", "AWAIT_EXPR"), ("BlockExpr", "BLOCK_EXPR"), ("ForExpr", "FOR_EXPR"), ("IfExpr", "IF_EXPR"),
    ("LoopExpr", "LOOP_EXPR"), ("MatchExpr", "MATCH_EXPR"), ("ParenExpr", "PAREN_EXPR"), ("PathExpr", "PATH_EXPR"),
    ("TryExpr", "TRY_EXPR"), ("WhileExpr", "WHILE_EXPR"),
]  # fmt: skip


def function(ra: ModuleType, root: Any, name: str) -> Any:
    """The first function, in preorder, called `name`."""
    return next(f for n in root.descendants() if (f := ra.AstFn.cast(n)) is not None and f"fn {name}(" in str(n.text()))


def written(node: Any) -> str:
    return str(node.syntax().text())


def test_a_body_lists_its_statements_and_its_final_value(ra: ModuleType, root: Any) -> None:
    flows = function(ra, root, "flows")
    body = flows.body()
    assert body.syntax().parent() == flows.syntax()
    statements = [repr(s.syntax().kind()) for s in body.stmt_list().statements()]
    assert statements == ["LET_STMT"] * 3 + ["EXPR_STMT"] * 5 + ["LET_STMT"] * 3 + ["EXPR_STMT"]
    assert written(body.stmt_list().tail_expr()) == "Some(total)"
    assert function(ra, root, "area").body() is None  # declared in the trait
    assert function(ra, root, "inside").body().stmt_list().tail_expr() is None


def test_statements_give_their_expression_or_pattern(ra: ModuleType, root: Any) -> None:
    statements = [s.syntax() for s in function(ra, root, "flows").body().stmt_list().statements()]
    expressions = [s.expr() for n in statements if (s := ra.AstExprStmt.cast(n)) is not None]
    assert [repr(e.syntax().kind()) for e in expressions] == [
        "FOR_EXPR", "WHILE_EXPR", "LOOP_EXPR", "MATCH_EXPR", "IF_EXPR", "BLOCK_EXPR",
    ]  # fmt: skip
    patterns = [s.pat() for n in statements if (s := ra.AstLetStmt.cast(n)) is not None]
    # `first`, the two `Record { .. }`, `total`, `read`, `_`
    assert [(p.as_IdentPat() is not None, p.as_WildcardPat() is not None) for p in patterns] == [
        (True, False), (False, False), (False, False), (True, False), (True, False), (False, True),
    ]  # fmt: skip


@pytest.mark.parametrize(("variant", "kind"), EXPR_VARIANTS)
def test_an_expression_variant_is_read_exactly_for_its_kind(ra: ModuleType, root: Any, variant: str, kind: str) -> None:
    pairs = [(n, e) for n in root.descendants() if (e := ra.AstExpr.cast(n)) is not None]
    read = [getattr(e, f"as_{variant}")() is not None for _, e in pairs]
    assert read == [repr(n.kind()) == kind for n, _ in pairs]
    assert any(read)


def test_an_expression_gives_the_expression_inside(ra: ModuleType, root: Any) -> None:
    expressions = [e for n in root.descendants() if (e := ra.AstExpr.cast(n)) is not None]
    assert [written(a.expr()) for e in expressions if (a := e.as_AwaitExpr()) is not None] == ["ready()"]
    assert [written(p.expr()) for e in expressions if (p := e.as_ParenExpr()) is not None] == ["found + 1"]
    tried = [written(t.expr()) for e in expressions if (t := e.as_TryExpr()) is not None]
    assert tried == ["values.first()", "(found + 1).checked_add(0)"]
    calls = [written(c.expr()) for n in root.descendants() if (c := ra.AstCallExpr.cast(n)) is not None]
    assert calls == ["Record::new", "Record::new", "Some", "Record::new", "Record::new", "Some", "ready"]
    fields = [written(f.expr()) for n in root.descendants() if (f := ra.AstFieldExpr.cast(n)) is not None]
    assert fields == ["self", "Record::new(2)"]


def test_a_path_expression_gives_its_path(ra: ModuleType, root: Any) -> None:
    pairs = [(n, p) for n in root.descendants() if (e := ra.AstExpr.cast(n)) is not None and (p := e.as_PathExpr())]
    assert pairs
    assert all(p.path().syntax().parent() == n for n, p in pairs)


def test_a_function_lists_its_attributes(ra: ModuleType, root: Any) -> None:
    attributes = list(function(ra, root, "flows").attrs())
    assert [written(a) for a in attributes] == [
        "#[allow(dead_code)]", "#[cfg_attr(built, deny(unused_variables))]", '#[doc = "Flows."]', "#[must_use]",
    ]  # fmt: skip


def test_an_attribute_gives_its_meta_in_each_form(ra: ModuleType, root: Any) -> None:
    metas = [a.meta() for a in function(ra, root, "flows").attrs()]
    assert [written(m) for m in metas] == [
        "allow(dead_code)",
        "cfg_attr(built, deny(unused_variables))",
        'doc = "Flows."',
        "must_use",
    ]
    forms = [
        [
            m.as_TokenTreeMeta() is not None,
            m.as_CfgAttrMeta() is not None,
            m.as_KeyValueMeta() is not None,
            m.as_PathMeta() is not None,
        ]
        for m in metas
    ]
    assert forms == [
        [True, False, False, False],
        [False, True, False, False],
        [False, False, True, False],
        [False, False, False, True],
    ]
    allow, cfg_attr, doc, must_use = metas
    assert written(allow.as_TokenTreeMeta().path()) == "allow"
    assert written(allow.as_TokenTreeMeta().token_tree()) == "(dead_code)"
    assert written(cfg_attr.as_CfgAttrMeta().cfg_predicate()) == "built"
    assert [written(m) for m in cfg_attr.as_CfgAttrMeta().metas()] == ["deny(unused_variables)"]
    assert written(doc.as_KeyValueMeta().path()) == "doc"
    assert written(must_use.as_PathMeta().path()) == "must_use"


def test_a_function_has_its_name(ra: ModuleType, root: Any) -> None:
    assert function(ra, root, "flows").name().text() == "flows"
    assert function(ra, root, "inside").name().text() == "inside"


def test_every_plain_comment_is_found_with_its_text(ra: ModuleType, root: Any, text: str) -> None:
    tokens = [t for e in root.descendants_with_tokens() if (t := e.as_Token()) is not None]
    comments = [c for t in tokens if (c := ra.AstComment.cast(t)) is not None]
    # rust-analyzer 0.0.352 lexes `///` and `//!` as OUTER_DOC_COMMENT and INNER_DOC_COMMENT, not as COMMENT.
    lines = [line.strip() for line in text.splitlines()]
    assert [c.text() for c in comments] == [
        line for line in lines if line.startswith("//") and not line.startswith(("///", "//!"))
    ]
    assert all(c.syntax().text() == c.text() and repr(c.syntax().kind()) == "COMMENT" for c in comments)


def casts(ra: ModuleType, root: Any, name: str) -> list[tuple[Any, Any]]:
    """Each node of the tree that the class `name` casts, with the cast value."""
    return [(n, cast) for n in root.descendants() if (cast := getattr(ra, name).cast(n)) is not None]


def test_an_associated_item_list_lists_its_items(ra: ModuleType, root: Any) -> None:
    lists = casts(
        ra, root, "AstAssocItemList"
    )  # trait Shape, impl Record, impl Shape for Record, impl Display for Unit
    assert [len(list(items.assoc_items())) for _, items in lists] == [2, 1, 1, 1]
    node, shape = lists[0]
    assert [written(i).splitlines()[0] for i in shape.assoc_items()] == [
        "fn area(&self) -> u32;",
        "fn label(&self) -> &str {",
    ]
    assert all(i.syntax().parent() == node for i in shape.assoc_items())


def test_a_module_lists_its_items_if_written_inline(ra: ModuleType, built_loaded: Loaded, root: Any) -> None:
    [(_, nested)] = casts(ra, root, "AstModule")
    assert [repr(i.syntax().kind()) for i in nested.item_list().items()] == ["FN"]
    [(_, shapes)] = casts(ra, parse(built_loaded, "src/lib.rs").syntax(), "AstModule")  # `pub mod shapes;`
    assert shapes.item_list() is None


def test_record_fields_and_variants_are_listed_with_their_doc_comments(ra: ModuleType, root: Any) -> None:
    (record_node, record), (third_node, third) = casts(ra, root, "AstRecordFieldList")  # Record, Choice::Third
    fields = list(record.fields())
    assert [written(f).splitlines()[-1].strip() for f in fields] == ["pub count: u32", "hidden: bool"]
    assert [len(list(f.doc_comments())) for f in fields] == [1, 0]
    assert all(f.syntax().parent() == record_node for f in fields)
    assert [written(f) for f in third.fields()] == ["inner: u8"]
    [(variants_node, choice)] = casts(ra, root, "AstVariantList")
    variants = list(choice.variants())
    assert [written(v).splitlines()[-1].strip() for v in variants] == ["First", "Second(u8)", "Third { inner: u8 }"]
    assert [len(list(v.doc_comments())) for v in variants] == [1, 0, 0]
    assert all(v.syntax().parent() == variants_node for v in variants)
    assert third_node.parent() == variants[2].syntax()  # `FieldList` is an enum, not a node


def test_a_record_pattern_gives_its_path_its_fields_and_its_rest(ra: ModuleType, root: Any) -> None:
    # `let Record { count, .. }` and `let Record { count: _, hidden }`
    (_, first), (_, second) = casts(ra, root, "AstRecordPat")
    assert [written(p.path()) for p in (first, second)] == ["Record", "Record"]

    def fields(pattern: Any) -> list[tuple[bool, bool]]:
        found = [f.pat() for f in pattern.record_pat_field_list().fields()]
        return [(p.as_IdentPat() is not None, p.as_WildcardPat() is not None) for p in found]

    assert fields(first) == [(True, False)]
    assert fields(second) == [(False, True), (True, False)]
    assert first.record_pat_field_list().rest_pat() is not None
    assert second.record_pat_field_list().rest_pat() is None
