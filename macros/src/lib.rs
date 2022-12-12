#![feature(box_patterns)]

extern crate proc_macro;
use {
    proc_macro::TokenStream,
    quote::{format_ident, quote},
    syn::{
        parse::{Parse, ParseStream},
        parse_macro_input,
        punctuated::Punctuated,
        token::Comma,
        Expr, ExprLit, ExprRange, ExprTuple, Lit, RangeLimits, Result,
    },
};

struct Year(usize);
struct DayRange(std::ops::RangeInclusive<u8>);

struct FinishedParts(Vec<(Year, DayRange)>);

impl Parse for FinishedParts {
    fn parse(input: ParseStream) -> Result<Self> {
        let args = Punctuated::<Expr, Comma>::parse_terminated(input)?;
        let mut finished_parts = FinishedParts(Vec::new());
        for arg in args {
            match arg {
                Expr::Tuple(ExprTuple { elems, .. }) => {
                    let mut elems = elems.iter();
                    let year = Year(
                        match elems
                            .next()
                            .ok_or(syn::Error::new(input.span(), "Expected year literal"))?
                        {
                            Expr::Lit(ExprLit {
                                lit: Lit::Int(year),
                                ..
                            }) => year.base10_parse::<usize>()?,
                            _ => {
                                return Err(syn::Error::new(
                                    input.span(),
                                    "Expected year to be a base 10 number",
                                ))
                            }
                        },
                    );
                    let day_range = DayRange(
                        match elems
                            .next()
                            .ok_or(syn::Error::new(input.span(), "Expected day range literal"))?
                        {
                            Expr::Range(ExprRange {
                                from:
                                    Some(box Expr::Lit(ExprLit {
                                        lit: Lit::Int(day_range_begin),
                                        ..
                                    })),
                                limits: RangeLimits::Closed(_),
                                to:
                                    Some(box Expr::Lit(ExprLit {
                                        lit: Lit::Int(day_range_end),
                                        ..
                                    })),
                                ..
                            }) => {
                                day_range_begin.base10_parse::<u8>()?
                                    ..=day_range_end.base10_parse::<u8>()?
                            }
                            _ => {
                                return Err(syn::Error::new(
                                    input.span(),
                                    "Expected year to be a base 10 number",
                                ))
                            }
                        },
                    );
                    finished_parts.0.push((year, day_range));
                }
                _ => return Err(syn::Error::new(input.span(), "Expected (year, day) tuple")),
            }
        }
        Ok(finished_parts)
    }
}
#[proc_macro]
pub fn make_init_parts(input: TokenStream) -> TokenStream {
    let finished_parts: FinishedParts = parse_macro_input!(input);
    let (years, days, year_day_idents) =
        finished_parts
            .0
            .iter()
            .fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, p| {
                acc.0.extend(vec![p.0 .0; p.1 .0.len()]);
                acc.1.extend(p.1 .0.clone());
                acc.2.extend(
                    p.1 .0
                        .clone()
                        .map(|d| format_ident!("year{}day{:02}", p.0 .0, d))
                        .collect::<Vec<_>>(),
                );
                acc
            });
    quote! {
        pub fn init_parts() {
            #(PARTS.register((#years, #days), [#year_day_idents::part1, #year_day_idents::part2]);)*
        }

        #(pub mod #year_day_idents;)*
    }
    .into()
}
