Gem::Specification.new do |s|
  s.name     = "classmate"
  s.version  = "0.1.0"
  s.authors  = "George Claghorn"
  s.email    = "georgeclaghorn@gmail.com"
  s.summary  = "CSS toolkit"
  s.homepage = "https://github.com/georgeclaghorn/classmate"
  s.license  = "MIT"

  s.required_ruby_version = ">= 3.1"
  s.required_rubygems_version = "> 3.3"

  s.files = Dir["Cargo.toml", "Cargo.lock", "src/**/*", "lib/**/*"]
  s.extensions = ["Cargo.toml"]

  s.add_development_dependency "rake",  "~> 13.0"
  s.add_development_dependency "rspec", "~> 3.11"
end
