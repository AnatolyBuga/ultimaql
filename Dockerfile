FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS server_builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin server --target x86_64-unknown-linux-musl


FROM alpine AS runtime
COPY --from=server_builder /app/target/x86_64-unknown-linux-musl/release/server /usr/local/bin/
EXPOSE 8000
ENV RUST_LOG debug
ENV RUST_BACKTRACE full
ENV MONGO_URI mongodb://quantlib_db:27017/
ENV ADDRESS 0.0.0.0:8000
CMD ["/usr/local/bin/server"]
