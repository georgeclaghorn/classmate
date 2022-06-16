# `Classmate::Stylesheet`

A `Classmate::Stylesheet` represents a [CSS] stylesheet parsed via [@parcel/css]. A `Stylesheet` is
created by calling [`.parse`](#parse) with a textual stylesheet as a `String`.

A `Stylesheet` implements a few transformations:
* [`#minify`](#minify) reduces its size by combining longhand properties into shorthands, combining
  rules with equivalent bodies, removing default values, and more.
* [`#proxy`](#proxy) rewrites subresource URLs.

A `Stylesheet` can be converted back to a String with [`#to_css`](#to_css) or [`to_s`](#to_s).

[CSS]: https://developer.mozilla.org/en-US/docs/Web/CSS
[@parcel/css]: https://github.com/parcel-bundler/parcel-css

## Class methods

<h3 id="parse"><code>.parse(code, filename: nil) → Stylesheet</code></h3>

Parses a `String` containing a CSS stylesheet into a `Stylesheet`. The resulting `Stylesheet` can
be transformed and converted back to a `String` via its instance methods.

#### Arguments

* `code`: a `String` containing a CSS stylesheet.
* `filename`: a `String` filename for the stylesheet, used only for error messages (optional).

#### Errors

Raises [`TypeError`][TypeError] if `code` is anything other than a `String`, or if `filename` is
anything other than a `String` or `nil`.

Raises [`Classmate::ParseError`][ParseError] when `code` cannot be parsed as CSS.

[TypeError]: https://ruby-doc.org/core-3.1.0/TypeError.html
[ParseError]: ./ParseError.md

## Instance methods

<h3 id="minify"><code>#minify → self</code></h3>

Optimizes the size of the stylesheet in place. According to the [@parcel/css README], minification involves:

> * Combining longhand properties into shorthands where possible.
> * Merging adjacent rules with the same selectors or declarations when it is safe to do so.
> * Combining CSS transforms into a single matrix or vice versa when smaller.
> * Removing vendor prefixes that are not needed, based on the provided browser targets.
> * Reducing calc() expressions where possible.
> * Converting colors to shorter hex notation where possible.
> * Minifying gradients.
> * Minifying CSS grid templates.
> * Normalizing property value order.
> * Removing default property sub-values which will be inferred by browsers.
> * Many micro-optimizations, e.g. converting to shorter units, removing unnecessary quotation marks, etc.

Returns self for convenient chaining.

[@parcel/css README]: https://github.com/parcel-bundler/parcel-css/blob/3dd150d90c11d8f963e55866dbee4c9f42e6cf42/README.md

#### Errors

Raises [`Classmate::MinifyError`][MinifyError] when the stylesheet cannot be minified.

[MinifyError]: ./MinifyError.md

#### Examples

```ruby
stylesheet = Classmate::Stylesheet.parse(<<~CSS)
  .foo {
    padding-top: 5px;
    padding-left: 5px;
    padding-right: 5px;
    padding-bottom: 5px;
  }

  .bar {
    padding: 5px;
  }
CSS

stylesheet.minify

# Output:
#
#   .foo, .bar {
#     padding: 5px;
#   }
puts stylesheet.to_css
```

Chaining:

```ruby
# Output:
#
#   .foo, .bar {
#     padding: 5px;
#   }
puts Classmate::Stylesheet.parse(<<~CSS).minify.to_css
  .foo {
    padding-top: 5px;
    padding-left: 5px;
    padding-right: 5px;
    padding-bottom: 5px;
  }

  .bar {
    padding: 5px;
  }
CSS
```

<h3 id="proxy"><code>#proxy(&block) → self</code></h3>

Rewrites the stylesheet's subresource URLs in place.

Performs a depth-first search for URLs in the stylesheet. For each URL found, the given block is
called with the original URL. If the block returns a `String`, that string is substituted for the
original URL. If the block returns `nil`, the original URL is preserved.

This is useful for ensuring third-party assets are loaded through a trusted proxy server.

Returns self for convenient chaining.

#### Errors

Raises [`LocalJumpError`][LocalJumpError] if no block is provided.

Raises [`TypeError`][TypeError] if the provided block returns anything other than a `String` or `nil`.

Raises any error the provided block raises.

If an error is raised, the stylesheet may be partially modified.

[LocalJumpError]: https://ruby-doc.org/core-3.1.0/LocalJumpError.html

#### Examples

```ruby
stylesheet = Classmate::Stylesheet.parse(<<~CSS)
  @font-face {
    font-family: Inter;
    src: url("https://example.com/Inter.woff2");
    font-weight: 300;
  }

  @media screen and (min-width: 900px) {
    body {
      background-image: url("https://example.com/foo.png");
    }
  }

  body {
    background-image: url("https://example.com/bar.png");
  }
CSS

stylesheet.proxy { |url| "https://proxy.example.com/#{url}" }

# Output:
#
#   @font-face {
#     font-family: Inter;
#     src: url("https://proxy.example.com/https://example.com/Inter.woff2");
#     font-weight: 300;
#   }
#
#   @media screen and (min-width: 900px) {
#     body {
#       background-image: url("https://proxy.example.com/https://example.com/foo.png");
#     }
#   }
#
#   body {
#     background-image: url("https://proxy.example.com/https://example.com/bar.png");
#   }
puts stylesheet.to_css
```

<h3 id="to_css"><code>#to_css(minify: false) → String</code></h3>

Serializes the stylesheet to a `String`.

When `minify` is true, whitespace and other unnecessary characters are removed.

*Aliased as [`#to_s`](#to_s).*

#### Errors

Raises [`Classmate::PrintError`][PrintError] when the stylesheet cannot be converted to a `String`.

[PrintError]: ./PrintError.md

<h3 id="to_s"><code>#to_s(minify: false) → String</code></h3>

*Alias for [`#to_css`](#to_css).*
