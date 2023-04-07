FROM debian:11.6-slim

ARG SUBNET_BASE

RUN apt update -y && apt install dnsutils dnsmasq -y

RUN echo "address=/dev.local/${SUBNET_BASE}.10" >> /etc/dnsmasq.conf
RUN echo "port=53" >> /etc/dnsmasq.conf

CMD ["dnsmasq", "-k"]
