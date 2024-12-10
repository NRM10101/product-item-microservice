# Use an official Rust image as a builder
FROM rust:1.82 as builder

# Set the working directory in the builder stage
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock to cache dependencies
# This trick avoids rebuilding all dependencies if they haven't changed
COPY Cargo.toml Cargo.lock ./
# Create a dummy source file to compile dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/product_item_microservice*

# Copy the actual source code
COPY . .

# Build the release version of the application
RUN cargo build --release

# Use Ubuntu 22.04 LTS for the final container
FROM ubuntu:22.04

# Set the working directory in the final image
WORKDIR /usr/src/app

# Install required dependencies for a minimal Rust runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder
COPY --from=builder /usr/src/app/target/release/product-item-microservice .

# Expose the port the Axum app is running on
EXPOSE 3000

# Environment variables
ENV DATABASE_URL=postgres://postgres:1002@host.docker.internal:5432
ENV DB_NAME=product_item_db

# Set the startup command
CMD ["./product-item-microservice"]