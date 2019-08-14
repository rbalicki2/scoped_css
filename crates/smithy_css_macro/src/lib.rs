extern crate proc_macro;

mod types;

use nom::{
  branch::alt,
  bytes::streaming::take_while_m_n,
  error::ErrorKind,
  sequence::tuple,
  Err,
  IResult,
  Needed,
};

use proc_macro2::{
  Delimiter,
  Group,
  Ident,
  Literal,
  Punct,
  Span,
  TokenStream,
  TokenTree,
};

type TokenTreeSlice<'a> = &'a [TokenTree];
type TokenTreeVec = Vec<TokenTree>;
type TokenStreamIResult<'a, T> = IResult<TokenTreeSlice<'a>, T>;
type TokenStreamIResult2<T> = IResult<TokenStream, T>;

fn stream_to_tree_vec(input: &TokenStream) -> TokenTreeVec {
  input.clone().into_iter().collect::<TokenTreeVec>()
}

fn slice_to_stream(input: TokenTreeSlice) -> TokenStream {
  input.iter().map(|x| x.clone()).collect()
}

fn get_group_contents<'a>(
  tree: &'a TokenTree,
  delimiter: Option<Delimiter>,
) -> Option<TokenTreeVec> {
  match tree {
    TokenTree::Group(g) => match (delimiter, g.delimiter()) {
      (Some(target_delimiter), group_delimiter) if target_delimiter == group_delimiter => {
        Some(stream_to_tree_vec(&g.stream()))
      },
      _ => Some(stream_to_tree_vec(&g.stream())),
    },
    _ => None,
  }
}

/// Parse and yield the contents of a group, if that group is the first item
/// in the token stream
fn parse_group_with_delimiter(
  input: TokenStream,
  delimiter: Option<Delimiter>,
) -> TokenStreamIResult2<TokenStream> {
  // let cloned = input.clone();
  let vec = stream_to_tree_vec(&input);
  match vec.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Group(ref g) => {
        if let Some(target_delimiter) = delimiter {
          if (g.delimiter() == target_delimiter) {
            Ok((slice_to_stream(rest), g.stream()))
          } else {
            Err(Err::Error((input, ErrorKind::TakeTill1)))
          }
        } else {
          Ok((slice_to_stream(rest), g.stream()))
        }
      },
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
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

fn parse_ident(input: TokenStream) -> IResult<TokenStream, Ident> {
  println!("parse ident input {:?}", input);
  if let Some((first_tree, rest)) = stream_to_tree_vec(&input).split_first() {
    match first_tree {
      TokenTree::Ident(ident) => Ok((slice_to_stream(rest), ident.clone())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    }
  } else {
    Err(Err::Incomplete(Needed::Size(1)))
  }
}

fn parse_attribute_symbol(input: TokenStream) -> TokenStreamIResult2<String> {
  // let cloned = input.clone();

  // match input.split_first() {
  //   Some((first, rest)) => match first {
  //     TokenTree::Punct(punct) => Ok((rest, punct.to_string())),
  //     _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
  //   },
  //   None => Err(Err::Incomplete(Needed::Size(1))),
  // }
  unimplemented!()
}

fn parse_literal_or_ident(input: TokenTreeSlice) -> TokenStreamIResult<String> {
  match input.split_first() {
    Some((first, rest)) => match first {
      TokenTree::Literal(l) => Ok((rest, l.to_string())),
      TokenTree::Ident(i) => Ok((rest, i.to_string())),
      _ => Err(Err::Error((input, ErrorKind::TakeTill1))),
    },
    None => Err(Err::Incomplete(Needed::Size(1))),
  }
}

fn parse_attribute_contents<'a>(
  (rest, input): (TokenStream, TokenStream),
) -> TokenStreamIResult2<types::AttributeModifier> {
  // let (rest, (lhs, symbol, rhs)) = tuple((
  //   // TODO parse idents with dashes in them
  //   parse_ident,
  //   parse_attribute_symbol,
  //   parse_literal_or_ident
  // ))(&input)?;
  // println!("rest={:?}", rest);
  // println!("lhs {:?}, symbol {:?}, rhs {:?}", lhs, symbol, rhs);
  // let input = stream_to_tree_vec(input);

  // let (rest, (lhs, symbol, rhs)) =
  //   tuple((parse_ident, parse_attribute_symbol, parse_literal_or_ident))(&input)?;
  // parse_ident(&input);
  println!("ABC!");
  let (rest, (lhs, symbol, rhs)) =
    tuple((parse_ident, parse_attribute_symbol, parse_ident))(input)?;
  // println!("something {:?}", something);
  // let something: () = something;
  unimplemented!()
}

fn parse_attribute(input: TokenStream) -> TokenStreamIResult2<types::AttributeModifier> {
  let a =
    parse_group_with_delimiter(input, Some(Delimiter::Bracket)).and_then(parse_attribute_contents);
  println!("a = {:?}", a);
  unimplemented!()
}

#[proc_macro]
pub fn css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: TokenStream = input.into();
  println!("{:?}", input);
  // this seems like a hack. What I want is an iterator of &TokenTree's,
  // but TokenStream only implements into_iter for some reason
  //
  // (We actually need a slice of TokenTree's)
  // let input = input.into_iter().collect::<TokenTreeVec>();

  let foo = parse_attribute(input);
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
