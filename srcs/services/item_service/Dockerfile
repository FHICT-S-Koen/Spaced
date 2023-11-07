FROM rust:1.70.0 as build
WORKDIR /app
COPY srcs srcs
COPY Cargo.toml Cargo.lock ./

RUN cargo update -p item_service
RUN cargo build -p item_service --release

FROM gcr.io/distroless/static
COPY --from=build /app/target/release/item_service /
CMD ["./item_service"]
