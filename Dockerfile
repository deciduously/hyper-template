FROM rust:1.41.0 AS builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new hyper-template
WORKDIR /usr/src/hyper-template
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
COPY templates ./templates
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=builder /usr/local/cargo/bin/hyper-template .
COPY images ./images
USER 1000
CMD ["./hyper-template", "-a", "0.0.0.0", "-p", "8080"]