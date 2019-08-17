use crate::parser_types::*;

pub fn parse_class_selector(input: TokenStream) -> TokenStreamIResult<String> {
  let (rest, _punct) = crate::core::parse_punct(input, None, Some('.'))?;
  let (rest, ident) = crate::core::parse_ident(rest)?;
  Ok((rest, ident.to_string()))
}
