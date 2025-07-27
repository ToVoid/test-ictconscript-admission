# syntax=docker/dockerfile:1

FROM rust:1.88-trixie

WORKDIR /usr/src/test-ictconscript-admission

COPY . .

RUN cargo build --release

EXPOSE 8000

CMD ["target/release/test-ictconscript-admission"]