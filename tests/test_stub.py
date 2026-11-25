"""The stub `rust_analyzer.pyi` names what the module has, and nothing else."""

import ast
from pathlib import Path
from types import ModuleType

STUB = Path(__file__).parent.parent / "rust_analyzer.pyi"
# What every class has without a binding.
INHERITED = set(dir(object)) - {"__eq__", "__hash__", "__new__", "__repr__", "__str__"}


def stubbed() -> tuple[dict[str, set[str]], set[str]]:
    """The methods and properties of each class of the stub, and its functions."""
    classes: dict[str, set[str]] = {}
    functions: set[str] = set()
    for node in ast.parse(STUB.read_text()).body:
        if isinstance(node, ast.ClassDef):
            classes[node.name] = {member.name for member in node.body if isinstance(member, ast.FunctionDef)}
        if isinstance(node, ast.FunctionDef):
            functions.add(node.name)
    return classes, functions


def bound(cls: type) -> set[str]:
    """What the class binds itself: its own attributes, without what Python gives every class."""
    own = {name for name, value in vars(cls).items() if name not in INHERITED and value is not None}
    return own - {"__module__", "__doc__"}


def test_the_stub_has_the_classes_and_functions_of_the_module(ra: ModuleType) -> None:
    classes, functions = stubbed()
    public = {name: value for name, value in vars(ra).items() if not name.startswith("_")}
    assert {name for name, value in public.items() if isinstance(value, type)} == classes.keys()
    assert {name for name, value in public.items() if not isinstance(value, type)} == functions


def test_each_class_of_the_stub_has_the_bindings_of_its_class(ra: ModuleType) -> None:
    classes, _ = stubbed()
    different = {
        name: (sorted(members - bound(getattr(ra, name))), sorted(bound(getattr(ra, name)) - members))
        for name, members in classes.items()
        if name != "PanicException" and members != bound(getattr(ra, name))
    }
    assert different == {}
