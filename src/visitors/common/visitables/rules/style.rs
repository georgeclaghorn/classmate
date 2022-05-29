use crate::visitors::common::prelude::*;

use parcel_css::rules::style::StyleRule;

impl Visitable for StyleRule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            self.declarations.accept(visitor)?;
            self.rules.accept(visitor)
        })
    }
}
