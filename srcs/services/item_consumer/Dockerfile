FROM rust:1.70.0 as build
WORKDIR /app
COPY srcs srcs
COPY Cargo.toml Cargo.lock .sqlx ./

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
RUN cargo update -p item_consumer
RUN cargo build -p item_consumer --release

FROM gcr.io/distroless/static
# libz.so.1 is required for rdkafka
COPY --from=build /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1
COPY --from=build /app/target/release/item_consumer /
CMD ["./item_consumer"]
