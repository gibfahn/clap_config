use syn::{punctuated::Punctuated, spanned::Spanned, Field, Ident, Meta, Token};

const CLAP_ATTR_NAME: &str = "clap";

pub(crate) const CLAP_SUBCOMMAND_ATTR: &str = "subcommand";
pub(crate) const CLAP_SKIP_ATTR: &str = "skip";

// Returns whether the field has a field attribute `#[clap(field_name)]`.
pub(crate) fn has_field(f: &Field, field_name: &str) -> Result<bool, syn::Error> {
    let mut field_found = false;
    'outer: for attr in f.attrs.iter() {
        if attr.path().is_ident(CLAP_ATTR_NAME) {
            for meta in attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)? {
                if let Meta::Path(path) = meta {
                    if path.is_ident(&Ident::new(field_name, path.span())) {
                        field_found = true;
                        break 'outer;
                    }
                }
            }
        }
    }

    Ok(field_found)
}
