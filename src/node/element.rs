use syn::parenthesized;
use syn::parse::discouraged::Speculative;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::token::Paren;
use syn::Error;
use syn::Ident;
use syn::Result;
use syn::Token;

use super::Field;
use super::Node;

#[derive(Clone, Debug)]
pub struct Element {
    pub tag: Ident,
    pub fields: Vec<Field>,
    pub children: Vec<Node>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse element tag
        // TODO: also support string literals, e.g "my-web-component"
        let tag: Ident = input.parse()?;
        if !tag.to_string().chars().any(|c| c.is_alphabetic()) {
            return Err(Error::new(tag.span(), "invalid element name"));
        }

        // If no parenthesis after the tag, then it's an empty tag
        if !input.peek(Paren) {
            return Ok(Self {
                tag,
                fields: vec![],
                children: vec![],
            });
        }

        // Parse parenthesized node body
        let body;
        let _ = parenthesized!(body in input);

        // Parse fields while can
        let mut fields = vec![];
        loop {
            let fork = body.fork();
            match Field::parse(&fork) {
                Ok(v) => fields.push(v),
                Err(_) => break,
            }
            if fork.peek(Token![,]) {
                let _ = fork.parse::<Token![,]>();
            }
            body.advance_to(&fork);
        }

        // Parse children until the end of body
        let children = body
            .parse_terminated(Node::parse, Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            tag,
            fields,
            children,
        })
    }
}
