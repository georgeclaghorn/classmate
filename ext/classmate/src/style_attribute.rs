use magnus::{
    define_module, current_receiver,
    Value, RModule, RClass, Module, Object, Error,
    TypedData, DataType, DataTypeFunctions,
    class::object,
    block::{block_given, yield_value}
};
use parcel_css::stylesheet::{ParserOptions, MinifyOptions};
use tap::Tap;
use crate::{
    providers::style_attribute::{StyleAttributeProvider, ParsedStyleAttributeProvider},
    visitors::ProxyVisitor,
    shared::printing::scan_printer_options_from_args
};

struct StyleAttribute<'a> {
    provider: Box<dyn StyleAttributeProvider<'a>>
}

impl<'a> StyleAttribute<'a> {
    fn parse(code: String) -> Result<Value, Error> {
        let maybe_provider =
            ParsedStyleAttributeProvider::try_new(
                code,

                |code| parcel_css::stylesheet::StyleAttribute::parse(
                    code,
                    ParserOptions::default()
                )
            );

        maybe_provider
            .map(|provider| Value::from(
                StyleAttribute { provider: Box::new(provider) }
            ))
            .map_err(|error| Error::new(
                crate::errors::parse_error(),
                error.to_string()
            ))
    }

    fn new() -> Result<StyleAttribute<'a>, Error> {
        Err(
            Error::new(
                magnus::exception::not_imp_error(),
                "not implemented"
            )
        )
    }

    fn minify(&'a self) -> Result<Value, Error> {
        self.provider.borrow_mut().minify(MinifyOptions::default());

        current_receiver()
    }

    fn proxy(&'a self) -> Result<Value, Error> {
        if block_given() {
            ProxyVisitor::new(|resource| yield_value(resource.to_string()))
                .visit_from(&mut self.provider.borrow_mut())
                .and_then(|_| current_receiver())
        } else {
            Err(
                Error::new(
                    magnus::exception::arg_error(),
                    "no block given"
                )
            )
        }
    }

    fn to_css(&'a self, args: &[Value]) -> Result<Value, Error> {
        self.provider
            .borrow()
            .to_css(scan_printer_options_from_args(args)?)
            .map(|output| Value::from(output.code))
            .map_err(|error| Error::new(
                crate::errors::print_error(),
                error.to_string()
            ))
    }
}

unsafe impl<'a> TypedData for StyleAttribute<'a> {
  fn class() -> RClass {
        *memoize!(
            RClass:
                object()
                    .const_get("Classmate")
                    .and_then(|module: RModule| module.const_get("StyleAttribute"))
                    .unwrap()
        )
    }

    fn data_type() -> &'static DataType {
        memoize!(
            DataType: {
                DataType::builder::<Self>("Classmate::StyleAttribute").tap_mut(|builder| {
                    builder.free_immediatly();
                    builder.size();
                }).build()
            }
        )
    }
}

impl<'a> DataTypeFunctions for StyleAttribute<'a> { }

pub fn initialize() -> Result<(), Error> {
    let module = define_module("Classmate")?;
    let class = module.define_class("StyleAttribute", object())?;

    class.define_singleton_method("parse", function!(StyleAttribute::parse, 1))?;
    class.define_singleton_method("new", function!(StyleAttribute::new, 0))?;

    class.define_method("minify", method!(StyleAttribute::minify, 0))?;

    class.define_method("proxy", method!(StyleAttribute::proxy, 0))?;

    class.define_method("to_css", method!(StyleAttribute::to_css, -1))?;
    class.define_alias("to_s", "to_css")
}
