use std::cell::{RefCell, Ref, RefMut};
use parcel_css::stylesheet::StyleSheet;
use owning_ref::OwningHandle;

pub trait StylesheetProvider<'a, 'o> {
    fn borrow(&self) -> Ref<'_, StyleSheet<'a, 'o>>;
    fn borrow_mut(&self) -> RefMut<'_, StyleSheet<'a, 'o>>;
}

type ParsedStylesheetHandle<'a, 'o> = OwningHandle<
    String,
    OwningHandle<
        String,
        Box<RefCell<StyleSheet<'a, 'o>>>
    >
>;

pub struct ParsedStylesheetProvider<'a, 'o> {
    handle: ParsedStylesheetHandle<'a, 'o>
}

impl<'a, 'o> ParsedStylesheetProvider<'a, 'o> {
    pub fn try_new<E>(
        filename: String,
        code: String,
        parser: impl FnOnce(&'a str, &'a str) -> Result<StyleSheet<'a, 'o>, E>
    ) -> Result<ParsedStylesheetProvider<'a, 'o>, E> {
        Ok(
            ParsedStylesheetProvider {
                handle: OwningHandle::try_new(
                    filename,

                    |filename_ptr| OwningHandle::try_new(
                        code,

                        |code_ptr| parser(
                            unsafe { &*filename_ptr },
                            unsafe { &*code_ptr }
                        ).map(|stylesheet| Box::new(RefCell::new(stylesheet)))
                    )
                )?
            }
        )
    }
}

impl<'a, 'o> StylesheetProvider<'a, 'o> for ParsedStylesheetProvider<'a, 'o> {
    fn borrow(&self) -> Ref<'_, StyleSheet<'a, 'o>> {
        self.handle.borrow()
    }

    fn borrow_mut(&self) -> RefMut<'_, StyleSheet<'a, 'o>> {
        self.handle.borrow_mut()
    }
}
