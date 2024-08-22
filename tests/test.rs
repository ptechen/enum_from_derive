use enum_from_derive::From;

#[repr(i32)]
#[derive(From, Debug)]
pub enum Test {
    #[default]
    Name,
    Test = 100,
    TTT,
    FFF = 1000
}

#[test]
fn test() {
    let t = Test::from("test1");
    println!("{:?}", t);
    let t = Test::from(100);
    println!("{:?}", t);
    let t = Test::from("");
    println!("{:?}", t);
}
