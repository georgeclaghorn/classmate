use magnus::{
    define_module, current_receiver,
    Value, RModule, RClass, Module, Object, Error,
    TypedData, DataType, DataTypeFunctions,
    class::object,
    block::yield_value,
    scan_args::{scan_args, get_kwargs}
};
use parcel_css::stylesheet::{StyleSheet, ParserOptions, MinifyOptions};
use tap::Tap;
use crate::{
    providers::stylesheet::{StylesheetProvider, ParsedStylesheetProvider},
    visitors::ProxyVisitor,
    shared::printing::scan_printer_options_from_args
};

struct Stylesheet<'a, 'o> {
    provider: Box<dyn StylesheetProvider<'a, 'o>>
}

impl<'a, 'o> Stylesheet<'a, 'o> {
    fn parse(args: &[Value]) -> Result<Value, Error> {
        let (code, filename) = scan_parse_args(args)?;

        let maybe_provider = ParsedStylesheetProvider::try_new(
            filename.unwrap_or_else(|| String::from("(unknown)")),
            code,

            |filename, code| StyleSheet::parse(
                filename,
                code,
                ParserOptions::default()
            )
        );

        maybe_provider
            .map(|provider| Value::from(
                Stylesheet { provider: Box::new(provider) }
            ))
            .map_err(|error| Error::new(
                crate::errors::parse_error(),
                error.to_string()
            ))
    }

    fn new() -> Result<Stylesheet<'a, 'o>, Error> {
        Err(
            Error::new(
                magnus::exception::not_imp_error(),
                "not implemented"
            )
        )
    }

    fn minify(&'a self) -> Result<Value, Error> {
        self.provider
            .borrow_mut()
            .minify(MinifyOptions::default())
            .map_err(|error| Error::new(
                crate::errors::minify_error(),
                error.to_string()
            ))
            .and_then(|_| current_receiver())
    }

    fn proxy(&'a self) -> Result<Value, Error> {
        ProxyVisitor::new(|resource| yield_value(resource))
            .visit_from(&mut self.provider.borrow_mut())
            .and_then(|_| current_receiver())
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

unsafe impl<'a, 'o> TypedData for Stylesheet<'a, 'o> {
    fn class() -> RClass {
        *memoize!(
            RClass:
                object()
                    .const_get("Classmate")
                    .and_then(|module: RModule| module.const_get("Stylesheet"))
                    .unwrap()
        )
    }

    fn data_type() -> &'static DataType {
        memoize!(
            DataType: {
                DataType::builder::<Self>("Classmate::Stylesheet").tap_mut(|builder| {
                    builder.free_immediatly();
                    builder.size();
                }).build()
            }
        )
    }
}

impl<'a, 'o> DataTypeFunctions for Stylesheet<'a, 'o> { }

fn scan_parse_args(args: &[Value]) -> Result<(String, Option<String>), Error> {
    let args = scan_args(args)?;
    let (code,): (String,) = args.required;
    let _: () = args.optional;
    let _: () = args.splat;
    let _: () = args.trailing;
    let _: () = args.block;

    let kwargs = get_kwargs(args.keywords, &[], &["filename"])?;
    let _: () = kwargs.required;
    let (filename,): (Option<Option<String>>,) = kwargs.optional;
    let _: () = kwargs.splat;

    Ok((code, filename.flatten()))
}

pub fn initialize() -> Result<(), Error> {
    let module = define_module("Classmate")?;
    let class = module.define_class("Stylesheet", object())?;

    class.define_singleton_method("parse", function!(Stylesheet::parse, -1))?;
    class.define_singleton_method("new", function!(Stylesheet::new, 0))?;

    class.define_method("minify", method!(Stylesheet::minify, 0))?;

    class.define_method("proxy", method!(Stylesheet::proxy, 0))?;

    class.define_method("to_css", method!(Stylesheet::to_css, -1))?;
    class.define_alias("to_s", "to_css")
}
