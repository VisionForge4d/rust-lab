default: build

build:
    cargo build

fmt:
    cargo fmt

lint:
    cargo clippy --all-targets -- -D warnings

parhash dir="." min_kb=0:
    cargo run -p parhash -- {{dir}} --min-kb {{min_kb}}

lines dir=".":
    cargo run -p lines -- {{dir}}

assetsize dir=".":
    cargo run -p assetsize -- {{dir}}
