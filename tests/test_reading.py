"""Reading harbor's code from Python: its workspace, its syntax and its types, with the standard library loaded."""

import shutil
from collections.abc import Callable, Iterator
from dataclasses import dataclass
from pathlib import Path
from types import ModuleType
from typing import Any

import pytest

PACKAGES = {
    "harbor-domain", "harbor-ports", "harbor-scratch", "harbor-storage", "harbor-testing", "harbor-wire",
    "mutants", "pixelkit", "spinloop",
}  # fmt: skip


@dataclass
class Loaded:
    workspace: Any
    db: Any
    vfs: Any
    client: Any
    semantics: Any
    root: Path


def absolute(ra: ModuleType, path: Path) -> Any:
    return ra.AbsPathBuf.try_from_str(str(path)).deref()


def ignore(_: str) -> None:
    pass


def config(ra: ModuleType) -> Any:
    cargo = ra.CargoConfig.default()
    cargo.sysroot = ra.RustLibSource.Discover()
    return cargo


def load(ra: ModuleType, root: Path, *, with_build_scripts: bool = False) -> Loaded:
    workspace = ra.ProjectWorkspace.load(ra.ProjectManifest.discover_single(absolute(ra, root)), config(ra), ignore)
    if with_build_scripts:
        workspace.set_build_scripts(workspace.run_build_scripts(config(ra), ignore))
    db, vfs, client = ra.load_workspace(
        workspace, {}, ra.LoadCargoConfig(False, ra.ProcMacroServerChoice.Sysroot(), False, 1, 1)
    )
    return Loaded(workspace, db, vfs, client, ra.SemanticsRootDatabase.new(db), root)


@pytest.fixture(scope="module")
def loaded(ra: ModuleType, harbor: Path) -> Loaded:
    return load(ra, harbor)


def file_id(loaded: Loaded, relative: str) -> Any:
    wanted = str(loaded.root / relative)
    return next(
        file for file, path in loaded.vfs.iter() if path.as_path() is not None and str(path.as_path()) == wanted
    )


def parse(loaded: Loaded, relative: str) -> Any:
    return loaded.semantics.deref().parse_guess_edition(file_id(loaded, relative))


def crate(ra: ModuleType, loaded: Loaded, root_file: str, sysroot: str | None = None) -> Any:
    """The crate whose root is that file of the workspace or, if `sysroot` is given, that file of the sysroot."""
    if sysroot is not None:
        return next(
            c
            for c in ra.Crate.all(loaded.db)
            if str(loaded.vfs.file_path(c.root_file(loaded.db)).as_path()).endswith(f"/{sysroot}")
        )
    wanted = str(loaded.root / root_file)
    return next(
        c for c in ra.Crate.all(loaded.db) if str(loaded.vfs.file_path(c.root_file(loaded.db)).as_path()) == wanted
    )


def functions(ra: ModuleType, loaded: Loaded, relative: str) -> Iterator[tuple[Any, Any]]:
    """Each function of a file that the build compiles, with its syntax node."""
    for node in parse(loaded, relative).syntax().descendants():
        syntax = ra.AstFn.cast(node)
        if syntax is not None and (function := loaded.semantics.deref().to_def(syntax)) is not None:
            yield node, function


def function(ra: ModuleType, loaded: Loaded, relative: str, name: str) -> tuple[Any, Any]:
    return next(f for f in functions(ra, loaded, relative) if f[1].name(loaded.db).as_str() == name)


def declared(ra: ModuleType, loaded: Loaded, variant: str, name: str, *, of_harbor: bool = True) -> Any:
    """The type or trait of that name, declared by a crate of harbor or, if not `of_harbor`, by any crate."""
    for c in ra.Crate.all(loaded.db):
        if of_harbor and not str(loaded.vfs.file_path(c.root_file(loaded.db)).as_path()).startswith(str(loaded.root)):
            continue
        for module in c.modules(loaded.db):
            for definition in module.declarations(loaded.db):
                found = getattr(definition, f"as_{variant}")()
                if found is not None and found.name(loaded.db).as_str() == name:
                    return found
    raise LookupError(name)


