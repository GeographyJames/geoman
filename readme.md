## Cargo Watch

`cargo watch -ignore react-frontend -x run - | bunyan`

## OGC Testing Suite

- `docker run -p 8081:8080 --add-host=host.docker.internal:host-gateway ogccite/ets-ogcapi-features10` to run OGC testing suite Docker container with access to localhost
- `http://host.docker.internal:8000/ogcapi` - Application URL to test when running OGC testing suit in Docker container.
- `http://localhost:8081/teamengine` - URL to access OGC testing suit in browser.
