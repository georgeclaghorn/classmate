# Classmate

Classmate is a CSS toolkit for Ruby. It parses, transforms, and generates CSS.

It is based on [@parcel/css], a browser-grade CSS parser, and written in Rust as a Ruby native extension.

[@parcel/css]: https://github.com/parcel-bundler/parcel-css

**Quick links**

* [API documentation]
* [Releases/changelog]
* [`classmate` on Rubygems.org]

[API documentation]: https://github.com/georgeclaghorn/classmate/blob/main/docs/api/index.md
[Releases/changelog]: https://github.com/georgeclaghorn/classmate/releases
[`classmate` on Rubygems.org]: https://rubygems.org/gems/classmate

## Installation

Run `gem install classmate` or `bundle add classmate`.

## Usage

Load Classmate:

```ruby
require "classmate"
```

Parse a stylesheet:

```ruby
stylesheet = Classmate::Stylesheet.parse(<<~CSS)
  body {
    background-image: url("https://example.com/background.png");
  }
CSS
```

Parse a style attribute (from an HTML or SVG element, for example):

```ruby
attribute = Classmate::StyleAttribute.parse('background-image: url("https://example.com/background.png");')
```

Minify a stylesheet or style attribute. Combine longhand properties into shorthands, merge similar
style rules, omit default values, remove unnecessary quotes, and more:

```ruby
stylesheet.minify
attribute.minify
```

Rewrite each URL in a stylesheet or style attribute to point to a proxy server:

```ruby
stylesheet.proxy { |url| "https://proxy.example.com/#{url}" }
attribute.proxy { |url| "https://proxy.example.com/#{url}" }
```

Convert a stylesheet or style attribute to CSS:

```ruby
stylesheet.to_css
stylesheet.to_s

attribute.to_css
attribute.to_s
```

**See the [API documentation] for more advanced usage.**

## Requirements

Classmate requires Ruby 3.1 or later.

Rust 1.61 or later is required to compile Classmate’s native extension.

### Native gems

A native gem contains a precompiled native extension and does not require a compiler to install.

Classmate native gems are available for the following platforms:

* Linux: x86-64 and aarch64
* macOS: x86-64 and arm64

No special installation method is needed for native gems. Rubygems and Bundler automatically
download them on supported platforms.

## License

Classmate is released under the terms of the MIT License. See `LICENSE` for details.

Releases of Classmate include third-party library dependencies. Please consult `DEPENDENCIES.md`
for a listing of these dependencies and the licenses thereof.
