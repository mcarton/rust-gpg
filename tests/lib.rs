extern crate gnupg;
use gnupg::gpgme;

#[test]
fn init_ctx() {
    gpgme::init();
    gpgme::Context::new();
}
