#![cfg(feature = "serde-codec")]

extern crate alloc;

use alloc::collections::BTreeMap;
use core::convert::TryFrom;

use serde::Deserialize;
use serde_bytes::ByteBuf;

use libipld_core::cid::Cid;
use libipld_core::ipld::Ipld;

/// This function is to test that all IPLD kinds except the given one errors, when trying to
/// deserialize to the given Rust type.
fn error_except<'de, T>(_input: T, except: &Ipld)
where
    T: Deserialize<'de> + core::fmt::Debug,
{
    if !matches!(except, Ipld::Null) {
        assert!(T::deserialize(Ipld::Null).is_err());
    }
    if !matches!(except, Ipld::Bool(_)) {
        assert!(T::deserialize(Ipld::Bool(true)).is_err());
    }
    if !matches!(except, Ipld::Integer(_)) {
        assert!(T::deserialize(Ipld::Integer(22)).is_err());
    }
    if !matches!(except, Ipld::Float(_)) {
        assert!(T::deserialize(Ipld::Float(5.3)).is_err());
    }
    if !matches!(except, Ipld::String(_)) {
        assert!(T::deserialize(Ipld::String("hello".into())).is_err());
    }
    if !matches!(except, Ipld::Bytes(_)) {
        assert!(T::deserialize(Ipld::Bytes(vec![0x68, 0x65, 0x6c, 0x6c, 0x6f])).is_err());
    }
    if !matches!(except, Ipld::List(_)) {
        assert!(T::deserialize(Ipld::List(vec![Ipld::Integer(22), Ipld::Bool(false)])).is_err());
    }
    if !matches!(except, Ipld::Map(_)) {
        assert!(T::deserialize(Ipld::Map(BTreeMap::from([
            ("hello".into(), Ipld::Null),
            ("world!".into(), Ipld::Float(7.4))
        ])))
        .is_err());
    }
    if !matches!(except, Ipld::Link(_)) {
        assert!(T::deserialize(Ipld::Link(
            Cid::try_from("bafkreie74tgmnxqwojhtumgh5dzfj46gi4mynlfr7dmm7duwzyvnpw7h7m").unwrap()
        ))
        .is_err());
    }
}

#[test]
#[allow(clippy::unit_cmp)]
fn ipld_deserializer_unit() {
    let unit = ();
    let ipld = Ipld::Null;
    error_except(unit, &ipld);

    let deserialized = <()>::deserialize(ipld).unwrap();
    assert_eq!(deserialized, unit);
}

#[test]
fn ipld_deserializer_unit_struct() {
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    struct UnitStruct;

    let unit_struct = UnitStruct;
    let ipld = Ipld::Null;
    error_except(unit_struct.clone(), &ipld);

    let deserialized = UnitStruct::deserialize(ipld).unwrap();
    assert_eq!(deserialized, unit_struct);
}

#[test]
fn ipld_deserializer_bool() {
    let bool = false;
    let ipld = Ipld::Bool(bool);
    error_except(bool, &ipld);

    let deserialized = bool::deserialize(ipld).unwrap();
    assert_eq!(deserialized, bool);
}

#[test]
fn ipld_deserializer_u8() {
    let integer = 34u8;
    let ipld = Ipld::Integer(integer.into());
    error_except(integer, &ipld);

    let deserialized = u8::deserialize(ipld).unwrap();
    assert_eq!(
        deserialized, integer,
        "Correctly deserialize Ipld::Integer to u8."
    );

    let too_large = u8::deserialize(Ipld::Integer((u8::MAX as i128) + 10));
    assert!(too_large.is_err(), "Number must be within range.");
    let too_small = u8::deserialize(Ipld::Integer((u8::MIN as i128) - 10));
    assert!(too_small.is_err(), "Number must be within range.");
}

#[test]
fn ipld_deserializer_u16() {
    let integer = 345u16;
    let ipld = Ipld::Integer(integer.into());
    error_except(integer, &ipld);

    let deserialized = u16::deserialize(ipld).unwrap();
    assert_eq!(
        deserialized, integer,
        "Correctly deserialize Ipld::Integer to u16."
    );

    let too_large = u16::deserialize(Ipld::Integer((u16::MAX as i128) + 10));
    assert!(too_large.is_err(), "Number must be within range.");
    let too_small = u16::deserialize(Ipld::Integer((u16::MIN as i128) - 10));
    assert!(too_small.is_err(), "Number must be within range.");
}

