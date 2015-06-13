extern crate gnupg;
use gnupg::gpgme;
use gnupg::keys;

fn main() {
    let init = gpgme::init();

    for (i, key) in keys::KeyIterator::new(init).enumerate() {
        println!("key {}:", i);
        for subkey in key.subkeys() {
            println!("\t{}", subkey.keyid());
        }
        for uid in key.uids() {
            println!("\t({})\t{}", uid.validity(), uid.uid());
        }
    }
}
