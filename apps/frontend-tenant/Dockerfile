FROM node:24-alpine AS base
RUN corepack enable pnpm

FROM base AS builder
WORKDIR /app
COPY package.json .
RUN pnpm i
COPY . .
RUN pnpm build 

FROM base AS prod
WORKDIR /app
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/package.json ./package.json

CMD ["node", "./dist/server/entry.mjs"]
