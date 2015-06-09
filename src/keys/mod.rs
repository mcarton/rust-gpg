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
            Some(Key{raw: key})
        }
    }

}

pub struct Key {
    raw: ::bindings::gpgme::gpgme_key_t,
}

impl Key {

    pub fn subkeys(&self) -> SubKeyIterator {
        SubKeyIterator{current: SubKey{raw: unsafe { self.raw.as_ref() }.unwrap().subkeys}}
    }

}

impl Drop for Key {

    fn drop(&mut self) {
        unsafe { ::bindings::gpgme::gpgme_key_release(self.raw) };
    }

}

pub struct SubKeyIterator {
    current: SubKey,
}

impl Iterator for SubKeyIterator {
    type Item = SubKey;

    fn next(&mut self) -> Option<SubKey> {
        let raw = self.current.raw;

        if raw.is_null() {
            None
        }
        else {
            self.current = SubKey{raw: unsafe { raw.as_ref() }.unwrap().next};
            Some(SubKey{raw: raw})
        }
    }

}

#[derive(Clone)]
pub struct SubKey {
    raw: ::bindings::gpgme::gpgme_subkey_t,
}

impl SubKey {

    pub fn keyid(&self) -> String {
        let keyid = unsafe { ::std::ffi::CStr::from_ptr(self.raw.as_ref().unwrap().keyid) };
        ::std::str::from_utf8(keyid.to_bytes()).unwrap().to_owned()
    }

}
