#include "postgres.h"

#include "executor/execExpr.h"
#include "jit/jit.h"

PG_MODULE_MAGIC;

extern void _PG_jit_provider_init(JitProviderCallbacks *cb);

// These are all defined in the rust library
extern bool _cranelift_compile_expr(ExprState *state);
extern void _cranelift_release_context(JitContext *ctx);
extern void _cranelift_reset_after_error(void);

/* Function where we initialize JIT compilation callbacks. */
void _PG_jit_provider_init(JitProviderCallbacks *cb) {
  cb->compile_expr = _cranelift_compile_expr;
  cb->release_context = _cranelift_release_context;
  cb->reset_after_error = _cranelift_reset_after_error;
}
