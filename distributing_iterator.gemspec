# frozen_string_literal: true

$:.push File.expand_path("lib", __dir__)
require "distributing_iterator/version"

Gem::Specification.new do |spec|
  spec.name = "distributing_iterator"
  spec.version = DistributingIterator::VERSION
  spec.authors = ["Fetlife", "Andrii Dmytrenko"]
  spec.email = ["dev@fetlife.com", "andrii@fetlife.com"]

  spec.summary = "Distributing Iterator"
  spec.description = "Distributing Iterator"
  spec.homepage = "https://github.com/fetlife/distributing-iterator"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.0.0"

  spec.platform = Gem::Platform::RUBY

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir["lib/**/*.rb", "src/**/*.rs", "benches/**/*.rs", ".cargo/*", "Cargo.toml", "Cargo.lock", "README.md", "LICENSE.txt"]
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["Cargo.toml"]
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "gem-compiler"
  spec.add_development_dependency "ffi"
  spec.required_rubygems_version = Gem::Requirement.new(">= 3.4.0") if spec.respond_to? :required_rubygems_version=

  spec.metadata = { "github_repo" => "ssh://github.com/fetlife/distributing-iterator" }

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
