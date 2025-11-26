Gem::Specification.new do |spec|
  spec.name          = 'pg-ephemeral'
  spec.version       = '0.0.1'
  spec.authors       = ['Markus Schirp']
  spec.email         = ['mbj@schirp-dso.com']

  spec.summary       = 'Ruby wrapper for pg-ephemeral PostgreSQL testing utility'
  spec.description   = 'Provides ephemeral PostgreSQL instances for testing, wrapping the pg-ephemeral project binary'
  spec.homepage      = 'https://github.com/mbj/mrs/tree/main/pg-ephemeral'
  spec.license       = 'MIT'
  spec.required_ruby_version = '>= 3.2'

  spec.metadata['homepage_uri'] = spec.homepage
  spec.metadata['source_code_uri'] = 'https://github.com/mbj/mrs'
  spec.metadata['changelog_uri'] = 'https://github.com/mbj/mrs/blob/main/pg-ephemeral/CHANGELOG.md'

  spec.files = Dir['lib/**/*', 'bin/*', 'README.md', 'LICENSE.txt']
  spec.require_paths = ['lib']

  spec.add_dependency 'pg', '~> 1.5'

  spec.add_development_dependency 'bundler', '~> 2.0'
  spec.add_development_dependency 'mutant-rspec', '~> 0.13.0'
  spec.add_development_dependency 'rspec', '~> 3.0'
end
