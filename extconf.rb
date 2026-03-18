# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("distributing_iterator") do |config|
  config.ext_dir = __dir__
  config.features = ["ruby_ext"]
  config.profile = ENV.fetch("RB_SYS_CARGO_PROFILE", "release").to_sym
end
