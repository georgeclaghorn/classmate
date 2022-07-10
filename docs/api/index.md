# Classmate API

Classmate is a [CSS] toolkit for Ruby. It parses, transforms, and generates CSS.

It’s based on [@parcel/css], a browser-grade CSS parser, and written in [Rust] as a Ruby native extension.

The primary Classmate interfaces are the [`Classmate::Stylesheet`](./classes/Stylesheet.md) and
[`Classmate::StyleAttribute`](./classes/StyleAttribute.md) classes. `Classmate::Stylesheet`
represents a CSS stylesheet—the contents of a `.css` file or an HTML `<style>` element.
`Classmate::StyleAttribute` represents a CSS style attribute, as might be found in HTML or SVG documents.

[CSS]: https://developer.mozilla.org/en-US/docs/Web/CSS
[@parcel/css]: https://github.com/parcel-bundler/parcel-css
[Rust]: https://www.rust-lang.org

## Classes

* [`Classmate::Stylesheet`](./classes/Stylesheet.md)
* [`Classmate::StyleAttribute`](./classes/StyleAttribute.md)
* [`Classmate::Error`](./classes/Error.md)
* [`Classmate::ParseError`](./classes/ParseError.md)
* [`Classmate::MinifyError`](./classes/MinifyError.md)
* [`Classmate::PrintError`](./classes/PrintError.md)

## Constants

* [`Classmate::VERSION`](./constants/VERSION.md)
