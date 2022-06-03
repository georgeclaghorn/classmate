require "bundler/setup"
require "bundler/gem_tasks"
require "rspec/core/rake_task"
require "rake/extensiontask"

task default: :spec

RSpec::Core::RakeTask.new spec: :compile do |t|
  t.verbose = false
end

Rake::ExtensionTask.new("classmate") do |ext|
  ext.lib_dir = "lib"
  ext.source_pattern = "*.{rs,toml}"
end

task console: :compile do
  ruby "bin/console"
end
