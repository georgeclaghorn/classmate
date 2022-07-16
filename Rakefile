require "bundler/setup"
require "bundler/gem_tasks"
require "rspec/core/rake_task"
require "rake/extensiontask"

GEMSPEC = Bundler.load_gemspec("classmate.gemspec")

PLATFORMS = %w[ x86_64-linux aarch64-linux x86_64-darwin arm64-darwin ]

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
    task native: "gem:native:#{platform}"

    task "native:#{platform}": :prepare do
      RakeCompilerDock.sh <<~SH, image: "georgeclaghorn/classmate-build:#{platform}", platform: platform
        bundle --local
        PROFILE=#{ENV.fetch("PROFILE", "release")} rake native:#{platform} gem
      SH
    end
  end
end

PLATFORMS.each do |platform|
  task "native:#{GEMSPEC.name}:#{platform}" do
    # FIXME: Gem::PackageTask defines a task with the gemspec files as prerequisites, even though
    # Rake::ExtensionTask clears Gem::PackageTask#package_files. This causes e.g.
    # `rake native:aarch64-linux gem` to build an x86_64-linux native extension unnecessarily when
    # run from an x86_64-linux environment, because lib/classmate.so is in the gem task's prerequisites.
    Rake::Task["pkg/#{GEMSPEC.name}-#{GEMSPEC.version}-#{platform}.gem"].tap do |task|
      task.clear_prerequisites
      task.enhance [ "pkg", "pkg/#{GEMSPEC.name}-#{GEMSPEC.version}-#{platform}" ]
    end
  end
end

task :clippy do
  cd "ext/classmate"
  sh "cargo", "clippy", "--no-deps", "--", "-D", "warnings"
end

task console: :compile do
  ruby "bin/console"
end
