FROM rust:latest as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/activity-box-rs /
CMD ["./activity-box-rs"]
