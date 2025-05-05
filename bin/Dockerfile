FROM ubuntu:latest

WORKDIR /usr/local/app

ARG FILE_URL="https://github.com/sertschgi/foodapp/releases/download/pre/server-latest.tar.gz"

RUN apt-get update && \
    apt-get install -y wget tar libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Download and extract (creates a `server-latest/` directory)
RUN wget ${FILE_URL} && \
    tar -xzf server-latest.tar.gz && \
    rm server-latest.tar.gz

COPY .env ./.env

CMD ["./server"]
