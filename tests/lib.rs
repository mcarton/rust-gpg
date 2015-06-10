extern crate gnupg;
use gnupg::gpgme;
use gnupg::keys;

#[test]
fn test_key_list() {
    gpgme::init();

    let mut it = keys::KeyIterator::new();
    while it.next().is_some() {}
}
