FROM docker.io/node:22-alpine

RUN apk add --no-cache openssl

WORKDIR /app

COPY package.json ./
RUN npm install

COPY . .

RUN npx prisma generate

CMD ["node", "test_connection.js"]
