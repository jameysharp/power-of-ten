#![feature(box_syntax)]
#![feature(plugin_registrar)]
#![feature(rustc_private)]
#![warn(unused)]

extern crate syntax;

// Load rustc as a plugin to get macros
#[macro_use]
extern crate rustc;

use syntax::ast;
use syntax::codemap::Spanned;
use rustc::lint::{Context, LintPass, LintPassObject, LintArray};
use rustc::plugin::Registry;

declare_lint!(LOOP_BOUNDS, Warn, "loops must either have trivial finite bounds or be provably infinite");

struct Pass;

impl LintPass for Pass {
    fn get_lints(&self) -> LintArray {
        lint_array!(LOOP_BOUNDS)
    }

    fn check_expr(&mut self, cx: &Context, ex: &ast::Expr) {
        if let ast::ExprCall(ref method, ref args) = ex.node {
            if let ast::ExprPath(None, ast::Path { global: true, ref segments, .. }) = method.node {
                if segments.iter().map(|seg| seg.identifier.name.as_str()).collect::<Vec<_>>() == ["std", "iter", "IntoIterator", "into_iter"]
                   && !bounded_iterator(&*args[0]) {
                    cx.span_lint(LOOP_BOUNDS, args[0].span, "can't prove that loop has static iteration bounds")
                }
            }
        }
    }
}

fn bounded_iterator(iter: &ast::Expr) -> bool {
    match iter.node {
        ast::ExprParen(ref iter) => bounded_iterator(&*iter),
        ast::ExprMethodCall(Spanned { node: ast::Ident { name, .. }, .. }, ref _ty, ref args) =>
            match &*name.as_str() {
                // these methods yield no more than the underlying iterator, so if that is bounded than so are these
                "map" | "filter" | "filter_map" | "enumerate" | "skip_while" | "take_while" | "skip" | "scan" | "inspect" | "by_ref" | "rev" | "cloned" => bounded_iterator(&*args[0]),
                // .take() imposes a bound on any iterator
                "take" => true,
                // TODO: should be able to prove these if both iterators are bounded
                "zip" | "chain" => false,
                _ => false,
            },
        ast::ExprRange(Some(_), Some(_)) => true,
        _ => false,
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box Pass as LintPassObject);
}
