FROM rust:bullseye AS build
WORKDIR /src

COPY . .
RUN cargo install --path . --root /app/

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=build /app/bin ./
ENTRYPOINT ["./health"]