def shown(ra: ModuleType, loaded: Loaded, ty: Any, root_file: str = "crates/domain/src/lib.rs") -> str:
    target = crate(ra, loaded, root_file).to_display_target(loaded.db)
    return str(ty.display(loaded.db, target))


def returned(ra: ModuleType, loaded: Loaded, name: str) -> Any:
    return function(ra, loaded, "crates/domain/src/cargo.rs", name)[1].ret_type(loaded.db)


def panics(call: Callable[[], object], message: str) -> None:
    with pytest.raises(BaseException, match=message) as raised:
        call()
    assert type(raised.value).__name__ == "PanicException"


# The workspace


def test_the_sysroot_setting_reads_back(ra: ModuleType) -> None:
    cargo = ra.CargoConfig.default()
    assert cargo.sysroot is None
    cargo.sysroot = ra.RustLibSource.Discover()
    assert cargo.sysroot.as_Discover()
    cargo.sysroot = ra.RustLibSource.Path(ra.AbsPathBuf.try_from_str("/sysroot"))
    assert not cargo.sysroot.as_Discover()


def test_the_proc_macro_server_of_the_sysroot_starts(ra: ModuleType, loaded: Loaded) -> None:
    assert isinstance(loaded.client, ra.ProcMacroClient)


def test_the_proc_macro_server_choice_reads_back_and_takes_effect(ra: ModuleType, harbor: Path) -> None:
    def client(choice: Any) -> Any:
        workspace = ra.ProjectWorkspace.load(
            ra.ProjectManifest.discover_single(absolute(ra, harbor)), config(ra), ignore
        )
        return ra.load_workspace(workspace, {}, choice)[2]

    off = ra.LoadCargoConfig(False, ra.ProcMacroServerChoice.None_(), False, 1, 1)
    on = ra.LoadCargoConfig(False, ra.ProcMacroServerChoice.Sysroot(), False, 1, 1)
    assert client(off) is None
    off.with_proc_macro_server = on.with_proc_macro_server
    assert isinstance(client(off), ra.ProcMacroClient)


def test_the_cargo_workspace_names_every_package(loaded: Loaded) -> None:
    cargo = loaded.workspace.kind.as_Cargo().cargo
    assert {cargo.index_Package(package).name for package in cargo.packages()} == PACKAGES


def test_a_crate_knows_its_root_file(ra: ModuleType, loaded: Loaded) -> None:
    roots = {str(loaded.vfs.file_path(c.root_file(loaded.db)).as_path()) for c in ra.Crate.all(loaded.db)}
    assert str(loaded.root / "tools/scratch/src/lib.rs") in roots


def test_build_scripts_report_their_progress(ra: ModuleType, harbor: Path) -> None:
    workspace = ra.ProjectWorkspace.load(ra.ProjectManifest.discover_single(absolute(ra, harbor)), config(ra), ignore)
    messages: list[str] = []
    workspace.run_build_scripts(config(ra), messages.append)
    assert "build script harbor-storage run" in messages


def test_a_raising_build_script_progress_panics(ra: ModuleType, harbor: Path) -> None:
    def fail(_: str) -> None:
        raise KeyError("from progress")

    workspace = ra.ProjectWorkspace.load(ra.ProjectManifest.discover_single(absolute(ra, harbor)), config(ra), ignore)
    panics(lambda: workspace.run_build_scripts(config(ra), fail), "the progress callback raised")


@pytest.fixture(scope="module")
def built(ra: ModuleType, tmp_path_factory: pytest.TempPathFactory) -> Path:
    """A crate whose build script switches on the cfg `built`."""
    copy = tmp_path_factory.mktemp("built") / "built"
    shutil.copytree(Path(__file__).parent / "fixtures" / "built", copy)
    return copy


