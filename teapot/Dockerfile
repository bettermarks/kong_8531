FROM golang:1.19-alpine

WORKDIR /src
COPY teapot.go teapot.go
RUN ["go", "build", "teapot.go"]

CMD ["./teapot"]
