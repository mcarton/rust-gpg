extern crate gnupg;
use gnupg::gpgme;
use gnupg::keys;

#[test]
fn test_init() {
    gpgme::init();
}

#[test]
fn test_init_twice() {
    gpgme::init();
    gpgme::init();
}

#[test]
fn test_key_list() {
    let init = gpgme::init();

    let mut it = keys::KeyIterator::new(init);
    while it.next().is_some() {}
}
