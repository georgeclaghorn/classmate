require "bundler/setup"
require "bundler/gem_tasks"
require "rspec/core/rake_task"
require "rake/extensiontask"

GEMSPEC = Bundler.load_gemspec("classmate.gemspec")

PLATFORMS = %w[ x86_64-linux x86_64-darwin arm64-darwin ]

ENV["RUBY_CC_VERSION"] = "3.1.0"

task default: :spec

RSpec::Core::RakeTask.new spec: :compile do |t|
  t.verbose = false
end

Rake::ExtensionTask.new("classmate", GEMSPEC) do |ext|
  ext.lib_dir = "lib"
  ext.source_pattern = "*.{rs,toml}"

  ext.cross_compile  = true
  ext.cross_platform = PLATFORMS

  ext.cross_compiling do |gemspec|
    gemspec.required_rubygems_version = nil

    gemspec.files.reject! { |file| file.match?(/\Aext\//) }
    gemspec.dependencies.reject! { |dependency| dependency.name == "rb_sys" }
  end
end

namespace :gem do
  task :prepare do
    require "rake_compiler_dock"

    sh "bundle package --all"
  end

  PLATFORMS.each do |platform|
    multitask native: "gem:native:#{platform}"

    task "native:#{platform}": :prepare do
      RakeCompilerDock.sh <<~SH, image: "georgeclaghorn/classmate-build:#{platform}", platform: platform
        bundle --local
        rake native:#{platform} gem
      SH
    end
  end
end

task console: :compile do
  ruby "bin/console"
end
