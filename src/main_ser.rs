// mod ser;
use std::path::Path;

use super::*;
use ser::*;

use ser::methods::*;


#[allow(unused)]
pub fn main() {
    // let root: RootNode = RootNode {
    //     info_for_contributions: Some(CInfo(vec![str!("One-info"), str!("Two-info")])),
    //     file_type: vec![str!(".rs")],
    //     version: str!("1.0.0"),
    //     name: str!("Rust"),
    //     scope_name: ScopeName(str!("source.rust")),
    //     first_line_match: None,
    //     patterns: vec![
    //         Pattern::Internal(InternalPattern {
    //             comment:Some(str!("This is comment")),
    //             scope_name: Some(ScopeName(str!("loop.for.rust"))),
    //             match_pattern: Some(MatchPattern {
    //                 match_exp: RegExp(str!("Regex Expression")),
    //                 captures: Some(Captures(VecMap(vec![
    //                     (Position(str!("1")), Capture { name: ScopeName(str!("other.for.rust"))}),
    //                     (Position(str!("2")), Capture { name: ScopeName(str!("other.for.rust"))}),
    //                     (Position(str!("3")), Capture { name: ScopeName(str!("other.for.rust"))}),
    //                 ]))),
    //             }),
    //             block_pattern: Some(BlockPattern {
    //                 begin: RegExp(str!("Regex Expression")),
    //                 end: RegExp(str!("Regex Expression")),
    //                 content_name: Some(ScopeName(str!("scope.name.rust"))),
    //                 begin_captures: Some(Captures(VecMap(vec![
    //                     (Position(str!("1")), Capture { name: ScopeName(str!("other.for.rust"))}),
    //                 ]))),
    //                 end_captures: Some(Captures(VecMap(vec![
    //                     (Position(str!("2")), Capture { name: ScopeName(str!("other.for.rust"))}),
    //                     (Position(str!("3")), Capture { name: ScopeName(str!("other.for.rust"))}),
    //                 ]))),
    //             }),
    //             patterns: Some(Patterns(vec![
    //                 Pattern::External(ExternalPattern {
    //                     include: Include::Ref(IncludeRef(str!("#commants"))),
    //                 })
    //             ])),
    //         }),
    //         Pattern::Internal(InternalPattern {
    //             comment:Some(str!("This is comment")),
    //             scope_name: Some(ScopeName(str!("loop.for.rust"))),
    //             match_pattern: Some(MatchPattern {
    //                 match_exp: RegExp(str!("Regex Expression")),
    //                 captures: None,
    //             }),
    //             block_pattern: Some(BlockPattern {
    //                 begin: RegExp(str!("Regex Expression")),
    //                 end: RegExp(str!("Regex Expression")),
    //                 content_name: Some(ScopeName(str!("scope.name.rust"))),
    //                 begin_captures: None,
    //                 end_captures: None,
    //             }),
    //             patterns: Some(Patterns(vec![
    //                 Pattern::External(ExternalPattern {
    //                     include: Include::Lang(ScopeName(str!("souce.php"))),
    //                 })
    //             ])),
    //         }),
    //         Pattern::Internal(InternalPattern {
    //             comment:None,
    //             scope_name: None,
    //             match_pattern: None,
    //             block_pattern: Some(BlockPattern {
    //                 begin: RegExp(str!("Regex Expression")),
    //                 end: RegExp(str!("Regex Expression")),
    //                 content_name: None,
    //                 begin_captures: None,
    //                 end_captures: None,
    //             }),
    //             patterns: Some(Patterns(vec![
    //                 Pattern::External(ExternalPattern {
    //                     include: Include::Itself(IncludeItself(str!("$self"))),
    //                 })
    //             ])),
    //         }),
    //         Pattern::Internal(InternalPattern {
    //             comment:None,
    //             scope_name: None,
    //             match_pattern: Some(MatchPattern {
    //                 match_exp: RegExp(str!("Regex Expression")),
    //                 captures: None,
    //             }),
    //             block_pattern: None,
    //             patterns: None,
    //         }),
    //     ],
    //     repository: Some(VecMap(vec![
    //         (
    //             Label(str!("comments1")),
    //             Patterns(vec![
    //                 Pattern::Internal(InternalPattern {
    //                     comment:None,
    //                     scope_name: None,
    //                     match_pattern: Some(MatchPattern {
    //                         match_exp: RegExp(str!("Regex Expression")),
    //                         captures: None,
    //                     }),
    //                     block_pattern: None,
    //                     patterns: None,
    //                 }),
    //             ])
    //         ),
    //         (
    //             Label(str!("comments2")),
    //             Patterns(vec![
    //                 Pattern::Internal(InternalPattern {
    //                     comment:None,
    //                     scope_name: None,
    //                     match_pattern: Some(MatchPattern {
    //                         match_exp: RegExp(str!("Regex Expression")),
    //                         captures: None,
    //                     }),
    //                     block_pattern: None,
    //                     patterns: None,
    //                 }),
    //                 Pattern::Internal(InternalPattern {
    //                     comment:None,
    //                     scope_name: None,
    //                     match_pattern: Some(MatchPattern {
    //                         match_exp: RegExp(str!("Regex Expression")),
    //                         captures: None,
    //                     }),
    //                     block_pattern: None,
    //                     patterns: None,
    //                 }),
    //             ])
    //         ),
    //     ])),
    // };

    // to_file_pretty(Path::new("output.json"), &root).unwrap();
    println!("serialization done!")
}
