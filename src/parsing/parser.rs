use crate::parsing::raw::Exp;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(unused_imports)]
    #[rustfmt::skip]
    pub spec);

pub fn parse(s: String) -> (Exp, Exp, bool) {
    spec::InputParser::new().parse(&s).unwrap()
}
