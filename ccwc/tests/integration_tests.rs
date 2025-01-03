use ccwc::CcwcCount;
use std::fs::File;
use std::io::Read;

#[test]
fn test_count_all() {
    let mut input = File::open("tests/test.txt").unwrap();

    let counts = CcwcCount::from(&mut input as &mut dyn Read);

    assert_eq!(342_190, counts.bytes);
    assert_eq!(7_145, counts.lines);
    assert_eq!(58_164, counts.words);
    assert_eq!(339_292, counts.characters);
}
