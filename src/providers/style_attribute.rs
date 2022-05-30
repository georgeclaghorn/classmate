use std::cell::{RefCell, Ref, RefMut};
use parcel_css::stylesheet::StyleAttribute;
use owning_ref::OwningHandle;

pub trait StyleAttributeProvider<'a> {
    fn borrow(&self) -> Ref<'a, StyleAttribute>;
    fn borrow_mut(&self) -> RefMut<'a, StyleAttribute>;
}

type ParsedStyleAttributeHandle<'a> = OwningHandle<
    String,
    Box<RefCell<StyleAttribute<'a>>>
>;

pub struct ParsedStyleAttributeProvider<'a> {
    handle: ParsedStyleAttributeHandle<'a>
}

impl<'a> ParsedStyleAttributeProvider<'a> {
    pub fn try_new<E>(
        code: String,
        parser: impl FnOnce(&'a str) -> Result<StyleAttribute<'a>, E>
    ) -> Result<ParsedStyleAttributeProvider<'a>, E> {
        Ok(
            ParsedStyleAttributeProvider {
                handle: OwningHandle::try_new(
                    code,

                    |code_ptr| parser(
                        unsafe { &*code_ptr }
                    ).map(|attribute| Box::new(RefCell::new(attribute)))
                )?
            }
        )
    }
}

impl<'a> StyleAttributeProvider<'a> for ParsedStyleAttributeProvider<'a> {
    fn borrow(&self) -> Ref<'a, StyleAttribute> {
        self.handle.borrow()
    }

    fn borrow_mut(&self) -> RefMut<'a, StyleAttribute> {
        self.handle.borrow_mut()
    }
}
