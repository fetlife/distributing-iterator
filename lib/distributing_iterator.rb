# frozen_string_literal: true

require_relative "distributing_iterator/version"

module DistributingIterator
  RUBY_API_VERSION = RUBY_VERSION.split(".")[0, 2].join(".").freeze
end

begin
  [
    "distributing_iterator/#{DistributingIterator::RUBY_API_VERSION}/distributing_iterator",
    "distributing_iterator/distributing_iterator",
    "../ext/distributing_iterator/distributing_iterator"
  ].each do |path|
    begin
      require_relative path
      break
    rescue LoadError
      next
    end
  end
end
