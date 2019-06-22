#!/bin/bash

cd rust_ipv8_in_python && \
pipenv run pyo3-pack build && \
pipenv sync && \
cd ../py-ipv8/ && \
pipenv run ./run_all_tests_unix.sh
