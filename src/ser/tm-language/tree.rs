// use serde::{self, ser::Serialize};

use std::fmt;
use serde::{ser::SerializeMap, Serialize};


#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Position(pub String);


#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct RegExp(pub String);


#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct ScopeName(pub String);


#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Capture {
    pub name: ScopeName,
}

#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Captures(pub VecMap<Position, Capture>);



#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockPattern {
    pub begin: RegExp,
    pub end: RegExp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_name: Option<ScopeName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_captures: Option<Captures>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_captures: Option<Captures>,
}

#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchPattern {
    #[serde(rename = "match")]
    pub match_exp: RegExp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub captures: Option<Captures>,
}



#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalPattern {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub scope_name: Option<ScopeName>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_pattern: Option<MatchPattern>,

    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_pattern: Option<BlockPattern>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patterns: Option<Patterns>,
}



#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct IncludeItself(pub String);

#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct IncludeRef(pub String);


#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum Include {
    Lang(ScopeName),
    Itself(IncludeItself),
    Ref(IncludeRef),
}

#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct ExternalPattern {
    pub include: Include,
}


#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum Pattern {
    Internal(InternalPattern),
    External(ExternalPattern),
}


#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Label(pub String);


#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct CInfo (
    #[serde(serialize_with = "serialize_vec")]
    pub Vec<String>
);


#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
pub struct Patterns(
    #[serde(serialize_with = "serialize_vec")]
    pub Vec<Pattern>,
);


#[derive(Debug)]
// #[derive(serde::Deserialize)]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RootNode {
    #[serde(rename = "information_for_contributors")]
    pub info_for_contributions: Option<CInfo>,

    #[serde(serialize_with = "serialize_vec")]
    pub file_type: Vec<String>,
    pub version: String,
    pub name: String,
    pub scope_name: ScopeName,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_line_match: Option<RegExp>,
    pub patterns: Vec<Pattern>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<VecMap<Label, Patterns>>,
}



#[derive(Debug)]
// #[derive(serde::Deserialize)]
// #[derive(serde::Serialize)]
pub struct VecMap<K, V>(pub Vec<(K, V)>);

impl<K, V> serde::Serialize for VecMap<K, V>
where 
    K: serde::Serialize + fmt::Debug,
    V: serde::Serialize + fmt::Debug,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let vec = &self.0;
        let len = self.0.len();
        let mut map = serializer.serialize_map(Some(len))?;
        for i in 0..len { 
            map.serialize_entry(&vec[i].0, &vec[i].1)?;
        }
        map.end()
    }
}



pub fn serialize_vec<S, T>(pattrens: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: serde::Serialize,
    S: serde::Serializer,
{
    if pattrens.len() == 1 {
        pattrens[0].serialize(serializer)
    }
    else {
        pattrens.serialize(serializer)
    }
}
