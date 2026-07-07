ARG RUST_VERSION=1.96.1
FROM rust:${RUST_VERSION}

ARG UID=1000
ARG GID=1000

RUN rustup component add rustfmt clippy \
    && cargo install cargo-tarpaulin \
    && groupadd -g ${GID} sdk 2>/dev/null || true \
    && useradd -m -u ${UID} -g ${GID} sdk 2>/dev/null || true \
    && mkdir -p /home/sdk/.cargo/registry /home/sdk/.cargo/bin \
    && cp /usr/local/cargo/bin/cargo-tarpaulin /home/sdk/.cargo/bin/ \
    && chown -R ${UID}:${GID} /home/sdk/.cargo

ENV CARGO_HOME=/home/sdk/.cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH="/home/sdk/.cargo/bin:/usr/local/cargo/bin:${PATH}"

WORKDIR /usr/src
USER sdk

ENTRYPOINT ["tail", "-f", "/dev/null"]
