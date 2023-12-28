################
##### Builder
FROM rust:alpine as builder

WORKDIR /usr/src/game

RUN USER=root

RUN apk add --no-cache openssl-dev build-base

COPY . .

ENV RUSTFLAGS="-Ctarget-feature=-crt-static"

RUN cargo build --release

################
##### Runtime
FROM alpine:latest AS runtime

# Install dependencies
RUN apk add --no-cache openssl ca-certificates libgcc

# Copy application binary from builder image
COPY --from=builder /usr/src/game/target/release/MSD_Game_Service /usr/local/bin

# Run the application
CMD ["/usr/local/bin/MSD_Game_Service"]


