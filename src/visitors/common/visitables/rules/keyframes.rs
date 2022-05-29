use crate::visitors::common::prelude::*;

use parcel_css::rules::keyframes::{KeyframesRule, Keyframe};

impl Visitable for KeyframesRule<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.keyframes.accept(visitor))
    }
}

impl Visitable for Keyframe<'_> {
    fn accept<V: Visitor<E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}
