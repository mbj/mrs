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
end
