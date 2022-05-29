use crate::visitors::common::prelude::*;

use parcel_css::declaration::DeclarationBlock;

impl Visitable for DeclarationBlock<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            self.important_declarations.accept(visitor)?;
            self.declarations.accept(visitor)
        })
    }
}
