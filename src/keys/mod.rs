extern crate core;
use self::core::marker::PhantomData;

pub struct KeyIterator {
    ctx : ::gpgme::Context,
}

/// An iterator over the keys in your keyring.
impl KeyIterator {

    /// Construct a `KeyIterator` over your whole keyring.
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

/// Represents a GPG key.
///
/// You can get one from `KeyIterator`.
pub struct Key {
    raw: ::bindings::gpgme::gpgme_key_t,
}

impl Key {

    pub fn subkeys<'a>(&'a self) -> SubKeyIterator<'a> {
        SubKeyIterator{
            current: SubKey{
                raw: unsafe { self.raw.as_ref() }.unwrap().subkeys,
                lifetime: PhantomData,
            }
        }
    }

    pub fn uids<'a>(&'a self) -> UserIdIterator<'a> {
        UserIdIterator{
            current: UserId{
                raw: unsafe { self.raw.as_ref() }.unwrap().uids,
                lifetime: PhantomData,
            }
        }
    }

}

impl Drop for Key {

    fn drop(&mut self) {
        unsafe { ::bindings::gpgme::gpgme_key_release(self.raw) };
    }

}

pub struct SubKeyIterator<'a> {
    current: SubKey<'a>,
}

macro_rules! gpgme_linked_list_next {
    ($Item: ident, $ItemA: ty) => {
        fn next(&mut self) -> Option<$ItemA> {
            let raw = self.current.raw;

            if raw.is_null() {
                None
            }
            else {
                self.current = $Item{
                    raw: unsafe { raw.as_ref() }.unwrap().next,
                    lifetime: PhantomData,
                };
                Some($Item{raw: raw, lifetime: PhantomData})
            }
        }
    }
}

impl<'a> Iterator for SubKeyIterator<'a> {
    type Item = SubKey<'a>;

    gpgme_linked_list_next!(SubKey, SubKey<'a>);

}

#[derive(Clone)]
pub struct SubKey<'a> {
    raw: ::bindings::gpgme::gpgme_subkey_t,
    lifetime: PhantomData<&'a Key>,
}

impl<'a> SubKey<'a> {

    pub fn keyid(&self) -> String {
        let keyid = unsafe { ::std::ffi::CStr::from_ptr(self.raw.as_ref().unwrap().keyid) };
        ::std::str::from_utf8(keyid.to_bytes()).unwrap().to_owned()
    }

}

pub struct UserIdIterator<'a> {
    current: UserId<'a>,
}

impl<'a> Iterator for UserIdIterator<'a> {
    type Item = UserId<'a>;

    gpgme_linked_list_next!(UserId, UserId<'a>);
}

pub struct UserId<'a> {
    raw: ::bindings::gpgme::gpgme_user_id_t,
    lifetime: PhantomData<&'a Key>,
}

/// Specify the validity of a user ID.
#[derive(Debug)]
pub enum Validity {
    Unknown,
    Undefined,
    Never,
    Marginal,
    Full,
    Ultimate,
}

impl ::std::fmt::Display for Validity {

    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        ::std::fmt::Debug::fmt(self, f)
    }

}

impl From<Validity> for char {

    /// Convert a `Validity` to a char as `gpg` does.
    fn from(v: Validity) -> char {
        match v {
            Validity::Unknown   => '?',
            Validity::Undefined => 'q',
            Validity::Never     => 'n',
            Validity::Marginal  => 'm',
            Validity::Full      => 'f',
            Validity::Ultimate  => 'u',
        }
    }

}

impl<'a> UserId<'a> {

    pub fn uid(&self) -> String {
        let uid = unsafe { ::std::ffi::CStr::from_ptr(self.raw.as_ref().unwrap().uid) };
        ::std::str::from_utf8(uid.to_bytes()).unwrap().to_owned()
    }

    pub fn validity(&self) -> Validity {
        let validity = unsafe { self.raw.as_ref().unwrap().validity };
        match validity {
            ::bindings::gpgme::GPGME_VALIDITY_UNKNOWN   => Validity::Unknown,
            ::bindings::gpgme::GPGME_VALIDITY_UNDEFINED => Validity::Undefined,
            ::bindings::gpgme::GPGME_VALIDITY_NEVER     => Validity::Never,
            ::bindings::gpgme::GPGME_VALIDITY_MARGINAL  => Validity::Marginal,
            ::bindings::gpgme::GPGME_VALIDITY_FULL      => Validity::Full,
            ::bindings::gpgme::GPGME_VALIDITY_ULTIMATE  => Validity::Ultimate,
            _ => unreachable!(),
        }
    }

}
