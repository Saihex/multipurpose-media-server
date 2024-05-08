FROM rust:1.77.2-buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release
RUN strip target/release/multipurpose-media-server

FROM debian:buster-slim

# Install OpenSSL
RUN apt-get update && apt-get install -y libssl-dev fontconfig

COPY --from=builder /app/target/release/multipurpose-media-server /usr/local/bin/multipurpose-media-server

# Register custom fonts
COPY ./font /usr/share/fonts/custom
RUN fc-cache -f -v

# Expose things
EXPOSE 8080

CMD ["multipurpose-media-server"]
