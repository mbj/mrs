FROM docker.io/ruby:3.4-alpine

RUN apk add --no-cache \
    postgresql-dev \
    build-base \
    tzdata

WORKDIR /app

COPY Gemfile Gemfile.lock ./
RUN bundle install

COPY . .

CMD ["ruby", "test_connection.rb"]
