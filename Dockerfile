FROM --platform=linux/amd64 clux/muslrust:stable AS build-env
WORKDIR /app
COPY . /app
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM --platform=linux/amd64 gcr.io/distroless/static-debian12:latest
COPY --from=build-env /app/target/x86_64-unknown-linux-musl/release/activity-box-rs /github/workspace/activity-box-rs
CMD ["/github/workspace/activity-box-rs"]
