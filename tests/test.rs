use enum_from_derive::From;

#[repr(i32)]
#[derive(From, Debug, PartialEq)]
pub enum Test {
    #[default]
    Name,
    #[from_str(test)]
    #[True]
    #[rate]
    Test = 100,
    #[False]
    TTT,
    #[True]
    FFF = 1000,
}

#[test]
fn test() {
    // let t = Test::from("test1");
    // assert_eq!(t, Test::Name);
    // let b = Test::from(1000);
    // assert_eq!(b, Test::FFF);
    // let c = Test::from("test");
    // assert_eq!(c, Test::Test);
    let a = Test::from("test");
    // assert_eq!(a, Test::Name);
    assert_eq!(true, a.to_bool());
    assert_eq!(true, a.is_rate());
}
