require 'rubygems/package_task'

spec = eval(File.read("distributing_iterator.gemspec"))
GEM_RUBY_VERSION = "3.3.0"
DOCKER_IMAGE = "ruby:#{GEM_RUBY_VERSION}-bullseye"

def compile_cmd(_arch)
  "gem instal gem-compiler; apt-get update && apt-get install -y libclang-dev; curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y; source '/root/.cargo/env'; rake gem:native"
end

gem_task = Gem::PackageTask.new(spec) do |pkg|
  pkg.need_zip = true
  pkg.need_tar = true
end

desc "Generate a pre-compiled native gem for #{RUBY_PLATFORM}"
task "gem:native" => ["gem"] do
  sh "gem compile #{gem_task.package_dir_path}.gem"
end

desc "Generate a pre-compiled native gem for aarch64-linux"
task "gem:native:aarch64-linux" => ["gem"] do
  sh %{docker run --rm --platform linux/arm64 -v $(pwd):/src -w /src #{DOCKER_IMAGE} /bin/bash -c "#{compile_cmd('aarch64-unknown-linux-musl')}"}
end

desc "Generate a pre-compiled native gem for x86_64-linux"
task "gem:native:x86_64-linux" => ["gem"] do
  sh %{docker run --rm --platform linux/amd64 -v $(pwd):/src -w /src #{DOCKER_IMAGE} /bin/bash -c "#{compile_cmd('x86_64-unknown-linux-musl')}"}
end
