use crate::visitors::common::prelude::*;

use parcel_css::rules::keyframes::{KeyframesRule, Keyframe};

impl Visitable for KeyframesRule<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.keyframes.accept(visitor))
    }
}

impl Visitable for Keyframe<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.declarations.accept(visitor))
    }
}
