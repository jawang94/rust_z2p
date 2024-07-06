#!/bin/bash

set -x
set -eo pipefail

# Ensure dependencies are installed before proceedng to avoid broken state
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx not installed."
  echo >&2 "Use:"
  echo >&2 " cargo install --version='~0.7' sqlx-cli --no-default-features --features rustls,postgres"
  echo >&2 " to install it"
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"

# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"

# Check if a custom database name as been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"

# Check if a custom database port as been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

# Check if a custom database host as been set, otherwise default to '5432'
DB_HOST="${POSTGRES_HOST:=localhost}"

if [[ -z "${SKIP_DOCKER}" ]]; then
  docker run \
    -e POSTGRES_USER="${DB_USER}" \
    -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
    -e POSTGRES_DB="${DB_NAME}" \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
  # ^- inc # of connections for testing purposes
fi

# Keep pinging postgres until it's ready to accept new commands
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c "\q"; do
  echo >&2 "Postgres is still unavailable - sleeping"
  sleep 1
done

echo >&2 "Postgres is up and running on port ${DB_PORT}! - running migrations now"

# Set the url to a system variable once postgres is up and running
DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
# Then, proceed to create the sqlx database and migrations
export DATABASE_URL
sqlx database create
sqlx migrate add create_subscriptions_table
sqlx migrate run

echo >&2 "Postgres has been migrated, ready to go!"
