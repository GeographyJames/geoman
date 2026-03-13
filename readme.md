## Database initialisation

`./scripts/init_db.sh` to initialise local development database

## Cargo Watch

- `cargo watch -i react-frontend -i test-tools -x run | bunyan`

## Logging

- `cargo run >&1 | tee server.log | bunyan` Saves logs to file 'server.log' in project root.
- To set logging level use environment variable `RUST_LOG=<level>` e.g: `RUST_LOG=trace cargo run`

## OGC Testing Suite

### Docker

- `docker run -p 8081:8080 --add-host=host.docker.internal:host-gateway ogccite/ets-ogcapi-features10` Runs OGC testing suite Docker container with access to localhost.
- `http://host.docker.internal:8000/ogcapi` - Application URL to test when running OGC testing suit in Docker container.
- `http://localhost:8081/teamengine` - URL to access OGC testing suit in browser.

### Command Shell

- `./test-tools/run-tests.sh` - to run the OGC test suit for root endopint.
- `./test-tools/run-tests.sh -c test-tools/test-run-props-project.xml -n project` - to run test suit for project endpoint.

## Postgres connection string:

- `postgresql://<user>:<pwd>@<host>:<port>/<db_name>`
- eg: `postgresql://app_local:secret@localhost:5432/geodata_local`

## Qgis Server

- For local: `QGIS_SERVER_PORT=8001 PGUSER=app_local PGPASSWORD=secret PGHOST=localhost PGPORT=5434 qgis_mapserver` to run qgis server. Ensuer you are running the server from a directory with a qgis project file.
- For local docker: `docker run --name qgis-server -p 8001:5555 -e PGUSER=<username> -e PGPASSWORD=<password> -e PGHOST=<host> -e PGPORT=<port> -e PGSSMODE=<sslmode>  qgis-server`

## Command to run Geoservere Docker image on DO Droplet **This has been superseded by adding a docker compose yml file in the home directory**

- `docker run -d --name geoserver --restart always -p 8080:8080 --env GEOSERVER_CSRF_WHITELIST=geoserver.geodata-manager.com --env SKIP_DEMO_DATA=true --mount src="/root/geoserver_data/",target=/opt/geoserver_data/,type=bind docker.osgeo.org/geoserver:2.28.x`

## Droplet SSH

- `ssh root@<droplet_id>`
