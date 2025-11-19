# frozen_string_literal: true

require 'pg_ephemeral'

RSpec.describe PgEphemeral do
  describe '.platform_supported?' do
    it 'calls pg-ephemeral binary with platform argument' do
      status = instance_double(Process::Status, success?: true)

      expect(Open3).to receive(:capture2)
        .with(PgEphemeral.binary_path, 'platform')
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
