FROM registry.fedoraproject.org/fedora:33 as rust
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN dnf -y install cmake gcc \
 && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup \
 && chmod +x /tmp/rustup \
 && /tmp/rustup -y --no-modify-path \
 && rm -f /tmp/rustup

FROM rust as planner
WORKDIR app
RUN cargo install cargo-chef
COPY helloworld-rs .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY helloworld-rs .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

FROM registry.fedoraproject.org/fedora:33
COPY --from=builder /app/target/release/helloworld /usr/local/bin/helloworld
COPY helloworld-rs/Rocket.toml /etc/helloworld.toml
ENV ROCKET_CONFIG=/etc/helloworld.toml
USER 1001
EXPOSE 8000
CMD ["/usr/local/bin/helloworld"]
