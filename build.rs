extern crate bindgen;

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Write the wrapper.h file to the output directory.
    let wrapper_h_path = out_path.join("wrapper.h");
    std::fs::write(
        &wrapper_h_path,
        r#"
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
    "#,
    )
    .expect("Couldn't write wrapper.h");

    let bindings = bindgen::Builder::default()
        // TODO: Hardcoded for now, we should call pg_config to get this
        .clang_arg("-I/usr/include/postgresql/14/server/")
        .header(wrapper_h_path.to_str().unwrap())
        .parse_callbacks(Box::new(IgnoreMacros::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings!");
}

const IGNORE_MACROS: [&str; 20] = [
    "FE_DIVBYZERO",
    "FE_DOWNWARD",
    "FE_INEXACT",
    "FE_INVALID",
    "FE_OVERFLOW",
    "FE_TONEAREST",
    "FE_TOWARDZERO",
    "FE_UNDERFLOW",
    "FE_UPWARD",
    "FP_INFINITE",
    "FP_INT_DOWNWARD",
    "FP_INT_TONEAREST",
    "FP_INT_TONEARESTFROMZERO",
    "FP_INT_TOWARDZERO",
    "FP_INT_UPWARD",
    "FP_NAN",
    "FP_NORMAL",
    "FP_SUBNORMAL",
    "FP_ZERO",
    "IPPORT_RESERVED",
];

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        if self.0.contains(name) {
            MacroParsingBehavior::Ignore
        } else {
            MacroParsingBehavior::Default
        }
    }
}

impl IgnoreMacros {
    fn new() -> Self {
        Self(IGNORE_MACROS.into_iter().map(|s| s.to_owned()).collect())
    }
}
