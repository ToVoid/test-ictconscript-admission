# syntax=docker/dockerfile:1

FROM rust:1.88-trixie AS builder

WORKDIR /usr/src/test-ictconscript-admission

COPY . .

RUN cargo build --release


FROM debian:trixie-20250721-slim AS runner

WORKDIR /usr/src/test-ictconscript-admission

COPY --from=builder /usr/src/test-ictconscript-admission/target/release/test-ictconscript-admission ./target/release/test-ictconscript-admission
COPY sample-data/data.json sample-data/data.json

EXPOSE 8000

CMD ["target/release/test-ictconscript-admission"]