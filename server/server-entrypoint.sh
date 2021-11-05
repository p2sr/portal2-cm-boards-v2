#!/bin/bash

cd $PATH_TO_VOLUME
#cargo build
cargo install cargo-watch

if [ "$DATABASE" = "postgres" ]
then
    echo "Waiting for postgres..."

    while ! nc -z $SQL_HOST $SQL_PORT; do
      sleep 0.1
    done

    echo "PostgreSQL started"
fi


exec "$@"