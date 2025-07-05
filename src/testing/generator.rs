use crate::{
    kernel1, kernel2,
    parsing::*,
    syntax::{Gkat, SATGkat},
};
use pretty::BoxAllocator;
use rand::{distr::slice::Choose, rngs::StdRng, seq::IndexedRandom, *};

pub struct Generator {
    bexp_max_size: u64,
    exp_max_size: u64,
    pbool_max_count: u64,
    rng: StdRng,
    metric: u64,
}

impl Generator {
    pub fn new(bexp_max_size: u64, exp_max_size: u64, pbool_max_count: u64) -> Self {
        Self {
            bexp_max_size,
            exp_max_size: exp_max_size * 2,
            pbool_max_count,
            rng: StdRng::seed_from_u64(346608),
            metric: 0,
        }
    }

    pub fn get_metric(&mut self) -> u64 {
        let metric = self.metric;
        self.metric = 0;
        return metric;
    }

    pub fn mk_pbool(&mut self) -> BExp {
        let i = self.rng.random_range(1..self.pbool_max_count);
        BExp::PBool(format!("b{}", i))
    }

    pub fn mk_bexp(&mut self) -> BExp {
        self.mk_bexp_sized(self.bexp_max_size)
    }

    fn mk_bexp_sized(&mut self, size: u64) -> BExp {
        if size <= 1 {
            self.mk_pbool()
        } else {
            let choices = ["bool", "and", "or", "not"];
            match *choices.choose(&mut self.rng).unwrap() {
                "bool" => self.mk_pbool(),
                "and" => {
                    let m = self.mk_bexp_sized(size / 2);
                    let n = self.mk_bexp_sized(size / 2);
                    m.and(&n)
                }
                "or" => {
                    let m = self.mk_bexp_sized(size / 2);
                    let n = self.mk_bexp_sized(size / 2);
                    m.or(&n)
                }
                "not" => {
                    let m = self.mk_bexp_sized(size - 1);
                    m.not()
                }
                _ => panic!("mk_bexp_sized"),
            }
        }
    }

    pub fn mk_exp_ne(&mut self) -> (Exp, Exp) {
        loop {
            let m1 = self.mk_exp_sized(self.exp_max_size);
            let m2 = self.mk_exp_sized(self.exp_max_size);
            let mut gkat = SATGkat::new();
            let mut solver = kernel1::Solver::new();
            let exp1 = gkat.from_exp(&m1);
            let exp2 = gkat.from_exp(&m2);
            if !solver.equiv_iter(&mut gkat, &exp1, &exp2) {
                return (m1, m2);
            }
        }
    }

    pub fn mk_exp(&mut self) -> Exp {
        self.mk_exp_sized(self.exp_max_size)
    }

    fn mk_exp_sized(&mut self, size: u64) -> Exp {
        if size <= 1 {
            let choices = [("action", 50), ("skip", 1)];
            let item = choices
                .choose_weighted(&mut self.rng, |item| item.1)
                .unwrap();
            match item.0 {
                "action" => {
                    let p = self.rng.random_range(0..100);
                    Exp::act(&format!("p{}", p))
                }
                "skip" => Exp::skip(),
                _ => panic!(),
            }
        } else {
            let choices = [("seq", 15), ("if", 3), ("while", 1)];
            let item = choices
                .choose_weighted(&mut self.rng, |item| item.1)
                .unwrap();
            match item.0 {
                "seq" => {
                    let m = self.mk_exp_sized(size / 2);
                    let n = self.mk_exp_sized(size / 2);
                    m.seq(&n)
                }
                "if" => {
                    let bexp_size = self.rng.random_range(1..=self.bexp_max_size);
                    let b = self.mk_bexp_sized(bexp_size);
                    let m = self.mk_exp_sized(size / 2);
                    let n = self.mk_exp_sized(size / 2);
                    b.ifte(&m, &n)
                }
                "while" => {
                    let bexp_size = self.rng.random_range(1..=self.bexp_max_size);
                    let b = self.mk_bexp_sized(bexp_size);
                    let m = self.mk_exp_sized(size - 1);
                    b.while_(&m)
                }
                _ => panic!("mk_exp"),
            }
        }
    }

