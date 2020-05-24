use syn::Attribute;
use syn::parse::{
    Parse,
    ParseStream,
    Result,
};

/*
fn parse_attributes(attrs: &[syn::Attribute]) -> HashMap<&str, &str> {

}

*/

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct MethodInput {
    attrs: Vec<Attribute>,
}

impl Parse for MethodInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(
            MethodInput {
                attrs: input.call(Attribute::parse_outer)?,
            }
        )
    }
}
