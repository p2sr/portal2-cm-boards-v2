FROM rust:1.56-bullseye

RUN apt-get update && \
    apt-get install -y netcat && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /server
COPY src src
COPY .env.server.dev .env
COPY Cargo.lock Cargo.toml server-entrypoint.sh ./
ENV SQLX_OFFLINE true
RUN cargo build --release
RUN cargo install cargo-watch
RUN chmod +x server-entrypoint.sh
EXPOSE 8080
ENTRYPOINT ["/server/server-entrypoint.sh"]