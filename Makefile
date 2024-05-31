MODULE_big = cranelift
EXTENSION = cranelift

OBJS = cranelift.o

# Disable LLVM bitcodes generation.
override with_llvm = no

SHLIB_LINK = -l:libpg_cranelift.a
PG_LDFLAGS += -L$(shell pwd)/target/debug

PG_CONFIG := pg_config
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)
