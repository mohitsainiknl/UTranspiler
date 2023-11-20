// eslint-disable

Repetition(
    Repetition {
        op: RepetitionOp::OneOrMore
        },
        greedy: true,
        ast: Group(
            Group::),
                ast: Alternation(
                    Alternation {
                        asts: [
                            Literal(
                                Literal::a'
                            }),
                            Literal(
                                Literal::b'
                            }),
                            Literal(
                                Literal::c'
                            })
                        ]
                    }
                )
            }
        )
    }
)


(test,\\s)+(?:test)

Hir::Concat([
            Hir::Repetition(
                    Repetition::OneOrMore,
                        greedy: true,
                        hir: Hir::Group(
                                Group::CaptureIndex(
                                        1,
                                    ),
                                    hir: Hir::Concat([
                                                Hir::Literal(
                                                        Unicode(
                                                            't',
                                                        ),
                                                    ),
                                                },
                                                Hir::Literal(
                                                        Unicode(
                                                            'e',
                                                        ),
                                                    ),
                                                },
                                                Hir::Literal(
                                                        Unicode(
                                                            's',
                                                        ),
                                                    ),
                                                },
                                                Hir::Literal(
                                                        Unicode(
                                                            't',
                                                        ),
                                                    ),
                                                },
                                                Hir::Literal(
                                                        Unicode(
                                                            ',',
                                                        ),
                                                    ),
                                                },
                                                Hir::Class(
                                                        Unicode(
                                                            ClassUnicode {
                                                                set: IntervalSet {[
                                                                        ClassUnicodeRange {
                                                                            start: "0x9",
                                                                            end: "0xD",
                                                                        },
                                                                        ClassUnicodeRange {
                                                                            start: "0x20",
                                                                            end: "0x20",
                                                                        },
                                                                        ClassUnicodeRange {
                                                                            start: "0x85",
                                                                            end: "0x85",
                                                                        },
                                                                        ClassUnicodeRange {
                                                                            start: "0xA0",
                                                                            end: "0xA0",
                                                                        },
                                                                        ClassUnicodeRange {
                                                                            start: "0x1680",
                                                                            end: "0x1680",
                                                                        },
                                                                        ClassUnicodeRange {
                                                                            start: "0x2000",
                                                                            end: "0x200A",
                                                                        },
                                                                        ClassUnicodeRange {
                                                                            start: "0x2028",
                                                                            end: "0x2029",
                                                                        },
                                                                        ClassUnicodeRange {
                                                                            start: "0x202F",
                                                                            end: "0x202F",
                                                                        },
                                                                        ClassUnicodeRange {
                                                                            start: "0x205F",
                                                                            end: "0x205F",
                                                                        },
                                                                        ClassUnicodeRange {
                                                                            start: "0x3000",
                                                                            end: "0x3000",
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ],
                                        ),
                                    },
                                },
                            ),
                        },
                    },
                ),
            },
            Hir::Group(
                    Group::NonCapturing,
                        hir: Hir::Concat([
                                    Hir::Literal(
                                            Unicode(
                                                't',
                                            ),
                                        ),
                                    },
                                    Hir::Literal(
                                            Unicode(
                                                'e',
                                            ),
                                        ),
                                    },
                                    Hir::Literal(
                                            Unicode(
                                                's',
                                            ),
                                        ),
                                    },
                                    Hir::Literal(
                                            Unicode(
                                                't',
                                            ),
                                        ),
                                    },
                                ],
                            ),
                        },
                    },
                ),
            },
        ],
    ),
}