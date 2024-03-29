# Build target base #
#####################
FROM node:20-alpine AS base
WORKDIR /app
ARG NODE_ENV=production
ENV PATH=/app/node_modules/.bin:$PATH \
    NODE_ENV="$NODE_ENV"
RUN apk --no-cache add curl && \
    rm -rf /var/cache/apk/*
COPY package.json panda.config.ts /app/

# Build target dependencies #
#############################
FROM base AS dependencies
# Install dependencies
RUN yarn install --production=false \
    && yarn cache clean


# Build target builder #
########################
FROM base AS builder
COPY --from=dependencies /app/node_modules /app/node_modules
COPY . /app
RUN yarn panda generate \
    && yarn build \
    && rm -rf node_modules

# Build target production #
###########################
FROM base AS production
EXPOSE 3000
COPY --from=builder /app/package.json .
COPY --from=builder /app/next.config.js ./
COPY --from=builder /app/public ./public
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static

ENTRYPOINT ["node", "server.js"]
