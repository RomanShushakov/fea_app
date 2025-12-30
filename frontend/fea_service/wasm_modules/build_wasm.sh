#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
OUT="$ROOT/../wasm"

build_one () {
  local crate="$1"
  local bindgen_target="$2"

  echo "==> Building $crate"
  cargo build -p "$crate" --release --no-default-features --target wasm32-unknown-unknown

  echo "==> wasm-bindgen $crate ($bindgen_target)"
  wasm-bindgen \
    --target "$bindgen_target" \
    --out-name "$crate" \
    --out-dir "$OUT/$crate" \
    --no-typescript \
    "$ROOT/target/wasm32-unknown-unknown/release/$crate.wasm"
}

cd "$ROOT"

build_one actions_router web
build_one preprocessor web
build_one renderer web
build_one mesher web
build_one solver no-modules
