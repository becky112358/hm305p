use crate::message::*;

#[test]
fn test_verify_read() {
    assert!(verify_read([1, 3, 2, 0, 227, 249, 205, 0]).is_ok());
    assert!(verify_read([1, 3, 2, 0, 227, 249, 204, 0]).is_err());
}
