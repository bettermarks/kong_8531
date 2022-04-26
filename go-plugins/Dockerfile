FROM golang:1.16-alpine
#FROM golang:1.15-stretch
#FROM diefans/centos-7-golang:0.2.0

RUN apk add --no-cache git gcc libc-dev bash

ADD entrypoint.sh /entrypoint.sh
RUN mkdir /src
WORKDIR /src

ENTRYPOINT ["/entrypoint.sh"]

