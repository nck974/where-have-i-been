# docker build -f Dockerfile -t nck974/where-have-i-been-rs:0.0.0 .
# docker push nck974/where-have-i-been-rs:0.0.0
#################
# Build the app #
#################
FROM rust:1.81.0-bullseye AS builder

WORKDIR /app

COPY . .
RUN cargo build --release

#####################
# Run the built app #
#####################
FROM debian:bullseye-slim

WORKDIR /app

ENV TRACKS_DIRECTORY=/app/tracks-data
ENV CACHE_DIRECTORY=/app/data/.cached_tracks
ENV DATABASE_PATH=/app/data/tracks_database.db
EXPOSE 3000

COPY --from=builder /app/target/release/wherehaveibeen-rs .
RUN chmod +x /app/wherehaveibeen-rs

CMD ["./wherehaveibeen-rs"]
