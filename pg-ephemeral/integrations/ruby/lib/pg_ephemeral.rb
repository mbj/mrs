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

  def self.start(instance_name: 'main', config: nil)
    result_read, result_write = IO.pipe
    control_read, control_write = IO.pipe

    command = [binary_path]
    command.concat(['--config-file', config]) if config
    command.concat([
      'integration-server',
      '--instance', instance_name,
      '--result-fd', result_write.fileno.to_s,
      '--control-fd', control_read.fileno.to_s
    ])

    pid = Process.spawn(*command, result_write.fileno => result_write, control_read.fileno => control_read)

    result_write.close
    control_read.close

    config_json = result_read.gets
    result_read.close

    raise 'Failed to read server configuration' unless config_json

    url = JSON.parse(config_json).fetch('url')
    Server.new(url, control_write, pid)
  end

  def self.with_server(instance_name: 'main', config: nil, &block)
    server = start(instance_name: instance_name, config: config)
    begin
      block.call(server)
    ensure
      server.shutdown
    end
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

  class Server
    attr_reader :url

    def initialize(url, control_write, pid)
      @url = url
      @control_write = control_write
      @pid = pid
    end

    def shutdown
      @control_write.close unless @control_write.closed?
      Process.wait(@pid)
    end
  end
end
