require_relative "lib/classmate/version"

Gem::Specification.new do |s|
  s.name     = "classmate"
  s.version  = Classmate::VERSION
  s.authors  = "George Claghorn"
  s.email    = "georgeclaghorn@gmail.com"
  s.summary  = "CSS toolkit"
  s.homepage = "https://github.com/georgeclaghorn/classmate"
  s.license  = "MIT"

  s.required_ruby_version = ">= 3.1"

  s.files = Dir["ext/**/*", "lib/**/*"]
  s.extensions = ["ext/classmate/extconf.rb"]

  s.add_dependency "rb_sys", "~> 0.9.3"

  s.add_development_dependency "rake", "~> 13.0"
  s.add_development_dependency "rake-compiler", "~> 1.2"
  s.add_development_dependency "rake-compiler-dock", "~> 1.2"
  s.add_development_dependency "rspec", "~> 3.11"
end
