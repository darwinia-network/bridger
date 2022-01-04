FROM debian:stable-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
        openssl \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY dist/ /usr/local/bin

EXPOSE 1098

ENTRYPOINT [ "/usr/local/bin/bridger" ]
