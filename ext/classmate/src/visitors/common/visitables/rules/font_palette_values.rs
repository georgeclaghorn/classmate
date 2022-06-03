use crate::visitors::common::prelude::*;

use parcel_css::rules::font_palette_values::{FontPaletteValuesRule, FontPaletteValuesProperty};

impl Visitable for FontPaletteValuesRule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.properties.accept(visitor))
    }
}

impl Visitable for FontPaletteValuesProperty<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
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
