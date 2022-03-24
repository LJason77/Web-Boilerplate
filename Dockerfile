FROM rust:alpine as builder

RUN apk add -qq --repository=https://dl-cdn.alpinelinux.org/alpine/edge/community musl-dev tzdata

WORKDIR /app

COPY . .

RUN cargo build --release -q

FROM alpine:latest

COPY --from=builder /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
COPY --from=builder /app/target/release/runner /usr/local/bin/

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV ROCKET_WORKERS=8

WORKDIR /app

RUN addgroup -g 1000 pi && adduser -D -s /bin/sh -u 1000 -G pi pi && chown -R pi:pi .

USER pi

EXPOSE 8080

CMD runner
