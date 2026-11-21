"""Paths, loading harbor, and walking its declarations from Python."""

from pathlib import Path
from types import ModuleType
from typing import Any

import pytest


def absolute(ra: ModuleType, path: Path | str) -> Any:
    return ra.AbsPathBuf.try_from_str(str(path)).deref()


# The structs and enums that harbor's files declare.
STRUCTS = {
    "Bin",
    "Cargo",
    "CargoAttrs",
    "Keeper",
    "Line",
    "LineAttrs",
    "MemoryStore",
    "Plain",
    "Port",
    "Report",
    "RouteAttrs",
    "ShipCommand",
    "Stack",
    "StorageError",
    "Weight",
    "WeightAttrs",
    "WireCargo",
    "WireError",
    "WireLine",
}
ENUMS = {"ApplicationError", "DomainError", "Mode", "ModeAttrs", "Signal", "WireMode"}


def ignore(_: str) -> None:
    pass


@pytest.fixture(scope="module")
def loaded(ra: ModuleType, harbor: Path) -> tuple[Any, Any, Any]:
    workspace = ra.ProjectWorkspace.load(
        ra.ProjectManifest.discover_single(absolute(ra, harbor)), ra.CargoConfig.default(), ignore
    )
    return ra.load_workspace(workspace, {}, ra.LoadCargoConfig(False, ra.ProcMacroServerChoice.Sysroot(), False, 1, 1))


@pytest.fixture(scope="module")
def adts(ra: ModuleType, loaded: tuple[Any, Any, Any]) -> list[Any]:
    db = loaded[0]
    modules = [module for crate in ra.Crate.all(db) for module in crate.modules(db)]
    return [d.as_Adt() for module in modules for d in module.declarations(db) if d.as_Adt() is not None]


def test_an_absolute_path_reads_back(ra: ModuleType) -> None:
    assert str(absolute(ra, "/tmp/x")) == "/tmp/x"


def test_a_relative_path_is_refused(ra: ModuleType) -> None:
    with pytest.raises(RuntimeError, match="^relative$"):  # rust-analyzer's error is the path it refused
        ra.AbsPathBuf.try_from_str("relative")


def test_discover_single_refuses_a_folder_without_a_manifest(ra: ModuleType, tmp_path: Path) -> None:
    with pytest.raises(RuntimeError, match="no projects"):
        ra.ProjectManifest.discover_single(absolute(ra, tmp_path))


def test_loading_reports_progress(ra: ModuleType, harbor: Path) -> None:
    messages: list[str] = []
    ra.ProjectWorkspace.load(
        ra.ProjectManifest.discover_single(absolute(ra, harbor)), ra.CargoConfig.default(), messages.append
    )
    assert "querying project metadata" in messages


def test_loading_panics_when_progress_raises(ra: ModuleType, harbor: Path) -> None:
    def fail(_: str) -> None:
        raise KeyError("from progress")

    manifest = ra.ProjectManifest.discover_single(absolute(ra, harbor))
    with pytest.raises(BaseException, match="the progress callback raised") as raised:
        ra.ProjectWorkspace.load(manifest, ra.CargoConfig.default(), fail)
    assert type(raised.value).__name__ == "PanicException"


def test_a_load_config_is_made_with_every_field(ra: ModuleType) -> None:
    config = ra.LoadCargoConfig(True, ra.ProcMacroServerChoice.Sysroot(), False, 2, 3)
    assert (config.load_out_dirs_from_check, config.prefill_caches) == (True, False)
    assert (config.num_worker_threads, config.proc_macro_processes) == (2, 3)


@pytest.mark.parametrize(
    ("field", "value"),
    [
        ("load_out_dirs_from_check", True),
        ("prefill_caches", True),
        ("num_worker_threads", 3),
        ("proc_macro_processes", 2),
    ],
)
def test_a_load_config_field_reads_back(ra: ModuleType, field: str, value: object) -> None:
    config = ra.LoadCargoConfig(False, ra.ProcMacroServerChoice.Sysroot(), False, 1, 1)
    setattr(config, field, value)
    assert getattr(config, field) == value


def test_loading_gives_a_database(ra: ModuleType, loaded: tuple[Any, Any, Any]) -> None:
    assert isinstance(loaded[0], ra.RootDatabase)
    assert isinstance(loaded[1], ra.Vfs)


def test_the_declarations_hold_every_struct_and_enum_of_the_source(
    loaded: tuple[Any, Any, Any], adts: list[Any]
) -> None:
    db = loaded[0]
    assert {a.name(db).as_str() for a in adts if a.as_struct() is not None} == STRUCTS
    assert {a.name(db).as_str() for a in adts if a.as_enum() is not None} == ENUMS
    assert all((a.as_struct() is None) != (a.as_enum() is None) for a in adts)


def test_the_module_of_a_type_declares_it(loaded: tuple[Any, Any, Any], adts: list[Any]) -> None:
    db = loaded[0]
    assert len(adts) == len(STRUCTS) + len(ENUMS)
    for adt in adts:
        names = {d.as_Adt().name(db).as_str() for d in adt.module(db).declarations(db) if d.as_Adt() is not None}
        assert adt.name(db).as_str() in names
