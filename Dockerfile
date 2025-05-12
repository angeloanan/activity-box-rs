FROM clux/muslrust:stable as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/static-debian12:latest
COPY --from=build-env /app/target/release/activity-box-rs /
CMD ["./activity-box-rs"]
