use super::*;

#[test]
fn with_integer_right_returns_bitwise_and() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::integer::big(arc_process.clone()),
                    strategy::term::is_integer(arc_process.clone()),
                ),
                |(left, right)| {
                    let result = native(&arc_process, left, right);

                    prop_assert!(result.is_ok());

                    let band = result.unwrap();

                    prop_assert!(band.is_integer());
                    prop_assert!(count_ones(band) <= count_ones(left));
                    prop_assert!(count_ones(band) <= count_ones(right));

                    Ok(())
                },
            )
            .unwrap();
    });
}
