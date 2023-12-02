# Base image with Rust
FROM rust:1.72-buster as base

# Set arguments for user, group, and target
ARG USER=pypes_exporter
ARG USER_ID=1000
ARG GROUP_ID=1000
ARG TARGET=x86_64-unknown-linux-musl

# Create a directory for the app
RUN mkdir /app
WORKDIR /app

# Update and install necessary packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    libc6-dev \
    && rm -rf /var/lib/apt/lists/*

# Add the target for cross-compilation
RUN rustup target add ${TARGET}

# Create a new user and group
RUN groupadd -g ${GROUP_ID} ${USER} && \
    useradd -l -m -u ${USER_ID} -g ${USER} ${USER}

# Change ownership of the /app directory
RUN chown ${USER}:${USER} /app
USER ${USER}

# Copy the Cargo.toml and Cargo.lock files
COPY --chown=${USER}:${USER} Cargo.toml Cargo.lock /app/

# Development stage with additional tools
FROM base as dev

USER root
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    git \
    tig \
    vim \
    jq \
    && rm -rf /var/lib/apt/lists/*
USER ${USER}

# Builder stage to compile the application
FROM base as builder

COPY --chown=${USER}:${USER} . /app
RUN cargo build --release --target ${TARGET}

# Tester stage
FROM builder as tester
ENV TARGET=${TARGET}
CMD cargo test --release --target $TARGET

# Installer stage
FROM builder as installer
RUN cargo install --target ${TARGET} --path .

# Final release stage
# FROM scratch as release

# Temp debug image
FROM debian:buster-slim as release
VOLUME /root/.agents/db/
COPY --from=installer --chown=root:root /usr/local/cargo/bin/pypes /bin/pypes
ENTRYPOINT ["/bin/pypes", "start", "-p", "8080", "-a"]
