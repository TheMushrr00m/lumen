use super::*;

use proptest::prop_oneof;
use proptest::strategy::Strategy;

#[test]
fn with_small_integer_divisor_returns_small_integer() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(strategy::term::integer::small::isize(), divisor()),
                |(dividend, divisor)| {
                    prop_assert_eq!(
                        native(
                            &arc_process,
                            arc_process.integer(dividend).unwrap(),
                            arc_process.integer(divisor).unwrap(),
                        ),
                        Ok(arc_process.integer(dividend % divisor).unwrap())
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_big_integer_divisor_returns_dividend() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::integer::small(arc_process.clone()),
                    strategy::term::integer::big(arc_process.clone()),
                ),
                |(dividend, divisor)| {
                    prop_assert_eq!(native(&arc_process, dividend, divisor), Ok(dividend));

                    Ok(())
                },
            )
            .unwrap();
    });
}

fn divisor() -> BoxedStrategy<isize> {
    prop_oneof![
        (SmallInteger::MIN_VALUE..=-1),
        (1..=SmallInteger::MAX_VALUE)
    ]
    .boxed()
}
