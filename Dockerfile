## Step 1: Base Image Setup
FROM rust:latest AS base

## Step 2: Source Code Build
FROM base AS builder
WORKDIR /app

# Copy the entire source code
COPY . .

# Build the application
RUN rustup toolchain install nightly
RUN RUSTFLAGS="-Z threads=8" cargo +nightly build --release --locked

## Step 3: Production Image Setup
FROM base AS runner
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/voyager-backend .
COPY --from=builder /app/.env .

# ## Step 4: User Setup
# # Creates the group and the user
# RUN addgroup --system --gid 1001 rust
# RUN adduser --system --uid 1001 voyager

# # Changes the ownership of the workdir and docker sock
# RUN chown -R voyager:rust ./
# RUN chown -R voyager:rust /var/run

# # Changes the user to the created user
# USER voyager
# RUN chmod -R 666 /var/run

## Step 5: Host Configuration
ENV HOSTNAME=0.0.0.0
ENV PORT=8765
EXPOSE 8765

## Step 6: Container Execution
CMD ["./voyager-backend"]