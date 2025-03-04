# Use the official Rust image as the base image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Build the Rust project
RUN cargo build --release

# Use the same Rust image as the base image for the final container
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/lifeTracker ./life-tracker

# Copy the necessary files
COPY .env .
COPY templates ./templates
COPY static ./static

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the entrypoint to the compiled binary
ENTRYPOINT ["./life-tracker"]
