use crate::parsing::{self};
use gxhash::GxHasher;
use hashconsing::HConsed;
use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

// Trait for generic BExp.
pub trait BExp: Debug + Clone + Hash + Eq {}

// Type for generic Exp.
pub type Exp<B> = HConsed<Exp_<B>>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Exp_<B> {
    Act(u64),
    Seq(Exp<B>, Exp<B>),
    Ifte(B, Exp<B>, Exp<B>),
    Test(B),
    While(B, Exp<B>),
}

// Trait for generic Gkat manager.
pub trait Gkat<B: Clone + Hash + Eq> {
    // Methods for BExp.
    fn mk_zero(&mut self) -> B;
    fn mk_one(&mut self) -> B;
    fn mk_var(&mut self, s: String) -> B;
    fn mk_and(&mut self, b1: &B, b2: &B) -> B;
    fn mk_or(&mut self, b1: &B, b2: &B) -> B;
    fn mk_not(&mut self, b: &B) -> B;
    fn is_false(&mut self, b: &B) -> bool;
    fn is_equiv(&mut self, b1: &B, b2: &B) -> bool;

    // Create a new BExp from parsing.
    fn from_bexp(&mut self, raw: &parsing::BExp) -> B {
        use parsing::BExp::*;
        match raw {
            Zero => self.mk_zero(),
            One => self.mk_one(),
            PBool(s) => self.mk_var(s.to_string()),
            Or(b1, b2) => {
                let b1 = self.from_bexp(b1);
                let b2 = self.from_bexp(b2);
                self.mk_or(&b1, &b2)
            }
            And(b1, b2) => {
                let b1 = self.from_bexp(b1);
                let b2 = self.from_bexp(b2);
                self.mk_and(&b1, &b2)
            }
            Not(b) => {
                let b = self.from_bexp(b);
                self.mk_not(&b)
            }
        }
    }

    // Methods for program expressions.
    fn hashcons(&mut self, e: Exp_<B>) -> Exp<B>;

    fn mk_act(&mut self, s: String) -> Exp<B> {
        let mut hasher = GxHasher::default();
        s.hash(&mut hasher);
        let a = hasher.finish();
        self.hashcons(Exp_::Act(a))
    }

    fn mk_fail(&mut self) -> Exp<B> {
        let b = self.mk_zero();
        self.mk_test(b)
    }

    #[inline]
    fn mk_test(&mut self, b: B) -> Exp<B> {
        self.hashcons(Exp_::Test(b))
    }

    fn mk_seq(&mut self, p1: Exp<B>, p2: Exp<B>) -> Exp<B> {
        use Exp_::*;
        match (p1.get(), p2.get()) {
            (Test(b1), Test(b2)) => {
                let b3 = self.mk_and(&b1, &b2);
                self.mk_test(b3)
            }
            (Test(b), _) if b == &self.mk_zero() => self.mk_fail(),
            (_, Test(b)) if b == &self.mk_zero() => self.mk_fail(),
            (Test(b), _) if b == &self.mk_one() => p2,
            (_, Test(b)) if b == &self.mk_one() => p1,
            _ => self.hashcons(Exp_::Seq(p1, p2)),
        }
    }

    fn mk_ifte(&mut self, b: B, p1: Exp<B>, p2: Exp<B>) -> Exp<B> {
        if p1 == self.mk_fail() {
            let nb = self.mk_not(&b);
            let p1 = self.mk_test(nb);
            self.mk_seq(p1, p2)
        } else if p2 == self.mk_fail() {
            let p0 = self.mk_test(b);
            self.mk_seq(p0, p1)
        } else {
            self.hashcons(Exp_::Ifte(b, p1, p2))
        }
    }

    #[inline]
    fn mk_while(&mut self, b: B, p: Exp<B>) -> Exp<B> {
        self.hashcons(Exp_::While(b, p))
    }

    // Create a new Exp from parsing.
    fn from_exp(&mut self, raw: &parsing::Exp) -> Exp<B> {
        use parsing::Exp::*;
        match raw {
            Act(s) => self.mk_act(s.to_string()),
            Seq(p1, p2) => {
                let p1 = self.from_exp(p1);
                let p2 = self.from_exp(p2);
                self.mk_seq(p1, p2)
            }
            Ifte(b, p1, p2) => {
                let b = self.from_bexp(b);
                let p1 = self.from_exp(p1);
                let p2 = self.from_exp(p2);
                self.mk_ifte(b, p1, p2)
            }
            Test(b) => {
                let b = self.from_bexp(b);
                self.mk_test(b)
            }
            While(b, p) => {
                let b = self.from_bexp(b);
                let p = self.from_exp(p);
                self.mk_while(b, p)
            }
        }
    }
}
