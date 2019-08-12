extern crate proc_macro;

mod types;

use nom::{
  bytes::streaming::take_while_m_n,
  error::ErrorKind,
  sequence::tuple,
  Err,
  IResult,
  Needed,
  branch::alt,
};

use proc_macro2::{
  Delimiter,
  Group,
  TokenStream,
  TokenTree,
  Ident,
  Literal,
  Punct,
};

type TokenTreeSlice<'a> = &'a [TokenTree];
type TokenTreeVec = Vec<TokenTree>;
type TokenStreamIResult<'a, T> = IResult<TokenTreeSlice<'a>, T>;

fn stream_to_tree_vec(input: TokenStream) -> TokenTreeVec {
  input.into_iter().collect::<TokenTreeVec>()
}

fn get_group_contents<'a>(
  tree: &'a TokenTree,
  delimiter: Option<Delimiter>,
) -> Option<TokenTreeVec> {
  match tree {
    TokenTree::Group(g) => match (delimiter, g.delimiter()) {
      (Some(target_delimiter), group_delimiter) if target_delimiter == group_delimiter => {
        Some(g.stream())
      },
      _ => Some(g.stream()),
    },
    _ => None,
  }
  .map(stream_to_tree_vec)
}

/// Parse and yield the contents of a group, if that group is the first item
/// in the token stream
fn parse_group_with_delimiter<'a>(
  input: TokenTreeSlice<'a>,
  delimiter: Option<Delimiter>,
) -> TokenStreamIResult<'a, TokenTreeSlice<'a>> {
  match input.split_first() {
    Some((first, rest)) => {
      if let Some(ref group_contents_stream) = get_group_contents(first, delimiter) {
        Ok((rest, group_contents_stream))
      } else {
        Err(Err::Error((input, ErrorKind::TakeTill1)))
      }
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

// // Do not need this
// fn many_groups(input: TokenTreeSlice) -> TokenStreamIResult<Vec<TokenStream>> {
//   let parse_group = |input| parse_group_with_delimiter(input, None);
//   let (rest, (g1, g2, g3)) = tuple((parse_group, parse_group, parse_group))(input)?;
//   Ok((rest, vec![g1, g2, g3]))
// }

fn parse_ident(input: TokenTreeSlice) -> TokenStreamIResult<Ident> {
  match input.split_first() {
    Some((first, rest)) => {
      match first {
        TokenTree::Ident(ident) => Ok((rest, ident.clone())),
        _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
      }
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

fn parse_attribute_symbol(input: TokenTreeSlice) -> TokenStreamIResult<String> {
  match input.split_first() {
    Some((first, rest)) => {
      match first {
        TokenTree::Punct(punct) => Ok((rest, punct.to_string())),
        _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
      }
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

fn parse_literal_or_ident(input: TokenTreeSlice) -> TokenStreamIResult<String> {
  match input.split_first() {
    Some((first, rest)) => {
      match first {
        TokenTree::Literal(l) => Ok((rest, l.to_string())),
        TokenTree::Ident(i) => Ok((rest, i.to_string())),
        _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
      }
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

fn parse_attribute_contents<'a>(
  (rest, input): (TokenTreeSlice<'a>, TokenTreeSlice<'a>),
) -> TokenStreamIResult<'a, types::AttributeModifier> {
  // let (rest, (lhs, symbol, rhs)) = tuple((
  //   // TODO parse idents with dashes in them
  //   parse_ident,
  //   parse_attribute_symbol,
  //   parse_literal_or_ident
  // ))(&input)?;
  // println!("rest={:?}", rest);
  // println!("lhs {:?}, symbol {:?}, rhs {:?}", lhs, symbol, rhs);

  unimplemented!()
}

fn parse_attribute(input: TokenTreeSlice) -> TokenStreamIResult<types::AttributeModifier> {
  parse_group_with_delimiter(input, Some(Delimiter::Bracket))
    .and_then(parse_attribute_contents)
}

#[proc_macro]
pub fn css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: TokenStream = input.into();
  println!("{:?}", input);
  // this seems like a hack. What I want is an iterator of &TokenTree's,
  // but TokenStream only implements into_iter for some reason
  //
  // Turns out we are using a slice of TokenTree's, which also seems wrong.
  let input = input.into_iter().collect::<TokenTreeVec>();

  let foo = parse_attribute(&input);
  match foo {
    Ok((rest, some_vec)) => {
      let foo = format!("{:?}", some_vec);
      quote::quote!({
        #foo
        // vec![
        //   #(#some_vec),*
        // ]
      })
    }
    .into(),
    _ => unimplemented!("NOOO"),
  }
  // println!("foo = {:?}", foo);
  // quote::quote!(123).into()

  // let starts_with_group = parse_group_with_delimiter(tree_vec.as_slice(), None);

  // println!("starts with group {:?}", starts_with_group);
  // // println!("css input {}", input);

  // // println!(
  // //   "CSS is some ???? {:?}",
  // //   get_group_contents(&input.into_iter().next().unwrap(), Some(Delimiter::Brace))
  // // );

  // quote::quote!({
  //   #[derive(Debug, Clone)]
  //   struct CssClasses {
  //     my_class: String,
  //   }
  //   #[derive(Debug, Clone)]
  //   struct CssIds {}
  //   #[derive(Debug, Clone)]
  //   struct CssWrapper {
  //     classes: CssClasses,
  //     ids: CssIds,
  //   }
  //   // TODO figure out why this doesn't work
  //   // = help: message: attempt to subtract with overflow
  //   //
  //   // TODO: divide this into smithy_css_core and include this impl only later
  //   // impl CssWrapper {
  //   //   pub fn style_tag<'a>(&self) -> smithy::types::SmithyComponent<'a> {
  //   //     // smithy::smd!(<style>
  //   //     //   foo
  //   //     // </style>)
  //   //     let a = smithy::smd!(<div>a</div>);
  //   //     smithy::smd!()
  //   //   }
  //   // }

  //   impl ToString for CssWrapper {
  //     fn to_string(&self) -> String {
  //       format!(".{} {{ background-color: red; }}", self.classes.my_class)
  //     }
  //   }

  //   let my_class = "foo".into();
  //   CssWrapper {
  //     classes: CssClasses { my_class },
  //     ids: CssIds {},
  //   }
  // })
  // .into()
}