mod component;
mod element;
mod field;

use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Error;
use syn::Expr;
use syn::Result;

use self::component::Component;
use self::element::Element;
use self::field::Field;

#[derive(Clone, Debug)]
pub enum Node {
    Element(Element),
    Component(Component),
    Expression(Expr),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self::Element(Element::parse(&input)?))
        // if let Ok(v) = Element::parse(&input) {
        //     Ok(Self::Element(v))
        // } else if let Ok(v) = Component::parse(&input) {
        //     Ok(Self::Component(v))
        // } else if let Ok(v) = Expr::parse(&input) {
        //     Ok(Self::Expression(v))
        // } else {
        //     Err(Error::new(input.span(), "failed to parse node"))
        // }
    }
}
