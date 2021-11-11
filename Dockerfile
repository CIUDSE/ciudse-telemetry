FROM rust:1-buster as telemetry-server-builder

WORKDIR /usr/src

RUN USER=root cargo new telemetry-server
WORKDIR /usr/src/telemetry-server
COPY ./telemetry-server/Cargo.toml ./telemetry-server/Cargo.lock ./
# Cache dependency compilation
RUN cargo build --release

COPY ./telemetry-server/src ./src
RUN cargo build --release



FROM node:14-alpine as telemetry-frontend-builder

RUN apk update
RUN apk add git rsync python3 gcc g++ make

WORKDIR /usr/src/telemetry-frontend
COPY ./telemetry-frontend/package.json ./telemetry-frontend/package-lock.json ./
RUN npm ci
# Cache OpenMCT compilation
RUN npm run get-openmct

COPY ./telemetry-frontend ./
RUN npm run build



FROM debian:buster-slim
COPY --from=telemetry-server-builder /usr/src/telemetry-server/target/release/telemetry-server /usr/bin
COPY --from=telemetry-frontend-builder /usr/src/telemetry-frontend/dist /static
EXPOSE 80
CMD ["telemetry-server"]