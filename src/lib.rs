pub mod pg;

#[repr(C)]
struct PgCraneliftContext {
    base: pg::JitContext,
}

#[no_mangle]
pub extern "C" fn _cranelift_compile_expr(state: *mut pg::ExprState) -> bool {
    println!("In Rust: cranelift_compile_expr");

    // Make this a reference so that it's easier to work with
    assert!(!state.is_null());
    let state = unsafe { &mut *state };

    // Parent should never be null
    assert!(!state.parent.is_null());
    let parent = unsafe { &mut *state.parent };

    assert!(!parent.state.is_null());
    let estate = unsafe { &mut *parent.state };

    let jit_ctx_ptr: *mut PgCraneliftContext = if !estate.es_jit.is_null() {
        // This means that we have previously created a JIT context, lets reuse it
        estate.es_jit.cast::<PgCraneliftContext>()
    } else {
        unsafe {
            pg::ResourceOwnerEnlargeJIT(pg::CurrentResourceOwner);

            let alloc = pg::MemoryContextAllocZero(
                pg::TopMemoryContext,
                std::mem::size_of::<PgCraneliftContext>(),
            );

            let jit_ctx = alloc.cast::<PgCraneliftContext>();
            (*jit_ctx).base.flags = estate.es_jit_flags;

            /* ensure cleanup */
            (*jit_ctx).base.resowner = pg::CurrentResourceOwner;
            pg::ResourceOwnerRememberJIT(pg::CurrentResourceOwner, jit_ctx as pg::Datum);

            /* For re-using the JIT context. */
            estate.es_jit = &mut (*jit_ctx).base;

            jit_ctx
        }
    };

    let _jit_ctx = unsafe { &mut (*jit_ctx_ptr) };

    dbg!(jit_ctx_ptr);

    /* Returning 'false' indicates we won't jit the current expression. */
    false
}

#[no_mangle]
pub extern "C" fn _cranelift_release_context(_ctx: *mut pg::JitContext) {
    println!("In Rust: cranelift_release_context");
}

#[no_mangle]
pub extern "C" fn _cranelift_reset_after_error() {
    println!("In Rust: cranelift_reset_after_error");
}
