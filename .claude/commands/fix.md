The build is failing in the current repository. Investigate the root cause of this bug.

# commands I executed

### Build all binaries
cargo build --all-features --package blg
cargo build --all-features --package mcp-backlog-server

## Test Minimal Features Job
cargo test --package backlog-api-client --no-default-features
cargo check --package blg --no-default-features

### Run tests
cargo test --all-features --all-targets

### Run clippy
cargo clippy --all-features --all-targets -- -D warnings

### Check formatting
cargo fmt --all -- --check

### Check documentation
cargo doc --all-features --no-deps
