# Build Stage
FROM rust:1.84.0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release
RUN strip target/release/multipurpose-media-server

# Runtime Stage
FROM debian:stable-slim

# Update and install dependencies
RUN apt update && apt upgrade -y
RUN apt install -y libssl-dev fontconfig openssl

COPY --from=builder /app/target/release/multipurpose-media-server /usr/local/bin/multipurpose-media-server

# Register custom fonts
COPY ./font /usr/share/fonts/custom
RUN fc-cache -f -v

# Expose things
EXPOSE 8080

# Default command
CMD ["multipurpose-media-server"]
