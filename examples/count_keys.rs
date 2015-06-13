extern crate gnupg;
use gnupg::gpgme;
use gnupg::keys;

fn main() {
    let init = gpgme::init();

    let it = keys::KeyIterator::new(init);
    println!("Number of keys in keyring: {}.", it.count());
}
