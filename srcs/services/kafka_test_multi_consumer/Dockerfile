# Available tags: latest, nonroot, debug, debug-nonroot
# For more, see https://github.com/GoogleContainerTools/distroless#how-do-i-use-distroless-images
ARG IMAGE_TAG=nonroot

# TODO: bind to the rust-version from Cargo.toml intead or latest
FROM rust:1.70.0 as build
WORKDIR /app
COPY srcs srcs
COPY .sqlx .sqlx
COPY Cargo.toml Cargo.lock ./

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
RUN cargo build -p item_consumer --release

# TODO: use static and copy dynamically linked libraries or build with static linking
FROM gcr.io/distroless/cc:$IMAGE_TAG
WORKDIR /home/nonroot
# libz.so.1 is required for rdkafka
COPY --from=build /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1
COPY --from=build --chown=nonroot:nonroot /app/target/release/item_consumer /usr/bin
ENTRYPOINT ["item_consumer"]
