#!/bin/sh
set -eu

# pg-ephemeral has set:
#   PGHOST, PGPORT, PGUSER, PGDATABASE, PGPASSWORD, PGSSLMODE (and friends)
#   DATABASE_URL  e.g. postgres://postgres:...@127.0.0.1:54321/postgres
#
# Any tool that reads libpq env vars or DATABASE_URL just works.

echo "DATABASE_URL=${DATABASE_URL}"
echo "Connecting via libpq env vars..."

psql --no-psqlrc --no-align --tuples-only --single-transaction --set ON_ERROR_STOP=1 \
     --command 'SELECT count(*) FROM users' | {
    read -r count
    echo "users: ${count}"
    test "${count}" = "2"
}

echo "OK"
