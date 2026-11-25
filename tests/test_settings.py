"""Loading the crate `written` with the settings of a build, and reading what its conditions switch on."""

import shutil
import subprocess
from collections.abc import Callable
from pathlib import Path
from types import ModuleType
from typing import Any

import pytest
from helpers import Loaded, absolute, crate, ignore, parse

LIB = "src/lib.rs"
MANIFEST = '[package]\nname = "{name}"\nversion = "0.1.0"\nedition = "2024"\n'


def manifest(ra: ModuleType, root: Path) -> Any:
    return ra.ProjectManifest.from_manifest_file(ra.AbsPathBuf.try_from_str(str(root / "Cargo.toml")))


def config(ra: ModuleType) -> Any:
    cargo = ra.CargoConfig.default()
    cargo.sysroot = ra.RustLibSource.Discover()
    return cargo


def functions(ra: ModuleType, root: Path, configure: Callable[[Any], None]) -> set[str]:
    """The functions that the root module of the library declares, loaded with a changed configuration."""
    cargo = config(ra)
    configure(cargo)
    workspace = ra.ProjectWorkspace.load(manifest(ra, root), cargo, ignore)
    db, vfs, _ = ra.load_workspace(
        workspace, {}, ra.LoadCargoConfig(False, ra.ProcMacroServerChoice.Sysroot(), False, 1, 1)
    )
    [library] = [c for c in ra.Crate.all(db) if str(vfs.file_path(c.root_file(db)).as_path()) == str(root / LIB)]
    declared = [d.as_Function() for d in library.root_module(db).declarations(db)]
    return {f.name(db).as_str() for f in declared if f is not None}


ALWAYS = {"switched", "body"}


def test_a_manifest_is_taken_by_its_file(ra: ModuleType, written: Path) -> None:
    assert functions(ra, written, lambda _: None) == ALWAYS | {"usual"}
    with pytest.raises(RuntimeError, match="project root must point to"):
        ra.ProjectManifest.from_manifest_file(ra.AbsPathBuf.try_from_str(str(written / "notes.txt")))


def test_the_features_of_the_configuration_are_switched_on(ra: ModuleType, written: Path) -> None:
    def with_features(features: Any) -> set[str]:
        return functions(ra, written, lambda cargo: setattr(cargo, "features", features))

    assert with_features(ra.CargoFeatures.Selected(["extra"], False)) == ALWAYS | {"usual", "extra"}
    assert with_features(ra.CargoFeatures.Selected([], True)) == ALWAYS
    assert with_features(ra.CargoFeatures.Selected(["extra"], True)) == ALWAYS | {"extra"}
    assert with_features(ra.CargoFeatures.All()) == ALWAYS | {"usual", "extra"}
    # The features read back: given to another configuration they have the same effect.
    first = config(ra)
    first.features = ra.CargoFeatures.Selected(["extra"], True)
    assert with_features(first.features) == ALWAYS | {"extra"}


def test_the_target_of_the_configuration_is_read(ra: ModuleType, written: Path) -> None:
    cargo = config(ra)
    assert cargo.target is None
    cargo.target = "wasm32-unknown-unknown"
    assert cargo.target == "wasm32-unknown-unknown"
    assert functions(ra, written, lambda cargo: setattr(cargo, "target", "wasm32-unknown-unknown")) == ALWAYS | {
        "usual",
        "wasm",
    }


def diff(ra: ModuleType) -> Any:
    """Switches `special` and `flavor = "sweet"` on, and the feature `usual` off."""
    special = ra.CfgAtom.Flag(ra.Symbol.intern("special"))
    sweet = ra.CfgAtom.KeyValue(ra.Symbol.intern("flavor"), ra.Symbol.intern("sweet"))
    usual = ra.CfgAtom.KeyValue(ra.Symbol.intern("feature"), ra.Symbol.intern("usual"))
    return ra.CfgDiff.new([special, sweet], [usual])


def overrides(ra: ModuleType) -> Any:
    return ra.CfgOverrides(diff(ra), {})


def test_the_cfg_overrides_of_the_configuration_switch_options_on_and_off(ra: ModuleType, written: Path) -> None:
    def with_overrides(value: Any) -> set[str]:
        return functions(ra, written, lambda cargo: setattr(cargo, "cfg_overrides", value))

    # A feature is switched on after the overrides, so `usual` stays.
    assert with_overrides(overrides(ra)) == ALWAYS | {"usual", "special", "sweet"}
    first = config(ra)
    first.cfg_overrides = overrides(ra)
    assert with_overrides(first.cfg_overrides) == ALWAYS | {"usual", "special", "sweet"}


