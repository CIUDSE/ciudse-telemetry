FROM rust:1.55.0 as telemetry-server-builder
WORKDIR /usr/src/telemetry-server
COPY ./telemetry-server .
RUN cargo install --path .

FROM node:14 as telemetry-frontend-builder
WORKDIR /usr/src/telemetry-frontend
COPY ./telemetry-frontend .
RUN npm ci
RUN npm run build

FROM debian:buster-slim
COPY --from=telemetry-server-builder /usr/local/cargo/bin/telemetry-server /usr/local/bin/telemetry-server
COPY --from=telemetry-frontend-builder /usr/src/telemetry-frontend/dist /static
CMD ["telemetry-server"]