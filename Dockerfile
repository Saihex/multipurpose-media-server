FROM rust:1.79.0-buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release
RUN strip target/release/multipurpose-media-server

FROM debian:buster-slim

RUN apt update && apt upgrade -y
RUN apt install -y libssl-dev fontconfig openssl

COPY --from=builder /app/target/release/multipurpose-media-server /usr/local/bin/multipurpose-media-server

# Register custom fonts
COPY ./font /usr/share/fonts/custom
RUN fc-cache -f -v

# Expose things
EXPOSE 8080

CMD ["multipurpose-media-server"]