def test_build_scripts_that_are_set_switch_on_their_cfg(ra: ModuleType, built: Path) -> None:
    def types(with_build_scripts: bool) -> set[str]:
        crate = load(ra, built, with_build_scripts=with_build_scripts)
        found = [
            d.as_Adt() for c in ra.Crate.all(crate.db) for m in c.modules(crate.db) for d in m.declarations(crate.db)
        ]
        return {adt.name(crate.db).as_str() for adt in found if adt is not None}

    assert "Built" not in types(with_build_scripts=False)
    assert {"Always", "Built"} <= types(with_build_scripts=True)


# The syntax


def test_the_vfs_lends_the_path_of_every_rust_file(loaded: Loaded) -> None:
    paths = {str(path.as_path()) for _, path in loaded.vfs.iter() if path.as_path() is not None}
    files = {str(path) for path in loaded.root.rglob("*.rs") if "target" not in path.relative_to(loaded.root).parts}
    assert files <= paths


def test_parsing_gives_the_text_of_the_file(loaded: Loaded) -> None:
    node = parse(loaded, "tools/scratch/src/lib.rs").syntax()
    assert str(node.text()) == (loaded.root / "tools/scratch/src/lib.rs").read_text()


def test_a_file_lists_its_items(loaded: Loaded) -> None:
    assert len(list(parse(loaded, "crates/adapters/storage/src/lib.rs").items())) == 8


def test_every_function_of_a_file_is_found(ra: ModuleType, loaded: Loaded) -> None:
    names = [f.name(loaded.db).as_str() for _, f in functions(ra, loaded, "tools/scratch/src/lib.rs")]
    assert names == ["ledger", "grade", "tally"]


def test_whitespace_stands_before_a_function(ra: ModuleType, loaded: Loaded) -> None:
    node, _ = function(ra, loaded, "tools/scratch/src/lib.rs", "ledger")
    assert node.prev_sibling_or_token().into_token().kind() == ra.SyntaxKind.WHITESPACE()
    assert parse(loaded, "tools/scratch/src/lib.rs").syntax().prev_sibling_or_token() is None


def test_token_kinds_differ_where_the_tokens_differ(ra: ModuleType, loaded: Loaded) -> None:
    node, _ = function(ra, loaded, "tools/scratch/src/lib.rs", "ledger")
    before = [p for n in node.descendants() if (p := n.prev_sibling_or_token()) is not None]
    tokens = [token for p in before if (token := p.into_token()) is not None]
    assert any(p.into_token() is None for p in before)  # some nodes follow a node
    kinds = [token.kind() for token in tokens]
    assert any(kind == ra.SyntaxKind.WHITESPACE() for kind in kinds)
    assert any(kind != ra.SyntaxKind.WHITESPACE() for kind in kinds)  # `(` before the parameter


def test_a_function_starts_on_the_line_of_its_attribute(ra: ModuleType, loaded: Loaded) -> None:
    node, _ = function(ra, loaded, "tools/scratch/src/lib.rs", "ledger")
    lines = ra.LineIndex.new((loaded.root / "tools/scratch/src/lib.rs").read_text())
    start = node.text_range().start()
    assert lines.line_col(start).line == 25  # line 26, counted from 0
    assert lines.line_col(start.sub_TextSize(ra.TextSize.from_u32(1))).line == 24
    assert lines.line_col(ra.TextSize.from_u32(0)).line == 0


def test_nodes_are_equal_and_hash_alike_when_they_are_the_same_node(loaded: Loaded) -> None:
    first = list(parse(loaded, "tools/scratch/src/lib.rs").syntax().descendants())
    second = list(parse(loaded, "tools/scratch/src/lib.rs").syntax().descendants())
    assert first[5] == second[5]
    assert first[5] != second[6]
    assert len({first[5], second[5], second[6]}) == 2


def test_kinds_hash_alike_when_they_are_equal(ra: ModuleType) -> None:
    assert len({ra.SyntaxKind.WHITESPACE(), ra.SyntaxKind.WHITESPACE()}) == 1


# The types


