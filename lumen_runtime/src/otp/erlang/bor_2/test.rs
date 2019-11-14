mod with_big_integer_left;
mod with_small_integer_left;

use proptest::test_runner::{Config, TestRunner};
use proptest::{prop_assert, prop_assert_eq};

use liblumen_alloc::badarith;
use liblumen_alloc::erts::term::prelude::Encoded;

use crate::otp::erlang::bor_2::native;
use crate::scheduler::{with_process, with_process_arc};
use crate::test::{count_ones, strategy};

#[test]
fn without_integer_left_errors_badarith() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::is_not_integer(arc_process.clone()),
                    strategy::term::is_integer(arc_process.clone()),
                ),
                |(left, right)| {
                    prop_assert_eq!(native(&arc_process, left, right), Err(badarith!().into()));

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_integer_left_without_integer_right_errors_badarith() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::is_integer(arc_process.clone()),
                    strategy::term::is_not_integer(arc_process.clone()),
                ),
                |(left, right)| {
                    prop_assert_eq!(native(&arc_process, left, right), Err(badarith!().into()));

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_same_integer_returns_same_integer() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &strategy::term::is_integer(arc_process.clone()),
                |operand| {
                    prop_assert_eq!(native(&arc_process, operand, operand), Ok(operand));

                    Ok(())
                },
            )
            .unwrap();
    });
}
