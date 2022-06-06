require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile "classmate" do |ext|
  ext.profile = ENV.fetch("PROFILE", :dev).to_sym
end
