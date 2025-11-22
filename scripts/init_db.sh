#!/bin/bash
set -eo pipefail

remove_docker_container() {
    echo >&2 "Removing '${CONTAINER_NAME}' docker container..."
    docker rm -f "${CONTAINER_NAME}" | sed 's/^/  [docker]  /'
    echo >&2 "Container removed successfully"
}

# Cleanup function to remove container on failure
cleanup() {
    echo >&2 "Error detected - aborting"
}

# Set trap to call cleanup on error
trap cleanup ERR

echo >&2 "Initialising GeoMan database"

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    exit
fi

DB_PORT="${POSTGRES_PORT:=5432}"
SUPERUSER="${SUPERUSER:=postgres}"
SUPERUSER_PWD="${SUPERUSER_PWD:=password}"

APP_USER="${APP_USER:=app_local}"
APP_USER_PWD="${APP_USER_PWD:=secret}"
APP_DB_NAME="${APP_DB_NAME:=geoman_local}"

CONTAINER_NAME="postgres"

# Skip Docker setup if SKIP_DOCKER is set (useful for CI where Postgres is already running)
if [[ -z "${SKIP_DOCKER}" ]]; then
    # Destroy existing docker container if one exists
    if docker ps -a --format '{{.Names}}' | grep -q "^${CONTAINER_NAME}$"; then
        echo >&2 "Found existing '${CONTAINER_NAME}' docker container"
        remove_docker_container
    fi
    echo >&2 "Starting '${CONTAINER_NAME}' docker container..."
    docker run \
        --restart unless-stopped \
        --env POSTGRES_USER=${SUPERUSER} \
        --env POSTGRES_PASSWORD=${SUPERUSER_PWD} \
        --health-cmd="pg_isready -U ${SUPERUSER} || exit 1" \
        --health-interval=1s \
        --health-timeout=5s \
        --health-retries=5 \
        --publish "${DB_PORT}":5432 \
        --detach \
        --name "${CONTAINER_NAME}" \
        postgis/postgis:18-3.6 -N 500 | sed 's/^/  [docker]  /'
        # ^ x-x.x = Postgres Version-PostGIS Version

    until [ \
        "$(docker inspect -f "{{.State.Health.Status}}" ${CONTAINER_NAME})" == \
        "healthy" \
    ]; do
        >&2 echo "Postgers is still unavailable - sleeping"
        sleep 1
    done

    >&2 echo "Postgres is up and running on port ${DB_PORT}"
else
    >&2 echo "Skipping Docker setup (SKIP_DOCKER is set)"
    >&2 echo "Assuming Postgres is already running on port ${DB_PORT}"
fi

MAINTENANCE_URL=postgres://${SUPERUSER}:${SUPERUSER_PWD}@localhost:${DB_PORT}/postgres
DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}
SUPERUSER_URL=postgres://${SUPERUSER}:${SUPERUSER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME}
export DATABASE_URL

run_psql_query() {
    local connection_url=$1
    local query=$2
    # Extract first two words
    read -r word1 word2 rest <<< "$query"
    >&2 echo "Executing query: ${word1} ${word2}..."
    psql "${connection_url}" -q -c "${query}" 2>&1 | sed 's/^/  [psql]  /' || {
        >&2 echo "Error executing query"
        return 1
    }
}

run_sqlx_command() {
    local command=$1
    # Extract first two words
    read -r word1 word2 rest <<< "$command"
    >&2 echo "Executing sqlx: ${word1} ${word2}..."
    sqlx ${command} || {
        >&2 echo "Error executing sqlx command"
        return 1
    }
}

run_psql_query "${MAINTENANCE_URL}" "CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
run_psql_query "${MAINTENANCE_URL}" "ALTER USER ${APP_USER} CREATEDB;"

run_sqlx_command "database create"

run_psql_query "${SUPERUSER_URL}" "CREATE EXTENSION postgis;"
run_psql_query "${SUPERUSER_URL}" "GRANT REFERENCES ON spatial_ref_sys TO ${APP_USER};"
# BTREE index required for checking turbines are not on top of each other
# psql postgres://${SUPERUSER}:${SUPERUSER_PWD}@localhost:${DB_PORT}/${APP_DB_NAME} -q -c "CREATE EXTENSION btree_gist;" 

run_sqlx_command "migrate run"

# Seed data
SEED_DATA_DIRECTORY="seed_data/"

run_sql_file() {
    local sql_file=$1
    >&2 echo "Executing ${sql_file}"
    psql ${DATABASE_URL} -q -v ON_ERROR_STOP=1 -f ${SEED_DATA_DIRECTORY}"$sql_file" 2>&1 | sed 's/^/  [psql]  /' || {
        >&2 echo "error executing $sql_file"
        return 1
    }
}

if [[ -z "${SKIP_SEED}" ]]
then
    >&2 echo "Seeding data..."

    # Array of SQL files to run in order
    sql_files=(
        "teams.sql"
        "users.sql"
        "projects.sql"
        "collections.sql"
        "features.sql"
        "crs.sql"
    )

    # Iterate through the array
    for sql_file in "${sql_files[@]}"; do
        run_sql_file "$sql_file"
    done

    >&2 echo "Data seeded successfully"
fi

# Disable cleanup trap on successful completion
trap - ERR
echo >&2 "Database initialisation completed successfully!"