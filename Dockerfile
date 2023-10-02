FROM rust:1.72.1 as builder

WORKDIR /home/builder

RUN rustup target add aarch64-unknown-linux-musl

COPY . .

RUN cargo build --release --target aarch64-unknown-linux-musl

FROM scratch

WORKDIR /rustwi

COPY --from=builder /home/builder/target/aarch64-unknown-linux-musl/release/rustwi .

EXPOSE 8080

ENTRYPOINT [ "./rustwi" ] 
