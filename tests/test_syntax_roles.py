"""Read the children of Rust constructs through their typed AST roles."""

from types import ModuleType
from typing import Any

import pytest
from helpers import Loaded, load, parse

SOURCE = """\
/// line comment
/** block comment */
#[doc = r#"words "in quotes""#]
#[doc = "a\\nb"]
pub struct Raw { pub r#type: i32 }
pub struct Pair(i32, i32);
pub struct Point { pub x: i32, pub y: i32 }
pub fn sample((a, b): (i32, i32), point: Point) {
    if a > b { first(a, b) } else if a == 0 { second() } else { third() }
    while a > 0 { break }
    for x in [a, b] { use_it(x) }
    loop { break }
    match a { x if x > 5 => first(x, b), _ => second() }
    let callback = |x, y| x + y;
    let Some(value) = None::<i32> else { return };
    let Pair(p, q) = Pair(a, b);
    let &(ref borrowed) = &b;
    match [a, b] { [1 | 2, tail] => (), _ => () }
    let Point { x, y: z } = point;
    let _ = "\\q";
    let r#type = 0;
    let Raw { r#type } = Raw { r#type };
    let Raw { r#type: renamed } = Raw { r#type: a };
    let _ = a.checked_add(b);
    let _ = (a, b).1;
    let _ = point.x;
    let _ = [a; 3];
    let _ = unsafe {};
    let _ = async {};
    let _ = const {};
    let _ = gen {};
    let _ = async gen {};
    let _ = try {};
    let _ = try bikeshed Result<(), ()> {};
    let _ = 'here: {};
    let _ = ..b;
    let _ = a..=b;
    match a { 10.. => (), 3..=5 => (), _ => () }
    return ();
}
unsafe extern "C" { fn external(); static EXTERNAL: i32; }
"""


@pytest.fixture(scope="module")
def loaded(ra: ModuleType, tmp_path_factory: pytest.TempPathFactory) -> Loaded:
    root = tmp_path_factory.mktemp("syntax-roles")
    (root / "Cargo.toml").write_text('[package]\nname="syntax_roles"\nversion="0.1.0"\nedition="2024"\n[workspace]\n')
    (root / "src").mkdir()
    (root / "src/lib.rs").write_text(SOURCE)
    return load(ra, root)


def text(value: Any) -> str:
    return str(value.syntax().text())


def node(loaded: Loaded, spelling: str) -> Any:
    return next(n for n in parse(loaded, "src/lib.rs").syntax().descendants() if str(n.text()) == spelling)


def expression(ra: ModuleType, loaded: Loaded, spelling: str) -> Any:
    return next(
        expr
        for n in parse(loaded, "src/lib.rs").syntax().descendants()
        if str(n.text()) == spelling and (expr := ra.AstExpr.cast(n)) is not None
    )


def test_function_and_closure_parameters(ra: ModuleType, loaded: Loaded) -> None:
    function = next(
        f
        for n in parse(loaded, "src/lib.rs").syntax().descendants()
        if (f := ra.AstFn.cast(n)) is not None and f.name().text() == "sample"
    )
    assert [text(p.pat()) for p in function.param_list().params()] == ["(a, b)", "point"]
    closure = ra.AstClosureExpr.cast(node(loaded, "|x, y| x + y"))
    assert [text(p.pat()) for p in closure.param_list().params()] == ["x", "y"]
    assert text(closure.body()) == "x + y"


def test_branches_and_loop_roles(ra: ModuleType, loaded: Loaded) -> None:
    code = "if a > b { first(a, b) } else if a == 0 { second() } else { third() }"
    outer = expression(ra, loaded, code).as_IfExpr()
    assert text(outer.condition()) == "a > b"
    assert text(outer.then_branch()) == "{ first(a, b) }"
    inner = outer.else_branch()
    assert text(inner) == "if a == 0 { second() } else { third() }"
    assert text(ra.AstExpr.cast(inner.syntax()).as_IfExpr().else_branch()) == "{ third() }"
    while_loop = expression(ra, loaded, "while a > 0 { break }").as_WhileExpr()
    assert text(while_loop.condition()) == "a > 0"
    assert text(while_loop.loop_body()) == "{ break }"
    for_loop = expression(ra, loaded, "for x in [a, b] { use_it(x) }").as_ForExpr()
    assert text(for_loop.pat()) == "x"
    assert text(for_loop.iterable()) == "[a, b]"
    assert text(for_loop.loop_body()) == "{ use_it(x) }"
    assert text(expression(ra, loaded, "loop { break }").as_LoopExpr().loop_body()) == "{ break }"


