use crate::visitors::common::prelude::*;

use parcel_css::rules::font_palette_values::{FontPaletteValuesRule, FontPaletteValuesProperty};

impl Visitable for FontPaletteValuesRule<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.properties.accept(visitor))
    }
}

impl Visitable for FontPaletteValuesProperty<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                FontPaletteValuesProperty::Custom(property)  => property.accept(visitor),

                FontPaletteValuesProperty::FontFamily(_)     |
                FontPaletteValuesProperty::BasePalette(_)    |
                FontPaletteValuesProperty::OverrideColors(_) => Ok(())
            }
        })
    }
}
