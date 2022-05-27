use std::{
    cell::{RefCell, Ref, RefMut},
    rc::Rc
};
use magnus::{
    define_module,
    Value, RModule, RClass, Module, Object, Error,
    TypedData, DataType, DataTypeFunctions,
    class::object,
    block::yield_value
};
use parcel_css::{
    declaration::DeclarationBlock,
    stylesheet::{ParserOptions, PrinterOptions},
    traits::ToCss
};
use owning_ref::OwningHandle;
use tap::Tap;
use crate::visitors::ProxyVisitor;

struct DeclarationList<'a> {
    provider: Rc<dyn DeclarationBlockProvider<'a> + 'a>
}

impl<'a> DeclarationList<'a> {
    fn parse(code: String) -> Result<Value, Error> {
        let maybe_provider = ParsedDeclarationBlockProvider::try_new(
            code,

            |code| DeclarationBlock::parse_string(
                code,
                ParserOptions::default()
            )
        );

        maybe_provider
            .map(|provider| Value::from(
                DeclarationList { provider: Rc::new(provider) }
            ))
            .map_err(|_error| Error::new(
                crate::errors::parse_error(),
                String::from("error parsing declaration list")
            ))
    }

    fn new() -> Result<DeclarationList<'a>, Error> {
        Err(
            Error::new(
                magnus::exception::not_imp_error(),
                "not implemented"
            )
        )
    }

    fn proxy(&'a self) -> Result<(), Error> {
        ProxyVisitor::new(|resource| yield_value(resource.to_string())).visit_from(&mut self.provider.borrow_mut())
    }

    fn to_css(&'a self) -> Result<String, Error> {
        self.provider
            .borrow()
            .to_css_string(PrinterOptions::default())
            .map_err(|error| Error::new(
                crate::errors::print_error(),
                error.to_string()
            ))
    }
}

unsafe impl<'a> TypedData for DeclarationList<'a> {
  fn class() -> RClass {
        *memoize!(
            RClass:
                object()
                    .const_get("Classmate")
                    .and_then(|module: RModule| module.const_get("DeclarationList"))
                    .unwrap()
        )
    }

    fn data_type() -> &'static DataType {
        memoize!(
            DataType: {
                DataType::builder::<Self>("Classmate::DeclarationList").tap_mut(|builder| {
                    builder.free_immediatly();
                    builder.size();
                }).build()
            }
        )
    }
}

impl<'a> DataTypeFunctions for DeclarationList<'a> { }

pub trait DeclarationBlockProvider<'a> {
    fn borrow(&self) -> Ref<'a, DeclarationBlock>;
    fn borrow_mut(&self) -> RefMut<'a, DeclarationBlock>;
}

type ParsedDeclarationBlockHandle<'a> = OwningHandle<
    String,
    Rc<RefCell<DeclarationBlock<'a>>>
>;

struct ParsedDeclarationBlockProvider<'a> {
    handle: ParsedDeclarationBlockHandle<'a>
}

impl<'a> ParsedDeclarationBlockProvider<'a> {
    fn try_new<P, E>(code: String, parser: P) -> Result<ParsedDeclarationBlockProvider<'a>, E>
    where
        P: FnOnce(&'a str) -> Result<DeclarationBlock<'a>, E>
    {
        Ok(
            ParsedDeclarationBlockProvider {
                handle: OwningHandle::try_new(
                    code,

                    |code_ptr| parser(
                        unsafe { &*code_ptr }
                    ).map(|stylesheet| Rc::new(RefCell::new(stylesheet)))
                )?
            }
        )
    }
}

impl<'a> DeclarationBlockProvider<'a> for ParsedDeclarationBlockProvider<'a> {
  fn borrow(&self) -> Ref<'a, DeclarationBlock> {
      self.handle.borrow()
  }

  fn borrow_mut(&self) -> RefMut<'a, DeclarationBlock> {
      self.handle.borrow_mut()
  }
}

pub fn initialize() -> Result<(), Error> {
  let module = define_module("Classmate")?;
  let class = module.define_class("DeclarationList", object())?;

  class.define_singleton_method("parse", function!(DeclarationList::parse, 1))?;
  class.define_singleton_method("new", function!(DeclarationList::new, 0))?;

  class.define_method("proxy", method!(DeclarationList::proxy, 0))?;

  class.define_method("to_css", method!(DeclarationList::to_css, 0))?;
  class.define_alias("to_s", "to_css")
}
