use crate::visitors::common::prelude::*;

use parcel_css::values::image::{Image, ImageSet, ImageSetOption};

impl<'a> Visitable<'a> for Image<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                Image::Url(resource) => resource.accept(visitor),
                Image::ImageSet(set) => set.accept(visitor),

                Image::None | Image::Gradient(_) => Ok(())
            }
        })
    }
}

impl<'a> Visitable<'a> for ImageSet<'a> {
  fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.options.accept(visitor))
  }
}

impl<'a> Visitable<'a> for ImageSetOption<'a> {
  fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.image.accept(visitor))
  }
}

use parcel_css::values::url::Url;

impl<'a> Visitable<'a> for Url<'a> {
    fn accept<V: Visitor<'a, E>, E>(&mut self, visitor: &V) -> Result<(), E> {
        visitor.visit(self)
    }
}
