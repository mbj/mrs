# frozen_string_literal: true

require 'open3'
require 'pathname'
require 'json'
require 'pg'
require 'rubygems'

module PgEphemeral
  def self.binary_path
    gem_dir = Gem::Specification.find_by_name('pg-ephemeral').gem_dir
    File.join(gem_dir, 'bin', 'pg-ephemeral')
  end

  def self.version
    stdout, _stderr, status = Open3.capture3(binary_path, '--version')

    unless status.success?
      raise "Failed to get version from pg-ephemeral binary"
    end

    match = stdout.match(/\Apg-ephemeral (?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)(-(?<pre>.+))?\n\z/)

    raise "Failed to parse version from pg-ephemeral binary" unless match

    version = "#{match[:major]}.#{match[:minor]}.#{match[:patch]}"
    version += ".#{match[:pre]}" if match[:pre]
    version
  end

  def self.platform_supported?
    _stdout, status = Open3.capture2(binary_path, 'platform', 'support')
    status.success?
  end

  def self.with_server(instance_name: 'main', config: nil, &block)
    run_server(instance_name, config, &block)
  end

  def self.with_connection(instance_name: 'main', config: nil, &block)
    with_server(instance_name: instance_name, config: config) do |server|
      connection = PG.connect(server.url)

      begin
        block.call(connection)
      ensure
        connection.close
      end
    end
  end

  def self.run_server(instance_name, config, &block)
    command = [binary_path]

    if config
      command.concat(['--config-file', config])
    end

    command.concat([
      'integration-server',
      '--instance', instance_name,
      '--protocol', 'v0'
    ])

    Open3.popen2(*command) do |stdin, stdout, wait_thread|
      config_json = stdout.gets

      raise 'Failed to read server configuration' unless config_json

      url = JSON.parse(config_json).fetch('url')
      server = Server.new(url, stdin, wait_thread)

      begin
        block.call(server)
      ensure
        server.shutdown
      end
    end
  end
  private_class_method :run_server

  class Server
    attr_reader :url

    def initialize(url, stdin, wait_thread)
      @url = url
      @stdin = stdin
      @wait_thread = wait_thread
    end

    def shutdown
      @stdin.close unless @stdin.closed?
      @wait_thread.value
    end
  end
end
