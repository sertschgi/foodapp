FROM rust:latest AS builder

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash && \
    cargo binstall dioxus-cli --version 0.6.3 --force

RUN ARCH=$(uname -m) && \
    if [ "$ARCH" = "x86_64" ]; then \
        curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64; \
    elif [ "$ARCH" = "aarch64" ]; then \
        curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-arm64; \
    else \
        echo "Unsupported architecture"; exit 1; \
    fi && \
    mv tailwindcss-linux-* tailwindcss && \
    chmod +x tailwindcss


FROM rust:slim

RUN apt-get update && apt-get full-upgrade -y \
    && apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev libpq-dev expect \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

COPY --from=builder /usr/local/cargo/bin/dx /usr/local/cargo/bin/dx
COPY --from=builder /tailwindcss /usr/local/bin/tailwindcss
