use std::collections::HashSet;

use biodivine_lib_bdd::BddPathIterator;
use hashconsing::HConsign;
use rsdd::{builder::BottomUpBuilder, repr::BddPtr};

use crate::{exp::*, is_false, BExp, BExp_};

enum VisitResult {
    Dead,
    Live,
    Unknown,
}

fn visit_descendants<'a, Builder>(
    fb: &mut HConsign<BExp_>,
    fp: &mut HConsign<Exp_>,
    bdd: &'a Builder,
    dead_states: &HashSet<Exp>,
    explored: &mut HashSet<Exp>,
    exps: Vec<Exp>,
) -> VisitResult
where
    Builder: BottomUpBuilder<'a, BddPtr<'a>>,
{
    use VisitResult::*;
    let mut result = Unknown;
    for e in exps {
        match visit(fb, fp, bdd, dead_states, explored, &e) {
            Live => {
                result = Live;
                break;
            }
            Dead => {
                explored.insert(e);
            }
            Unknown => {}
        }
    }
    result
}

fn visit<'a, Builder>(
    fb: &mut HConsign<BExp_>,
    fp: &mut HConsign<Exp_>,
    bdd: &'a Builder,
    dead_states: &HashSet<Exp>,
    explored: &mut HashSet<Exp>,
    exp: &Exp,
) -> VisitResult
where
    Builder: BottomUpBuilder<'a, BddPtr<'a>>,
{
    use VisitResult::*;
    if dead_states.contains(exp) {
        Dead
    } else if explored.contains(exp) {
        Unknown
    } else {
        explored.insert(exp.clone());
        let eps = epsilon(fb, exp);
        if is_false(bdd, &eps) {
            let dexp = derivative(fb, fp, exp);
            let next_exps: Vec<Exp> = dexp
                .into_iter()
                .filter_map(|(b, (e, _))| if is_false(bdd, &b) { None } else { Some(e) })
                .collect();
            visit_descendants(fb, fp, bdd, dead_states, explored, next_exps)
        } else {
            Live
        }
    }
}
