dev:
 nix develop --command fish

clippy:
  cargo clippy

fmt:
  cargo fmt --all -v

watch:
  cargo watch -c -x 'build --all'

build:
  cargo b && rm ~/.cargo/__cache/target/debug/neorg-analyzer && cp target/debug/neorg-analyzer ~/.cargo/__cache/target/debug/neorg-analyzer
