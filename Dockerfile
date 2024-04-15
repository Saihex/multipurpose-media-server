FROM rust:1.77.2-buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release
RUN strip target/release/image-server-downscaller

FROM debian:buster-slim

COPY --from=builder /app/target/release/image-server-downscaller /usr/local/bin/image-server-downscaller
EXPOSE 8080
VOLUME [ "/collection" ]

CMD ["image-server-downscaller"]