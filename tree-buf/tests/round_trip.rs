use tree_buf::prelude::*;
use tree_buf::{Readable, Writable};
use std::fmt::Debug;

// Create this namespace to hide the prelude. This is a check that the hygenics do not require any types from tree_buf to be imported
mod hide_namespace {
    use tree_buf_macros::{Read, Write};

    #[derive(Read, Write, PartialEq, Debug, Clone)]
    pub struct Bits {
        pub int: u32,
        pub obj_array: Vec<Bobs>,
        pub extra: Option<Bobs>,
    }

    #[derive(Read, Write, PartialEq, Debug, Clone)]
    pub struct Bobs {
        pub one: Vec<u32>,
    }
}
use hide_namespace::{Bits, Bobs};

fn make_item() -> Bits {
    Bits {
        int: 5,
        extra: Some(Bobs { one: vec![99] }),
        obj_array: vec![
            Bobs { one: vec![3, 2, 1, 0] },
            Bobs { one: vec![] },
            Bobs {
                one: vec![20, 20, 20, 20, 20, 20, 20],
            },
        ],
    }
}

fn round_trip<T: Readable + Writable + Debug + PartialEq>(value: &T) {
    let bytes = write(value);
    let result = read(&bytes);
    match result {
        Ok(parsed) => assert_eq!(value, &parsed),
        _ => assert!(false),
    }
}

#[test]
fn round_trip_item() {
    let item = make_item();
    round_trip(&item);
}


#[test]
fn round_trip_vec() {
    let item = make_item();
    let item = vec![item; 5];
    round_trip(&item);
}

#[test]
fn size_check() {
    // TODO: Compare output size of file and time to encode and decode as compared to a variety of other formats.
    let item = make_item();
    let item = vec![item; 5];
    let bytes = write(&item);
    dbg!(bytes.len());
}