# Stage 1: Build the application
FROM rust:1.81 as builder

WORKDIR /usr/src/app

# Copy the backend source code
COPY oneAI-backend/ .

# Build the backend
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/oneAI-backend .

# Command to run the application
CMD ["./oneAI-backend"]
