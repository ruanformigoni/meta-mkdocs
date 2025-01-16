FROM alpine:latest as builder
# Install packages
RUN apk update
RUN apk add --no-cache curl build-base rustup
# Symlink compilers
RUN ln -sfT /usr/bin/gcc /usr/bin/musl-gcc
RUN ln -sfT /usr/bin/g++ /usr/bin/musl-g++
# Compile as shared libraries
ENV RUSTFLAGS='-C target-feature=-crt-static'
# Install rust
RUN rustup-init -y --default-toolchain nightly
# Update env
ENV PATH="/root/.cargo/bin:$PATH"
# Copy sources
COPY . /meta_mkdocs
WORKDIR /meta_mkdocs
# Build project
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
# Install packages
RUN apk add --no-cache libgcc pipx
# Update env
ENV PATH="/root/.local/bin:$PATH"
# Install mkdocs
RUN pipx install mkdocs
# Create projects directory
RUN mkdir -p /app/projects
# Enter app directory
WORKDIR /app
# Copy built app from previous image
COPY --from=builder /meta_mkdocs/target/x86_64-unknown-linux-musl/release/meta_mkdocs .
# Set app as entrypoint
ENTRYPOINT ["/app/meta_mkdocs"]
