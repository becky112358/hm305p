use crate::current::*;

#[test]
fn test_get() {
    assert_eq!(5, get([0x01, 0x03, 0x02, 0x00, 0x05, 0x78, 0x47, 0x00]));
}

#[test]
fn test_set() {
    helper_set(
        1234,
        [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        [0x00, 0x00, 0x00, 0x00, 0x04, 0xd2, 0x00, 0x00],
    );
}

fn helper_set(current_ma: u16, input: [u8; MESSAGE_LENGTH], output: [u8; MESSAGE_LENGTH]) {
    let mut message = input;
    set(current_ma, &mut message);
    assert_eq!(output, message);
}
