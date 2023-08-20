use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::Result;
use syn::Token;

use crate::node::Node;

#[derive(Clone, Debug)]
pub struct Body {
    pub nodes: Vec<Node>,
}

impl Parse for Body {
    fn parse(input: ParseStream) -> Result<Self> {
        let nodes = input.parse_terminated(Node::parse, Token![,])?;
        Ok(Self {
            nodes: nodes.into_iter().collect(),
        })
    }
}
