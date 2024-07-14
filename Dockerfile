# syntax=docker/dockerfile:1

# AMD64
FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:x86_64-musl as builder-amd64

# ARM64
FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:aarch64-musl as builder-arm64

ARG TARGETARCH
FROM builder-$TARGETARCH as builder

RUN apt update && apt install -y protobuf-compiler

RUN adduser --disabled-password --disabled-login --gecos "" --no-create-home ul-api

RUN cargo init

# touch lib.rs as we combine both
RUN touch src/lib.rs

# touch benches as it's part of Cargo.toml
RUN mkdir benches
RUN touch benches/a_benchmark.rs

# copy cargo.*
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

# cache depencies
RUN mkdir .cargo
RUN cargo vendor > .cargo/config
RUN --mount=type=cache,id=cargo,target=$CARGO_HOME/registry \
    --mount=type=cache,id=git,target=$CARGO_HOME/.git \
    --mount=type=cache,id=target,target=./ul-api/target,sharing=locked \
    cargo build --target $CARGO_BUILD_TARGET --bin ul-api-app --release

# copy src
COPY src ./src
# copy benches
COPY benches ./benches

# copy config
COPY config ./config

# final build for release
RUN rm ./target/$CARGO_BUILD_TARGET/release/deps/*ul_api*
RUN cargo build --target $CARGO_BUILD_TARGET --bin ul-api-app --release

RUN musl-strip ./target/$CARGO_BUILD_TARGET/release/ul-api-app

RUN mv ./target/$CARGO_BUILD_TARGET/release/ul-api* /usr/local/bin
RUN mv ./config /etc/config

FROM scratch

ARG backtrace=0
ARG log_level=info

ENV RUST_BACKTRACE=${backtrace} \
    RUST_LOG=${log_level}

COPY --from=builder /usr/local/bin/ul-api* .
COPY --from=builder /etc/config ./config
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

USER ul-api:ul-api

EXPOSE 3000
EXPOSE 4000
ENTRYPOINT ["./ul-api-app"]
