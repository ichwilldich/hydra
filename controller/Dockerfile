ARG TARGET=x86_64-unknown-linux-gnu
ARG RUSTFLAGS="-C target-feature=+crt-static"

FROM ghcr.io/profiidev/images/rust-gnu-builder:main AS toml-patcher

WORKDIR /
COPY ./Cargo.toml ./

# replace members with only controller
RUN sed -i 's/\(^members = \[\).*\(]\)$/\1\n"controller"\n\2/' Cargo.toml

FROM ghcr.io/profiidev/images/rust-gnu-builder:main AS backend-planner

ARG TARGET
ARG RUSTFLAGS

COPY controller/Cargo.toml controller/
COPY ./Cargo.lock ./
COPY --from=toml-patcher /Cargo.toml ./

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cargo chef prepare --recipe-path recipe.json --bin controller

FROM ghcr.io/profiidev/images/rust-gnu-builder:main AS backend-builder

ARG TARGET
ARG RUSTFLAGS

COPY --from=backend-planner /app/recipe.json .

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cargo chef cook --release --target $TARGET

COPY controller/Cargo.toml controller/
COPY controller/src controller/src
COPY ./Cargo.lock ./
COPY --from=toml-patcher /Cargo.toml ./

RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cd controller && cargo build --release --target $TARGET \
  && mv ../target/$TARGET/release/controller ../app

FROM gcr.io/distroless/static:latest

COPY --from=backend-builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

WORKDIR /app
COPY --from=backend-builder /app/app /usr/local/bin/

CMD ["app"]