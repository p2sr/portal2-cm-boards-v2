#!/bin/bash

function timed_echo () {
  echo -e "$(date -u +'%F %T.%3N %Z'):\t${@}"
}

timed_echo "Entrypoint Script Started"
cd $PATH_TO_VOLUME
cargo install cargo-watch && cargo build &

if [ "$DATABASE" = "postgres" ]
then
    timed_echo "Waiting for postgres..."

    while ! nc -z $SQL_HOST $SQL_PORT; do
      sleep 0.1
    done

    timed_echo "PostgreSQL started"
fi

wait
timed_echo "Background commands finished"

exec "$@"