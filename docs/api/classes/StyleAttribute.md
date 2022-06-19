# `Classmate::StyleAttribute`

A `Classmate::StyleAttribute` represents a [CSS] [style attribute] parsed via [@parcel/css].
A `StyleAttribute` is created by calling [`.parse`](#parse) with a textual style attribute as a `String`.

A `StyleAttribute` implements a few transformations:
* [`#minify`](#minify) reduces its size by combining longhand properties into shorthands, removing
  default values, and more.
* [`#proxy`](#proxy) rewrites subresource URLs.

A `StyleAttribute` can be converted back to a String with [`#to_css`](#to_css) or [`to_s`](#to_s).

[CSS]: https://developer.mozilla.org/en-US/docs/Web/CSS
[style attribute]: https://drafts.csswg.org/css-style-attr/#style-attribute
[@parcel/css]: https://github.com/parcel-bundler/parcel-css

## Class methods

<h3 id="parse"><code>.parse(code) → StyleAttribute</code></h3>

Parses a `String` containing a CSS style attribute into a `StyleAttribute`. The resulting
`StyleAttribute` can be transformed and converted back to a `String` via its instance methods.

#### Arguments

* `code`: a `String` containing a CSS style attribute.

#### Errors

Raises [`TypeError`][TypeError] if `code` is anything other than a `String`.

Raises [`Classmate::ParseError`][ParseError] when `code` cannot be parsed as a CSS style attribute.

[TypeError]: https://ruby-doc.org/core-3.1.0/TypeError.html
[ParseError]: ./ParseError.md

## Instance methods

<h3 id="minify"><code>#minify → self</code></h3>

Optimizes the size of the style attribute in place. According to the [@parcel/css README],
minification involves:

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

Raises [`Classmate::MinifyError`][MinifyError] when the style attribute cannot be minified.

[MinifyError]: ./MinifyError.md

#### Examples

```ruby
attribute = Classmate::StyleAttribute.parse(<<~CSS)
  padding-top: 5px;
  padding-left: 5px;
  padding-right: 5px;
  padding-bottom: 5px;
CSS

attribute.minify

# Output:
#
#   padding: 5px
puts attribute.to_css
```

Chaining:

```ruby
# Output:
#
#   padding: 5px
puts Classmate::StyleAttribute.parse(<<~CSS).minify.to_css
  padding-top: 5px;
  padding-left: 5px;
  padding-right: 5px;
  padding-bottom: 5px;
CSS
```

<h3 id="proxy"><code>#proxy(&block) → self</code></h3>

Rewrites the style attribute's subresource URLs in place.

Performs a depth-first search for URLs in the style attribute. For each URL found, the given block
is called with the original URL. If the block returns a `String`, that string is substituted for
the original URL. If the block returns `nil`, the original URL is preserved.

This is useful for ensuring third-party assets are loaded through a trusted proxy server.

Returns self for convenient chaining.

#### Errors

Raises [`ArgumentError`][ArgumentError] if no block is provided.

Raises [`TypeError`][TypeError] if the provided block returns anything other than a `String` or `nil`.

Raises any error the provided block raises.

If an error is raised, the style attribute may be partially modified.

[ArgumentError]: https://ruby-doc.org/core-3.1.0/ArgumentError.html

#### Examples

```ruby
attribute = Classmate::StyleAttribute.parse('background-image: url("https://example.com/foo.png")')

attribute.proxy { |url| "https://proxy.example.com/#{url}" }

# Output:
#
#   background-image: url("https://proxy.example.com/https://example.com/foo.png")
puts attribute.to_css
```

<h3 id="to_css"><code>#to_css(minify: false) → String</code></h3>

Serializes the style attribute to a `String`.

When `minify` is true, whitespace and other unnecessary characters are removed.

*Aliased as [`#to_s`](#to_s).*

#### Errors

Raises [`Classmate::PrintError`][PrintError] when the style attribute cannot be converted to a `String`.

[PrintError]: ./PrintError.md

##### Examples

```ruby
attribute = Classmate::StyleAttribute.parse(<<~CSS)
  font-family: Helvetica Neue, Helvetica, Arial, sans-serif;
  font-size: 16px;
CSS

# Output:
#
#   font-family: Helvetica Neue, Helvetica, Arial, sans-serif; font-size: 16px
puts attribute.to_css

# Output:
#
#   font-family:Helvetica Neue,Helvetica,Arial,sans-serif;font-size:16px
puts attribute.to_css(minify: true)
```

<h3 id="to_s"><code>#to_s(minify: false) → String</code></h3>

*Alias for [`#to_css`](#to_css).*
