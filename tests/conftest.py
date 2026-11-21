"""Build the wrapper with cargo and import it, so the tests run against the code beside them."""

import importlib
import shutil
import subprocess
import sys
from pathlib import Path
from types import ModuleType

import pytest

CRATE = Path(__file__).parent.parent
HARBOR = CRATE.parent / "tests" / "fixtures" / "harbor"


@pytest.fixture(scope="session")
def ra(tmp_path_factory: pytest.TempPathFactory) -> ModuleType:
    subprocess.run(["cargo", "build", "--quiet"], cwd=CRATE, check=True)
    built = next(
        p
        for p in (CRATE / "target" / "debug").iterdir()
        if p.stem == "librust_analyzer_wrapper" and p.suffix in (".dylib", ".so")
    )
    folder = tmp_path_factory.mktemp("module")
    shutil.copy(built, folder / "rust_analyzer_wrapper.so")
    sys.path.insert(0, str(folder))
    return importlib.import_module("rust_analyzer_wrapper")


@pytest.fixture(scope="session")
def harbor(tmp_path_factory: pytest.TempPathFactory) -> Path:
    """A copy of the harbor fixture, because loading runs cargo in it."""
    copy = tmp_path_factory.mktemp("harbor") / "harbor"
    shutil.copytree(HARBOR, copy, ignore=shutil.ignore_patterns("target"))
    return copy
