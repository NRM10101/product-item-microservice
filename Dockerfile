# Use an official Rust image as a builder
FROM rust:1.72 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the release version of the application
RUN cargo build --release

# Use a smaller base image for the final container
FROM debian:buster-slim

# Set the working directory in the final image
WORKDIR /usr/src/app

# Copy the built binary from the builder
COPY --from=builder /usr/src/app/target/release/product-item-microservice .

# Expose the port the Axum app is running on
EXPOSE 3000

# Set the startup command
CMD ["./product-item-microservice"]
