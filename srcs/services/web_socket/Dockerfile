FROM rust:1.70.0 as build
WORKDIR /app
COPY srcs srcs
COPY Cargo.toml Cargo.lock ./

RUN cargo update -p web_socket
RUN cargo build -p web_socket --release

FROM gcr.io/distroless/static
COPY --from=build /app/target/release/web_socket /
CMD ["./web_socket"]
