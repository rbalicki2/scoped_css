use nom::{
  error::ErrorKind,
  Err,
  IResult,
};
use proc_macro2::{
  TokenStream,
  TokenTree,
};
type TokenStreamIResult<T> = IResult<TokenStream, T>;
type TokenTreeSlice<'a> = &'a [TokenTree];
type TokenTreeVec = Vec<TokenTree>;

pub fn ensure_consumed(rest: TokenStream) -> TokenStreamIResult<()> {
  if !rest.is_empty() {
    Err(Err::Error((rest, ErrorKind::TakeTill1)))
  } else {
    Ok((rest, ()))
  }
}

pub fn stream_to_tree_vec(input: &TokenStream) -> TokenTreeVec {
  input.clone().into_iter().collect::<TokenTreeVec>()
}

pub fn slice_to_stream(input: TokenTreeSlice) -> TokenStream {
  input.iter().map(|x| x.clone()).collect()
}

pub fn alt<T>(
  b1: impl Fn(TokenStream) -> TokenStreamIResult<T>,
  b2: impl Fn(TokenStream) -> TokenStreamIResult<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<T> {
  move |input| {
    let cloned = input.clone();
    b1(input).or_else(|_| b2(cloned))
  }
}

pub fn many_0<T: std::fmt::Debug>(
  f: impl Fn(TokenStream) -> TokenStreamIResult<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<Vec<T>> {
  move |mut i: TokenStream| {
    let mut acc = vec![];
    let mut last_len = stream_to_tree_vec(&i).len();
    loop {
      match f(i.clone()) {
        Err(Err::Error(_)) => return Ok((i, acc)),
        Err(e) => return Err(e),
        Ok((i1, o)) => {
          // TODO I'm not sure if this block is necessary, but there was a similar
          // block in the original (nom) source code.
          let new_len = stream_to_tree_vec(&i1).len();
          if last_len == new_len {
            if acc.len() > 0 {
              return Ok((i, acc));
            }
            return Err(Err::Error((i, ErrorKind::Many0)));
          }
          last_len = new_len;

          i = i1;
          acc.push(o);
        },
      }
    }
  }
}

fn to_adjacency_vec(input: &TokenStream) -> Vec<bool> {
  let mut starts_and_ends = input
    .clone()
    .into_iter()
    .map(|x| {
      let span = x.span();
      (span.start(), span.end())
    })
    .peekable();

  let mut adjacency_vec: Vec<bool> = vec![];
  loop {
    if let (Some((_current_start, current_end)), Some((next_start, _next_end))) =
      (starts_and_ends.next(), starts_and_ends.peek())
    {
      adjacency_vec.push(current_end == *next_start);
    } else {
      break;
    }
  }
  adjacency_vec
}

/// Like many_0, except it only continues to match if adjacent
/// matched results have adjacent spans. That is, if
/// first_matched.span().start() == last_matched.span().end()
/// (TODO check that shit)
pub fn many_0_joint<T>(
  f: impl Fn(TokenStream) -> TokenStreamIResult<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<Vec<T>> {
  move |mut i: TokenStream| {
    let adjacency_vec = to_adjacency_vec(&i);
    let len = adjacency_vec.len();
    let is_adjacent = |remaining_length: usize| {
      return *adjacency_vec.get(len - remaining_length).unwrap_or(&false);
    };

    let mut acc = vec![];
    let mut last_len = stream_to_tree_vec(&i).len();
    loop {
      match f(i.clone()) {
        Err(Err::Error(_)) => return Ok((i, acc)),
        Err(e) => return Err(e),
        Ok((i1, o)) => {
          // TODO I'm not sure if this block is necessary, but there was a similar
          // block in the original (nom) source code.
          let new_len = stream_to_tree_vec(&i1).len();
          if last_len == new_len {
            if acc.len() > 0 {
              return Ok((i, acc));
            }
            return Err(Err::Error((i, ErrorKind::Many0)));
          }
          last_len = new_len;

          i = i1;
          acc.push(o);

          if !is_adjacent(stream_to_tree_vec(&i).len()) {
            return Ok((i, acc));
          }
        },
      }
    }
  }
}

pub fn take_until_and_match<T>(
  predicate: impl Fn(TokenStream) -> TokenStreamIResult<T>,
) -> impl Fn(TokenStream) -> TokenStreamIResult<(TokenStream, T)> {
  move |mut input: TokenStream| {
    let mut acc: Vec<TokenTree> = vec![];
    loop {
      if input.is_empty() {
        // is this correct? is this block necessary?
        return Err(Err::Error((input, ErrorKind::Many0)));
      }
      match predicate(input.clone()) {
        Ok((rest, t)) => return Ok((rest, (slice_to_stream(&acc), t))),
        Err(Err::Error(_)) => {
          let vec = stream_to_tree_vec(&input);
          let (first, rest) = vec.split_first().unwrap();
          acc.push(first.clone());
          input = slice_to_stream(&rest);
        },
        Err(e) => return Err(e),
      }
    }
  }
}
