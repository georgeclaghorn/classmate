use crate::visitors::common::prelude::*;

use parcel_css::rules::style::StyleRule;

impl Visitable for StyleRule<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            self.declarations.accept(visitor)?;
            self.rules.accept(visitor)
        })
    }
}
