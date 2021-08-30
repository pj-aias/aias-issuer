FROM alpine:latest

RUN apk update && apk add tor curl
CMD tor -f /etc/tor/torrc
