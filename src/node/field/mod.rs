mod attribute;

use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Error;
use syn::Result;

use self::attribute::Attribute;

#[derive(Clone, Debug)]
pub enum Field {
    Attribute(Attribute),
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(v) = Attribute::parse(&input) {
            Ok(Self::Attribute(v))
        } else {
            Err(Error::new(input.span(), "failed to parse field"))
        }
    }
}
