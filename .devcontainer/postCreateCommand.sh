#!/usr/bin/env bash

set -euo pipefail

git config --global --add safe.directory /workspaces/vscode-remote-try-rust

rustup self update
cargo install --locked cargo-compete cargo-member
