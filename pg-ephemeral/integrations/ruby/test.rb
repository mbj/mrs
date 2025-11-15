#!/usr/bin/env ruby
# frozen_string_literal: true

def run_command(*args)
  puts "Running: #{args.join(' ')}"
  system(*args, exception: true)
end

integration_dir = __dir__

expected_version = ENV.fetch('EXPECTED_PG_EPHEMERAL_VERSION')
puts "Using pg-ephemeral version: #{expected_version}"

puts "\nInstalling gem locally..."
gem_file = File.join(integration_dir, "pg-ephemeral-#{expected_version}.gem")

run_command('gem', 'install', gem_file)

puts "\nRunning bundle install..."
Dir.chdir(integration_dir) do
  run_command('bundle', 'install')

  puts "\nRunning RSpec tests..."
  run_command('bundle', 'exec', 'rspec')

  puts "\nRunning Mutant..."
  run_command('bundle', 'exec', 'mutant', 'run')
end

puts "\nLocal integration tests complete!"
