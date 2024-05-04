FROM rust:1.77.2-buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release
RUN strip target/release/multipurpose-media-server

FROM debian:buster-slim

# Install OpenSSL
RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /app/target/release/multipurpose-media-server /usr/local/bin/multipurpose-media-server
EXPOSE 8080
VOLUME [ "/collection" ]

CMD ["multipurpose-media-server"]
