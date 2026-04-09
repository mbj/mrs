ARG BASE_IMAGE
FROM ${BASE_IMAGE}

RUN apk add --no-cache \
    postgresql-dev \
    build-base \
    tzdata

WORKDIR /app

COPY Gemfile ./
RUN bundle install

COPY . .

CMD ["ruby", "test_connection.rb"]
