FROM rust:1.72.1 as builder
WORKDIR /builder
RUN rustup target add aarch64-unknown-linux-musl
COPY . .
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM gcr.io/distroless/static:latest
WORKDIR /rustwi
COPY --from=builder /builder/target/aarch64-unknown-linux-musl/release/rustwi .
COPY ./.env .
EXPOSE 8080
ENTRYPOINT [ "./rustwi" ]
