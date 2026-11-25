"""Closures, `async` blocks and `macro_rules!` definitions in the syntax tree."""

from types import ModuleType
from typing import Any

import pytest
from helpers import Loaded, load, parse

LIB = """\
macro_rules! twice {
    ($value:expr) => {
        $value * 2
    };
}

pub fn runs() -> u8 {
    let double = |value: u8| twice!(value);
    let _later = async { 1 };
    let plain = { 2 };
    double(plain)
}
"""


@pytest.fixture(scope="module")
def nodes(ra: ModuleType, tmp_path_factory: pytest.TempPathFactory) -> list[Any]:
    root = tmp_path_factory.mktemp("closures")
    (root / "Cargo.toml").write_text(
        '[package]\nname = "closures"\nversion = "0.1.0"\nedition = "2021"\n\n[workspace]\n'
    )
    (root / "src").mkdir()
    (root / "src" / "lib.rs").write_text(LIB)
    loaded: Loaded = load(ra, root)
    return list(parse(loaded, "src/lib.rs").syntax().descendants())


@pytest.mark.parametrize(("name", "kind"), [("AstClosureExpr", "CLOSURE_EXPR"), ("AstMacroRules", "MACRO_RULES")])
def test_can_cast_accepts_exactly_its_kind(ra: ModuleType, nodes: list[Any], name: str, kind: str) -> None:
    accepted = [getattr(ra, name).can_cast(node.kind()) for node in nodes]
    assert accepted == [repr(node.kind()) == kind for node in nodes]
    assert accepted.count(True) == 1


def test_a_block_gives_its_async_token(ra: ModuleType, nodes: list[Any]) -> None:
    blocks = [block for node in nodes if (expr := ra.AstExpr.cast(node)) and (block := expr.as_BlockExpr())]
    tokens = {str(block.syntax().text()): block.async_token() for block in blocks}
    assert {text: None if token is None else token.text() for text, token in tokens.items()} == {
        "{\n    let double = |value: u8| twice!(value);\n    let _later = async { 1 };\n    let plain = { 2 };\n"
        "    double(plain)\n}": None,
        "async { 1 }": "async",
        "{ 2 }": None,
    }
