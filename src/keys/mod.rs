pub struct KeyIterator {
    ctx : ::gpgme::Context,
}

impl KeyIterator {

    pub fn new() -> KeyIterator {
        let ctx = ::gpgme::Context::new();

        let err = unsafe {
            ::bindings::gpgme::gpgme_op_keylist_start(
                ctx.raw(), ::std::ptr::null(), 0
            )
        };

        assert_eq!(err, ::bindings::gpg_error::GPG_ERR_NO_ERROR);

        KeyIterator{ctx: ctx}
    }

}

impl Drop for KeyIterator {

    fn drop(&mut self) {
        unsafe {
            ::bindings::gpgme::gpgme_op_keylist_end(self.ctx.raw());
        }
    }

}

impl Iterator for KeyIterator {
    type Item = Key;

    fn next(&mut self) -> Option<Key> {
        let mut key : ::bindings::gpgme::gpgme_key_t;
        let err = unsafe {
            key = ::std::mem::uninitialized();
            ::bindings::gpgme::gpgme_op_keylist_next(
                self.ctx.raw(), &mut key
            )
        };

        if err != ::bindings::gpg_error::GPG_ERR_NO_ERROR {
            None
        }
        else {
            Some(Key(key))
        }
    }

}

pub struct Key(::bindings::gpgme::gpgme_key_t);

impl Drop for Key {

    fn drop(&mut self) {
        let &mut Key(key) = self;
        unsafe { ::bindings::gpgme::gpgme_key_release(key) };
    }

}
