use crate::visitors::common::prelude::*;

use parcel_css::stylesheet::{StyleSheet, StyleAttribute};

impl Visitable for StyleSheet<'_, '_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}

impl Visitable for StyleAttribute<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}
