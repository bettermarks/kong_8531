FROM rust:latest as builder

WORKDIR /stuff
ADD . .

RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /stuff/target/release/reflector /usr/local/bin

EXPOSE 8080
CMD ["reflector", "0.0.0.0:8080"]
