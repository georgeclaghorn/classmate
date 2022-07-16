mod media;
mod import;
mod style;
mod keyframes;
mod font_face;
mod font_palette_values;
mod page;
mod supports;
mod counter_style;
mod document;
mod nesting;
mod viewport;
mod layer;
mod container;

use crate::visitors::common::prelude::*;

use parcel_css::rules::CssRuleList as RuleList;

impl Visitable for RuleList<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.0.accept(visitor))
    }
}

use parcel_css::rules::CssRule as Rule;

impl Visitable for Rule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                Rule::Media(rule)             => rule.accept(visitor),
                Rule::Import(rule)            => rule.accept(visitor),
                Rule::Style(rule)             => rule.accept(visitor),
                Rule::Keyframes(rule)         => rule.accept(visitor),
                Rule::FontFace(rule)          => rule.accept(visitor),
                Rule::FontPaletteValues(rule) => rule.accept(visitor),
                Rule::Page(rule)              => rule.accept(visitor),
                Rule::Supports(rule)          => rule.accept(visitor),
                Rule::CounterStyle(rule)      => rule.accept(visitor),
                Rule::MozDocument(rule)       => rule.accept(visitor),
                Rule::Nesting(rule)           => rule.accept(visitor),
                Rule::Viewport(rule)          => rule.accept(visitor),
                Rule::LayerBlock(rule)        => rule.accept(visitor),
                Rule::Container(rule)         => rule.accept(visitor),

                Rule::Namespace(_)            |
                Rule::CustomMedia(_)          |
                Rule::LayerStatement(_)       |
                Rule::Property(_)             |
                Rule::Ignored                 => Ok(())
            }
        })
    }
}
