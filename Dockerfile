FROM alpine:3.23 AS build
RUN apk add -U cargo
COPY . /src
WORKDIR /src
RUN cargo build --release

FROM alpine:3.23
RUN apk add -U libgcc
COPY --from=build /src/target/release/tinyrick /usr/bin/tinyrick
ENTRYPOINT ["/usr/bin/tinyrick"]
