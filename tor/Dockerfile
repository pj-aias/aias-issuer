FROM alpine:latest

RUN apk update && apk add tor curl

RUN chown -R tor /etc/tor /var/lib/tor
RUN chmod -R 700 /etc/tor /var/lib/tor

USER tor
CMD sleep 10; tor -f /etc/tor/torrc
