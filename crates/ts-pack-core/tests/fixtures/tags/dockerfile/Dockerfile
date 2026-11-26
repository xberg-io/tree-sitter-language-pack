ARG GO_VERSION=1.23

FROM golang:${GO_VERSION} AS builder
WORKDIR /src
COPY . .
RUN go build -o /out/server ./cmd/server

FROM gcr.io/distroless/static AS runtime
ENV PORT=8080 LOG_LEVEL=info
COPY --from=builder /out/server /server
EXPOSE $PORT
ENTRYPOINT ["/server"]
