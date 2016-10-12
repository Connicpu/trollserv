FROM alpine:3.4
ADD target/x86_64-unknown-linux-musl/release/trollserv /app/
EXPOSE 1337
WORKDIR /app
CMD ["/app/trollserv"]