def test_a_diff_for_one_package_changes_only_that_package(ra: ModuleType, written: Path) -> None:
    special = ra.CfgDiff.new([ra.CfgAtom.Flag(ra.Symbol.intern("special"))], [])
    nothing = ra.CfgDiff.new([], [])

    def with_selective(name: str) -> set[str]:
        return functions(
            ra, written, lambda cargo: setattr(cargo, "cfg_overrides", ra.CfgOverrides(nothing, {name: special}))
        )

    assert with_selective("written") == ALWAYS | {"usual", "special"}
    assert with_selective("another") == ALWAYS | {"usual"}


def scripts(ra: ModuleType, root: Path, configure: Callable[[Any], None]) -> Any:
    cargo = config(ra)
    configure(cargo)
    workspace = ra.ProjectWorkspace.load(manifest(ra, root), cargo, ignore)
    return workspace.run_build_scripts(cargo, ignore)


def test_the_build_scripts_are_built_in_the_target_folder_of_the_configuration(
    ra: ModuleType, built: Path, tmp_path: Path
) -> None:
    def elsewhere(cargo: Any) -> None:
        cargo.target_dir_config = ra.TargetDirectoryConfig.Directory(str(tmp_path / "elsewhere"))

    # A copy that no other test has built in.
    alone = tmp_path / "built"
    shutil.copytree(built, alone, ignore=shutil.ignore_patterns("target"))
    assert scripts(ra, alone, elsewhere).error() is None
    assert (tmp_path / "elsewhere").is_dir() and not (alone / "target").exists()
    # The folder reads back: given to another configuration it has the same effect.
    first = config(ra)
    first.target_dir_config = ra.TargetDirectoryConfig.Directory(str(tmp_path / "again"))
    assert (
        scripts(ra, built, lambda cargo: setattr(cargo, "target_dir_config", first.target_dir_config)).error() is None
    )
    assert (tmp_path / "again").is_dir()


def test_the_build_scripts_run_with_the_extra_arguments_and_say_why_they_fail(
    ra: ModuleType, built: Path, tmp_path: Path
) -> None:
    def refused(cargo: Any) -> None:
        cargo.target_dir_config = ra.TargetDirectoryConfig.Directory(str(tmp_path / "target"))
        cargo.extra_args = ["--no-such-argument"]

    cargo = config(ra)
    assert cargo.extra_args == []
    cargo.extra_args = ["--offline"]
    assert cargo.extra_args == ["--offline"]
    assert "--no-such-argument" in scripts(ra, built, refused).error()


def test_a_workspace_says_why_cargo_could_not_resolve_its_dependencies(
    ra: ModuleType, written_loaded: Loaded, tmp_path: Path
) -> None:
    assert written_loaded.workspace.kind.as_Cargo().error is None
    (tmp_path / "src").mkdir()
    (tmp_path / LIB).write_text("")
    (tmp_path / "Cargo.toml").write_text(
        MANIFEST.format(name="alone") + '\n[dependencies]\nmissing = { path = "missing" }\n'
    )
    workspace = ra.ProjectWorkspace.load(manifest(ra, tmp_path), config(ra), ignore)
    assert "missing" in workspace.kind.as_Cargo().error


def test_a_workspace_lends_its_sysroot_which_says_whether_it_has_the_standard_library(
    ra: ModuleType, written: Path, written_loaded: Loaded, tmp_path: Path
) -> None:
    sysroot = written_loaded.workspace.sysroot
    assert not sysroot.is_rust_lib_src_empty() and sysroot.error() is None
    # A toolchain with its programs and without the source of the standard library.
    real = subprocess.run(["rustc", "--print", "sysroot"], check=True, capture_output=True, text=True, cwd=written)
    (tmp_path / "bin").symlink_to(Path(real.stdout.strip()) / "bin")
    cargo = config(ra)
    cargo.sysroot = ra.RustLibSource.Path(ra.AbsPathBuf.try_from_str(str(tmp_path)))
    without = ra.ProjectWorkspace.load(manifest(ra, written), cargo, ignore).sysroot
    assert without.is_rust_lib_src_empty()
    assert "can't find standard library sources" in without.error()


def test_a_package_lists_its_targets_with_their_name_root_and_kind(written_loaded: Loaded) -> None:
    cargo = written_loaded.workspace.kind.as_Cargo().cargo
    [package] = [cargo.index_Package(p) for p in cargo.packages() if cargo.index_Package(p).name == "written"]
    targets = [cargo.index_Target(target) for target in package.targets]
    assert sorted((t.name, str(t.root), t.kind.as_Lib(), t.kind.as_Bin()) for t in targets) == [
        ("written", str(written_loaded.root / "src/lib.rs"), False, False),  # a library that is no procedural macro
        ("written", str(written_loaded.root / "src/main.rs"), None, True),
    ]


# ---------------------------------------------------------------- conditions


