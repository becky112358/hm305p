use crate::current::*;

#[test]
fn test_set() {
    helper_set(1234, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], [0x00, 0x00, 0x00, 0x00, 0x04, 0xd2, 0x00, 0x00]);
}

fn helper_set(current_ma: u16, input: [u8; MESSAGE_LENGTH], output: [u8; MESSAGE_LENGTH]) {
    let mut message = input;
    set(current_ma, &mut message);
    assert_eq!(output, message);
}
