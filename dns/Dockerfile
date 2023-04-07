FROM debian:11.6-slim

RUN apt update -y && apt install dnsutils dnsmasq -y

RUN echo "address=/dev.local/127.0.0.1" >> /etc/dnsmasq.conf
RUN echo "port=53" >> /etc/dnsmasq.conf

EXPOSE 53/udp

CMD ["dnsmasq", "-k"]
