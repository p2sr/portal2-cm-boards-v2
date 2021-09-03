#!/usr/bin/env bash
dbmate wait
dbmate up
echo "done"
exec "$@"