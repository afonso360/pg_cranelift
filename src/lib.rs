use expr_translator::PGJit;

mod expr_translator;
pub mod pg;

#[repr(C)]
struct PgCraneliftContext {
    base: pg::JitContext,

    // Managed by the rust memory allocator.
    jit: Option<Box<PGJit>>,
}

#[no_mangle]
pub extern "C" fn _cranelift_compile_expr(state: *mut pg::ExprState) -> bool {
    // println!("In Rust: cranelift_compile_expr");

    // Make this a reference so that it's easier to work with
    assert!(!state.is_null());
    let state = unsafe { &mut *state };

    // Parent should never be null
    assert!(!state.parent.is_null());
    let parent = unsafe { &mut *state.parent };

    assert!(!parent.state.is_null());
    let estate = unsafe { &mut *parent.state };

    let jit_ctx_ptr: *mut PgCraneliftContext = if !estate.es_jit.is_null() {
        println!("Reusing JIT Context");
        // This means that we have previously created a JIT context, lets reuse it
        estate.es_jit.cast::<PgCraneliftContext>()
    } else {
        println!("Allocating JIT Context");
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

            (*jit_ctx).jit = Some(Box::new(PGJit::default()));

            jit_ctx
        }
    };

    let jit_ctx = unsafe { &mut (*jit_ctx_ptr) };

    let jitmodule = jit_ctx.jit.as_mut().unwrap();

    match jitmodule.build(state) {
        Ok(func_id) => {
            let func_addr = jitmodule.get_func_addr(func_id);

            // Assing the function address to the state
            state.evalfunc = func_addr;

            // Returning 'true' indicates we won't jit the current expression.
            true
        }
        Err(e) => {
            println!("Unable to compile experssion with error: {}", e);
            false
        }
    }

    // {
    //     SlowJitCompiledExprState *cstate =
    //         palloc0(sizeof(SlowJitCompiledExprState));

    //     cstate->jit_ctx = jit_ctx;
    //     cstate->funcname = funcname;

    //     state->evalfunc = slowjit_exec_compiled_expr;
    //     state->evalfunc_private = cstate;
    //   }
}

#[no_mangle]
pub extern "C" fn _cranelift_release_context(ctx: *mut pg::JitContext) {
    println!("Cleaning up JitContext");

    let jit_ctx = unsafe { &mut *(ctx as *mut PgCraneliftContext) };

    // PgJIT is owned by the rust memory allocator, so we have to manually drop it to ensure it gets
    // correctly free'd.
    jit_ctx.jit = None;
}

#[no_mangle]
pub extern "C" fn _cranelift_reset_after_error() {
    // println!("In Rust: cranelift_reset_after_error");
    unimplemented!();
}
