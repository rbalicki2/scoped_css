use std::{
  collections::{
    HashMap,
    HashSet,
  },
  ops::Deref,
};

/* -------- SELECTORS -------- */

// TODO handle +, >, pseudo selectors, etc.
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
      "~=" => Some(AttributeRelation::Containing(s2)),
      "|=" => Some(AttributeRelation::BeginsWithWord(s2)),
      "^=" => Some(AttributeRelation::BeginsWith(s2)),
      "$=" => Some(AttributeRelation::EndsWith(s2)),
      "*=" => Some(AttributeRelation::Contains(s2)),
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

pub type NestedSelector = Vec<Selector>;
pub type NestedSelectorList = Vec<NestedSelector>;

/* -------- PROPERTIES, RULES (putting it all together) -------- */

// TODO this should be a struct, preventing you from setting
// invalid css values, like margin: red
pub type Properties = HashMap<String, String>;

/// This is the most important struct, the unit of functionality
///
/// TODO: A better description
#[derive(Debug, Clone)]
pub struct Rule {
  pub nested_selector_list: NestedSelectorList,
  pub property_block: PropertyBlock,
}

#[derive(Debug, Clone)]
pub enum RuleOrProperty {
  Rule(Rule),
  Property((String, String)),
}

#[derive(Debug, Clone)]
pub struct PropertyBlock {
  pub properties: Properties,
  pub nested_rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct RuleSet(pub Vec<Rule>);

impl Deref for RuleSet {
  type Target = Vec<Rule>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Rule {
  pub fn classes_and_ids(&self) -> (HashSet<String>, HashSet<String>) {
    // TODO avoid allocating and calling .cloned
    let mut classes = HashSet::new();
    let mut ids = HashSet::new();
    for nested_selector in self.nested_selector_list.iter() {
      for selector in nested_selector.iter() {
        for modifier in selector.modifiers.iter() {
          match modifier {
            Modifier::Class(class) => {
              classes.insert(class.into());
            },
            Modifier::Id(id) => {
              ids.insert(id.into());
            },
            _ => {},
          }
        }
      }
    }

    for child_rule in self.property_block.nested_rules.iter() {
      let (new_classes, new_ids) = child_rule.classes_and_ids();

      classes = classes.union(&new_classes).cloned().collect();
      ids = ids.union(&new_ids).cloned().collect();
    }
    (classes, ids)
  }
}

impl RuleSet {
  pub fn classes_and_ids(&self) -> (HashSet<String>, HashSet<String>) {
    let mut classes = HashSet::new();
    let mut ids = HashSet::new();
    for rule in self.0.iter() {
      let (new_classes, new_ids) = rule.classes_and_ids();
      classes = classes.union(&new_classes).cloned().collect();
      ids = ids.union(&new_ids).cloned().collect();
    }
    (classes, ids)
  }

  pub fn to_string(class_map: HashMap<String, String>, id_map: HashMap<String, String>) -> String {
    unimplemented!()
  }
}
