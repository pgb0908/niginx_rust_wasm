FROM rust:1.81-bookworm AS auth_builder

WORKDIR /workspace/auth-filter
COPY v3/plugins/auth-filter/Cargo.toml .
COPY v3/plugins/auth-filter/src ./src
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release --target wasm32-unknown-unknown

FROM rust:1.81-bookworm AS header_builder

WORKDIR /workspace/header-filter
COPY v3/plugins/header-filter/Cargo.toml .
COPY v3/plugins/header-filter/src ./src
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release --target wasm32-unknown-unknown

FROM rust:1.81-bookworm AS observe_builder

WORKDIR /workspace/observe-filter
COPY v3/plugins/observe-filter/Cargo.toml .
COPY v3/plugins/observe-filter/src ./src
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release --target wasm32-unknown-unknown

FROM ubuntu:22.04 AS gateway_runtime

ARG WASMX_RELEASE=prerelease-0.1.0
ARG WASMX_ARCHIVE=wasmx-prerelease-0.1.0-wasmtime-x86_64-ubuntu22.04.tar.gz

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl libgcc-s1 libssl3 zlib1g \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /opt
RUN curl -L -o /tmp/wasmx.tar.gz "https://github.com/Kong/ngx_wasm_module/releases/download/${WASMX_RELEASE}/${WASMX_ARCHIVE}" \
    && tar -xzf /tmp/wasmx.tar.gz -C /opt \
    && mv "/opt/${WASMX_ARCHIVE%.tar.gz}" /opt/wasmx \
    && rm /tmp/wasmx.tar.gz

WORKDIR /opt/wasmx
COPY v3/gateway/nginx.conf /opt/wasmx/nginx.conf
COPY v3/config /opt/wasmx/config
COPY v3/docs /opt/wasmx/docs
RUN mkdir -p /opt/wasmx/filters
COPY --from=auth_builder /workspace/auth-filter/target/wasm32-unknown-unknown/release/auth_filter.wasm /opt/wasmx/filters/auth_filter.wasm
COPY --from=header_builder /workspace/header-filter/target/wasm32-unknown-unknown/release/header_filter.wasm /opt/wasmx/filters/header_filter.wasm
COPY --from=observe_builder /workspace/observe-filter/target/wasm32-unknown-unknown/release/observe_filter.wasm /opt/wasmx/filters/observe_filter.wasm

EXPOSE 8080
CMD ["./nginx", "-g", "daemon off;"]
