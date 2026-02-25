# --------------------
# A quick overview of targets and steps. This is a multi-target Dockerfile
# Intermediate targets:
# - node-base: pnpm-enabled version of node 24
# - node-deps: installed deps of node
# - node-builder: built version of @foodbasket/ui and @foodbasket/types
# Final targets:
# - frontend-pos (SPA)
# - frontend-admin (SPA)
# - frontend-home (Node SSR)
# - frontend-tenant (Node SSR)

# --------------------

FROM node:24-alpine AS node-base
WORKDIR /app
RUN corepack enable pnpm

# --------------------

FROM node-base AS node-deps
WORKDIR /app
COPY pnpm-lock.yaml pnpm-workspace.yaml package.json ./
COPY packages/ui/package.json ./packages/ui/
COPY packages/types/package.json ./packages/types/
COPY apps/frontend-pos/package.json ./apps/frontend-pos/
COPY apps/frontend-admin/package.json ./apps/frontend-admin/
COPY apps/frontend-home/package.json ./apps/frontend-home/
COPY apps/frontend-tenant/package.json ./apps/frontend-tenant/
RUN pnpm i --frozen-lockfile

# --------------------

FROM node-deps AS node-builder
WORKDIR /app
COPY ./packages ./packages
RUN pnpm --filter @foodbasket/ui build

# --------------------

FROM node-builder AS frontend-pos-builder
WORKDIR /app
COPY ./apps/frontend-pos ./apps/frontend-pos
RUN pnpm --filter @foodbasket/pos build

FROM caddy:2.11-alpine AS frontend-pos
WORKDIR /srv
COPY --from=frontend-pos-builder /app/apps/frontend-pos/Caddyfile /etc/caddy/Caddyfile
COPY --from=frontend-pos-builder /app/apps/frontend-pos/dist /srv
EXPOSE 80

# --------------------

FROM node-builder AS frontend-admin-builder
WORKDIR /app
COPY ./apps/frontend-admin ./apps/frontend-admin
RUN pnpm --filter @foodbasket/admin build

FROM caddy:2.11-alpine AS frontend-admin
WORKDIR /srv
COPY --from=frontend-admin-builder /app/apps/frontend-admin/Caddyfile /etc/caddy/Caddyfile
COPY --from=frontend-admin-builder /app/apps/frontend-admin/dist /srv
EXPOSE 80

# --------------------

FROM node-builder AS frontend-home-builder
WORKDIR /app
COPY ./apps/frontend-home ./apps/frontend-home
RUN pnpm --filter foodbasket build

FROM node-base AS frontend-home-deps
WORKDIR /app
COPY pnpm-lock.yaml pnpm-workspace.yaml package.json ./
COPY packages/ui/package.json ./packages/ui/
COPY packages/types/package.json ./packages/types/
COPY apps/frontend-home/package.json ./apps/frontend-home/
RUN pnpm i --prod --frozen-lockfile

FROM node-base AS frontend-home
WORKDIR /app
ENV NODE_ENV=production HOST=0.0.0.0
COPY --from=frontend-home-deps /app/node_modules ./node_modules
COPY --from=frontend-home-deps /app/apps/frontend-home/node_modules ./apps/frontend-home/node_modules
COPY --from=frontend-home-builder /app/packages ./packages
COPY --from=frontend-home-builder /app/apps/frontend-home/dist ./apps/frontend-home/dist
COPY --from=frontend-home-builder /app/apps/frontend-home/package.json ./apps/frontend-home/
EXPOSE 3000

CMD ["node", "apps/frontend-home/dist/server/entry.mjs"]

# --------------------

FROM node-builder AS frontend-tenant-builder
WORKDIR /app
COPY ./apps/frontend-tenant ./apps/frontend-tenant
RUN pnpm --filter @foodbasket/tenant build

FROM node-base AS frontend-tenant-deps
WORKDIR /app
COPY pnpm-lock.yaml pnpm-workspace.yaml package.json ./
COPY packages/ui/package.json ./packages/ui/
COPY packages/types/package.json ./packages/types/
COPY apps/frontend-tenant/package.json ./apps/frontend-tenant/
RUN pnpm i --prod --frozen-lockfile

FROM node-base AS frontend-tenant
WORKDIR /app
ENV NODE_ENV=production HOST=0.0.0.0
COPY --from=frontend-tenant-deps /app/node_modules ./node_modules
COPY --from=frontend-tenant-deps /app/apps/frontend-tenant/node_modules ./apps/frontend-tenant/node_modules
COPY --from=frontend-tenant-builder /app/packages ./packages
COPY --from=frontend-tenant-builder /app/apps/frontend-tenant/dist ./apps/frontend-tenant/dist
COPY --from=frontend-tenant-builder /app/apps/frontend-tenant/package.json ./apps/frontend-tenant/
EXPOSE 3000

CMD ["node", "apps/frontend-tenant/dist/server/entry.mjs"]
