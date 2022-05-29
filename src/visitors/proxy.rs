use super::{Visitor, Visit, Visitable, visit};

pub struct ProxyVisitor<E> {
    callback: Box<dyn Fn(&str) -> Result<Option<String>, E>>
}

impl<'a, E> ProxyVisitor<E> {
    pub fn new(callback: impl Fn(&str) -> Result<Option<String>, E> + 'static) -> ProxyVisitor<E> {
        ProxyVisitor { callback: Box::new(callback) }
    }

    pub fn visit_from<V: Visitable<'a> + ?Sized>(&self, visitable: &mut V) -> Result<(), E> {
        visitable.accept(self)
    }

    fn translate(&self, resource: &str) -> Result<Option<String>, E> {
        (self.callback)(resource)
    }
}

impl<'a, E> Visitor<'a, E> for ProxyVisitor<E> { }

use parcel_css::values::url::Url;

impl<'a, E> Visit<'a, Url<'a>, E> for ProxyVisitor<E> {
    fn visit(&self, token: &mut Url) -> Result<(), E> {
        self.translate(&token.url).map(|result| {
            if let Some(url) = result {
                if token.url != url {
                    token.url = url.into();
                }
            }
        })
    }
}

use parcel_css::rules::import::ImportRule;

impl<'a, E> Visit<'a, ImportRule<'a>, E> for ProxyVisitor<E> {
    fn visit(&self, rule: &mut ImportRule) -> Result<(), E> {
        self.translate(&rule.url).map(|result| {
            if let Some(url) = result {
                if rule.url != url {
                    rule.url = url.into()
                }
            }
        })
    }
}

visit! {
    impl<'a, T, E> Visit<'a, T, E> for ProxyVisitor<E> where T in [
        std::cell::RefMut<'a, parcel_css::stylesheet::StyleSheet<'a>>,
        parcel_css::stylesheet::StyleSheet<'a>,
        std::cell::RefMut<'a, parcel_css::stylesheet::StyleAttribute<'a>>,
        parcel_css::stylesheet::StyleAttribute<'a>,

        parcel_css::rules::CssRuleList<'a>,
        parcel_css::rules::CssRule<'a>,
        parcel_css::rules::media::MediaRule<'a>,
        parcel_css::rules::style::StyleRule<'a>,
        parcel_css::rules::page::PageRule<'a>,
        parcel_css::rules::supports::SupportsRule<'a>,
        parcel_css::rules::counter_style::CounterStyleRule<'a>,
        parcel_css::rules::document::MozDocumentRule<'a>,
        parcel_css::rules::nesting::NestingRule<'a>,
        parcel_css::rules::viewport::ViewportRule<'a>,
        parcel_css::rules::layer::LayerBlockRule<'a>,
        parcel_css::rules::keyframes::KeyframesRule<'a>,
        parcel_css::rules::keyframes::Keyframe<'a>,
        parcel_css::rules::font_face::FontFaceRule<'a>,
        parcel_css::rules::font_face::FontFaceProperty<'a>,
        parcel_css::rules::font_face::Source<'a>,
        parcel_css::rules::font_face::UrlSource<'a>,
        parcel_css::rules::font_palette_values::FontPaletteValuesRule<'a>,
        parcel_css::rules::font_palette_values::FontPaletteValuesProperty<'a>,

        std::cell::RefMut<'a, parcel_css::declaration::DeclarationBlock<'a>>,
        parcel_css::declaration::DeclarationBlock<'a>,

        parcel_css::properties::Property<'a>,
        parcel_css::properties::background::Background<'a>,
        parcel_css::properties::border_image::BorderImage<'a>,
        parcel_css::properties::list::ListStyle<'a>,
        parcel_css::properties::svg::SVGPaint<'a>,
        parcel_css::properties::svg::Marker<'a>,
        parcel_css::properties::masking::ClipPath<'a>,
        parcel_css::properties::masking::Mask<'a>,
        parcel_css::properties::masking::MaskBorder<'a>,
        parcel_css::properties::effects::FilterList<'a>,
        parcel_css::properties::effects::Filter<'a>,
        parcel_css::properties::custom::UnparsedProperty<'a>,
        parcel_css::properties::custom::CustomProperty<'a>,
        parcel_css::properties::custom::TokenList<'a>,
        parcel_css::properties::custom::TokenOrValue<'a>,

        parcel_css::values::image::Image<'a>,
        parcel_css::values::image::ImageSet<'a>,
        parcel_css::values::image::ImageSetOption<'a>
    ] {
        fn visit(&self, _: &mut T) -> Result<(), E> {
            Ok(())
        }
    }
}
