# Stage 1: Build
FROM rust:latest AS builder

# Install necessary build tools
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config

# Set the working directory for the build
WORKDIR /app

# Copy the application source code from the backend folder (relative to the build context)
COPY ../backend /app

# Pre-fetch dependencies to cache them
RUN cargo build --release

# Stage 2: Run
FROM debian:bullseye-slim

# Install required runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1

# Set the working directory for the runtime
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/virtumart /app/virtumart

# Copy the .env file to the container (if needed)
COPY .env .env

# Expose the application port
EXPOSE 8080

# Set the binary as the entrypoint
ENTRYPOINT ["/app/virtumart"]
