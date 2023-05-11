#[test]
fn env() {
    use env::cotime_out;
    assert_eq!(cotime_out!(), 23);
}