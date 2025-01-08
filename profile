# Build with optimizations and suppress warnings
RUSTFLAGS="-C opt-level=3 -A warnings" cargo build --release

# Profile with perf: 2 options
#perf record -e cycles --call-graph dwarf target/release/mcts-rs
#perf record ./target/release/mcts-rs

# Analyze the profile: 2 options
#hotspot perf.data
#perf report
