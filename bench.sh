#!/usr/bin/bash
cargo build --release &&
cargo build &&
hyperfine --min-runs 20 \
  "node node.js" \
  "deno run --allow-read deno.js" \
  "target/debug/tomwords" \
  "target/release/tomwords"
