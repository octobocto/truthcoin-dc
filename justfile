default:
    @just --list

fmt:
    cargo fmt --all

build:
    cargo build

clippy:
    cargo clippy --all-targets --all-features

# Run integration tests. Pass a test name to run a single test:
# `just test-it deposit_withdraw_roundtrip`
test-it *args:
    #!/usr/bin/env bash
    set -euo pipefail
    if [ ! -f integrationtests.env ] && [ -z "${TRUTHCOIN_INTEGRATION_TEST_ENV:-}" ]; then
        echo "No integrationtests.env found. Copy integration_tests/example.env" >&2
        echo "to integrationtests.env, or point TRUTHCOIN_INTEGRATION_TEST_ENV" >&2
        echo "at an existing env file." >&2
        exit 1
    fi
    cargo run --example integration_tests -- {{ args }}
