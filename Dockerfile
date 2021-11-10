FROM rust:1.55.0 as telemetry-server-builder

WORKDIR /usr/src/telemetry-server
COPY ./telemetry-server .
RUN cargo install --path .



FROM node:14.18.1-buster as telemetry-frontend-builder

RUN apt-get update
RUN apt-get install -y rsync git

WORKDIR /usr/src/telemetry-frontend

RUN git clone https://github.com/nasa/openmct.git
WORKDIR /usr/src/telemetry-frontend/openmct
RUN npm install
RUN npm build:prod

WORKDIR /usr/src/telemetry-frontend
COPY ./telemetry-frontend ./
RUN npm install

RUN mkdir -p ./dist/openmct
RUN rsync -a --delete ./openmct/dist/ ./dist/openmct

RUN npm run build

FROM debian:buster-slim
COPY --from=telemetry-server-builder /usr/local/cargo/bin/telemetry-server /usr/local/bin/telemetry-server
COPY --from=telemetry-frontend-builder /usr/src/telemetry-frontend/dist /static
CMD ["telemetry-server"]