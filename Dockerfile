FROM --platform=${BUILDPLATFORM:-"linux/amd64"} rust:1-buster as telemetry-server-builder

WORKDIR /usr/src
RUN USER=root cargo new telemetry-server
WORKDIR /usr/src/telemetry-server

ARG TARGETPLATFORM
RUN case ${TARGETPLATFORM:-"linux/amd64"} in \
    "linux/arm64") echo aarch64-unknown-linux-gnu > /rust_target.txt && \
    apt-get update && apt-get install -y gcc-aarch64-linux-gnu && \
    mkdir .cargo && echo [target.aarch64-unknown-linux-gnu] > .cargo/config &&  \
    echo linker=\"aarch64-linux-gnu-gcc\" >> .cargo/config ;; \
    "linux/amd64") echo x86_64-unknown-linux-gnu > /rust_target.txt ;; \
    *) exit 1 ;; \
esac

RUN rustup target add $(cat /rust_target.txt)

COPY ./telemetry-server/Cargo.toml ./telemetry-server/Cargo.lock ./
# Cache dependency compilation
RUN cargo build --release --target $(cat /rust_target.txt)

COPY ./telemetry-server/src ./src
RUN cargo build --release --target $(cat /rust_target.txt)
RUN mv ./target/$(cat /rust_target.txt) ./target/docker


FROM --platform=${BUILDPLATFORM:-"linux/amd64"} node:14-buster-slim as telemetry-frontend-builder

RUN apt-get update
RUN apt-get install -y git rsync python3 gcc g++ make

WORKDIR /usr/src/telemetry-frontend
COPY ./telemetry-frontend/package.json ./telemetry-frontend/package-lock.json ./

RUN npm ci

COPY ./telemetry-frontend ./

# Cache OpenMCT compilations
RUN npm run setup-deps
RUN npm run build



FROM debian:buster-slim
COPY --from=telemetry-server-builder /usr/src/telemetry-server/target/docker/release/telemetry-server /usr/bin
COPY --from=telemetry-frontend-builder /usr/src/telemetry-frontend/dist /static
EXPOSE 80
CMD ["telemetry-server"]