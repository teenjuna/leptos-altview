use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::Ident;
use syn::Result;
use syn::Token;

use super::Field;
use super::Node;

#[derive(Clone, Debug)]
pub struct Component {
    pub name: Ident,
    pub fields: Punctuated<Field, Token![,]>,
    pub children: Punctuated<Node, Token![,]>,
}

impl Parse for Component {
    fn parse(_input: ParseStream) -> Result<Self> {
        todo!("Component::parse is not implemented")
    }
}
