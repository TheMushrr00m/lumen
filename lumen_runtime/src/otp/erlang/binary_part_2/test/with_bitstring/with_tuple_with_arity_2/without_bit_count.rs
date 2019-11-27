use super::*;

#[test]
fn with_positive_start_and_positive_length_returns_subbinary() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &strategy::term::is_binary::with_byte_len_range(
                    (3..=6).into(),
                    arc_process.clone(),
                )
                .prop_flat_map(|binary| {
                    let byte_len = total_byte_len(binary);

                    // `start` must be 2 less than `byte_len` so that `length` can be at least 1
                    (Just(binary), (1..=(byte_len - 2)))
                })
                .prop_flat_map(|(binary, start)| {
                    (
                        Just(binary),
                        Just(start),
                        1..=(total_byte_len(binary) - start),
                    )
                })
                .prop_map(|(binary, start, length)| {
                    let mut heap = arc_process.acquire_heap();

                    (
                        binary,
                        heap.integer(start).unwrap(),
                        heap.integer(length).unwrap(),
                    )
                }),
                |(binary, start, length)| {
                    let start_length = arc_process.tuple_from_slice(&[start, length]).unwrap();

                    let result = native(&arc_process, binary, start_length);

                    prop_assert!(result.is_ok());

                    let returned = result.unwrap();

                    prop_assert!(returned.is_boxed_subbinary());

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_size_start_and_negative_size_length_returns_binary() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &strategy::term::is_binary::with_byte_len_range(1..=4, arc_process.clone())
                    .prop_map(|binary| {
                        let byte_len = total_byte_len(binary);

                        let mut heap = arc_process.acquire_heap();

                        (
                            binary,
                            heap.integer(byte_len).unwrap(),
                            heap.integer(-(byte_len as isize)).unwrap(),
                        )
                    }),
                |(binary, start, length)| {
                    let start_length = arc_process.tuple_from_slice(&[start, length]).unwrap();

                    prop_assert_eq!(native(&arc_process, binary, start_length), Ok(binary));

                    let returned_binary = native(&arc_process, binary, start_length).unwrap();

                    prop_assert_eq!(
                        returned_binary.is_boxed_subbinary(),
                        binary.is_boxed_subbinary()
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_zero_start_and_size_length_returns_binary() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &strategy::term::is_binary(arc_process.clone()).prop_map(|binary| {
                    let mut heap = arc_process.acquire_heap();

                    (
                        binary,
                        heap.integer(0).unwrap(),
                        heap.integer(total_byte_len(binary)).unwrap(),
                    )
                }),
                |(binary, start, length)| {
                    let start_length = arc_process.tuple_from_slice(&[start, length]).unwrap();

                    prop_assert_eq!(native(&arc_process, binary, start_length), Ok(binary));

                    let returned_binary = native(&arc_process, binary, start_length).unwrap();

                    prop_assert_eq!(
                        returned_binary.is_boxed_subbinary(),
                        binary.is_boxed_subbinary()
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}
