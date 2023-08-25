#!/bin/bash

set -euo pipefail

poetry run isort --check tests/requirements/test/*.py
poetry run black --check tests/requirements/test/*.py
poetry run flake8 tests/requirements/test/*.py
poetry run mypy tests/requirements/test/*.py

poetry run isort --check scripts/*.py
poetry run black --check scripts/*.py
poetry run flake8 scripts/*.py
poetry run mypy scripts/*.py
