# enum_from_derive

[![Version info](https://img.shields.io/crates/v/enum_from_derive.svg)](https://crates.io/crates/enum_from_derive)
[![Downloads](https://img.shields.io/crates/d/enum_from_derive.svg?style=flat-square)](https://crates.io/crates/enum_from_derive)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/enum_from_derive)
[![dependency status](https://deps.rs/crate/enum_from_derive/0.1.1/status.svg)](https://deps.rs/crate/enum_from_derive)


# example:
 
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
        }


    