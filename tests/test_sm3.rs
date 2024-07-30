use gmsm::sm3::sm3_hex;

#[test]
fn test_sm3() {
    assert_eq!(sm3_hex("abc"), "66C7F0F462EEEDD9D1F2D46BDC10E4E24167C4875CF2F7A2297DA02B8F4BA8E0");
}