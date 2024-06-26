use std::collections::HashMap;
use std::fmt;
use std::mem::offset_of;

use cranelift_codegen::ir::types::I8;
use cranelift_codegen::ir::{AbiParam, AliasRegion, Signature};
use cranelift_codegen::ir::{InstBuilder, Value};
use cranelift_codegen::ir::{MemFlags, Type};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, FuncId, Linkage, Module, ModuleError};

use crate::pg::{self, ExprState};

pub type PGJitResult<T> = Result<T, PGJitError>;

#[derive(Debug)]
pub enum PGJitError {
    /// A Opcode is not yet implemented in the JIT.
    UnimplementedOpcode(pg::ExprEvalOp),
    /// An error that occurs when registering a function in a module
    ModuleError(ModuleError),
}

impl fmt::Display for PGJitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PGJitError::UnimplementedOpcode(op) => {
                write!(f, "Unimplemented opcode: {}", op.name())
            }
            PGJitError::ModuleError(e) => write!(f, "Module error: {}", e),
        }
    }
}

impl From<ModuleError> for PGJitError {
    fn from(e: ModuleError) -> Self {
        PGJitError::ModuleError(e)
    }
}

pub struct PGJit {
    /// The function builder context, which is reused across multiple
    /// FunctionBuilder instances.
    builder_context: FunctionBuilderContext,

    /// The main Cranelift context, which holds the state for codegen.
    pub ctx: cranelift_codegen::Context,

    /// The data description, which is to data objects what `ctx` is to functions.
    _data_description: DataDescription,

    /// The module, with the jit backend, which manages the JIT'd
    /// functions.
    module: JITModule,

    /// All registered function names.
    func_names: HashMap<FuncId, String>,
}

impl Default for PGJit {
    fn default() -> Self {
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("opt_level", "speed").unwrap();
        // flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
            panic!("host machine is not supported: {}", msg);
        });
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .unwrap();
        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());

        let module = JITModule::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            _data_description: DataDescription::new(),
            module,
            func_names: HashMap::new(),
        }
    }
}

impl Drop for PGJit {
    fn drop(&mut self) {
        // The JIT Module contains the backing memory for all of the JITted functions.
        // It does not clear them on Drop. So we have to do it ourselves by first
        // replacing it with an empty module, and then freeing the previous ones.
        let empty_module =
            JITModule::new(JITBuilder::new(cranelift_module::default_libcall_names()).unwrap());
        let allocd_module = std::mem::replace(&mut self.module, empty_module);
        unsafe { allocd_module.free_memory() };
    }
}

impl PGJit {
    fn reset_builder(&mut self) {
        self.module.clear_context(&mut self.ctx);
        self.builder_context = FunctionBuilderContext::new();
    }

    pub fn eval_signature(&self) -> Signature {
        // Emulate the following signature:
        //
        // Datum ExecInterpExpr(ExprState *state, ExprContext *econtext, bool *isnull)
        //
        // The function returns a Datum, which is also a pointer.

        let cc = self.module.isa().default_call_conv();
        let pointer_ty = self.module.target_config().pointer_type();

        let mut sig = Signature::new(cc);
        sig.params.push(AbiParam::new(pointer_ty));
        sig.params.push(AbiParam::new(pointer_ty));
        sig.params.push(AbiParam::new(pointer_ty));
        sig.returns.push(AbiParam::new(pointer_ty));
        sig
    }

    pub fn build(&mut self, state: &mut ExprState) -> PGJitResult<FuncId> {
        // TODO: When we fail a compilation we should reset the context. We currently
        // don't deal with that very well, so just reset it before we start any compilation.
        self.reset_builder();

        self.ctx.func.signature = self.eval_signature();

        // Create the builder to build a function.
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);

        // Create the entry block, to start emitting code in.
        let entry_block = builder.create_block();

        // Since this is the entry block, add block parameters corresponding to
        // the function's parameters.
        builder.append_block_params_for_function_params(entry_block);

        // Tell the builder to emit code in this block.
        builder.switch_to_block(entry_block);

        // And, tell the builder that this block will have no further
        // predecessors. Since it's the entry block, it won't have any
        // predecessors.
        builder.seal_block(entry_block);

