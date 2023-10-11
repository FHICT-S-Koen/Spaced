FROM rust:1.70.0 as build
WORKDIR /app
COPY services/item_producer /app
COPY proto /proto

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
RUN cargo update
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build /app/target/release/item_producer /
CMD ["./item_producer"]
