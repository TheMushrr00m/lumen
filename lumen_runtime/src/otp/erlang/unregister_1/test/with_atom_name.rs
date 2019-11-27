use super::*;

mod with_registered_name;

#[test]
fn without_registered_name_errors_badarg() {
    let name = registered_name();

    assert_eq!(native(name), Err(badarg!().into()));
}