def test_match_arms_and_early_returns(ra: ModuleType, loaded: Loaded) -> None:
    match = expression(ra, loaded, "match a { x if x > 5 => first(x, b), _ => second() }").as_MatchExpr()
    first, second = match.match_arm_list().arms()
    assert text(ra.AstMatchArm.cast(node(loaded, "x if x > 5 => first(x, b),")).pat()) == "x"
    assert (text(first.pat()), text(first.guard().condition()), text(first.expr())) == ("x", "x > 5", "first(x, b)")
    assert (text(second.pat()), second.guard(), text(second.expr())) == ("_", None, "second()")
    let = ra.AstLetStmt.cast(node(loaded, "let Some(value) = None::<i32> else { return };"))
    assert text(let.let_else().block_expr()) == "{ return }"
    assert ra.AstReturnExpr.cast(node(loaded, "return")).expr() is None
    assert text(ra.AstReturnExpr.cast(node(loaded, "return ()")).expr()) == "()"


def test_operator_array_and_call_arguments(ra: ModuleType, loaded: Loaded) -> None:
    binary = ra.AstBinExpr.cast(node(loaded, "x + y"))
    assert (str(binary.op_kind()), text(binary.lhs()), text(binary.rhs())) == ("+", "x", "y")
    listed = ra.AstArrayExpr.cast(node(loaded, "[a, b]")).kind()
    assert [text(e) for e in listed.as_ElementList()] == ["a", "b"]
    assert listed.as_Repeat() is None
    repeated = ra.AstArrayExpr.cast(node(loaded, "[a; 3]")).kind()
    assert tuple(text(e) for e in repeated.as_Repeat()) == ("a", "3")
    assert repeated.as_ElementList() is None
    call = ra.AstCallExpr.cast(node(loaded, "first(a, b)"))
    assert [text(arg) for arg in call.arg_list().args()] == ["a", "b"]
    method = ra.AstMethodCallExpr.cast(node(loaded, "a.checked_add(b)"))
    assert text(method.receiver()) == "a"
    assert [text(arg) for arg in method.arg_list().args()] == ["b"]


def test_field_access_and_pattern_fields(ra: ModuleType, loaded: Loaded) -> None:
    named = ra.AstFieldExpr.cast(node(loaded, "point.x")).field_access()
    assert text(named.as_Name()) == "x"
    assert named.as_Index() is None
    positional = ra.AstFieldExpr.cast(node(loaded, "(a, b).1")).field_access()
    assert text(positional.as_Name()) == "1"
    assert positional.as_Index() is None
    record = ra.AstRecordPat.cast(node(loaded, "Point { x, y: z }"))
    assert [text(field.field_name()) for field in record.record_pat_field_list().fields()] == ["x", "y"]
    assert text(ra.AstRecordPatField.cast(node(loaded, "y: z")).field_name()) == "y"
    tup = ra.AstTuplePat.cast(node(loaded, "(a, b)"))
    assert [text(p) for p in tup.fields()] == ["a", "b"]
    pair = ra.AstTupleStructPat.cast(node(loaded, "Pair(p, q)"))
    assert [text(p) for p in pair.fields()] == ["p", "q"]
    slice_pat = ra.AstSlicePat.cast(node(loaded, "[1 | 2, tail]"))
    either, tail = slice_pat.pats()
    assert text(tail) == "tail"
    assert [text(p) for p in ra.AstOrPat.cast(either.syntax()).pats()] == ["1", "2"]
    ref_pat = ra.AstRefPat.cast(node(loaded, "&(ref borrowed)"))
    paren = ra.AstParenPat.cast(ref_pat.pat().syntax())
    assert text(paren.pat()) == "ref borrowed"


