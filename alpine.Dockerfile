# syntax=docker/dockerfile:1
FROM rust:alpine AS build
WORKDIR /src

COPY . .
RUN cargo install --path . --root /app/

FROM alpine:latest
WORKDIR /app
COPY --from=build /app/bin ./
ENTRYPOINT ["./health"]