    pub fn mk_bexp_eq(&mut self) -> (BExp, BExp) {
        self.mk_bexp_eq_sized(self.bexp_max_size)
    }

    fn mk_bexp_eq_sized(&mut self, size: u64) -> (BExp, BExp) {
        if size <= 1 {
            let b = self.mk_bexp_sized(size);
            (b.clone(), b)
        } else {
            let choices = [
                "symm",
                "congr_not",
                "congr_and",
                "congr_or",
                "comm_and",
                "comm_or",
                "assoc_and",
                "assoc_or",
                "comp_and",
                "comp_or",
                "idem_and",
                "idem_or",
            ];
            match *choices.choose(&mut self.rng).unwrap() {
                "symm" => {
                    let (b1, b2) = self.mk_bexp_eq_sized(size - 1);
                    (b2, b1)
                }
                "congr_not" => {
                    let (b1, b2) = self.mk_bexp_eq_sized(size - 1);
                    (b1.not(), b2.not())
                }
                "congr_and" => {
                    let (b11, b12) = self.mk_bexp_eq_sized(size / 2);
                    let (b21, b22) = self.mk_bexp_eq_sized(size / 2);
                    (b11.and(&b21), b12.and(&b22))
                }
                "congr_or" => {
                    let (b11, b12) = self.mk_bexp_eq_sized(size / 2);
                    let (b21, b22) = self.mk_bexp_eq_sized(size / 2);
                    (b11.or(&b21), b12.or(&b22))
                }
                "comm_and" => {
                    let (b11, b12) = self.mk_bexp_eq_sized(size / 2);
                    let (b21, b22) = self.mk_bexp_eq_sized(size / 2);
                    (b11.and(&b21), b22.and(&b12))
                }
                "comm_or" => {
                    let (b11, b12) = self.mk_bexp_eq_sized(size / 2);
                    let (b21, b22) = self.mk_bexp_eq_sized(size / 2);
                    (b11.or(&b21), b22.or(&b12))
                }
                "assoc_and" => {
                    let (b11, b12) = self.mk_bexp_eq_sized(size / 3);
                    let (b21, b22) = self.mk_bexp_eq_sized(size / 3);
                    let (b31, b32) = self.mk_bexp_eq_sized(size / 3);
                    (b11.and(&b21.and(&b31)), b12.and(&b22).and(&b32))
                }
                "assoc_or" => {
                    let (b11, b12) = self.mk_bexp_eq_sized(size / 3);
                    let (b21, b22) = self.mk_bexp_eq_sized(size / 3);
                    let (b31, b32) = self.mk_bexp_eq_sized(size / 3);
                    (b11.or(&b21.or(&b31)), b12.or(&b22).or(&b32))
                }
                "comp_and" => {
                    let (b1, b2) = self.mk_bexp_eq_sized(size / 2);
                    (b1.not().and(&b2), BExp::zero())
                }
                "comp_or" => {
                    let (b1, b2) = self.mk_bexp_eq_sized(size / 2);
                    (b1.not().or(&b2), BExp::one())
                }
                "idem_and" => {
                    let (b1, b2) = self.mk_bexp_eq_sized(size / 2);
                    (b1.and(&b2), b2)
                }
                "idem_or" => {
                    let (b1, b2) = self.mk_bexp_eq_sized(size / 2);
                    (b1.or(&b2), b2)
                }
                _ => panic!("mk_bexp_eq_sized"),
            }
        }
    }

    pub fn mk_exp_eq(&mut self) -> (Exp, Exp) {
        loop {
            let result = self.mk_exp_eq_sized(self.exp_max_size);
            if 4 <= self.get_metric() {
                return result;
            }
        }
    }