        // Debug print the opcodes for this expression, before compiling
        println!("Expression Steps:\n{}", state.format_steps());

        // Translate the steps of the expression.
        let types = ISATypes {
            bool: I8,
            pointer: self.module.target_config().pointer_type(),
        };
        let args = FuncArgs {
            state: builder.block_params(entry_block)[0],
            _econtext: builder.block_params(entry_block)[1],
            isnull: builder.block_params(entry_block)[2],
        };
        let tmp = StepResult {
            resnull: builder
                .ins()
                .iadd_imm(args.state, offset_of!(ExprState, resnull) as i64),
            resvalue: builder
                .ins()
                .iadd_imm(args.state, offset_of!(ExprState, resvalue) as i64),
        };
        let mut expr_translator = ExprTranslator {
            types,
            builder,
            results: vec![None; state.steps_len as usize],
            tmp,
            args,
        };
        for opno in 0..(state.steps_len as usize) {
            unsafe {
                let op = &mut *state.steps.offset(opno.try_into().unwrap());
                let opcode = pg::ExecEvalStepOp(state, op);
                expr_translator.translate_step(opno, opcode, op)?;
            }
        }

        // Tell the builder we're done with this function.
        expr_translator.builder.finalize();

        println!("Built Func: {}", self.ctx.func.display());

        self.register_function()
    }

    fn register_function(&mut self) -> PGJitResult<FuncId> {
        // To avoid duplicate names, we append the pid and function number to the
        // name.
        let func_number = self.func_names.len();
        let pid = unsafe { pg::MyProcPid };
        let name = format!("pgcranelift_eval_expr_{pid}_{func_number}");

        // Next, declare the function to jit. Functions must be declared
        // before they can be called, or defined.
        //
        // TODO: This may be an area where the API should be streamlined; should
        // we have a version of `declare_function` that automatically declares
        // the function?
        let id = self
            .module
            .declare_function(&name, Linkage::Export, &self.ctx.func.signature)?;

        // Register Name
        self.func_names.insert(id, name.clone());

        // Define the function to jit. This finishes compilation, although
        // there may be outstanding relocations to perform. Currently, jit
        // cannot finish relocations until all functions to be called are
        // defined.
        self.module.define_function(id, &mut self.ctx)?;

        println!("Optimized Func: {}", self.ctx.func.display());

        // Finalize the functions which we just defined, which resolves any
        // outstanding relocations (patching in addresses, now that they're
        // available).
        self.module.finalize_definitions()?;

        // Clear the context to free memory.
        self.ctx.clear();

        Ok(id)
    }

    pub fn get_func_addr(&self, id: FuncId) -> pg::ExprStateEvalFunc {
        let func_addr = self.module.get_finalized_function(id);

        unsafe { std::mem::transmute::<*const u8, pg::ExprStateEvalFunc>(func_addr) }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct StepResult {
    pub resnull: Value,  // bool,
    pub resvalue: Value, // pg::Datum,
}

struct ISATypes {
    bool: Type,
    pointer: Type,
}

struct FuncArgs {
    state: Value,     // ExprState *state
    _econtext: Value, // ExprContext *econtext,
    isnull: Value,    // bool *isnull
}

pub struct ExprTranslator<'a> {
    types: ISATypes,
    builder: FunctionBuilder<'a>,

    // The arguments to the expresion interp function.
    args: FuncArgs,

    /// Each bytecode step can have a result. By default the interpreter
    /// stores all of those into an array. But that causes many intermediate
    /// loads and stores that we can avoid.
    ///
    /// So we store Cranelift variables for each of the intermediate step. These
    /// carry no runtime cost. However it causes some other issues. When
    /// evaluating some steps we may need that they are all written into memory
    /// as the interpreter would have done. (Usually if we call into some other
    /// part of postgres)
    ///
    /// In that case we can commit (store) all of these results at once. But
    /// that still saves us in the common case where they aren't needed.
    results: Vec<Option<StepResult>>,

    // Some ops assign to a temporary result. We store that here.
    tmp: StepResult,
}

