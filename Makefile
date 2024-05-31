MODULE_big = cranelift
EXTENSION = cranelift

OBJS = cranelift.o

# Disable LLVM bitcodes generation.
override with_llvm = no

PG_CONFIG := pg_config
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)
