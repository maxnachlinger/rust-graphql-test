# Notes for running this:
# locally you can run:
# docker build -t rust-graphql-test .
# docker run -p 8080:8080 --env ENVIRONMENT=stage rust-graphql-test
# in all environments, set the ENVIRONMENT ENV var, e.g. --env ENVIRONMENT=prod

FROM rust:latest

RUN adduser --system --uid 10000 --group --shell /sbin/nologin --home /opt/app/ app

WORKDIR /usr/src/rust-graphql-test

RUN chown -R app:app /usr/src/rust-graphql-test

USER 10000:app

COPY Cargo.lock .
COPY Cargo.toml .
COPY config config/
COPY src src/

ENV RUST_LOG=info

RUN cargo install --path .

EXPOSE 8080

ENTRYPOINT [ "rust-graphql-test" ]
