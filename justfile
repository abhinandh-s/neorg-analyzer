dev:
 nix develop --command fish

clippy:
  cargo clippy

fmt:
  cargo fmt --all -v

watch:
  cargo watch -c -x 'build --all'
