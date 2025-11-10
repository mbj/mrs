#!/usr/bin/env ruby

require 'pg'

# DATABASE_URL should be set by pg-ephemeral run-env
database_url = ENV.fetch('DATABASE_URL')

puts "Connecting to: #{database_url}"

# Connect using libpq-style DATABASE_URL
# The pg gem natively supports libpq connection strings
conn = PG.connect(database_url)

# Test the connection
conn.exec("SELECT 1")

puts "SUCCESS: Connected to PostgreSQL successfully"
conn.close
