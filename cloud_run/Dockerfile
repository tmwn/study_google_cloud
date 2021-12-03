FROM golang:1.17-alpine

ENV ROOT=/go/src/app
WORKDIR ${ROOT}

RUN echo "Hello!"

COPY main.go ${ROOT}

RUN go build main.go

ENTRYPOINT ./main
