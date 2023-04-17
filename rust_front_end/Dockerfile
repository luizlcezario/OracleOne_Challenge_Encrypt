FROM rust:lastest
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install -f wasm-bindgen-cli
EXPOSE 8080
WORKDIR /app
CMD ["trunk", "serve"]