#[test]
fn ipld_deserializer_u32() {
    let integer = 345678u32;
    let ipld = Ipld::Integer(integer.into());
    error_except(integer, &ipld);

    let deserialized = u32::deserialize(ipld).unwrap();
    assert_eq!(
        deserialized, integer,
        "Correctly deserialize Ipld::Integer to u32."
    );

    let too_large = u32::deserialize(Ipld::Integer((u32::MAX as i128) + 10));
    assert!(too_large.is_err(), "Number must be within range.");
    let too_small = u32::deserialize(Ipld::Integer((u32::MIN as i128) - 10));
    assert!(too_small.is_err(), "Number must be within range.");
}

#[test]
fn ipld_deserializer_u64() {
    let integer = 34567890123u64;
    let ipld = Ipld::Integer(integer.into());
    error_except(integer, &ipld);

    let deserialized = u64::deserialize(ipld).unwrap();
    assert_eq!(
        deserialized, integer,
        "Correctly deserialize Ipld::Integer to u64."
    );

    let too_large = u64::deserialize(Ipld::Integer((u64::MAX as i128) + 10));
    assert!(too_large.is_err(), "Number must be within range.");
    let too_small = u64::deserialize(Ipld::Integer((u64::MIN as i128) - 10));
    assert!(too_small.is_err(), "Number must be within range.");
}

#[test]
fn ipld_deserializer_i8() {
    let integer = -23i8;
    let ipld = Ipld::Integer(integer.into());
    error_except(integer, &ipld);

    let deserialized = i8::deserialize(ipld).unwrap();
    assert_eq!(
        deserialized, integer,
        "Correctly deserialize Ipld::Integer to i8."
    );

    let too_large = i8::deserialize(Ipld::Integer((i8::MAX as i128) + 10));
    assert!(too_large.is_err(), "Number must be within range.");
    let too_small = i8::deserialize(Ipld::Integer((i8::MIN as i128) - 10));
    assert!(too_small.is_err(), "Number must be within range.");
}

#[test]
fn ipld_deserializer_i16() {
    let integer = 2345i16;
    let ipld = Ipld::Integer(integer.into());
    error_except(integer, &ipld);

    let deserialized = i16::deserialize(ipld).unwrap();
    assert_eq!(
        deserialized, integer,
        "Correctly deserialize Ipld::Integer to i16."
    );

    let too_large = i16::deserialize(Ipld::Integer((i16::MAX as i128) + 10));
    assert!(too_large.is_err(), "Number must be within range.");
    let too_small = i16::deserialize(Ipld::Integer((i16::MIN as i128) - 10));
    assert!(too_small.is_err(), "Number must be within range.");
}

#[test]
fn ipld_deserializer_i32() {
    let integer = 234567i32;
    let ipld = Ipld::Integer(integer.into());
    error_except(integer, &ipld);

    let deserialized = i32::deserialize(ipld).unwrap();
    assert_eq!(
        deserialized, integer,
        "Correctly deserialize Ipld::Integer to i32."
    );

    let too_large = i32::deserialize(Ipld::Integer((i32::MAX as i128) + 10));
    assert!(too_large.is_err(), "Number must be within range.");
    let too_small = i32::deserialize(Ipld::Integer((i32::MIN as i128) - 10));
    assert!(too_small.is_err(), "Number must be within range.");
}

#[test]
fn ipld_deserializer_i64() {
    let integer = 2345678901i64;
    let ipld = Ipld::Integer(integer.into());
    error_except(integer, &ipld);

    let deserialized = i64::deserialize(ipld).unwrap();
    assert_eq!(
        deserialized, integer,
        "Correctly deserialize Ipld::Integer to i64."
    );

    let too_large = i64::deserialize(Ipld::Integer((i64::MAX as i128) + 10));
    assert!(too_large.is_err(), "Number must be within range.");
    let too_small = i64::deserialize(Ipld::Integer((i64::MIN as i128) - 10));
    assert!(too_small.is_err(), "Number must be within range.");
}

#[test]
fn ipld_deserializer_f32() {
    let float = 7.3f32;
    let ipld = Ipld::Float(float.into());
    error_except(float, &ipld);

    let deserialized = f32::deserialize(ipld).unwrap();
    assert_eq!(deserialized, float);
}

