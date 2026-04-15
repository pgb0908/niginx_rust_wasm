FROM rust:1.81-bookworm AS wasm_builder

WORKDIR /workspace/wasm-filter
COPY v1/wasm-filter/Cargo.toml .
COPY v1/wasm-filter/src ./src
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
COPY v1/gateway/nginx.conf /opt/wasmx/nginx.conf
RUN mkdir -p /opt/wasmx/filters
COPY --from=wasm_builder /workspace/wasm-filter/target/wasm32-unknown-unknown/release/gateway_filter.wasm /opt/wasmx/filters/gateway_filter.wasm

EXPOSE 8080
CMD ["./nginx", "-g", "daemon off;"]