impl<'a> ExprTranslator<'a> {
    /// Memory flags to be used whenever we interact with postgres structures.
    /// These allow our alias analysis pass to deduplicate memory accesses.
    fn pg_memflags() -> MemFlags {
        MemFlags::trusted().with_alias_region(Some(AliasRegion::Vmctx))
    }

    unsafe fn translate_step(
        &mut self,
        opno: usize,
        opcode: pg::ExprEvalOp,
        op: &mut pg::ExprEvalStep,
    ) -> PGJitResult<()> {
        let op_resvalue_ptr = self
            .builder
            .ins()
            .iconst(self.types.pointer, op.resvalue as i64);
        let op_resnull_ptr = self
            .builder
            .ins()
            .iconst(self.types.pointer, op.resnull as i64);

        match opcode {
            pg::ExprEvalOp::EEOP_ASSIGN_TMP => {
                // EEO_CASE(EEOP_ASSIGN_TMP)
                //     int			resultnum = op->d.assign_tmp.resultnum;
                //     resultslot->tts_values[resultnum] = state->resvalue;
                //     resultslot->tts_isnull[resultnum] = state->resnull;

                let resultnum = op.d.assign_tmp.resultnum;

                // Load both tmp values
                let tmp_resnull = self.builder.ins().load(
                    self.types.pointer,
                    Self::pg_memflags(),
                    self.tmp.resnull,
                    0,
                );
                let tmp_resvalue = self.builder.ins().load(
                    self.types.pointer,
                    Self::pg_memflags(),
                    self.tmp.resvalue,
                    0,
                );

                // Compute the address of the targets
                let resultslot = self.builder.ins().load(
                    self.types.pointer,
                    Self::pg_memflags(),
                    self.args.state,
                    offset_of!(pg::ExprState, resultslot) as i32,
                );

                let resultvalues = self.builder.ins().load(
                    self.types.pointer,
                    Self::pg_memflags(),
                    resultslot,
                    offset_of!(pg::TupleTableSlot, tts_values) as i32,
                );

                let resultnulls = self.builder.ins().load(
                    self.types.pointer,
                    Self::pg_memflags(),
                    resultslot,
                    offset_of!(pg::TupleTableSlot, tts_isnull) as i32,
                );

                // Store the result
                self.builder.ins().store(
                    Self::pg_memflags(),
                    tmp_resnull,
                    resultnulls,
                    resultnum * (self.types.bool.bytes() as i32),
                );
                self.builder.ins().store(
                    Self::pg_memflags(),
                    tmp_resvalue,
                    resultvalues,
                    resultnum * (self.types.pointer.bytes() as i32),
                );
            }
            pg::ExprEvalOp::EEOP_CONST => {
                // EEO_CASE(EEOP_CONST)
                //     *op->resnull = op->d.constval.isnull;
                //     *op->resvalue = op->d.constval.value;

                let resnull = self
                    .builder
                    .ins()
                    .iconst(self.types.bool, op.d.constval.isnull as i64);
                self.builder
                    .ins()
                    .store(Self::pg_memflags(), resnull, op_resnull_ptr, 0);

                let resvalue = self
                    .builder
                    .ins()
                    .iconst(self.types.pointer, op.d.constval.value as i64);
                self.builder
                    .ins()
                    .store(Self::pg_memflags(), resvalue, op_resvalue_ptr, 0);
            }
            pg::ExprEvalOp::EEOP_DONE => {
                // 	*isnull = state->resnull;
                // return state->resvalue;

                // Store the isnull flag
                let isnull = self.builder.ins().load(
                    self.types.bool,
                    Self::pg_memflags(),
                    self.tmp.resnull,
                    0,
                );
                self.builder
                    .ins()
                    .store(Self::pg_memflags(), isnull, self.args.isnull, 0);

                // Load the result value and return it
                let resvalue = self.builder.ins().load(
                    self.types.pointer,
                    Self::pg_memflags(),
                    self.tmp.resvalue,
                    0,
                );
                self.builder.ins().return_(&[resvalue]);
            }
            _ => {
                return Err(PGJitError::UnimplementedOpcode(opcode));
            }
        }

        Ok(())
    }
}
