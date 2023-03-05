use std::sync::Once;

pub static INIT: Once = Once::new();

macro_rules! assert_lexical_analysis {
    ($code:expr, $snapshot_name:expr) => {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        // When
        let tokens = assert_ok!(crate::lexical_analysis::LexicalAnalysis::from($code));

        // Then
        insta::assert_debug_snapshot!(format!("test_{}_lexical_analysis", $snapshot_name), tokens);
    };
}

macro_rules! assert_syntax_analysis {
    ($code:expr, $snapshot_name:expr) => {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        // When
        let abstract_syntax_tree = assert_ok!(crate::syntax_analysis::SyntaxAnalysis::from(
            crate::lexical_analysis::LexicalAnalysis::from($code).unwrap(),
        ));

        // Then
        insta::assert_debug_snapshot!(
            format!("test_{}_syntax_analysis", $snapshot_name),
            abstract_syntax_tree
        );
    };
}

macro_rules! assert_evaluation {
    ($code:expr, $snapshot_name:expr) => {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        // When
        let mut evaluator = crate::evaluator::Evaluator::new();
        let evaluation = assert_ok!(evaluator.evaluate(
            crate::syntax_analysis::SyntaxAnalysis::from(
                crate::lexical_analysis::LexicalAnalysis::from($code).unwrap(),
            )
            .unwrap(),
        ));

        // Then
        insta::assert_debug_snapshot!(format!("test_{}_evaluation", $snapshot_name), evaluation);
    };
}

macro_rules! assert_environment {
    ($code:expr, $snapshot_name:expr) => {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        // When
        let mut evaluator = crate::evaluator::Evaluator::new();
        let _evaluation = assert_ok!(evaluator.evaluate(
            crate::syntax_analysis::SyntaxAnalysis::from(
                crate::lexical_analysis::LexicalAnalysis::from($code).unwrap(),
            )
            .unwrap(),
        ));

        // Then
        insta::assert_debug_snapshot!(format!("test_{}_environment", $snapshot_name), evaluator);
    };
}

macro_rules! assert_successive_environment {
    ($evaluator:expr, $code:expr, $snapshot_name:expr) => {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });

        // When
        let evaluation = assert_ok!($evaluator.evaluate(
            crate::syntax_analysis::SyntaxAnalysis::from(
                crate::lexical_analysis::LexicalAnalysis::from($code).unwrap(),
            )
            .unwrap(),
        ));

        // Then
        insta::assert_debug_snapshot!(format!("test_{}_evaluation", $snapshot_name), evaluation);
        insta::assert_debug_snapshot!(format!("test_{}_environment", $snapshot_name), $evaluator);
    };
}

macro_rules! assert_ok {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => {
                panic!("Got the unexpected error {error:?}.");
            }
        }
    };
}
