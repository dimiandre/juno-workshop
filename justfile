optimize:
    if [[ $(uname -m) =~ "arm64" ]]; then \
    docker run --rm -v "$(pwd)":/code \
        --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
        --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
        --platform linux/arm64 \
        cosmwasm/rust-optimizer-arm64:0.12.12; else \
    docker run --rm -v "$(pwd)":/code \
        --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
        --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
        --platform linux/amd64 \
        cosmwasm/rust-optimizer:0.12.12; fi

build:
    cargo build --target wasm32-unknown-unknown --release