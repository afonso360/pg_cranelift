#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl ExprEvalOp {
    pub fn name(&self) -> &'static str {
        match self {
            &ExprEvalOp::EEOP_DONE => "EEOP_DONE",
            &ExprEvalOp::EEOP_INNER_FETCHSOME => "EEOP_INNER_FETCHSOME",
            &ExprEvalOp::EEOP_OUTER_FETCHSOME => "EEOP_OUTER_FETCHSOME",
            &ExprEvalOp::EEOP_SCAN_FETCHSOME => "EEOP_SCAN_FETCHSOME",
            &ExprEvalOp::EEOP_INNER_VAR => "EEOP_INNER_VAR",
            &ExprEvalOp::EEOP_OUTER_VAR => "EEOP_OUTER_VAR",
            &ExprEvalOp::EEOP_SCAN_VAR => "EEOP_SCAN_VAR",
            &ExprEvalOp::EEOP_INNER_SYSVAR => "EEOP_INNER_SYSVAR",
            &ExprEvalOp::EEOP_OUTER_SYSVAR => "EEOP_OUTER_SYSVAR",
            &ExprEvalOp::EEOP_SCAN_SYSVAR => "EEOP_SCAN_SYSVAR",
            &ExprEvalOp::EEOP_WHOLEROW => "EEOP_WHOLEROW",
            &ExprEvalOp::EEOP_ASSIGN_INNER_VAR => "EEOP_ASSIGN_INNER_VAR",
            &ExprEvalOp::EEOP_ASSIGN_OUTER_VAR => "EEOP_ASSIGN_OUTER_VAR",
            &ExprEvalOp::EEOP_ASSIGN_SCAN_VAR => "EEOP_ASSIGN_SCAN_VAR",
            &ExprEvalOp::EEOP_ASSIGN_TMP => "EEOP_ASSIGN_TMP",
            &ExprEvalOp::EEOP_ASSIGN_TMP_MAKE_RO => "EEOP_ASSIGN_TMP_MAKE_RO",
            &ExprEvalOp::EEOP_CONST => "EEOP_CONST",
            &ExprEvalOp::EEOP_FUNCEXPR => "EEOP_FUNCEXPR",
            &ExprEvalOp::EEOP_FUNCEXPR_STRICT => "EEOP_FUNCEXPR_STRICT",
            &ExprEvalOp::EEOP_FUNCEXPR_FUSAGE => "EEOP_FUNCEXPR_FUSAGE",
            &ExprEvalOp::EEOP_FUNCEXPR_STRICT_FUSAGE => "EEOP_FUNCEXPR_STRICT_FUSAGE",
            &ExprEvalOp::EEOP_BOOL_AND_STEP_FIRST => "EEOP_BOOL_AND_STEP_FIRST",
            &ExprEvalOp::EEOP_BOOL_AND_STEP => "EEOP_BOOL_AND_STEP",
            &ExprEvalOp::EEOP_BOOL_AND_STEP_LAST => "EEOP_BOOL_AND_STEP_LAST",
            &ExprEvalOp::EEOP_BOOL_OR_STEP_FIRST => "EEOP_BOOL_OR_STEP_FIRST",
            &ExprEvalOp::EEOP_BOOL_OR_STEP => "EEOP_BOOL_OR_STEP",
            &ExprEvalOp::EEOP_BOOL_OR_STEP_LAST => "EEOP_BOOL_OR_STEP_LAST",
            &ExprEvalOp::EEOP_BOOL_NOT_STEP => "EEOP_BOOL_NOT_STEP",
            &ExprEvalOp::EEOP_QUAL => "EEOP_QUAL",
            &ExprEvalOp::EEOP_JUMP => "EEOP_JUMP",
            &ExprEvalOp::EEOP_JUMP_IF_NULL => "EEOP_JUMP_IF_NULL",
            &ExprEvalOp::EEOP_JUMP_IF_NOT_NULL => "EEOP_JUMP_IF_NOT_NULL",
            &ExprEvalOp::EEOP_JUMP_IF_NOT_TRUE => "EEOP_JUMP_IF_NOT_TRUE",
            &ExprEvalOp::EEOP_NULLTEST_ISNULL => "EEOP_NULLTEST_ISNULL",
            &ExprEvalOp::EEOP_NULLTEST_ISNOTNULL => "EEOP_NULLTEST_ISNOTNULL",
            &ExprEvalOp::EEOP_NULLTEST_ROWISNULL => "EEOP_NULLTEST_ROWISNULL",
            &ExprEvalOp::EEOP_NULLTEST_ROWISNOTNULL => "EEOP_NULLTEST_ROWISNOTNULL",
            &ExprEvalOp::EEOP_BOOLTEST_IS_TRUE => "EEOP_BOOLTEST_IS_TRUE",
            &ExprEvalOp::EEOP_BOOLTEST_IS_NOT_TRUE => "EEOP_BOOLTEST_IS_NOT_TRUE",
            &ExprEvalOp::EEOP_BOOLTEST_IS_FALSE => "EEOP_BOOLTEST_IS_FALSE",
            &ExprEvalOp::EEOP_BOOLTEST_IS_NOT_FALSE => "EEOP_BOOLTEST_IS_NOT_FALSE",
            &ExprEvalOp::EEOP_PARAM_EXEC => "EEOP_PARAM_EXEC",
            &ExprEvalOp::EEOP_PARAM_EXTERN => "EEOP_PARAM_EXTERN",
            &ExprEvalOp::EEOP_PARAM_CALLBACK => "EEOP_PARAM_CALLBACK",
            &ExprEvalOp::EEOP_CASE_TESTVAL => "EEOP_CASE_TESTVAL",
            &ExprEvalOp::EEOP_MAKE_READONLY => "EEOP_MAKE_READONLY",
            &ExprEvalOp::EEOP_IOCOERCE => "EEOP_IOCOERCE",
            &ExprEvalOp::EEOP_DISTINCT => "EEOP_DISTINCT",
            &ExprEvalOp::EEOP_NOT_DISTINCT => "EEOP_NOT_DISTINCT",
            &ExprEvalOp::EEOP_NULLIF => "EEOP_NULLIF",
            &ExprEvalOp::EEOP_SQLVALUEFUNCTION => "EEOP_SQLVALUEFUNCTION",
            &ExprEvalOp::EEOP_CURRENTOFEXPR => "EEOP_CURRENTOFEXPR",
            &ExprEvalOp::EEOP_NEXTVALUEEXPR => "EEOP_NEXTVALUEEXPR",
            &ExprEvalOp::EEOP_ARRAYEXPR => "EEOP_ARRAYEXPR",
            &ExprEvalOp::EEOP_ARRAYCOERCE => "EEOP_ARRAYCOERCE",
            &ExprEvalOp::EEOP_ROW => "EEOP_ROW",
            &ExprEvalOp::EEOP_ROWCOMPARE_STEP => "EEOP_ROWCOMPARE_STEP",
            &ExprEvalOp::EEOP_ROWCOMPARE_FINAL => "EEOP_ROWCOMPARE_FINAL",
            &ExprEvalOp::EEOP_MINMAX => "EEOP_MINMAX",
            &ExprEvalOp::EEOP_FIELDSELECT => "EEOP_FIELDSELECT",
            &ExprEvalOp::EEOP_FIELDSTORE_DEFORM => "EEOP_FIELDSTORE_DEFORM",
            &ExprEvalOp::EEOP_FIELDSTORE_FORM => "EEOP_FIELDSTORE_FORM",
            &ExprEvalOp::EEOP_SBSREF_SUBSCRIPTS => "EEOP_SBSREF_SUBSCRIPTS",
            &ExprEvalOp::EEOP_SBSREF_OLD => "EEOP_SBSREF_OLD",
            &ExprEvalOp::EEOP_SBSREF_ASSIGN => "EEOP_SBSREF_ASSIGN",
            &ExprEvalOp::EEOP_SBSREF_FETCH => "EEOP_SBSREF_FETCH",
            &ExprEvalOp::EEOP_DOMAIN_TESTVAL => "EEOP_DOMAIN_TESTVAL",
            &ExprEvalOp::EEOP_DOMAIN_NOTNULL => "EEOP_DOMAIN_NOTNULL",
            &ExprEvalOp::EEOP_DOMAIN_CHECK => "EEOP_DOMAIN_CHECK",
            &ExprEvalOp::EEOP_CONVERT_ROWTYPE => "EEOP_CONVERT_ROWTYPE",
            &ExprEvalOp::EEOP_SCALARARRAYOP => "EEOP_SCALARARRAYOP",
            &ExprEvalOp::EEOP_HASHED_SCALARARRAYOP => "EEOP_HASHED_SCALARARRAYOP",
            &ExprEvalOp::EEOP_XMLEXPR => "EEOP_XMLEXPR",
            &ExprEvalOp::EEOP_AGGREF => "EEOP_AGGREF",
            &ExprEvalOp::EEOP_GROUPING_FUNC => "EEOP_GROUPING_FUNC",
            &ExprEvalOp::EEOP_WINDOW_FUNC => "EEOP_WINDOW_FUNC",
            &ExprEvalOp::EEOP_SUBPLAN => "EEOP_SUBPLAN",
            &ExprEvalOp::EEOP_AGG_STRICT_DESERIALIZE => "EEOP_AGG_STRICT_DESERIALIZE",
            &ExprEvalOp::EEOP_AGG_DESERIALIZE => "EEOP_AGG_DESERIALIZE",
            &ExprEvalOp::EEOP_AGG_STRICT_INPUT_CHECK_ARGS => "EEOP_AGG_STRICT_INPUT_CHECK_ARGS",
            &ExprEvalOp::EEOP_AGG_STRICT_INPUT_CHECK_NULLS => "EEOP_AGG_STRICT_INPUT_CHECK_NULLS",
            &ExprEvalOp::EEOP_AGG_PLAIN_PERGROUP_NULLCHECK => "EEOP_AGG_PLAIN_PERGROUP_NULLCHECK",
            &ExprEvalOp::EEOP_AGG_PLAIN_TRANS_INIT_STRICT_BYVAL => {
                "EEOP_AGG_PLAIN_TRANS_INIT_STRICT_BYVAL"
            }
            &ExprEvalOp::EEOP_AGG_PLAIN_TRANS_STRICT_BYVAL => "EEOP_AGG_PLAIN_TRANS_STRICT_BYVAL",
            &ExprEvalOp::EEOP_AGG_PLAIN_TRANS_BYVAL => "EEOP_AGG_PLAIN_TRANS_BYVAL",
            &ExprEvalOp::EEOP_AGG_PLAIN_TRANS_INIT_STRICT_BYREF => {
                "EEOP_AGG_PLAIN_TRANS_INIT_STRICT_BYREF"
            }
            &ExprEvalOp::EEOP_AGG_PLAIN_TRANS_STRICT_BYREF => "EEOP_AGG_PLAIN_TRANS_STRICT_BYREF",
            &ExprEvalOp::EEOP_AGG_PLAIN_TRANS_BYREF => "EEOP_AGG_PLAIN_TRANS_BYREF",
            &ExprEvalOp::EEOP_AGG_ORDERED_TRANS_DATUM => "EEOP_AGG_ORDERED_TRANS_DATUM",
            &ExprEvalOp::EEOP_AGG_ORDERED_TRANS_TUPLE => "EEOP_AGG_ORDERED_TRANS_TUPLE",
            &ExprEvalOp::EEOP_LAST => "EEOP_LAST",
            _ => "EEOP UNKNOWN",
        }
    }
}

impl ExprState {
    pub fn format_steps(&mut self) -> String {
        let mut result = String::new();
        for opno in 0..(self.steps_len as usize) {
            unsafe {
                let op = &mut *self.steps.offset(opno.try_into().unwrap());
                let opcode = ExecEvalStepOp(self, op);
                result.push_str(&format!("{}: {}\n", opno, opcode.name()));
            }
        }
        result
    }
}
