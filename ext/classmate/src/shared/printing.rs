use magnus::{
    Value, Error,
    scan_args::{scan_args, get_kwargs, Args, KwArgs}
};
use parcel_css::stylesheet::PrinterOptions;

pub fn scan_printer_options_from_args(args: &[Value]) -> Result<PrinterOptions, Error> {
    scan_args(args)
        .and_then(|args: Args<(), (), (), (), _, ()>| get_kwargs(args.keywords, &[], &["minify"]))
        .map(|kwargs: KwArgs<(), (Option<bool>,), ()>| kwargs.optional)
        .map(|(minify,)| PrinterOptions {
            minify: minify.unwrap_or(false),
            ..PrinterOptions::default()
        })
}