@pytest.mark.parametrize(
    ("prefix", "variant", "token"),
    [
        ("unsafe", "Unsafe", "unsafe"),
        ("async", "Async", "async"),
        ("const", "Const", "const"),
        ("gen", "Gen", "gen"),
        ("async gen", "AsyncGen", "gen"),
    ],
)
def test_block_modifier_tokens(ra: ModuleType, loaded: Loaded, prefix: str, variant: str, token: str) -> None:
    block = expression(ra, loaded, prefix + " {}").as_BlockExpr()
    assert getattr(block.modifier(), "as_" + variant)().text() == token


def test_try_label_and_unmodified_blocks(ra: ModuleType, loaded: Loaded) -> None:
    attempt = expression(ra, loaded, "try {}").as_BlockExpr().modifier()
    token, bikeshed, ty = attempt.as_Try()
    assert (token.text(), bikeshed, ty) == ("try", None, None)
    typed = expression(ra, loaded, "try bikeshed Result<(), ()> {}").as_BlockExpr().modifier()
    token, bikeshed, ty = typed.as_Try()
    assert (token.text(), bikeshed.text(), text(ty)) == ("try", "bikeshed", "Result<(), ()>")
    label = expression(ra, loaded, "'here: {}").as_BlockExpr().modifier().as_Label()
    assert text(label) == "'here:"
    assert expression(ra, loaded, "{ third() }").as_BlockExpr().modifier() is None


def test_extern_block_items(ra: ModuleType, loaded: Loaded) -> None:
    block = ra.AstExternBlock.cast(node(loaded, 'unsafe extern "C" { fn external(); static EXTERNAL: i32; }'))
    assert [text(item) for item in block.extern_item_list().extern_items()] == [
        "fn external();",
        "static EXTERNAL: i32;",
    ]


def test_open_and_closed_ranges(ra: ModuleType, loaded: Loaded) -> None:
    before = ra.AstRangeExpr.cast(node(loaded, "..b"))
    assert (before.start(), before.op_token().text(), text(before.end())) == (None, "..", "b")
    inclusive = ra.AstRangeExpr.cast(node(loaded, "a..=b"))
    assert (text(inclusive.start()), inclusive.op_token().text(), text(inclusive.end())) == ("a", "..=", "b")
    after = ra.AstRangePat.cast(node(loaded, "10.."))
    assert (text(after.start()), after.op_token().text(), after.end()) == ("10", "..", None)
    bounded = ra.AstRangePat.cast(node(loaded, "3..=5"))
    assert (text(bounded.start()), bounded.op_token().text(), text(bounded.end())) == ("3", "..=", "5")


def test_written_raw_field_names_match_semantic_names(ra: ModuleType, loaded: Loaded) -> None:
    root = parse(loaded, "src/lib.rs").syntax()
    for cls, spellings in [
        (ra.AstRecordPatField, ["r#type", "r#type: renamed"]),
        (ra.AstRecordExprField, ["r#type", "r#type: a"]),
    ]:
        for spelling in spellings:
            field = next(
                value for n in root.descendants() if str(n.text()) == spelling and (value := cls.cast(n)) is not None
            )
            name = field.field_name()
            assert str(name.syntax().text()) == "r#type"
            assert name.as_name().as_str() == "type"


def test_native_doc_comments_and_string_contents(ra: ModuleType, loaded: Loaded) -> None:
    root = parse(loaded, "src/lib.rs").syntax()
    assert [d.text() for n in root.descendants() if (d := ra.AstDocComment.cast(n)) is not None] == [
        " line comment",
        " block comment ",
    ]
    strings = {
        token.text(): literal
        for e in root.descendants_with_tokens()
        if (token := e.as_Token()) is not None and (literal := ra.AstString.cast(token)) is not None
    }
    assert strings['r#"words "in quotes""#'].value() == 'words "in quotes"'
    assert strings[r'"a\nb"'].value() == "a\nb"
    with pytest.raises(ValueError):
        strings[r'"\q"'].value()
