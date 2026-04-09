ARG BASE_IMAGE
FROM ${BASE_IMAGE}

RUN apk add --no-cache openssl

WORKDIR /app

COPY package.json ./
RUN npm install

COPY . .

RUN npx prisma generate

CMD ["node", "test_connection.js"]
