/// Type used to ensure an `InitToken` cannot be constructed directly by user.
struct InitTokenImpl;

/// This type is used to ensure `gpgme` was uninitialized before any function is called.
pub struct InitToken(InitTokenImpl);

/// Initialize the library.
pub fn init() -> InitToken {
    static ONCE: ::std::sync::Once = ::std::sync::ONCE_INIT;

    ONCE.call_once(|| {
        unsafe {
            ::bindings::gpgme::gpgme_check_version(::std::ptr::null());
        }
    });

    InitToken(InitTokenImpl)
}

pub struct Context(::bindings::gpgme::gpgme_ctx_t);

impl Context {

    pub fn new(_: InitToken) -> Context {
        Context({
            let mut ctx = unsafe { ::std::mem::uninitialized() };
            let err = unsafe { ::bindings::gpgme::gpgme_new(&mut ctx) };

            assert_eq!(err, ::bindings::gpg_error::GPG_ERR_NO_ERROR);

            ctx
        })
    }

    pub fn raw(&self) -> ::bindings::gpgme::gpgme_ctx_t {
        self.0
    }

}

impl Drop for Context {

    fn drop(&mut self) {
        let &mut Context(ctx) = self;
        unsafe { ::bindings::gpgme::gpgme_release(ctx) };
    }

}
