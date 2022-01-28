mkdir out
docker buildx build --platform linux/arm64 --tag ciudse-telemetry:latest .
docker save ciudse-telemetry | gzip > out/ciudse-telemetry-arm64.tar.gz
docker buildx build --platform linux/amd64 --tag ciudse-telemetry:latest .
docker save ciudse-telemetry | gzip > out/ciudse-telemetry-amd64.tar.gz
cp docker-compose.yml out/docker-compose.yml
