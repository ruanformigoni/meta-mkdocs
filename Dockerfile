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

RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
RUN apk add --no-cache libgcc
RUN mkdir -p /app/projects
WORKDIR /app
COPY --from=builder /meta_mkdocs/target/x86_64-unknown-linux-musl/release/meta_mkdocs .
ENTRYPOINT ["/app/meta_mkdocs"]
