mod common;

mod proxy;
pub use proxy::ProxyVisitor;

pub trait Visitor<E>:
    for<'r, 's> Visit<parcel_css::stylesheet::StyleSheet<'r, 's>, E> +
    for<'r> Visit<parcel_css::stylesheet::StyleAttribute<'r>, E> +
    for<'r> Visit<parcel_css::rules::CssRuleList<'r>, E> +
    for<'r> Visit<parcel_css::rules::CssRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::media::MediaRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::import::ImportRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::style::StyleRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::page::PageRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::supports::SupportsRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::counter_style::CounterStyleRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::document::MozDocumentRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::nesting::NestingRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::viewport::ViewportRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::layer::LayerBlockRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::keyframes::KeyframesRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::keyframes::Keyframe<'r>, E> +
    for<'r> Visit<parcel_css::rules::font_face::FontFaceRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::font_face::FontFaceProperty<'r>, E> +
    for<'r> Visit<parcel_css::rules::font_face::Source<'r>, E> +
    for<'r> Visit<parcel_css::rules::font_face::UrlSource<'r>, E> +
    for<'r> Visit<parcel_css::rules::font_palette_values::FontPaletteValuesRule<'r>, E> +
    for<'r> Visit<parcel_css::rules::font_palette_values::FontPaletteValuesProperty<'r>, E> +
    for<'r> Visit<parcel_css::declaration::DeclarationBlock<'r>, E> +
    for<'r> Visit<parcel_css::properties::Property<'r>, E> +
    for<'r> Visit<parcel_css::properties::background::Background<'r>, E> +
    for<'r> Visit<parcel_css::properties::border_image::BorderImage<'r>, E> +
    for<'r> Visit<parcel_css::properties::list::ListStyle<'r>, E> +
    for<'r> Visit<parcel_css::properties::svg::SVGPaint<'r>, E> +
    for<'r> Visit<parcel_css::properties::svg::Marker<'r>, E> +
    for<'r> Visit<parcel_css::properties::masking::ClipPath<'r>, E> +
    for<'r> Visit<parcel_css::properties::masking::Mask<'r>, E> +
    for<'r> Visit<parcel_css::properties::masking::MaskBorder<'r>, E> +
    for<'r> Visit<parcel_css::properties::effects::FilterList<'r>, E> +
    for<'r> Visit<parcel_css::properties::effects::Filter<'r>, E> +
    for<'r> Visit<parcel_css::properties::custom::UnparsedProperty<'r>, E> +
    for<'r> Visit<parcel_css::properties::custom::CustomProperty<'r>, E> +
    for<'r> Visit<parcel_css::properties::custom::TokenList<'r>, E> +
    for<'r> Visit<parcel_css::properties::custom::TokenOrValue<'r>, E> +
    for<'r> Visit<parcel_css::properties::custom::Variable<'r>, E> +
    for<'r> Visit<parcel_css::values::image::Image<'r>, E> +
    for<'r> Visit<parcel_css::values::image::ImageSet<'r>, E> +
    for<'r> Visit<parcel_css::values::image::ImageSetOption<'r>, E> +
    for<'r> Visit<parcel_css::values::url::Url<'r>, E>
{ }

pub trait Visitable {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E>;
}

pub trait Visit<V: Visitable, E> {
    fn visit(&self, visitable: &mut V) -> Result<(), E>;
}

macro_rules! visit {
    (
        impl<T, E> Visit<T, E> for $visitor:ty where T in [ $( $visitable:ty ),+ ] {
            fn visit(&self, _: &mut T) -> Result<(), E> {
                $result:expr
            }
        }
    ) => {
        $(
            impl<E> $crate::visitors::Visit<$visitable, E> for $visitor {
                fn visit(&self, _: &mut $visitable) -> Result<(), E> {
                    $result
                }
            }
        )*
    }
}

pub(self) use visit;
