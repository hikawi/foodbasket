FROM golang:1.25-alpine AS builder

WORKDIR /app
COPY . .

RUN go build -o server .

FROM alpine:3 AS prod
RUN apk add --no-cache curl

EXPOSE 8080

WORKDIR /app
COPY --from=builder /app/server /app/server
HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD [ "curl", "-I", "localhost:8080" ]
CMD ["/app/server"]
