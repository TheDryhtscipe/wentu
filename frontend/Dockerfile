# syntax=docker/dockerfile:1.6

ARG NODE_VERSION=20
ARG VITE_API_URL=http://backend:3000

FROM node:${NODE_VERSION}-alpine AS build
WORKDIR /app

ENV VITE_API_URL=${VITE_API_URL}

COPY package.json package-lock.json ./
RUN npm ci

COPY . .
RUN npm run build

FROM nginx:1.27-alpine AS runtime
WORKDIR /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist ./

EXPOSE 4173

CMD ["nginx", "-g", "daemon off;"]
