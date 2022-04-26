FROM rust:1.60-alpine
RUN apk add --no-cache git bash gcc libc-dev protobuf-dev

ADD entrypoint.sh /entrypoint.sh
RUN mkdir /src
WORKDIR /src

ENTRYPOINT ["/entrypoint.sh"]

