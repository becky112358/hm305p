use crate::common::*;

#[test]
fn test_u8_high_low_get_u16() {
    assert_eq!(0x198a, u8_high_low_get_u16(0x19, 0x8a));
}

#[test]
fn test_u8_high_get_u16() {
    assert_eq!(0x0b00, u8_high_get_u16(0x0b));
}

#[test]
fn test_u8_low_get_u16() {
    assert_eq!(0x00c8, u8_low_get_u16(0xc8));
}

#[test]
fn test_u16_get_u8_high() {
    assert_eq!(0x00, u16_get_u8_high(0x0000));
    assert_eq!(0x44, u16_get_u8_high(0x4444));
    assert_eq!(0x73, u16_get_u8_high(0x73a9));
    assert_eq!(0xa7, u16_get_u8_high(0xa76c));
}

#[test]
fn test_u16_get_u8_low() {
    assert_eq!(0x00, u16_get_u8_low(0x0000));
    assert_eq!(0x9b, u16_get_u8_low(0x9c9b));
    assert_eq!(0x3f, u16_get_u8_low(0xc13f));
}

#[test]
fn state_to_bool() {
    assert_eq!(true, State::On.as_bool());
    assert_eq!(false, State::Off.as_bool());
}