#[test]
fn ipld_deserializer_f64() {
    let float = 427.8f64;
    let ipld = Ipld::Float(float);
    error_except(float, &ipld);

    let deserialized = f64::deserialize(ipld).unwrap();
    assert_eq!(deserialized, float);
}

#[test]
fn ipld_deserializer_char() {
    let char = 'x';
    let ipld = Ipld::String(char.to_string());
    error_except(char, &ipld);

    let deserialized = char::deserialize(ipld).unwrap();
    assert_eq!(deserialized, char);
}

#[test]
fn ipld_deserializer_str() {
    let str: &str = "hello";
    let ipld = Ipld::String(str.to_string());
    error_except(str, &ipld);

    // TODO vmx 2022-02-09: Doesn't work yet. If we would have a zero-copy version, it should
    //let deserialized = <&str>::deserialize(ipld).unwrap();
    //assert_eq!(deserialized, string);
}

#[test]
fn ipld_deserializer_string() {
    let string = "hello".to_string();
    let ipld = Ipld::String(string.clone());
    error_except(string.clone(), &ipld);

    let deserialized = String::deserialize(ipld).unwrap();
    assert_eq!(deserialized, string);
}

#[test]
fn ipld_deserializer_bytes() {
    let bytes = vec![0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let ipld = Ipld::Bytes(bytes.clone());
    error_except(&bytes[..], &ipld);

    // TODO vmx 2022-02-09: Doesn't work yet. If we would have a zero-copy version, it should
    //let deserialized = <&[u8]>::deserialize(ipld).unwrap();
    //assert_eq!(deserialized, bytes);
}

#[test]
fn ipld_deserializer_byte_buf() {
    let bytes = vec![0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let ipld = Ipld::Bytes(bytes.clone());
    error_except(ByteBuf::from(bytes.clone()), &ipld);

    let deserialized = ByteBuf::deserialize(ipld).unwrap();
    assert_eq!(deserialized, bytes);
}

#[test]
fn ipld_deserializer_list() {
    let list = vec![0x68, 0x65, 0x6c, 0x6c, 0x6f];
    let ipld = Ipld::List(vec![
        Ipld::Integer(0x68),
        Ipld::Integer(0x65),
        Ipld::Integer(0x6c),
        Ipld::Integer(0x6c),
        Ipld::Integer(0x6f),
    ]);
    error_except(list.clone(), &ipld);

    let deserialized = Vec::<u8>::deserialize(ipld).unwrap();
    assert_eq!(deserialized, list);
}

#[test]
fn ipld_deserializer_tuple() {
    let tuple = (true, "hello".to_string());
    let ipld = Ipld::List(vec![Ipld::Bool(tuple.0), Ipld::String(tuple.1.clone())]);
    error_except(tuple.clone(), &ipld);

    let deserialized = <(bool, String)>::deserialize(ipld).unwrap();
    assert_eq!(deserialized, tuple);
}

#[test]
fn ipld_deserializer_tuple_struct() {
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    struct TupleStruct(u8, ());

    let tuple_struct = TupleStruct(82, ());
    let ipld = Ipld::List(vec![Ipld::Integer(82), Ipld::Null]);
    error_except(tuple_struct.clone(), &ipld);

    let deserialized = TupleStruct::deserialize(ipld).unwrap();
    assert_eq!(deserialized, tuple_struct);
}

#[test]
fn ipld_deserializer_map() {
    let map = BTreeMap::from([("hello".to_string(), true), ("world!".to_string(), false)]);
    let ipld = Ipld::Map(BTreeMap::from([
        ("hello".to_string(), Ipld::Bool(true)),
        ("world!".to_string(), Ipld::Bool(false)),
    ]));
    error_except(map.clone(), &ipld);

    let deserialized = BTreeMap::deserialize(ipld).unwrap();
    assert_eq!(deserialized, map);
}

/// A CID is deserialized through a newtype struct.
#[test]
fn ipld_deserializer_cid() {
    let cid = Cid::try_from("bafkreie74tgmnxqwojhtumgh5dzfj46gi4mynlfr7dmm7duwzyvnpw7h7m").unwrap();
    let ipld = Ipld::Link(cid);
    error_except(cid, &ipld);

    let deserialized = Cid::deserialize(ipld).unwrap();
    assert_eq!(deserialized, cid);
}

#[test]
fn ipld_deserializer_newtype_struct() {
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    struct Wrapped(());

    let newtype_struct = Wrapped(());
    let ipld = Ipld::Null;
    error_except(newtype_struct.clone(), &ipld);

    let deserialized = Wrapped::deserialize(ipld).unwrap();
    assert_eq!(deserialized, newtype_struct);
}

/// An additional test, just to make sure that wrapped CIDs also work.
#[test]
fn ipld_deserializer_newtype_struct_cid() {
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    struct Wrapped(Cid);

    let cid = Cid::try_from("bafkreie74tgmnxqwojhtumgh5dzfj46gi4mynlfr7dmm7duwzyvnpw7h7m").unwrap();
    let newtype_struct = Wrapped(cid);
    let ipld = Ipld::Link(cid);
    error_except(newtype_struct.clone(), &ipld);

    let deserialized = Wrapped::deserialize(ipld).unwrap();
    assert_eq!(deserialized, newtype_struct);
}

#[test]
fn ipld_deserializer_option() {
    let option_some: Option<u8> = Some(58u8);
    let option_none: Option<u8> = None;
    let ipld_some = Ipld::Integer(option_some.unwrap().into());
    let ipld_none = Ipld::Null;

    // This is similar to `error_except`, which cannot be used here, as we need to excluse
    // `Ipld::Integer` *and* `Ipld::Null`.
    assert!(<Option<u8>>::deserialize(Ipld::Bool(true)).is_err());
    assert!(<Option<u8>>::deserialize(Ipld::Float(5.3)).is_err());
    assert!(<Option<u8>>::deserialize(Ipld::String("hello".into())).is_err());
    assert!(<Option<u8>>::deserialize(Ipld::Bytes(vec![0x01, 0x97])).is_err());
    assert!(
        <Option<u8>>::deserialize(Ipld::List(vec![Ipld::Integer(22), Ipld::Bool(false)])).is_err()
    );
    assert!(<Option<u8>>::deserialize(Ipld::Map(BTreeMap::from([
        ("hello".into(), Ipld::Null),
        ("world!".into(), Ipld::Float(7.4))
    ])))
    .is_err());
    assert!(<Option<u8>>::deserialize(Ipld::Link(
        Cid::try_from("bafkreie74tgmnxqwojhtumgh5dzfj46gi4mynlfr7dmm7duwzyvnpw7h7m").unwrap()
    ))
    .is_err());

    let deserialized_some = <Option<u8>>::deserialize(ipld_some).unwrap();
    assert_eq!(deserialized_some, option_some);
    let deserialized_none = <Option<u8>>::deserialize(ipld_none).unwrap();
    assert_eq!(deserialized_none, option_none);
}

#[test]
fn ipld_deserializer_enum() {
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    enum MyEnum {
        One,
        Two(u8),
        Three { value: bool },
    }

    let enum_one = MyEnum::One;
    let ipld_one = Ipld::String("One".into());
    error_except(enum_one.clone(), &ipld_one);
    let deserialized_one = MyEnum::deserialize(ipld_one).unwrap();
    assert_eq!(deserialized_one, enum_one);

    let enum_two = MyEnum::Two(4);
    let ipld_two = Ipld::Map(BTreeMap::from([("Two".into(), Ipld::Integer(4))]));
    error_except(enum_two.clone(), &ipld_two);
    let deserialized_two = MyEnum::deserialize(ipld_two).unwrap();
    assert_eq!(deserialized_two, enum_two);

    let enum_three = MyEnum::Three { value: true };
    let ipld_three = Ipld::Map(BTreeMap::from([(
        "Three".into(),
        Ipld::Map(BTreeMap::from([("value".into(), Ipld::Bool(true))])),
    )]));
    error_except(enum_three.clone(), &ipld_three);
    let deserialized_three = MyEnum::deserialize(ipld_three).unwrap();
    assert_eq!(deserialized_three, enum_three);
}

#[test]
fn ipld_deserializer_struct() {
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    struct MyStruct {
        hello: u8,
        world: bool,
    }

    let my_struct = MyStruct {
        hello: 91,
        world: false,
    };
    let ipld = Ipld::Map(BTreeMap::from([
        ("hello".into(), Ipld::Integer(my_struct.hello.into())),
        ("world".into(), Ipld::Bool(my_struct.world)),
    ]));
    error_except(my_struct.clone(), &ipld);

    let deserialized = MyStruct::deserialize(ipld).unwrap();
    assert_eq!(deserialized, my_struct);
}