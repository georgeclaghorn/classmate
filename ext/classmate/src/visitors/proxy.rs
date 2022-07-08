use super::{Visitor, Visit, Visitable, visit};

pub struct ProxyVisitor<E> {
    proxy: Box<dyn Fn(&str) -> Result<Option<String>, E>>
}

impl<E> ProxyVisitor<E> {
    pub fn new(proxy: impl Fn(&str) -> Result<Option<String>, E> + 'static) -> ProxyVisitor<E> {
        ProxyVisitor { proxy: Box::new(proxy) }
    }

    pub fn visit_from(&self, visitable: &mut impl Visitable) -> Result<(), E> {
        visitable.accept(self)
    }

    fn proxy(&self, url: &str) -> Result<Option<String>, E> {
        (self.proxy)(url)
    }
}

impl<E> Visitor<E> for ProxyVisitor<E> { }

use parcel_css::values::url::Url;

impl<E> Visit<Url<'_>, E> for ProxyVisitor<E> {
    fn visit(&self, token: &mut Url) -> Result<(), E> {
        self.proxy(&token.url).map(|result| {
            if let Some(url) = result {
                if token.url != url {
                    token.url = url.into();
                }
            }
        })
    }
}

use parcel_css::rules::import::ImportRule;

impl<E> Visit<ImportRule<'_>, E> for ProxyVisitor<E> {
    fn visit(&self, rule: &mut ImportRule) -> Result<(), E> {
        self.proxy(&rule.url).map(|result| {
            if let Some(url) = result {
                if rule.url != url {
                    rule.url = url.into()
                }
            }
        })
    }
}

visit! {
    impl<T, E> Visit<T, E> for ProxyVisitor<E> where T in [
        parcel_css::stylesheet::StyleSheet<'_, '_>,
        parcel_css::stylesheet::StyleAttribute<'_>,

        parcel_css::rules::CssRuleList<'_>,
        parcel_css::rules::CssRule<'_>,
        parcel_css::rules::media::MediaRule<'_>,
        parcel_css::rules::style::StyleRule<'_>,
        parcel_css::rules::page::PageRule<'_>,
        parcel_css::rules::supports::SupportsRule<'_>,
        parcel_css::rules::counter_style::CounterStyleRule<'_>,
        parcel_css::rules::document::MozDocumentRule<'_>,
        parcel_css::rules::nesting::NestingRule<'_>,
        parcel_css::rules::viewport::ViewportRule<'_>,
        parcel_css::rules::layer::LayerBlockRule<'_>,
        parcel_css::rules::keyframes::KeyframesRule<'_>,
        parcel_css::rules::keyframes::Keyframe<'_>,
        parcel_css::rules::font_face::FontFaceRule<'_>,
        parcel_css::rules::font_face::FontFaceProperty<'_>,
        parcel_css::rules::font_face::Source<'_>,
        parcel_css::rules::font_face::UrlSource<'_>,
        parcel_css::rules::font_palette_values::FontPaletteValuesRule<'_>,
        parcel_css::rules::font_palette_values::FontPaletteValuesProperty<'_>,

        parcel_css::declaration::DeclarationBlock<'_>,

        parcel_css::properties::Property<'_>,
        parcel_css::properties::background::Background<'_>,
        parcel_css::properties::border_image::BorderImage<'_>,
        parcel_css::properties::list::ListStyle<'_>,
        parcel_css::properties::svg::SVGPaint<'_>,
        parcel_css::properties::svg::Marker<'_>,
        parcel_css::properties::masking::ClipPath<'_>,
        parcel_css::properties::masking::Mask<'_>,
        parcel_css::properties::masking::MaskBorder<'_>,
        parcel_css::properties::effects::FilterList<'_>,
        parcel_css::properties::effects::Filter<'_>,
        parcel_css::properties::custom::UnparsedProperty<'_>,
        parcel_css::properties::custom::CustomProperty<'_>,
        parcel_css::properties::custom::TokenList<'_>,
        parcel_css::properties::custom::TokenOrValue<'_>,
        parcel_css::properties::custom::Variable<'_>,
        parcel_css::properties::custom::UnresolvedColor<'_>,

        parcel_css::values::image::Image<'_>,
        parcel_css::values::image::ImageSet<'_>,
        parcel_css::values::image::ImageSetOption<'_>
    ] {
        fn visit(&self, _: &mut T) -> Result<(), E> {
            Ok(())
        }
    }
}
