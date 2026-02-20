FROM lukemathwalker/cargo-chef:latest-rust-1.93.0-bookworm AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y gdal-bin libgdal-dev lld clang 

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE=true
# Build our project
RUN rustc --version && cargo --version
RUN cargo build --release --bin geoman
RUN strip target/release/geoman


# REACT builder stage
FROM node:22-alpine AS react-builder
WORKDIR /app
COPY config config
COPY react-frontend/package.json react-frontend/
COPY react-frontend/package-lock.json react-frontend/
RUN npm ci --prefix react-frontend
COPY react-frontend react-frontend
ARG VITE_CLERK_PUBLISHABLE_KEY
RUN npm run build --prefix react-frontend

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    libgdal32 \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Ensure GDAL libraries are linked properly
ENV LD_LIBRARY_PATH=/usr/lib

COPY --from=builder /app/target/release/geoman geoman
COPY config config
# COPY geoman-book/book geoman-book/book
COPY --from=react-builder /app/react-frontend/dist react-frontend/dist
ENTRYPOINT ["./geoman"]