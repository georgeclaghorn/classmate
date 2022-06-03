use crate::visitors::common::prelude::*;

use parcel_css::declaration::DeclarationBlock;

impl Visitable for DeclarationBlock<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            self.important_declarations.accept(visitor)?;
            self.declarations.accept(visitor)
        })
    }
}
