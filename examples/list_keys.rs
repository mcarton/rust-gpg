extern crate gnupg;
use gnupg::gpgme;
use gnupg::keys;

fn main() {
    gpgme::init();

    let mut i = 0;
    for key in keys::KeyIterator::new() {
        i += 1;
        println!("{}", i);
        for subkey in key.subkeys() {
            println!("{}", subkey.keyid());
        }
    }
}
