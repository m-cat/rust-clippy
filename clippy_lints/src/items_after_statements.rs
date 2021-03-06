// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! lint when items are used after statements

use crate::utils::{in_macro, span_lint};
use matches::matches;
use rustc::lint::{EarlyContext, EarlyLintPass, LintArray, LintPass};
use rustc::{declare_tool_lint, lint_array};
use syntax::ast::*;

/// **What it does:** Checks for items declared after some statement in a block.
///
/// **Why is this bad?** Items live for the entire scope they are declared
/// in. But statements are processed in order. This might cause confusion as
/// it's hard to figure out which item is meant in a statement.
///
/// **Known problems:** None.
///
/// **Example:**
/// ```rust
/// fn foo() {
///     println!("cake");
/// }
///
/// fn main() {
///     foo(); // prints "foo"
///     fn foo() {
///         println!("foo");
///     }
///     foo(); // prints "foo"
/// }
/// ```
declare_clippy_lint! {
    pub ITEMS_AFTER_STATEMENTS,
    pedantic,
    "blocks where an item comes after a statement"
}

pub struct ItemsAfterStatements;

impl LintPass for ItemsAfterStatements {
    fn get_lints(&self) -> LintArray {
        lint_array!(ITEMS_AFTER_STATEMENTS)
    }
}

impl EarlyLintPass for ItemsAfterStatements {
    fn check_block(&mut self, cx: &EarlyContext<'_>, item: &Block) {
        if in_macro(item.span) {
            return;
        }

        // skip initial items
        let stmts = item
            .stmts
            .iter()
            .map(|stmt| &stmt.node)
            .skip_while(|s| matches!(**s, StmtKind::Item(..)));

        // lint on all further items
        for stmt in stmts {
            if let StmtKind::Item(ref it) = *stmt {
                if in_macro(it.span) {
                    return;
                }
                if let ItemKind::MacroDef(..) = it.node {
                    // do not lint `macro_rules`, but continue processing further statements
                    continue;
                }
                span_lint(
                    cx,
                    ITEMS_AFTER_STATEMENTS,
                    it.span,
                    "adding items after statements is confusing, since items exist from the \
                     start of the scope",
                );
            }
        }
    }
}
