mod common;

mod proxy;
pub use proxy::ProxyVisitor;

pub trait Visitor<'a, E>:
    Visit<'a, std::cell::RefMut<'a, parcel_css::stylesheet::StyleSheet<'a>>, E> +
    Visit<'a, parcel_css::stylesheet::StyleSheet<'a>, E> +
    Visit<'a, parcel_css::rules::CssRuleList<'a>, E> +
    Visit<'a, parcel_css::rules::CssRule<'a>, E> +
    Visit<'a, parcel_css::rules::media::MediaRule<'a>, E> +
    Visit<'a, parcel_css::rules::import::ImportRule<'a>, E> +
    Visit<'a, parcel_css::rules::style::StyleRule<'a>, E> +
    Visit<'a, parcel_css::rules::page::PageRule<'a>, E> +
    Visit<'a, parcel_css::rules::supports::SupportsRule<'a>, E> +
    Visit<'a, parcel_css::rules::counter_style::CounterStyleRule<'a>, E> +
    Visit<'a, parcel_css::rules::document::MozDocumentRule<'a>, E> +
    Visit<'a, parcel_css::rules::nesting::NestingRule<'a>, E> +
    Visit<'a, parcel_css::rules::viewport::ViewportRule<'a>, E> +
    Visit<'a, parcel_css::rules::layer::LayerBlockRule<'a>, E> +
    Visit<'a, parcel_css::rules::keyframes::KeyframesRule<'a>, E> +
    Visit<'a, parcel_css::rules::keyframes::Keyframe<'a>, E> +
    Visit<'a, parcel_css::rules::font_face::FontFaceRule<'a>, E> +
    Visit<'a, parcel_css::rules::font_face::FontFaceProperty<'a>, E> +
    Visit<'a, parcel_css::rules::font_face::Source<'a>, E> +
    Visit<'a, parcel_css::rules::font_face::UrlSource<'a>, E> +
    Visit<'a, parcel_css::rules::font_palette_values::FontPaletteValuesRule<'a>, E> +
    Visit<'a, parcel_css::rules::font_palette_values::FontPaletteValuesProperty<'a>, E> +
    Visit<'a, std::cell::RefMut<'a, parcel_css::declaration::DeclarationBlock<'a>>, E> +
    Visit<'a, parcel_css::declaration::DeclarationBlock<'a>, E> +
    Visit<'a, parcel_css::properties::Property<'a>, E> +
    Visit<'a, parcel_css::properties::background::Background<'a>, E> +
    Visit<'a, parcel_css::properties::border_image::BorderImage<'a>, E> +
    Visit<'a, parcel_css::properties::list::ListStyle<'a>, E> +
    Visit<'a, parcel_css::properties::svg::SVGPaint<'a>, E> +
    Visit<'a, parcel_css::properties::svg::Marker<'a>, E> +
    Visit<'a, parcel_css::properties::masking::ClipPath<'a>, E> +
    Visit<'a, parcel_css::properties::masking::Mask<'a>, E> +
    Visit<'a, parcel_css::properties::masking::MaskBorder<'a>, E> +
    Visit<'a, parcel_css::properties::effects::FilterList<'a>, E> +
    Visit<'a, parcel_css::properties::effects::Filter<'a>, E> +
    Visit<'a, parcel_css::properties::custom::UnparsedProperty<'a>, E> +
    Visit<'a, parcel_css::properties::custom::CustomProperty<'a>, E> +
    Visit<'a, parcel_css::properties::custom::TokenList<'a>, E> +
    Visit<'a, parcel_css::properties::custom::TokenOrValue<'a>, E> +
    Visit<'a, parcel_css::values::image::Image<'a>, E> +
    Visit<'a, parcel_css::values::image::ImageSet<'a>, E> +
    Visit<'a, parcel_css::values::image::ImageSetOption<'a>, E> +
    Visit<'a, parcel_css::values::url::Url<'a>, E>
{ }

pub trait Visitable<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E>;
}

pub trait Visit<'a, V: Visitable<'a> + ?Sized, E> {
    fn visit(&self, _visitable: &mut V) -> Result<(), E> {
        Ok(())
    }
}

macro_rules! visit {
    (
        impl<'a, T, E> Visit<'a, T, E> for $visitor:ty where T in [ $( $visitable:ty ),+ ] {
            fn visit(&self, _: &mut T) -> Result<(), E> {
                $result:expr
            }
        }
    ) => {
        $(
            impl<'a, E> $crate::visitors::Visit<'a, $visitable, E> for $visitor {
                fn visit(&self, _: &mut $visitable) -> Result<(), E> {
                    $result
                }
            }
        )*
    }
}

pub(self) use visit;