    fn mk_exp_eq_sized(&mut self, size: u64) -> (Exp, Exp) {
        if size <= 1 {
            let choices = [("action", 100), ("skip", 1)];
            let item = choices
                .choose_weighted(&mut self.rng, |item| item.1)
                .unwrap();
            match item.0 {
                "action" => {
                    let p = self.rng.random_range(0..100);
                    (Exp::act(&format!("p{}", p)), Exp::act(&format!("p{}", p)))
                }
                "skip" => (Exp::skip(), Exp::skip()),
                _ => panic!("mk_exp_eq_sized"),
            }
        } else {
            let choices = [
                ("refl", 30),
                ("symm", 40),
                ("congr_seq", 50),
                ("congr_if", 20),
                ("congr_while", 2),
                ("idem_if", 2),
                ("skew_comm", 2),
                ("skew_assoc", 2),
                ("guard", 2),
                ("distr_right", 2),
                ("assoc_seq", 10),
                ("id_left", 2),
                ("id_right", 2),
                ("unroll", 5),
                ("tighten", 5),
            ];
            let item = choices
                .choose_weighted(&mut self.rng, |item| item.1)
                .unwrap();
            match item.0 {
                "refl" => {
                    let m = self.mk_exp_sized(size);
                    (m.clone(), m)
                }
                "symm" => {
                    let (m, n) = self.mk_exp_eq_sized(size - 1);
                    (n, m)
                }
                "congr_seq" => {
                    let (m11, m12) = self.mk_exp_eq_sized(size / 2);
                    let (m21, m22) = self.mk_exp_eq_sized(size / 2);
                    (m11.seq(&m21), m12.seq(&m22))
                }
                "congr_if" => {
                    let bexp_size = self.rng.random_range(1..=self.bexp_max_size);
                    let (b1, b2) = self.mk_bexp_eq_sized(bexp_size);
                    let (m11, m12) = self.mk_exp_eq_sized(size / 2);
                    let (m21, m22) = self.mk_exp_eq_sized(size / 2);
                    (b1.ifte(&m11, &m21), b2.ifte(&m12, &m22))
                }
                "congr_while" => {
                    let bexp_size = self.rng.random_range(1..=self.bexp_max_size);
                    let (b1, b2) = self.mk_bexp_eq_sized(bexp_size);
                    let (m1, m2) = self.mk_exp_eq_sized(size / 2);
                    (b1.while_(&m1), b2.while_(&m2))
                }
                "idem_if" => {
                    self.metric += 1;
                    let bexp_size = self.rng.random_range(1..=self.bexp_max_size);
                    let b = self.mk_bexp_sized(bexp_size);
                    let (m1, m2) = self.mk_exp_eq_sized(size / 2);
                    (b.ifte(&m1, &m2), m1)
                }
                "skew_comm" => {
                    self.metric += 1;
                    let bexp_size = self.rng.random_range(1..=self.bexp_max_size);
                    let (b1, b2) = self.mk_bexp_eq_sized(bexp_size);
                    let (m11, m12) = self.mk_exp_eq_sized(size / 2);
                    let (m21, m22) = self.mk_exp_eq_sized(size / 2);
                    (b1.ifte(&m11, &m21), b2.not().ifte(&m22, &m12))
                }
                "skew_assoc" => {
                    self.metric += 1;
                    let bexp_size1 = self.rng.random_range(1..=self.bexp_max_size);
                    let bexp_size2 = self.rng.random_range(1..=self.bexp_max_size);
                    let (b11, b12) = self.mk_bexp_eq_sized(bexp_size1);
                    let (b21, b22) = self.mk_bexp_eq_sized(bexp_size2);
                    let (m11, m12) = self.mk_exp_eq_sized(size / 3);
                    let (m21, m22) = self.mk_exp_eq_sized(size / 3);
                    let (m31, m32) = self.mk_exp_eq_sized(size / 3);
                    (
                        b21.ifte(&b11.ifte(&m11, &m21), &m31),
                        b12.and(&b22).ifte(&m12, &b21.ifte(&m22, &m32)),
                    )
                }
                "guard" => {
                    self.metric += 1;
                    let bexp_size = self.rng.random_range(1..=self.bexp_max_size);
                    let (b1, b2) = self.mk_bexp_eq_sized(bexp_size);
                    let (m11, m12) = self.mk_exp_eq_sized(size / 2);
                    let (m21, m22) = self.mk_exp_eq_sized(size / 2);
                    (b1.ifte(&m11, &m21), b2.ifte(&b2.test().seq(&m12), &m22))
                }
                "distr_right" => {
                    self.metric += 1;
                    let bexp_size = self.rng.random_range(1..=self.bexp_max_size);
                    let (b1, b2) = self.mk_bexp_eq_sized(bexp_size);
                    let (m11, m12) = self.mk_exp_eq_sized(size / 3);
                    let (m21, m22) = self.mk_exp_eq_sized(size / 3);
                    let (m31, m32) = self.mk_exp_eq_sized(size / 3);
                    (
                        b1.ifte(&m11, &m21).seq(&m31),
                        b2.ifte(&m12.seq(&m32), &m22.seq(&m32)),
                    )
                }
                "assoc_seq" => {
                    self.metric += 1;
                    let (m11, m12) = self.mk_exp_eq_sized(size / 3);
                    let (m21, m22) = self.mk_exp_eq_sized(size / 3);
                    let (m31, m32) = self.mk_exp_eq_sized(size / 3);
                    (m11.seq(&m21.seq(&m31)), m12.seq(&m22).seq(&m32))
                }
                "id_left" => {
                    self.metric += 1;
                    let (m1, m2) = self.mk_exp_eq_sized(size - 1);
                    (BExp::one().test().seq(&m1), m2)
                }
                "id_right" => {
                    self.metric += 1;
                    let (m1, m2) = self.mk_exp_eq_sized(size - 1);
                    (m1.seq(&BExp::one().test()), m2)
                }
                "unroll" => {
                    self.metric += 1;
                    let bexp_size = self.rng.random_range(1..=self.bexp_max_size);
                    let (b1, b2) = self.mk_bexp_eq_sized(bexp_size);
                    let (m1, m2) = self.mk_exp_eq_sized(size - 1);
                    (
                        b1.while_(&m1),
                        b2.ifte(&m2.seq(&b1.while_(&m2)), &Exp::skip()),
                    )
                }
                "tighten" => {
                    self.metric += 1;
                    let bexp_size1 = self.rng.random_range(1..=self.bexp_max_size);
                    let bexp_size2 = self.rng.random_range(1..=self.bexp_max_size);
                    let (b11, b12) = self.mk_bexp_eq_sized(bexp_size1);
                    let (b21, b22) = self.mk_bexp_eq_sized(bexp_size2);
                    let (m1, m2) = self.mk_exp_eq_sized(size - 1);
                    (
                        b11.while_(&b21.ifte(&m1, &Exp::skip())),
                        b12.while_(&b22.test().seq(&m2)),
                    )
                }
                _ => panic!("mk_exp_eq_sized"),
            }
        }
    }
}

#[test]
fn test_exp2() {
    let mut generator = Generator::new(20, 100, 100);
    for _ in 0..10000 {
        let (m1, m2) = generator.mk_exp_eq();
        let mut gkat = SATGkat::new();
        let mut solver = kernel1::Solver::new();
        let exp1 = gkat.from_exp(&m1);
        let exp2 = gkat.from_exp(&m2);
        let result = solver.equiv_iter(&mut gkat, &exp1, &exp2);
        assert!(result)
    }
}
