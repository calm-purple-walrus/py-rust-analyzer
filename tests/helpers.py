"""Loading a workspace and finding its files, for the wrapper's tests."""

from dataclasses import dataclass
from pathlib import Path
from types import ModuleType
from typing import Any


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


def config(ra: ModuleType, *, with_tests: bool = False) -> Any:
    cargo = ra.CargoConfig.default()
    cargo.sysroot = ra.RustLibSource.Discover()
    cargo.set_test = with_tests
    cargo.all_targets = with_tests
    return cargo


def load(ra: ModuleType, root: Path, *, with_build_scripts: bool = False, with_tests: bool = False) -> Loaded:
    cargo = config(ra, with_tests=with_tests)
    workspace = ra.ProjectWorkspace.load(ra.ProjectManifest.discover_single(absolute(ra, root)), cargo, ignore)
    if with_build_scripts:
        workspace.set_build_scripts(workspace.run_build_scripts(cargo, ignore))
    db, vfs, client = ra.load_workspace(
        workspace, {}, ra.LoadCargoConfig(False, ra.ProcMacroServerChoice.Sysroot(), False, 1, 1)
    )
    return Loaded(workspace, db, vfs, client, ra.SemanticsRootDatabase.new(db), root)


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


def declared(ra: ModuleType, loaded: Loaded, variant: str, name: str, *, in_workspace: bool = True) -> Any:
    """The type or trait of that name, declared by a crate of the loaded workspace or, if not `in_workspace`, by any
    crate."""
    for c in ra.Crate.all(loaded.db):
        if in_workspace and not str(loaded.vfs.file_path(c.root_file(loaded.db)).as_path()).startswith(
            str(loaded.root)
        ):
            continue
        for module in c.modules(loaded.db):
            for definition in module.declarations(loaded.db):
                found = getattr(definition, f"as_{variant}")()
                if found is not None and found.name(loaded.db).as_str() == name:
                    return found
    raise LookupError(name)