def test_return_types_display_as_written(ra: ModuleType, loaded: Loaded) -> None:
    assert {
        name: shown(ra, loaded, returned(ra, loaded, name)) for name in ("new", "label", "lines_mut", "parse_weight")
    } == {
        "new": "Result<Cargo, DomainError>",
        "label": "&str",
        "lines_mut": "&mut Vec<Line, Global>",
        "parse_weight": "Result<Weight, String>",
    }


def test_walking_a_type_reaches_every_type_inside(ra: ModuleType, loaded: Loaded) -> None:
    def inside(name: str) -> list[str]:
        found: list[Any] = []
        returned(ra, loaded, name).walk(loaded.db, found.append)
        return [t.as_adt().name(loaded.db).as_str() for t in found if t.as_adt() is not None]

    assert inside("new") == ["Result", "Cargo", "DomainError"]
    assert inside("lines_mut") == ["Vec", "Line", "Global"]


def test_a_raising_walk_panics_and_leaves_the_database_usable(ra: ModuleType, loaded: Loaded) -> None:
    def fail(_: Any) -> None:
        raise KeyError("from walk")

    new = returned(ra, loaded, "new")
    panics(lambda: new.walk(loaded.db, fail), "the walk callback raised")
    assert shown(ra, loaded, new) == "Result<Cargo, DomainError>"


def test_a_reference_says_what_it_refers_to(ra: ModuleType, loaded: Loaded) -> None:
    def reference(name: str) -> tuple[str, bool] | None:
        found = returned(ra, loaded, name).as_reference()
        return None if found is None else (shown(ra, loaded, found[0]), found[1].is_mut())

    assert reference("lines_mut") == ("Vec<Line, Global>", True)
    assert reference("label") == ("str", False)
    assert reference("new") is None


def test_only_types_with_an_implementation_implement_a_trait(ra: ModuleType, loaded: Loaded) -> None:
    store = declared(ra, loaded, "Trait", "Store")
    memory = function(ra, loaded, "crates/adapters/storage/src/memory.rs", "new")[1].ret_type(loaded.db)
    keeper = function(ra, loaded, "crates/adapters/storage/src/archive.rs", "new")[1].ret_type(loaded.db)
    assert memory.impls_trait(loaded.db, store, [])
    assert not keeper.impls_trait(loaded.db, store, [])  # only `Keeper<Cargo>` implements `Store`


def test_a_trait_with_a_parameter_is_implemented_only_for_that_argument(ra: ModuleType, loaded: Loaded) -> None:
    # Types given together must come from the same place, here from walking one type; rust-analyzer panics
    # otherwise ("failed to unify type owners").
    conversion = declared(ra, loaded, "Trait", "From", of_harbor=False)
    inside: list[Any] = []
    returned(ra, loaded, "new").walk(loaded.db, inside.append)
    named = {t.as_adt().name(loaded.db).as_str(): t for t in inside if t.as_adt() is not None}
    cargo, error = named["Cargo"], named["DomainError"]
    assert cargo.impls_trait(loaded.db, conversion, [cargo])  # core's `impl<T> From<T> for T`
    assert not cargo.impls_trait(loaded.db, conversion, [error])


def test_a_generic_type_names_its_type_parameters(ra: ModuleType, loaded: Loaded) -> None:
    def parameters(name: str) -> list[str]:
        generic = ra.GenericDef.from_Adt(declared(ra, loaded, "Adt", name))
        found = [p.as_type_param(loaded.db) for p in generic.type_or_const_params(loaded.db)]
        return [p.name(loaded.db).as_str() for p in found if p is not None]

    assert parameters("Stack") == ["T", "U"]
    assert parameters("Keeper") == ["T"]


def test_a_type_definition_gives_its_type(ra: ModuleType, loaded: Loaded) -> None:
    assert shown(ra, loaded, declared(ra, loaded, "Adt", "Cargo").ty(loaded.db)) == "Cargo"
    assert shown(ra, loaded, declared(ra, loaded, "Adt", "Keeper").ty(loaded.db)) == "Keeper<T>"


# Lint attributes


