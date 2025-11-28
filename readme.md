## Cargo Watch

- `cargo watch -i react-frontend -i test-tools -x run | bunyan`

## Logging

- `cargo run >&1 | tee server.log | bunyan` Saves logs to file 'server.log' in project root.

## OGC Testing Suite

### Docker

- `docker run -p 8081:8080 --add-host=host.docker.internal:host-gateway ogccite/ets-ogcapi-features10` Runs OGC testing suite Docker container with access to localhost.
- `http://host.docker.internal:8000/ogcapi` - Application URL to test when running OGC testing suit in Docker container.
- `http://localhost:8081/teamengine` - URL to access OGC testing suit in browser.

### Command Shell

- `./test-tools/run-tests.sh` - to run the OGC test suit for root endopint.
- `./test-tools/run-tests.sh -c test-tools/test-run-props-project.xml -n project` - to run test suit for project endpoint.
