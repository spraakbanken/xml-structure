from pathlib import Path

import pytest


@pytest.fixture()
def assets_path() -> Path:
    return Path(__file__).parent / "assets"
