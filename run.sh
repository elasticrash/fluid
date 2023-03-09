#/bin/bash
cargo run --bin fluid && \
cargo run --bin generator && \
cargo run --bin processor && \
cargo run --bin distributor