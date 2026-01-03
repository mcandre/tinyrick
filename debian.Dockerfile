FROM rust:1.92-trixie AS build
COPY . /src
WORKDIR /src
RUN make install

FROM debian:trixie
COPY --from=build /src/target/release/tinyrick /usr/bin
ENTRYPOINT ["/usr/bin/tinyrick"]
