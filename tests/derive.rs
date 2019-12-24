#![cfg(feature = "derive")]

use arbitrary::*;

fn finite_buffer(input: &[u8]) -> FiniteBuffer {
    assert!(!input.is_empty());
    FiniteBuffer::new(input, input.len()).expect("can create FiniteBuffer OK")
}

fn arbitrary_from<T: Arbitrary>(input: &[u8]) -> T {
    let mut buf = finite_buffer(input);
    T::arbitrary(&mut buf).expect("can create arbitrary instance OK")
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Arbitrary)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[test]
fn struct_with_named_fields() {
    let rgb: Rgb = arbitrary_from(&[4, 5, 6]);
    assert_eq!(rgb.r, 4);
    assert_eq!(rgb.g, 5);
    assert_eq!(rgb.b, 6);

    assert_eq!(
        rgb.shrink().collect::<Vec<_>>(),
        vec![
            Rgb { r: 0, g: 0, b: 0 },
            Rgb { r: 2, g: 2, b: 3 },
            Rgb { r: 1, g: 1, b: 1 }
        ]
    );
}

#[derive(Copy, Clone, Debug, Arbitrary)]
struct MyTupleStruct(u8, bool);

#[test]
fn tuple_struct() {
    let s: MyTupleStruct = arbitrary_from(&[43, 42]);
    assert_eq!(s.0, 43);
    assert_eq!(s.1, false);

    let s: MyTupleStruct = arbitrary_from(&[42, 43]);
    assert_eq!(s.0, 42);
    assert_eq!(s.1, true);

    for ((a, b), s) in 42.shrink().zip(true.shrink()).zip(s.shrink()) {
        assert_eq!(a, s.0);
        assert_eq!(b, s.1);
    }
}

#[derive(Copy, Clone, Debug, Arbitrary)]
enum MyEnum {
    Unit,
    Tuple(u8, u16),
    Struct { a: u32, b: (bool, u64) },
}

#[test]
fn derive_enum() {
    let mut raw = vec![
        // The choice of which enum variant takes 4 bytes.
        1, 2, 3, 4,
        // And then we need up to 13 bytes for creating `MyEnum::Struct`, the
        // largest variant.
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
    ];

    let mut saw_unit = false;
    let mut saw_tuple = false;
    let mut saw_struct = false;

    for i in 0..=255 {
        // Choose different variants each iteration.
        for el in &mut raw[..4] {
            *el = i;
        }

        let e: MyEnum = arbitrary_from(&raw);

        match e {
            MyEnum::Unit => {
                saw_unit = true;
                assert_eq!(e.shrink().count(), 0);
            }
            MyEnum::Tuple(a, b) => {
                saw_tuple = true;
                assert_eq!(a, arbitrary_from(&raw[4..5]));
                assert_eq!(b, arbitrary_from(&raw[5..]));

                for ((a, b), e) in a.shrink().zip(b.shrink()).zip(e.shrink()) {
                    match e {
                        MyEnum::Tuple(c, d) => {
                            assert_eq!(a, c);
                            assert_eq!(b, d);
                        }
                        _ => panic!("should never shrink to a different variant"),
                    }
                }
            }
            MyEnum::Struct { a, b } => {
                saw_struct = true;
                assert_eq!(a, arbitrary_from(&raw[4..8]));
                assert_eq!(b, arbitrary_from(&raw[8..]));
                for ((a, b), e) in a.shrink().zip(b.shrink()).zip(e.shrink()) {
                    match e {
                        MyEnum::Struct { a: c, b: d } => {
                            assert_eq!(a, c);
                            assert_eq!(b, d);
                        }
                        _ => panic!("should never shrink to a different variant"),
                    }
                }
            }
        }
    }

    assert!(saw_unit);
    assert!(saw_tuple);
    assert!(saw_struct);
}