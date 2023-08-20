use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Expr;
use syn::Ident;
use syn::Result;
use syn::Token;

// Static (once set, there's no going back) HTML attribute.
#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: Ident,
    pub equals_token: Token![=],
    pub value: Expr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            // TODO: support string literals, like "aria-label"
            name: input.parse()?,
            equals_token: input.parse()?,
            value: input.parse()?,
        })
    }
}
