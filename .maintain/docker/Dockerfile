FROM debian:stable-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
        openssl \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY dist/bridger /usr/local/bin/
COPY dist/bridge-pangolin-pangoro /usr/local/bin/
COPY dist/bridge-pangolin-ropsten /usr/local/bin/
COPY dist/bridge-darwinia-crab /usr/local/bin/
COPY dist/bridge-darwinia-ethereum /usr/local/bin/

EXPOSE 1098

ENTRYPOINT [ "/usr/local/bin/bridger" ]
