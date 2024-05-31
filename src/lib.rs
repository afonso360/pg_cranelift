use core::ffi::c_int;

pub mod pg;

#[no_mangle]
pub extern "C" fn cranelift_add(left: c_int, right: c_int) -> c_int {
    println!("In Rust: left = {}, right = {}", left, right);
    left + right
}

#[no_mangle]
pub extern "C" fn _cranelift_compile_expr(_state: pg::ExprState) -> bool {
    // pg::elog
    println!("In Rust: cranelift_compile_expr");
    cranelift_add(1, 2);
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

// /* Function prototypes for JIT compilation. */
// static bool cranelift_compile_expr(ExprState *state) {
//     /*
//      * Emit a notice message so that we can check if the JIT provider being
//      * loaded successfully.
//      */
//     elog(NOTICE, "cranelift_compile_expr");

//     cranelift_add(1, 2);

//     /* Returning 'false' indicates we won't jit the current expression. */
//     return false;
//   }
//   static void cranelift_release_context(JitContext *ctx) {
//     elog(NOTICE, "cranelift_release_context");
//   }
//   static void cranelift_reset_after_error(void) {
//     elog(NOTICE, "cranelift_reset_after_error");
//   }
