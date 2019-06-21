#!/usr/bin/env bash
pipenv sync
pipenv run pyo3-pack build
pipenv run pyo3-pack develop
pipenv shell
