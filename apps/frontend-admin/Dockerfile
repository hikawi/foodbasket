FROM node:24-alpine AS base
RUN corepack enable pnpm

FROM base AS builder
WORKDIR /app
COPY package.json .
RUN pnpm i

COPY . .
RUN pnpm build

# dist should now be available
FROM caddy:2.11-alpine AS prod
WORKDIR /srv
COPY Caddyfile /etc/caddy/Caddyfile
COPY --from=builder /app/dist /srv

