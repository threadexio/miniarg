//! Tests for the derive macro.
#![cfg(feature = "derive")]

use core::fmt;

use miniarg::{Key, ArgumentIterator, ParseError};

#[derive(Debug, Key, PartialEq)]
enum SimpleKeys {
    Key,
    Key1,
    Key2,
}

#[test]
/// Just calling a binary should produce an empty result.
fn basic() {
    let cmdline = "executable";
    assert_eq!(
        SimpleKeys::parse(&cmdline).collect::<Result<Vec<_>, _>>().unwrap(),
        Vec::new()
    );
}

#[test]
/// One key, one value.
fn key_value() {
    let cmdline = "executable -key value";
    assert_eq!(
        SimpleKeys::parse(&cmdline).collect::<Result<Vec<_>, _>>().unwrap(),
        vec![(&SimpleKeys::Key, "value")]
    );
}

#[test]
/// two keys, two values.
fn two_key_value() {
    let cmdline = "executable -key1 value1 -key2 value2";
    assert_eq!(
        SimpleKeys::parse(&cmdline).collect::<Result<Vec<_>, _>>().unwrap(),
        vec![(&SimpleKeys::Key1, "value1"), (&SimpleKeys::Key2, "value2")]
    );
}

#[test]
/// one key, two values.
fn key_two_value() {
    let cmdline = "executable -key value1 -key value2";
    assert_eq!(
        SimpleKeys::parse(&cmdline).collect::<Result<Vec<_>, _>>().unwrap(),
        vec![(&SimpleKeys::Key, "value1"), (&SimpleKeys::Key, "value2")]
    );
}

#[test]
/// Just a key should produce an empty vec.
fn value_missing() {
    let cmdline = "executable -key";
    assert_eq!(
        SimpleKeys::parse(&cmdline).collect::<Result<Vec<_>, _>>().unwrap(),
        vec![]
    );
}

#[test]
/// An invalid key should produce an error.
fn invalid_key() {
    let cmdline = "executable -invalid";
    assert_eq!(
        SimpleKeys::parse(&cmdline).collect::<Result<Vec<_>, _>>().unwrap_err(),
        ParseError::UnknownKey("invalid")
    );
}

#[test]
/// An option without a key should produce an error.
fn missing_key() {
    let cmdline = "executable value";
    assert_eq!(
        SimpleKeys::parse(&cmdline).collect::<Result<Vec<_>, _>>().unwrap_err(),
        ParseError::NotAKey("value")
    );
}
