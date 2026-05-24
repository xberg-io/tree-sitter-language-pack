# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "tree_sitter_language_pack"
  spec.version = "1.9.0.pre.rc.2"
  spec.authors       = ["Kreuzberg Team"]
  spec.summary       = "Pre-compiled tree-sitter grammars for 305 programming languages"
  spec.description   = "Pre-compiled tree-sitter grammars for 305 programming languages"
  spec.homepage      = "https://github.com/kreuzberg-dev/tree-sitter-language-pack"
  spec.license       = "MIT"
  spec.required_ruby_version = ">= 3.2.0"
  spec.metadata["keywords"] = %w[tree-sitter parser syntax language-pack].join(",")
  spec.metadata["rubygems_mfa_required"] = "true"

  spec.files         = Dir.glob(%w[lib/**/* ext/**/* sig/**/* Steepfile])
  spec.require_paths = ["lib"]
  spec.extensions    = ["ext/ts_pack_core_rb/extconf.rb"]

  spec.add_dependency "rb_sys", "~> 0.9"
  spec.add_dependency "sorbet-runtime", "~> 0.5"
end
