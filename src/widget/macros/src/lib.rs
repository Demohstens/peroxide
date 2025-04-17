use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
};

struct Field {
    name: Ident,
    colon_token: Token![:],
    value: Expr,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Field {
            name: input.parse()?,
            colon_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct Fields {
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for Fields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Fields {
            fields: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro]
pub fn container(input: TokenStream) -> TokenStream {
    let Fields { fields } = parse_macro_input!(input as Fields);

    // Set defaults
    let mut width = None;
    let mut height = None;
    let mut color = None;
    let mut child = None;
    let mut parent = quote! { None };
    let mut border = quote! { false };
    let mut x = quote! { 0 };
    let mut y = quote! { 0 };
    let mut id = quote! { 0 };
    let mut decoration = quote! { None };

    for field in fields {
        let name = field.name.to_string();
        let value = field.value;
        match name.as_str() {
            "width" => width = Some(quote! { #value }),
            "height" => height = Some(quote! { #value }),
            "color" => color = Some(quote! { #value }),
            "child" => child = Some(quote! { #value }),
            "parent" => parent = quote! { #value },
            "border" => border = quote! { #value },
            "x" => x = quote! { #value },
            "y" => y = quote! { #value },
            "id" => id = quote! { #value },
            "decoration" => decoration = quote! { #value },
            _ => {}
        }
    }

    let width = width.expect("Missing required field: width");
    let height = height.expect("Missing required field: height");
    let color = color.expect("Missing required field: color");
    let child = child.expect("Missing required field: child");

    let expanded = quote! {
        peroxide::widget::Container {
            width: #width,
            height: #height,
            color: #color,
            child: #child,
            parent: #parent,
            border: #border,
            x: #x,
            y: #y,
            id: #id,
            decoration: #decoration,
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn button(input: TokenStream) -> TokenStream {
    let Fields { fields } = parse_macro_input!(input as Fields);

    // Set defaults
    let mut width = Some(quote! { 100 });
    let mut height = Some(quote! { 100 });
    let mut on_click = None;
    let mut color = None;
    let mut child = None;
    let mut parent = quote! { None };
    // let mut border = quote! { false };
    let mut x = quote! { 0 };
    let mut y = quote! { 0 };
    let mut id = quote! { 0 };
    let mut decoration = quote! { None };

    for field in fields {
        let name = field.name.to_string();
        let value = field.value;
        match name.as_str() {
            "width" => width = Some(quote! { #value }),
            "height" => height = Some(quote! { #value }),
            "on_click" => on_click = Some(quote! { #value }),
            "color" => color = Some(quote! { #value }),
            "child" => child = Some(quote! { #value }),
            "parent" => parent = quote! { #value },
            // "border" => border = quote! { #value },
            "x" => x = quote! { #value },
            "y" => y = quote! { #value },
            "id" => id = quote! { #value },
            "decoration" => decoration = quote! { #value },
            _ => {}
        }
    }

    let color = color.expect("Missing required field: color");
    let child = child.expect("Missing required field: child");
    let on_click = on_click.expect("Missing required field: on_click");

    let expanded = quote! {
        peroxide::widget::Button {
            width: #width,
            height: #height,
            color: #color,
            child: #child,
            parent: #parent,
            // border: #border,
            x: #x,
            y: #y,
            id: #id,
            decoration: #decoration,
            on_click: #on_click,
        }
    };

    TokenStream::from(expanded)
}