def test_a_lint_attribute_is_read_with_its_lint(ra: ModuleType, loaded: Loaded) -> None:
    node, _ = function(ra, loaded, "tools/scratch/src/lib.rs", "ledger")
    scratch = crate(ra, loaded, "tools/scratch/src/lib.rs")
    file = file_id(loaded, "tools/scratch/src/lib.rs")
    found = [
        (a.as_Allow(), lint) for a, lint in loaded.semantics.lint_attrs(file, scratch, ra.AstAnyHasAttrs.cast(node))
    ]
    assert found == [(True, "clippy::too_many_lines")]


def test_lint_attributes_follow_the_cfg_of_the_crate_given(ra: ModuleType, built: Path) -> None:
    crate_of_built = load(ra, built, with_build_scripts=True)
    node, _ = function(ra, crate_of_built, "src/lib.rs", "flagged")
    file = file_id(crate_of_built, "src/lib.rs")
    item = ra.AstAnyHasAttrs.cast(node)
    other = crate(ra, crate_of_built, "src/lib.rs", sysroot="library/core/src/lib.rs")

    def attributes(krate: Any) -> list[tuple[bool, str]]:
        return [(a.as_Allow(), lint) for a, lint in crate_of_built.semantics.lint_attrs(file, krate, item)]

    assert attributes(crate(ra, crate_of_built, "src/lib.rs")) == [(True, "dead_code"), (False, "unused_variables")]
    assert attributes(other) == [(False, "unused_variables")]  # `built` is set only for the crate of `built`


# Where things are written


def test_a_type_definition_knows_where_it_is_written(ra: ModuleType, loaded: Loaded) -> None:
    source = declared(ra, loaded, "Adt", "Cargo").source(loaded.db)
    text = (loaded.root / "crates/domain/src/cargo.rs").read_text()

    def text_of(file: Any) -> str:
        return str(loaded.semantics.deref().parse_guess_edition(file).syntax().text())

    assert str(source.value.syntax().text()).startswith("pub struct Cargo {")
    assert text_of(source.file_id.file_id().file_id(loaded.db)) == text
    placed = ra.InFileTextRange.new(source.file_id, source.value.syntax().text_range())
    assert text_of(placed.file_id.file_id().file_id(loaded.db)) == text
    assert ra.LineIndex.new(text).line_col(placed.value.start()).line == 6  # line 7, counted from 0
    original = placed.original_node_file_range_rooted(loaded.db)
    assert ra.LineIndex.new(text).line_col(original.range.start()).line == 6
    assert text_of(original.file_id.file_id(loaded.db)) == text


def test_a_function_knows_where_it_is_written(ra: ModuleType, loaded: Loaded) -> None:
    _, ledger = function(ra, loaded, "tools/scratch/src/lib.rs", "ledger")
    source = ledger.source(loaded.db)
    assert loaded.semantics.deref().to_def(source.value).name(loaded.db).as_str() == "ledger"
    file = source.file_id.file_id().file_id(loaded.db)
    assert (
        str(loaded.semantics.deref().parse_guess_edition(file).syntax().text())
        == (loaded.root / "tools/scratch/src/lib.rs").read_text()
    )


# More kinds of value


def test_a_package_lends_its_dependencies(loaded: Loaded) -> None:
    cargo = loaded.workspace.kind.as_Cargo().cargo
    storage = next(cargo.index_Package(p) for p in cargo.packages() if cargo.index_Package(p).name == "harbor-storage")
    found = {d.name: (repr(d.kind), cargo.index_Package(d.pkg).name) for d in storage.dependencies}
    assert found == {  # as in harbor's crates/adapters/storage/Cargo.toml
        "harbor_domain": ("Normal", "harbor-domain"),
        "harbor_ports": ("Normal", "harbor-ports"),
        "spinloop": ("Normal", "spinloop"),
        "harbor_testing": ("Dev", "harbor-testing"),
        "harbor_scratch": ("Build", "harbor-scratch"),
    }


def test_extra_environment_reads_back(ra: ModuleType) -> None:
    cargo = ra.CargoConfig.default()
    cargo.extra_env = {"SET": "1", "UNSET": None}
    assert cargo.extra_env == {"SET": "1", "UNSET": None}


