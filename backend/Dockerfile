FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/harbor .

ENV PORT 4000

CMD ["./harbor"]
