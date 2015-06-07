extern crate gnupg;
use gnupg::gpgme;
use gnupg::keys;

fn main() {
    gpgme::init();

    let it = keys::KeyIterator::new();
    println!("Number of keys in keyring: {}.", it.count());
}
