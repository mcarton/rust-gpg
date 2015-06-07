pub fn init() {
    unsafe { ::bindings::gpgme::gpgme_check_version(::std::ptr::null()) };
}

pub struct Context(::bindings::gpgme::gpgme_ctx_t);

impl Context {

    pub fn new() -> Context {
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
