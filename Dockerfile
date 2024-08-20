FROM rust:bookworm AS build
WORKDIR /src

COPY . .
RUN cargo install --path . --root /app/

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/bin ./
ENTRYPOINT ["./health"]