def test_a_copy_equals_its_original(ra: ModuleType, loaded: Loaded) -> None:
    node = parse(loaded, "tools/scratch/src/lib.rs").syntax()
    assert node.clone() == node
    assert shown(ra, loaded, returned(ra, loaded, "new").clone()) == "Result<Cargo, DomainError>"
    cargo = loaded.workspace.kind.as_Cargo().cargo.clone()
    assert {cargo.index_Package(package).name for package in cargo.packages()} == PACKAGES


@pytest.fixture(scope="module")
def built_loaded(ra: ModuleType, built: Path) -> Loaded:
    return load(ra, built)


def test_a_function_pointer_lends_the_types_of_its_parameters(ra: ModuleType, built_loaded: Loaded) -> None:
    hook = declared(ra, built_loaded, "Adt", "Hook").as_struct()
    [run] = hook.fields(built_loaded.db)
    callable_ = run.ty(built_loaded.db).as_callable(built_loaded.db)
    [parameter] = callable_.params()
    assert shown(ra, built_loaded, parameter.ty(), "src/lib.rs") == "u32"
    assert shown(ra, built_loaded, callable_.return_type(), "src/lib.rs") == "String"


def test_a_macro_maps_what_it_writes_back_to_where_it_is_written(ra: ModuleType, built_loaded: Loaded) -> None:
    root = parse(built_loaded, "src/lib.rs").syntax()
    [call] = [m for n in root.descendants() if (m := ra.AstMacroCall.cast(n)) is not None]
    expansion = built_loaded.semantics.deref().expand_macro_call(call)
    assert str(expansion.value.text()) == "#[allow(unused)]pubstructMade;"  # an expansion has no whitespace
    assert expansion.file_id.file_id() is None  # a file the macro wrote, not a real one
    written = ra.InMacroFileSyntaxToken.new(expansion.file_id.as_MacroFile(), expansion.value.first_token())
    above = written.upmap_once(built_loaded.db)
    text = (built_loaded.root / "src/lib.rs").read_text()
    real = above.file_id.file_id().file_id(built_loaded.db)
    assert str(built_loaded.semantics.deref().parse_guess_edition(real).syntax().text()) == text
    lines = ra.LineIndex.new(text)
    assert [lines.line_col(r.start()).line for r in above.value] == [15]  # `#[allow(unused)]` on line 16


def test_a_type_is_shown_in_the_edition_of_the_crate_given(ra: ModuleType, built_loaded: Loaded) -> None:
    ty = declared(ra, built_loaded, "Adt", "gen").ty(built_loaded.db)
    shown_by = {
        "built": crate(ra, built_loaded, "src/lib.rs"),  # edition 2021
        "core": crate(ra, built_loaded, "src/lib.rs", sysroot="library/core/src/lib.rs"),  # edition 2024
    }
    assert {
        name: str(ty.display(built_loaded.db, c.to_display_target(built_loaded.db))) for name, c in shown_by.items()
    } == {
        "built": "gen",
        "core": "r#gen",  # `gen` is a keyword from edition 2024 on
    }


def test_a_range_a_macro_wrote_maps_to_the_call(ra: ModuleType, built_loaded: Loaded) -> None:
    root = parse(built_loaded, "src/lib.rs").syntax()
    [call] = [m for n in root.descendants() if (m := ra.AstMacroCall.cast(n)) is not None]
    expansion = built_loaded.semantics.deref().expand_macro_call(call)
    original = ra.InFileTextRange.new(expansion.file_id, expansion.value.text_range()).original_node_file_range_rooted(
        built_loaded.db
    )
    text = (built_loaded.root / "src/lib.rs").read_text()
    real = original.file_id.file_id(built_loaded.db)
    assert str(built_loaded.semantics.deref().parse_guess_edition(real).syntax().text()) == text
    # The struct comes from the macro's definition, not from the call's arguments, so the range falls back to
    # the call: `make!();` on line 21.
    assert ra.LineIndex.new(text).line_col(original.range.start()).line == 20
