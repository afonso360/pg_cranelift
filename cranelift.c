/* A bunch of header files. */
#include "postgres.h"

#include "c.h"
#include "executor/execExpr.h"
#include "fmgr.h"
#include "jit/jit.h"
#include "lib/stringinfo.h"
#include "miscadmin.h"
#include "nodes/execnodes.h"
#include "nodes/pg_list.h"
#include "pg_config_manual.h"
#include "utils/elog.h"
#include "utils/memutils.h"
#include "utils/palloc.h"
#include "utils/resowner.h"
#include "utils/resowner_private.h"

#include <dlfcn.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

PG_MODULE_MAGIC;

extern void _PG_jit_provider_init(JitProviderCallbacks *cb);

extern int cranelift_add(int a, int b);

/* Function prototypes for JIT compilation. */
static bool cranelift_compile_expr(ExprState *state) {
  /*
   * Emit a notice message so that we can check if the JIT provider being
   * loaded successfully.
   */
  elog(NOTICE, "cranelift_compile_expr");

  cranelift_add(1, 2);

  /* Returning 'false' indicates we won't jit the current expression. */
  return false;
}
static void cranelift_release_context(JitContext *ctx) {
  elog(NOTICE, "cranelift_release_context");
}
static void cranelift_reset_after_error(void) {
  elog(NOTICE, "cranelift_reset_after_error");
}

/* Function where we initialize JIT compilation callbacks. */
void _PG_jit_provider_init(JitProviderCallbacks *cb) {
  cb->compile_expr = cranelift_compile_expr;
  cb->release_context = cranelift_release_context;
  cb->reset_after_error = cranelift_reset_after_error;
}
