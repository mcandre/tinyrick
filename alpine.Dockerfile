FROM rust:1.92-alpine3.23 AS build
COPY . /src
WORKDIR /src
RUN cargo install --path .

FROM alpine:3.23
COPY --from=build /src/target/release/tinyrick /usr/bin
ENTRYPOINT ["/usr/bin/tinyrick"]
