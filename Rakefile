require "bundler/setup"
require "rspec/core/rake_task"

SOEXT = RbConfig::CONFIG["SOEXT"]
DLEXT = RbConfig::CONFIG["DLEXT"]

LIBRARY   = "target/debug/libclassmate.#{SOEXT}"
EXTENSION = "lib/classmate.#{DLEXT}"

task default: :spec

RSpec::Core::RakeTask.new spec: :compile do |t|
  t.verbose = false
end

task console: :compile do |t|
  ruby "bin/console"
end

task compile: EXTENSION

task :clean do
  rm_f EXTENSION
  sh "cargo", "clean"
end

file EXTENSION => LIBRARY do |task|
  rm_f task.name
  sh "cp", task.source, task.name
end

file LIBRARY => FileList["Cargo.toml", "Cargo.lock", "src/**/*.rs"] do
  sh "cargo", "build"
end
