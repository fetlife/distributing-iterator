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
  spec.required_ruby_version = ">= 3.2"

  spec.platform = Gem::Platform::RUBY

  spec.files = Dir[
    "lib/**/*.rb",
    "ext/**/*",
    ".cargo/*",
    "Cargo.toml",
    "Cargo.lock",
    "README.md",
    "LICENSE"
  ].reject do |path|
    path.match?(%r{\A(?:lib/distributing_iterator/(?:.*\.(?:bundle|so|dll|dylib))|ext/distributing_iterator/(?:Makefile|mkmf\.log|target/|.*\.(?:bundle|so|dll)))})
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/distributing_iterator/extconf.rb"]
  spec.add_dependency "rb_sys", "~> 0.9"
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rake-compiler", "~> 1.2"
  spec.add_development_dependency "minitest", "~> 5.0"

  spec.metadata = {
    "github_repo" => "ssh://github.com/fetlife/distributing-iterator",
    "cargo_crate_name" => "distributing-iterator"
  }
end
