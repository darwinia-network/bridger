##
# Builder
##

FROM rust:1 as builder

ARG RUST_TOOLCHAIN=nightly-2021-03-01
ENV CARGO_TERM_COLOR=always

RUN rustup update \
    && rustup install ${RUST_TOOLCHAIN} \
    && rustup default ${RUST_TOOLCHAIN} \
    && rustup target add wasm32-unknown-unknown --toolchain ${RUST_TOOLCHAIN}

WORKDIR /src
COPY . .

RUN cargo build --release

##
# Final stage
##

FROM debian:stable-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
        openssl \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /src/target/release/bridger /usr/local/bin/

ENTRYPOINT [ "/usr/local/bin/bridger" ]
