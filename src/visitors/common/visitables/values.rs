use crate::visitors::common::prelude::*;

use parcel_css::values::image::{Image, ImageSet, ImageSetOption};

impl Visitable for Image<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| {
            match self {
                Image::Url(resource) => resource.accept(visitor),
                Image::ImageSet(set) => set.accept(visitor),

                Image::None | Image::Gradient(_) => Ok(())
            }
        })
    }
}

impl Visitable for ImageSet<'_> {
  fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.options.accept(visitor))
  }
}

impl Visitable for ImageSetOption<'_> {
  fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self).and_then(|_| self.image.accept(visitor))
  }
}

use parcel_css::values::url::Url;

impl Visitable for Url<'_> {
    fn accept<E>(&mut self, visitor: &impl Visitor<E>) -> Result<(), E> {
        visitor.visit(self)
    }
}
