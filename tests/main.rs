use simple_rs_sha2::{sha2_224, sha2_256, sha2_384, sha2_512};

#[test]
fn hello() {
    assert_eq!(
        sha2_224("hello"),
        "ea09ae9cc6768c50fcee903ed054556e5bfc8347907f12598aa24193"
    );
    assert_eq!(
        sha2_256("hello"),
        "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
    );
    assert_eq!(sha2_384("hello"), "59e1748777448c69de6b800d7a33bbfb9ff1b463e44354c3553bcdb9c666fa90125a3c79f90397bdf5f6a13de828684f");
    assert_eq!(sha2_512("hello"), "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043");
}
