# frozen_string_literal: true

require "mkmf"
require "rb_sys/mkmf"

default_profile = ENV.fetch("CARGO_PROFILE", "release")

create_rust_makefile("ts_pack_core_rb") do |config|
  config.profile = default_profile.to_sym
  # extconf.rb and Cargo.toml are siblings under ext/ts_pack_core_rb/native/; rb_sys interprets
  # ext_dir relative to extconf.rb, so "." finds the sibling Cargo.toml. "native" would
  # resolve to native/native/Cargo.toml and break `gem install` on end-user machines.
  config.ext_dir = "."
end
