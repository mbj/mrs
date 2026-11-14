# frozen_string_literal: true

require 'pg_ephemeral'

RSpec.describe PgEphemeral do
  describe '.version' do
    let(:status) { instance_double(Process::Status, success?: true) }
    let(:stdout) { "pg-ephemeral 10.20.30\n" }

    before do
      allow(Open3).to receive(:capture3).and_return([stdout, '', status])
    end

    it 'parses version without prerelease' do
      expect(PgEphemeral.version).to eq('10.20.30')
    end

    context 'with prerelease' do
      let(:stdout) { "pg-ephemeral 1.2.3-rc1\n" }

      it 'converts to ruby convention' do
        expect(PgEphemeral.version).to eq('1.2.3.rc1')
      end
    end

    context 'with invalid version format' do
      let(:stdout) { "pg-ephemeral invalid\n" }

      it 'raises error' do
        expect { PgEphemeral.version }.to raise_error(RuntimeError, 'Failed to parse version from pg-ephemeral binary')
      end
    end
  end

  describe '.platform_supported?' do
    it 'calls pg-ephemeral binary with platform support argument' do
      status = instance_double(Process::Status, success?: true)

      expect(Open3).to receive(:capture2)
        .with(PgEphemeral.binary_path, 'platform', 'support')
        .and_return(['', status])

      expect(PgEphemeral.platform_supported?).to be(true)
    end

    it 'returns true when platform command exits with 0' do
      status = instance_double(Process::Status, success?: true)
      allow(Open3).to receive(:capture2).and_return(['', status])

      expect(PgEphemeral.platform_supported?).to be(true)
    end

    it 'returns false when platform command exits with non-zero' do
      status = instance_double(Process::Status, success?: false)
      allow(Open3).to receive(:capture2).and_return(['', status])

      expect(PgEphemeral.platform_supported?).to be(false)
    end
  end

  describe '.start' do
    let(:result_pipe) { IO.pipe }
    let(:control_pipe) { IO.pipe }

    after do
      (result_pipe + control_pipe).each { |io| io.close unless io.closed? }
    end

    it 'passes the result write end and control read end to the server process' do
      result_write = result_pipe.last
      control_read = control_pipe.first
      result_fd = result_write.fileno
      control_fd = control_read.fileno

      allow(IO).to receive(:pipe).and_return(result_pipe, control_pipe)

      expect(Process).to receive(:spawn).with(
        PgEphemeral.binary_path,
        'integration-server',
        '--instance', 'main',
        '--result-fd', result_fd.to_s,
        '--control-fd', control_fd.to_s,
        result_fd => result_write,
        control_fd => control_read
      ) do
        result_write.puts('{"url":"postgres://example"}')
        42
      end

      expect(PgEphemeral.start.url).to eq('postgres://example')
    end
  end
end
