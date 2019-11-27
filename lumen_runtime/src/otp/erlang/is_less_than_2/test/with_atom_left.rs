use super::*;

use proptest::strategy::Strategy;

#[test]
fn with_number_returns_false() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::atom(),
                    strategy::term::is_number(arc_process.clone()),
                ),
                |(left, right)| {
                    prop_assert_eq!(native(left, right), false.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_lesser_atom_returns_false() {
    is_less_than(|_, _| Atom::str_to_term("keft"), false);
}

#[test]
fn with_same_atom_value_returns_false() {
    is_less_than(|_, _| Atom::str_to_term("left"), false);
}

#[test]
fn with_greater_atom_returns_true() {
    is_less_than(|_, _| Atom::str_to_term("meft"), true);
}

#[test]
fn without_number_or_atom_returns_true() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::atom(),
                    strategy::term(arc_process.clone())
                        .prop_filter("Right cannot be a number or atom", |right| {
                            !(right.is_atom() || right.is_number())
                        }),
                ),
                |(left, right)| {
                    prop_assert_eq!(native(left, right), true.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}

fn is_less_than<R>(right: R, expected: bool)
where
    R: FnOnce(Term, &Process) -> Term,
{
    super::is_less_than(|_| Atom::str_to_term("left"), right, expected);
}
