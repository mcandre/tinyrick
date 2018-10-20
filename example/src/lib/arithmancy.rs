pub fn add_two(n: i64) -> i64 {
  return n + 2;
}

#[test]
fn smoketest() {
  assert_eq!(add_two(2), 4);
}