def attributes(ra: ModuleType, loaded: Loaded) -> list[Any]:
    root = parse(loaded, LIB).syntax()
    return [attribute for n in root.descendants() if (attribute := ra.AstAttr.cast(n)) is not None]


def test_the_options_of_a_crate_check_the_condition_of_a_cfg(ra: ModuleType, written_loaded: Loaded) -> None:
    options = crate(ra, written_loaded, LIB).cfg(written_loaded.db)
    checked: dict[str, bool | None] = {}
    for attribute in attributes(ra, written_loaded):
        condition = attribute.meta().as_CfgMeta()
        if condition is not None:
            predicate = condition.cfg_predicate()
            checked[str(predicate.syntax().text())] = options.check(ra.CfgExpr.parse_from_ast(predicate))
    assert checked == {
        'feature = "usual"': True,
        'feature = "extra"': False,
        'target_arch = "wasm32"': False,
        "special": False,
        'flavor = "sweet"': False,
    }
    assert [a.meta().as_CfgMeta() is None for a in attributes(ra, written_loaded)][-2:] == [True, True]  # cfg_attr


def test_a_diff_changes_the_options_it_is_applied_to(ra: ModuleType, written_loaded: Loaded) -> None:
    library = crate(ra, written_loaded, LIB)
    conditions = {
        str(predicate.syntax().text()): ra.CfgExpr.parse_from_ast(predicate)
        for attribute in attributes(ra, written_loaded)
        if (condition := attribute.meta().as_CfgMeta()) is not None and (predicate := condition.cfg_predicate())
    }
    names = ['feature = "usual"', "special", 'flavor = "sweet"']
    options = library.cfg(written_loaded.db)
    options.apply_diff(diff(ra))
    assert [options.check(conditions[name]) for name in names] == [False, True, True]
    # The options of the crate are a copy.
    assert [library.cfg(written_loaded.db).check(conditions[name]) for name in names] == [True, False, False]


def first_written(meta: Any, _: Any) -> str:
    return str(meta.syntax().text())


def test_cfg_attr_is_expanded_with_the_options_given(ra: ModuleType, written_loaded: Loaded) -> None:
    library = crate(ra, written_loaded, LIB)
    db = written_loaded.db
    switched = attributes(ra, written_loaded)[-2:]
    seen: list[tuple[str, str]] = []

    def collect(meta: Any, attribute: Any) -> None:
        seen.append((str(meta.syntax().text()), str(attribute.syntax().text()).split("(")[0]))

    usual = library.cfg(db)
    assert ra.expand_cfg_attr(switched, usual, collect) is None
    assert seen == [("allow(dead_code, clippy::all)", "#[cfg_attr")]
    extra = library.cfg(db)
    extra.apply_diff(ra.CfgDiff.new([ra.CfgAtom.KeyValue(ra.Symbol.intern("feature"), ra.Symbol.intern("extra"))], []))
    seen.clear()
    ra.expand_cfg_attr(switched, extra, collect)
    assert [written for written, _ in seen] == ["allow(dead_code, clippy::all)", "inline"]
    # What the callback returns ends the expansion and is returned.
    assert ra.expand_cfg_attr(switched, extra, first_written) == "allow(dead_code, clippy::all)"


def test_a_raising_cfg_attr_callback_panics(ra: ModuleType, written_loaded: Loaded) -> None:
    def fail(_: Any, __: Any) -> None:
        raise KeyError("from the callback")

    options = crate(ra, written_loaded, LIB).cfg(written_loaded.db)
    with pytest.raises(ra.PanicException):
        ra.expand_cfg_attr(attributes(ra, written_loaded), options, fail)


def test_a_token_tree_is_parsed_as_paths_between_commas(ra: ModuleType, written_loaded: Loaded) -> None:
    library = crate(ra, written_loaded, LIB)
    db = written_loaded.db
    found: list[Any] = []

    def keep(meta: Any, _: Any) -> None:
        found.append(meta)

    ra.expand_cfg_attr(attributes(ra, written_loaded)[-2:], library.cfg(db), keep)
    [allow] = found
    paths = ra.parse_tt_as_comma_sep_paths(allow.as_TokenTreeMeta().token_tree(), library.edition(db))
    assert [str(path.syntax().text()) for path in paths] == ["dead_code", "clippy::all"]


def test_the_panic_of_a_binding_is_the_exception_of_the_module(ra: ModuleType, written: Path) -> None:
    def fail(_: str) -> None:
        raise KeyError("from progress")

    assert issubclass(ra.PanicException, BaseException) and not issubclass(ra.PanicException, Exception)
    with pytest.raises(ra.PanicException, match="the progress callback raised"):
        ra.ProjectWorkspace.load(manifest(ra, written), config(ra), fail)
    assert absolute(ra, written) is not None
