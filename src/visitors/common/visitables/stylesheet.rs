use crate::visitors::common::prelude::*;

use parcel_css::stylesheet::{StyleSheet, StyleAttribute};

impl Visitable for StyleSheet<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.rules.accept(visitor))
    }
}

impl Visitable for StyleAttribute<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}
