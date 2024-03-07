use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::quote_spanned;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::AngleBracketedGenericArguments;
use syn::Data;
use syn::DeriveInput;
use syn::Field;
use syn::Fields;
use syn::GenericArgument;
use syn::Ident;
use syn::PathArguments;
use syn::PathSegment;
use syn::Type;
use syn::TypePath;

/**
Generate a config struct and a method to merge the two values together.

```no_run
# use clap::CommandFactory;
# use clap::Parser;
# use clap_config::ClapConfig;
# use std::fs;

#[derive(ClapConfig, Parser, Debug)]
pub struct Opts {
    #[clap(long)]
    flag: String,
}

// You can use any file format that implements Deserialize.
let config_str = fs::read_to_string("/path/to/config.yaml").unwrap();

// Build an ArgMatches so we can see where each value comes from.
let matches = <Opts as CommandFactory>::command().get_matches();
// Build an instance of the auto-generated <YourStruct>Config struct
let config: OptsConfig = serde_yaml::from_str(&config_str).unwrap();

// Merge the two together into your actual struct.
let opts = Opts::from_merged(matches, config);
```
*/
#[proc_macro_derive(ClapConfig)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Name of the struct we're creating a Config version of.
    let input_struct_name = input.ident;
    // Name of the config struct we' creating.
    let config_struct_name = format_ident!("{}Config", input_struct_name);

    let input_fields = input_struct_fields(&input.data);

    let config_fields = make_fields_optional(input_fields);

    let merge_method = generate_merge_method(&config_struct_name, input_fields);

    let output = quote!(
        #[derive(std::default::Default, std::fmt::Debug, serde::Deserialize, serde::Serialize)]
        pub struct #config_struct_name {
            #config_fields
        }

        impl #input_struct_name {
            #merge_method
        }
    );

    proc_macro::TokenStream::from(output)
}

/// An iterator over the fields in the input struct.
fn input_struct_fields(data: &Data) -> &Punctuated<Field, Comma> {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}

/// Convert any fields that aren't already `Option<...>` to `Option<...>` fields, ensuring
/// everything is optional.
fn make_fields_optional(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let optional_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if strip_optional_wrapper_if_present(f).is_some() {
            quote_spanned!(f.span()=> #name: #ty)
        } else {
            quote_spanned!(f.span()=> #name: std::option::Option<#ty>)
        }
    });

    quote! {
        #(#optional_fields),*
    }
}

/**
Generate method that merges our config into the clap-generated struct, with precedence being:

- Things specified via `--arg` or `$ENV_VAR`
- Things in the config
- Clap defaults
*/
fn generate_merge_method(
    config_struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    let struct_fields = fields.iter().map(|f| {
        let name = &f.ident;
        quote!(#name)
    });

    let field_updates = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let span = ty.span();
        let name_str = name.as_ref().map(|name| name.to_string()).expect("Expected field to have a name");

        if let Some(stripped_ty) = strip_optional_wrapper_if_present(f) {
            // User-specified field's type was `Option<T>`
            quote_spanned! {span=>
                let #name: #ty = {
                    let config_value: #ty = config.#name.take();
                    if matches.contains_id(#name_str) {
                        let value_source = matches.value_source(#name_str).expect("checked contains_id");
                        let matches_value: #stripped_ty = matches.remove_one(#name_str).expect("checked contains_id");
                        if value_source == clap::parser::ValueSource::DefaultValue {
                            Some(config_value.unwrap_or(matches_value))
                        } else {
                            Some(matches_value)
                        }
                    } else {
                        config_value
                    }
                };
            }
        } else if strip_vec_wrapper_if_present(f).is_some() {
            // User-specified field's type was `Vec<T>`
            quote_spanned! {span=>
                let #name: #ty = {
                    let config_value: std::option::Option<#ty> = config.#name.take();
                    if matches.contains_id(#name_str) {
                        let value_source = matches.value_source(#name_str).expect("checked contains_id");
                        let matches_value: #ty = matches.remove_many(#name_str).expect("checked contains_id").collect();
                        if value_source == clap::parser::ValueSource::DefaultValue {
                            config_value.unwrap_or(matches_value)
                        } else {
                            matches_value
                        }
                    } else {
                        config_value.unwrap_or_default()
                    }
                };
            }
        } else {
            quote_spanned! {span=>
                let #name: #ty = {
                    let config_value: std::option::Option<#ty> = config.#name.take();
                    if matches.contains_id(#name_str) {
                        let value_source = matches.value_source(#name_str).expect("checked contains_id");
                        let matches_value: #ty = matches.remove_one(#name_str).expect("checked contains_id");
                        if value_source == clap::parser::ValueSource::DefaultValue {
                            config_value.unwrap_or(matches_value)
                        } else {
                            matches_value
                        }
                    } else {
                        config_value.unwrap_or_default()
                    }
                };
            }
        }
    });

    quote! {
        pub fn from_merged(mut matches: clap::ArgMatches, mut config: #config_struct_name) -> Self {

            #(#field_updates)*

            Self {
                #(#struct_fields),*
            }
        }
    }
}

/// If the field type is `Option<Foo>`, return `Some(Foo)`. Else return `None`.
fn strip_optional_wrapper_if_present(f: &Field) -> Option<&Type> {
    let ty = &f.ty;
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(PathSegment { ident, arguments }) = path.segments.last() {
            if ident == &Ident::new("Option", f.span()) {
                if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    args, ..
                }) = arguments
                {
                    if let Some(GenericArgument::Type(inner_type)) = args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }
    None
}

/// If the field type is `Vec<Foo>`, return `Some(Foo)`. Else return `None`.
fn strip_vec_wrapper_if_present(f: &Field) -> Option<&Type> {
    let ty = &f.ty;

    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(PathSegment { ident, arguments }) = path.segments.last() {
            if ident == &Ident::new("Vec", f.span()) {
                if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    args, ..
                }) = arguments
                {
                    if let Some(GenericArgument::Type(inner_type)) = args.first() {
                        return Some(inner_type);
                    }
                }
            }
        }
    }

    None
}
