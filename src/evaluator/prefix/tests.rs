use insta::assert_json_snapshot;
use rstest::rstest;

use super::*;

#[rstest(
    abstract_syntax_tree,
    snapshot_name,
    case(
        AbstractSyntaxTree {
            abstract_syntax_tree: vec![
                SyntaxTreeNode::EXPRESSION {
                    expression: Expression::PREFIX {
                        prefix_token: Token::NOT,
                        right_hand_expression: Box::new(SyntaxTreeNode::EXPRESSION {
                            expression:Expression::BOOLEAN {
                                boolean_token: Token::TRUE
                            }
                        })
                    }
                }
            ],
            syntax_parsing_errors: vec![],
        },
        "test_evaluator_not_prefix_nodes_case1"
    ),
    case(
        AbstractSyntaxTree {
            abstract_syntax_tree: vec![
                SyntaxTreeNode::EXPRESSION {
                    expression: Expression::PREFIX {
                        prefix_token: Token::NOT,
                        right_hand_expression: Box::new(SyntaxTreeNode::EXPRESSION {
                            expression:Expression::BOOLEAN {
                                boolean_token: Token::FALSE
                            }
                        })
                    }
                }
            ],
            syntax_parsing_errors: vec![],
        },
        "test_evaluator_not_prefix_nodes_case2"
    ),
    case(
        AbstractSyntaxTree {
            abstract_syntax_tree: vec![
                SyntaxTreeNode::EXPRESSION {
                    expression: Expression::PREFIX {
                        prefix_token: Token::NOT,
                        right_hand_expression: Box::new(SyntaxTreeNode::EXPRESSION {
                            expression: Expression::PREFIX {
                                prefix_token: Token::NOT,
                                right_hand_expression: Box::new(SyntaxTreeNode::EXPRESSION {
                                    expression:Expression::BOOLEAN {
                                        boolean_token: Token::FALSE
                                    }
                                })
                            }
                        })
                    }
                }
            ],
            syntax_parsing_errors: vec![],
        },
        "test_evaluator_not_prefix_nodes_case3"
    ),
)]
fn test_evaluator_not_prefix_nodes(abstract_syntax_tree: AbstractSyntaxTree, snapshot_name: &str) {
    assert_expected_returned_object!(abstract_syntax_tree, snapshot_name);
}

#[rstest(
    abstract_syntax_tree,
    snapshot_name,
    case(
        AbstractSyntaxTree {
            abstract_syntax_tree: vec![
                SyntaxTreeNode::EXPRESSION {
                    expression: Expression::PREFIX {
                        prefix_token: Token::MINUS,
                        right_hand_expression: Box::new(SyntaxTreeNode::EXPRESSION {
                            expression: Expression::INTEGER {
                                integer_token: Token::INTEGER {
                                    literal: "5".to_string()
                                }
                            }
                        })
                    }
                }
            ],
            syntax_parsing_errors: vec![],
        },
        "test_evaluator_minus_prefix_nodes_case1"
    ),
    case(
        AbstractSyntaxTree {
            abstract_syntax_tree: vec![
                SyntaxTreeNode::EXPRESSION {
                    expression: Expression::PREFIX {
                        prefix_token: Token::MINUS,
                        right_hand_expression: Box::new(SyntaxTreeNode::EXPRESSION {
                            expression: Expression::INTEGER {
                                integer_token: Token::INTEGER {
                                    literal: "10".to_string()
                                }
                            }
                        })
                    }
                }
            ],
            syntax_parsing_errors: vec![],
        },
        "test_evaluator_minus_prefix_nodes_case2"
    )
)]
fn test_evaluator_minus_prefix_nodes(
    abstract_syntax_tree: AbstractSyntaxTree,
    snapshot_name: &str,
) {
    assert_expected_returned_object!(abstract_syntax_tree, snapshot_name);
}
