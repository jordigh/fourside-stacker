FROM rust:1.71-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo install --path .
RUN strip /usr/local/cargo/bin/rustwebgame

FROM node:20-slim AS node
ARG base_url
WORKDIR /app/frontend/
COPY frontend .
RUN npm ci
RUN echo "VITE_STACKED_FOURSIDE_HOST=$base_url" > .env
RUN npm run build

FROM debian:bookworm-slim
WORKDIR /app
RUN apt update && apt install -y libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=node /app/frontend/dist ./frontend/dist
COPY --from=builder /usr/local/cargo/bin/rustwebgame /usr/local/bin/rustwebgame
ENTRYPOINT ["rustwebgame"]
