FROM rust:1 as builder
WORKDIR /aws-test
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/aws-test /usr/local/bin/aws-test
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["aws-test"]