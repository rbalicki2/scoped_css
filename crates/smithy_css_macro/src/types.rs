// TODO handle +, >, pseudo selectors, etc.
use std::ops::Deref;

pub type Element = String;

// Going off of https://www.w3schools.com/css/css_attribute_selectors.asp
#[derive(Debug, Clone)]
pub enum AttributeRelation {
  Equal(String),          // =
  Containing(String),     // ~=
  BeginsWithWord(String), // |=
  BeginsWith(String),     // ^=
  EndsWith(String),       // $=
  Contains(String),       // *=
}

impl AttributeRelation {
  pub fn from_strings(s1: &str, s2: String) -> Option<AttributeRelation> {
    match s1 {
      "=" => Some(AttributeRelation::Equal(s2)),
      _ => None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct AttributeModifier {
  pub attribute: String,
  pub relation: Option<AttributeRelation>,
}

#[derive(Debug, Clone)]
pub enum Modifier {
  Class(String),
  Id(String),
  Attribute(AttributeModifier),
}

// TODO is Selector { element: None, modifiers: vec![] } valid?
#[derive(Debug, Clone)]
pub struct Selector {
  pub element: Option<Element>,
  pub modifiers: Vec<Modifier>,
}

/// A NestedSelector is something like body .foo .bar,
/// which would be a vec of length 3
#[derive(Debug, Clone)]
pub struct NestedSelector(Vec<Selector>);

impl Deref for NestedSelector {
  type Target = Vec<Selector>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

/// A SelectorList is something like body .foo .bar, body .baz .baz,
/// which would be a vec of length 2
#[derive(Debug, Clone)]
pub struct SelectorList(Vec<NestedSelector>);

impl Deref for SelectorList {
  type Target = Vec<NestedSelector>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
