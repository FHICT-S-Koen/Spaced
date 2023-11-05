FROM rust:1.70.0 as build
WORKDIR /app
COPY srcs srcs
COPY Cargo.toml Cargo.lock ./

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
RUN cargo update -p web_socket
RUN cargo build -p web_socket --release

FROM gcr.io/distroless/static
# libz.so.1 is required for rdkafka
COPY --from=build /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1
COPY --from=build /app/target/release/web_socket /
CMD ["./web_socket"]
