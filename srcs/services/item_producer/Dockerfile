FROM rust:1.70.0 as build
WORKDIR /app
COPY srcs srcs
COPY Cargo.toml Cargo.lock ./

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
RUN cargo update -p item_producer
RUN cargo build -p item_producer --release

FROM gcr.io/distroless/cc
# libz.so.1 is required for rdkafka
COPY --from=build /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1
COPY --from=build /app/target/release/item_producer /
CMD ["./item_producer"]
