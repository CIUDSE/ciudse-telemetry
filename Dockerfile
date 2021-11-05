FROM rust:1.55.0 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/telemetry-server /usr/local/bin/telemetry-server
CMD ["telemetry-server"]