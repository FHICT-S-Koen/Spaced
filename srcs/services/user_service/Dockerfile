FROM rust:1.70.0 as build
WORKDIR /app
COPY srcs srcs
COPY Cargo.toml Cargo.lock ./

RUN cargo update -p user_service
RUN cargo build -p user_service --release

FROM gcr.io/distroless/static
COPY --from=build /app/target/release/user_service /
CMD ["./user_service"]
