# Classmate

Classmate is a CSS toolkit for Ruby. It parses, transforms, and generates CSS.

It is based on [@parcel/css], a browser-grade CSS parser, and written in Rust as a Ruby native extension.

[@parcel/css]: https://github.com/parcel-bundler/parcel-css

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

## Requirements

Classmate requires Ruby 3.1 or later.

The following are required to compile Classmate’s native extension:

* Rubygems 3.3.10 or later
* Rust 1.61 or later

### Native gems

A native gem contains a precompiled native extension and does not require a compiler to install.

Classmate native gems are available for the following platforms:

* Linux: x86-64
* macOS: x86-64 and arm64

No special installation method is needed for native gems. Rubygems and Bundler automatically
download them on supported platforms.

## License

Classmate is released under the terms of the MIT License. See `LICENSE` for details.

Releases of Classmate include third-party library dependencies. Please consult `DEPENDENCIES.md`
for a listing of these dependencies and the licenses thereof.
