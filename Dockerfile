FROM rust:1.41-slim

ENV ROCKET_ADDRESS=0.0.0.0

WORKDIR /usr/src/app
COPY . ./

RUN rustup default nightly
RUN cargo build

EXPOSE 8000

CMD ["cargo", "run"]