# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"
require "rb_sys/extensiontask"

GEMSPEC = Gem::Specification.load("distributing_iterator.gemspec")

task default: :test
task gem: :build

RbSys::ExtensionTask.new("distributing-iterator", GEMSPEC) do |ext|
  ext.lib_dir = "lib/distributing_iterator"
end

Rake::TestTask.new do |t|
  t.deps << "compile:dev"
  t.libs << "lib"
  t.libs << "test"
  t.test_files = FileList["test/**/*_test.rb"]
end
