File {
    shebang: None,
    attrs: [],
    items: [
        Struct(
            ItemStruct {
                attrs: [],
                vis: Public(
                    VisPublic {
                        pub_token: Pub 
                    }
                ),
                struct_token: Struct,
                ident: Ident(Local),
                generics: Generics {
                    lt_token: None,
                    params: [],
                    gt_token: None,
                    where_clause: None 
                },
                fields: Named(
                    FieldsNamed {
                        brace_token: Brace,
                        named: [
                            Field {
                                attrs: [],
                                vis: Public(
                                    VisPublic {
                                        pub_token: Pub 
                                    }
                                ),
                                ident: Some(Ident(attrs)),
                                colon_token: Some(Colon),
                                ty: Path(
                                    TypePath {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident(String),
                                                    arguments: None 
                                                }
                                            ] 
                                        } 
                                    }
                                )   
                            },
                            Comma,
                            Field {
                                attrs: [],
                                vis: Public(
                                    VisPublic {
                                        pub_token: Pub 
                                    }
                                ),
                                ident: Some(Ident(pat)),
                                colon_token: Some(Colon),
                                ty: Path(
                                    TypePath {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident(String),
                                                    arguments: None 
                                                }
                                            ] 
                                        } 
                                    }
                                ) 
                            },
                            Comma
                        ] 
                    }
                ),
                semi_token: None 
            }
        )
    ] 
}