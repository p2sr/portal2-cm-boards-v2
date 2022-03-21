FROM rust:1.59-bullseye

RUN apt-get update && \
    apt-get install -y netcat && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /opt/entrypoint
ENV CARGO_HOME /opt/cargo
RUN PATH=$CARGO_HOME:$PATH
COPY server-entrypoint.sh ./
RUN chmod +x server-entrypoint.sh
ENV SQLX_OFFLINE true
EXPOSE 8080
ENTRYPOINT ["/opt/entrypoint/server-entrypoint.sh"]