FROM node:24-alpine AS base
RUN corepack enable pnpm

FROM base AS builder
WORKDIR /app
COPY pnpm-lock.yaml pnpm-workspace.yaml package.json ./
COPY packages/ui/package.json ./packages/ui/
COPY apps/frontend-pos/package.json ./apps/frontend-pos/
COPY apps/frontend-admin/package.json ./apps/frontend-admin/
COPY apps/frontend-home/package.json ./apps/frontend-home/
COPY apps/frontend-tenant/package.json ./apps/frontend-home/
RUN pnpm i

COPY . .
RUN pnpm build

# dist should now be available
FROM caddy:2.11-alpine AS prod
WORKDIR /srv
COPY Caddyfile /etc/caddy/Caddyfile
COPY --from=builder /app/dist /srv
