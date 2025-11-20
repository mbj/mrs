# frozen_string_literal: true

require 'pg_ephemeral'

RSpec.describe PgEphemeral do
  describe '.platform_supported?' do
    it 'returns a boolean value' do
      expect([true, false]).to include(PgEphemeral.platform_supported?)
    end
  end

  describe '.version' do
    it 'returns the expected pg-ephemeral version' do
      expected_version = ENV.fetch('EXPECTED_PG_EPHEMERAL_VERSION')

      expect(PgEphemeral.version).to eq(expected_version)
    end

    it 'returns a non-empty string' do
      expect(PgEphemeral.version).not_to be_empty
    end

    it 'returns a semantic version format' do
      expect(PgEphemeral.version).to match(/\A\d+\.\d+\.\d+\z/)
    end

    it 'raises an error when binary execution fails' do
      status = instance_double(Process::Status, success?: false)
      allow(Open3).to receive(:capture3).and_return(['', '', status])

      expect { PgEphemeral.version }.to raise_error(RuntimeError, 'Failed to get version from pg-ephemeral binary')
    end
  end

  describe '.with_server' do
    before do
      skip 'Platform does not support pg-ephemeral' unless PgEphemeral.platform_supported?
    end

    it 'yields a server with url' do
      PgEphemeral.with_server do |server|
        expect(server).to be_a(PgEphemeral::Server)
        expect(server.url).to be_a(String)
        expect(server.url).to match(/\Apostgres:\/\//)
      end
    end

    it 'returns the block result' do
      result = PgEphemeral.with_server { |_server| 42 }

      expect(result).to eq(42)
    end

    it 'supports custom instance names' do
      PgEphemeral.with_server(instance_name: 'custom') do |server|
        expect(server.url).to be_a(String)
      end
    end

    it 'allows connecting to the database and executing queries' do
      PgEphemeral.with_server do |server|
        connection = PG.connect(server.url)
        result = connection.exec('SELECT 1 AS value')

        expect(result.first['value']).to eq('1')

        connection.close
      end
    end

    it 'accepts a custom config file path' do
      config_path = File.join(__dir__, '../../database.toml')

      PgEphemeral.with_server(config: config_path) do |server|
        expect(server.url).to be_a(String)
        expect(server.url).to match(/\Apostgres:\/\//)
      end
    end
  end

  describe '.with_connection' do
    before do
      skip 'Platform does not support pg-ephemeral' unless PgEphemeral.platform_supported?
    end

    it 'yields a PG connection' do
      PgEphemeral.with_connection do |connection|
        expect(connection).to be_a(PG::Connection)

        result = connection.exec('SELECT 1 AS value')
        expect(result.first['value']).to eq('1')
      end
    end

    it 'returns the block result' do
      result = PgEphemeral.with_connection { |_connection| 42 }

      expect(result).to eq(42)
    end

    it 'supports custom instance names' do
      PgEphemeral.with_connection(instance_name: 'custom') do |connection|
        expect(connection).to be_a(PG::Connection)
      end
    end

    it 'closes the connection after the block' do
      connection_ref = nil

      PgEphemeral.with_connection do |connection|
        connection_ref = connection
        expect(connection.finished?).to be(false)
      end

      expect(connection_ref.finished?).to be(true)
    end

    it 'accepts a custom config file path' do
      config_path = File.join(__dir__, '../../database.toml')

      PgEphemeral.with_connection(config: config_path) do |connection|
        expect(connection).to be_a(PG::Connection)
        result = connection.exec('SELECT 1 AS value')
        expect(result.first['value']).to eq('1')
      end
    end
  end
end
