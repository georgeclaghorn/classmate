mod background;
mod border_image;
mod list;
mod svg;
mod masking;
mod effects;
mod custom;

use crate::visitors::common::prelude::*;

use parcel_css::properties::Property;

impl<'a> Visitable<'a> for Property<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                Property::BackgroundImage(images)             => images.accept(visitor),
                Property::Background(backgrounds)             => backgrounds.accept(visitor),
                Property::BorderImageSource(source)           => source.accept(visitor),
                Property::BorderImage(image, _)               => image.accept(visitor),
                Property::ListStyleImage(image)               => image.accept(visitor),
                Property::ListStyle(style)                    => style.accept(visitor),
                Property::Fill(fill)                          => fill.accept(visitor),
                Property::Stroke(stroke)                      => stroke.accept(visitor),
                Property::MarkerStart(marker)                 => marker.accept(visitor),
                Property::MarkerMid(marker)                   => marker.accept(visitor),
                Property::MarkerEnd(marker)                   => marker.accept(visitor),
                Property::Marker(marker)                      => marker.accept(visitor),
                Property::ClipPath(path, _)                   => path.accept(visitor),
                Property::MaskImage(images, _)                => images.accept(visitor),
                Property::Mask(masks, _)                      => masks.accept(visitor),
                Property::MaskBorderSource(source)            => source.accept(visitor),
                Property::MaskBorder(border)                  => border.accept(visitor),
                Property::WebKitMaskBoxImage(image, _)        => image.accept(visitor),
                Property::WebKitMaskBoxImageSource(source, _) => source.accept(visitor),
                Property::Filter(filters, _)                  => filters.accept(visitor),
                Property::BackdropFilter(filters, _)          => filters.accept(visitor),
                Property::Unparsed(property)                  => property.accept(visitor),
                Property::Custom(property)                    => property.accept(visitor),

                Property::BackgroundColor(_)                  |
                Property::BackgroundPositionX(_)              |
                Property::BackgroundPositionY(_)              |
                Property::BackgroundPosition(_)               |
                Property::BackgroundSize(_)                   |
                Property::BackgroundRepeat(_)                 |
                Property::BackgroundAttachment(_)             |
                Property::BackgroundClip(_, _)                |
                Property::BackgroundOrigin(_)                 |
                Property::BoxShadow(_, _)                     |
                Property::Opacity(_)                          |
                Property::Color(_)                            |
                Property::Display(_)                          |
                Property::Visibility(_)                       |
                Property::Width(_)                            |
                Property::Height(_)                           |
                Property::MinWidth(_)                         |
                Property::MinHeight(_)                        |
                Property::MaxWidth(_)                         |
                Property::MaxHeight(_)                        |
                Property::BlockSize(_)                        |
                Property::InlineSize(_)                       |
                Property::MinBlockSize(_)                     |
                Property::MinInlineSize(_)                    |
                Property::MaxBlockSize(_)                     |
                Property::MaxInlineSize(_)                    |
                Property::BoxSizing(_, _)                     |
                Property::Overflow(_)                         |
                Property::OverflowX(_)                        |
                Property::OverflowY(_)                        |
                Property::TextOverflow(_, _)                  |
                Property::Position(_)                         |
                Property::Top(_)                              |
                Property::Bottom(_)                           |
                Property::Left(_)                             |
                Property::Right(_)                            |
                Property::InsetBlockStart(_)                  |
                Property::InsetBlockEnd(_)                    |
                Property::InsetInlineStart(_)                 |
                Property::InsetInlineEnd(_)                   |
                Property::InsetBlock(_)                       |
                Property::InsetInline(_)                      |
                Property::Inset(_)                            |
                Property::BorderTopColor(_)                   |
                Property::BorderBottomColor(_)                |
                Property::BorderLeftColor(_)                  |
                Property::BorderRightColor(_)                 |
                Property::BorderBlockStartColor(_)            |
                Property::BorderBlockEndColor(_)              |
                Property::BorderInlineStartColor(_)           |
                Property::BorderInlineEndColor(_)             |
                Property::BorderTopStyle(_)                   |
                Property::BorderBottomStyle(_)                |
                Property::BorderLeftStyle(_)                  |
                Property::BorderRightStyle(_)                 |
                Property::BorderBlockStartStyle(_)            |
                Property::BorderBlockEndStyle(_)              |
                Property::BorderInlineStartStyle(_)           |
                Property::BorderInlineEndStyle(_)             |
                Property::BorderTopWidth(_)                   |
                Property::BorderBottomWidth(_)                |
                Property::BorderLeftWidth(_)                  |
                Property::BorderRightWidth(_)                 |
                Property::BorderBlockStartWidth(_)            |
                Property::BorderBlockEndWidth(_)              |
                Property::BorderInlineStartWidth(_)           |
                Property::BorderInlineEndWidth(_)             |
                Property::BorderTopLeftRadius(_, _)           |
                Property::BorderTopRightRadius(_, _)          |
                Property::BorderBottomLeftRadius(_, _)        |
                Property::BorderBottomRightRadius(_, _)       |
                Property::BorderStartStartRadius(_)           |
                Property::BorderStartEndRadius(_)             |
                Property::BorderEndStartRadius(_)             |
                Property::BorderEndEndRadius(_)               |
                Property::BorderRadius(_, _)                  |
                Property::BorderImageOutset(_)                |
                Property::BorderImageRepeat(_)                |
                Property::BorderImageWidth(_)                 |
                Property::BorderImageSlice(_)                 |
                Property::BorderColor(_)                      |
                Property::BorderStyle(_)                      |
                Property::BorderWidth(_)                      |
                Property::BorderBlockColor(_)                 |
                Property::BorderBlockStyle(_)                 |
                Property::BorderBlockWidth(_)                 |
                Property::BorderInlineColor(_)                |
                Property::BorderInlineStyle(_)                |
                Property::BorderInlineWidth(_)                |
                Property::Border(_)                           |
                Property::BorderTop(_)                        |
                Property::BorderBottom(_)                     |
                Property::BorderLeft(_)                       |
                Property::BorderRight(_)                      |
                Property::BorderBlock(_)                      |
                Property::BorderBlockStart(_)                 |
                Property::BorderBlockEnd(_)                   |
                Property::BorderInline(_)                     |
                Property::BorderInlineStart(_)                |
                Property::BorderInlineEnd(_)                  |
                Property::Outline(_)                          |
                Property::OutlineColor(_)                     |
                Property::OutlineStyle(_)                     |
                Property::OutlineWidth(_)                     |
                Property::FlexDirection(_, _)                 |
                Property::FlexWrap(_, _)                      |
                Property::FlexFlow(_, _)                      |
                Property::FlexGrow(_, _)                      |
                Property::FlexShrink(_, _)                    |
                Property::FlexBasis(_, _)                     |
                Property::Flex(_, _)                          |
                Property::Order(_, _)                         |
                Property::AlignContent(_, _)                  |
                Property::JustifyContent(_, _)                |
                Property::PlaceContent(_)                     |
                Property::AlignSelf(_, _)                     |
                Property::JustifySelf(_)                      |
                Property::PlaceSelf(_)                        |
                Property::AlignItems(_, _)                    |
                Property::JustifyItems(_)                     |
                Property::PlaceItems(_)                       |
                Property::RowGap(_)                           |
                Property::ColumnGap(_)                        |
                Property::Gap(_)                              |
                Property::BoxOrient(_, _)                     |
                Property::BoxDirection(_, _)                  |
                Property::BoxOrdinalGroup(_, _)               |
                Property::BoxAlign(_, _)                      |
                Property::BoxFlex(_, _)                       |
                Property::BoxFlexGroup(_, _)                  |
                Property::BoxPack(_, _)                       |
                Property::BoxLines(_, _)                      |
                Property::FlexPack(_, _)                      |
                Property::FlexOrder(_, _)                     |
                Property::FlexAlign(_, _)                     |
                Property::FlexItemAlign(_, _)                 |
                Property::FlexLinePack(_, _)                  |
                Property::FlexPositive(_, _)                  |
                Property::FlexNegative(_, _)                  |
                Property::FlexPreferredSize(_, _)             |
                Property::GridTemplateColumns(_)              |
                Property::GridTemplateRows(_)                 |
                Property::GridAutoColumns(_)                  |
                Property::GridAutoRows(_)                     |
                Property::GridAutoFlow(_)                     |
                Property::GridTemplateAreas(_)                |
                Property::GridTemplate(_)                     |
                Property::Grid(_)                             |
                Property::GridRowStart(_)                     |
                Property::GridRowEnd(_)                       |
                Property::GridColumnStart(_)                  |
                Property::GridColumnEnd(_)                    |
                Property::GridRow(_)                          |
                Property::GridColumn(_)                       |
                Property::GridArea(_)                         |
                Property::MarginTop(_)                        |
                Property::MarginBottom(_)                     |
                Property::MarginLeft(_)                       |
                Property::MarginRight(_)                      |
                Property::MarginBlockStart(_)                 |
                Property::MarginBlockEnd(_)                   |
                Property::MarginInlineStart(_)                |
                Property::MarginInlineEnd(_)                  |
                Property::MarginBlock(_)                      |
                Property::MarginInline(_)                     |
                Property::Margin(_)                           |
                Property::PaddingTop(_)                       |
                Property::PaddingBottom(_)                    |
                Property::PaddingLeft(_)                      |
                Property::PaddingRight(_)                     |
                Property::PaddingBlockStart(_)                |
                Property::PaddingBlockEnd(_)                  |
                Property::PaddingInlineStart(_)               |
                Property::PaddingInlineEnd(_)                 |
                Property::PaddingBlock(_)                     |
                Property::PaddingInline(_)                    |
                Property::Padding(_)                          |
                Property::ScrollMarginTop(_)                  |
                Property::ScrollMarginBottom(_)               |
                Property::ScrollMarginLeft(_)                 |
                Property::ScrollMarginRight(_)                |
                Property::ScrollMarginBlockStart(_)           |
                Property::ScrollMarginBlockEnd(_)             |
                Property::ScrollMarginInlineStart(_)          |
                Property::ScrollMarginInlineEnd(_)            |
                Property::ScrollMarginBlock(_)                |
                Property::ScrollMarginInline(_)               |
                Property::ScrollMargin(_)                     |
                Property::ScrollPaddingTop(_)                 |
                Property::ScrollPaddingBottom(_)              |
                Property::ScrollPaddingLeft(_)                |
                Property::ScrollPaddingRight(_)               |
                Property::ScrollPaddingBlockStart(_)          |
                Property::ScrollPaddingBlockEnd(_)            |
                Property::ScrollPaddingInlineStart(_)         |
                Property::ScrollPaddingInlineEnd(_)           |
                Property::ScrollPaddingBlock(_)               |
                Property::ScrollPaddingInline(_)              |
                Property::ScrollPadding(_)                    |
                Property::FontWeight(_)                       |
                Property::FontSize(_)                         |
                Property::FontStretch(_)                      |
                Property::FontFamily(_)                       |
                Property::FontStyle(_)                        |
                Property::FontVariantCaps(_)                  |
                Property::LineHeight(_)                       |
                Property::Font(_)                             |
                Property::VerticalAlign(_)                    |
                Property::FontPalette(_)                      |
                Property::TransitionProperty(_, _)            |
                Property::TransitionDuration(_, _)            |
                Property::TransitionDelay(_, _)               |
                Property::TransitionTimingFunction(_, _)      |
                Property::Transition(_, _)                    |
                Property::AnimationName(_, _)                 |
                Property::AnimationDuration(_, _)             |
                Property::AnimationTimingFunction(_, _)       |
                Property::AnimationIterationCount(_, _)       |
                Property::AnimationDirection(_, _)            |
                Property::AnimationPlayState(_, _)            |
                Property::AnimationDelay(_, _)                |
                Property::AnimationFillMode(_, _)             |
                Property::Animation(_, _)                     |
                Property::Transform(_, _)                     |
                Property::TransformOrigin(_, _)               |
                Property::TransformStyle(_, _)                |
                Property::TransformBox(_)                     |
                Property::BackfaceVisibility(_, _)            |
                Property::Perspective(_, _)                   |
                Property::PerspectiveOrigin(_, _)             |
                Property::Translate(_)                        |
                Property::Rotate(_)                           |
                Property::Scale(_)                            |
                Property::TextTransform(_)                    |
                Property::WhiteSpace(_)                       |
                Property::TabSize(_, _)                       |
                Property::WordBreak(_)                        |
                Property::LineBreak(_)                        |
                Property::Hyphens(_, _)                       |
                Property::OverflowWrap(_)                     |
                Property::WordWrap(_)                         |
                Property::TextAlign(_)                        |
                Property::TextAlignLast(_, _)                 |
                Property::TextJustify(_)                      |
                Property::WordSpacing(_)                      |
                Property::LetterSpacing(_)                    |
                Property::TextIndent(_)                       |
                Property::TextDecorationLine(_, _)            |
                Property::TextDecorationStyle(_, _)           |
                Property::TextDecorationColor(_, _)           |
                Property::TextDecorationThickness(_)          |
                Property::TextDecoration(_, _)                |
                Property::TextDecorationSkipInk(_, _)         |
                Property::TextEmphasisStyle(_, _)             |
                Property::TextEmphasisColor(_, _)             |
                Property::TextEmphasis(_, _)                  |
                Property::TextEmphasisPosition(_, _)          |
                Property::TextShadow(_)                       |
                Property::Resize(_)                           |
                Property::Cursor(_)                           |
                Property::CaretColor(_)                       |
                Property::CaretShape(_)                       |
                Property::Caret(_)                            |
                Property::UserSelect(_, _)                    |
                Property::AccentColor(_)                      |
                Property::Appearance(_, _)                    |
                Property::ListStyleType(_)                    |
                Property::ListStylePosition(_)                |
                Property::MarkerSide(_)                       |
                Property::Composes(_)                         |
                Property::FillRule(_)                         |
                Property::FillOpacity(_)                      |
                Property::StrokeOpacity(_)                    |
                Property::StrokeWidth(_)                      |
                Property::StrokeLinecap(_)                    |
                Property::StrokeLinejoin(_)                   |
                Property::StrokeMiterlimit(_)                 |
                Property::StrokeDasharray(_)                  |
                Property::StrokeDashoffset(_)                 |
                Property::ColorInterpolation(_)               |
                Property::ColorInterpolationFilters(_)        |
                Property::ColorRendering(_)                   |
                Property::ShapeRendering(_)                   |
                Property::TextRendering(_)                    |
                Property::ImageRendering(_)                   |
                Property::ClipRule(_)                         |
                Property::MaskMode(_)                         |
                Property::MaskRepeat(_, _)                    |
                Property::MaskPositionX(_)                    |
                Property::MaskPositionY(_)                    |
                Property::MaskPosition(_, _)                  |
                Property::MaskClip(_, _)                      |
                Property::MaskOrigin(_, _)                    |
                Property::MaskSize(_, _)                      |
                Property::MaskComposite(_)                    |
                Property::MaskType(_)                         |
                Property::MaskBorderMode(_)                   |
                Property::MaskBorderSlice(_)                  |
                Property::MaskBorderWidth(_)                  |
                Property::MaskBorderOutset(_)                 |
                Property::MaskBorderRepeat(_)                 |
                Property::WebKitMaskComposite(_)              |
                Property::WebKitMaskSourceType(_, _)          |
                Property::WebKitMaskBoxImageSlice(_, _)       |
                Property::WebKitMaskBoxImageWidth(_, _)       |
                Property::WebKitMaskBoxImageOutset(_, _)      |
                Property::WebKitMaskBoxImageRepeat(_, _)      |
                Property::ZIndex(_)                           => Ok(()),
            }
        })
    }
}
