FROM rust:1.72-bookworm

WORKDIR /app

RUN apt-get update && apt-get install -y \
    bash \
    g++ \
    make \
    npm \
    libssl-dev \
    curl

RUN npm install -g pnpm

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack worker-build

COPY . .
COPY .dev.vars.example .dev.vars

RUN pnpm install

CMD ["pnpm", "wrangler", "dev", "--env", "dev", "--test-scheduled"]
