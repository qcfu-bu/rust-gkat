use pretty::{BoxAllocator, DocAllocator, DocBuilder};
use std::fmt::Debug;

#[derive(Clone)]
pub enum BExp {
    Zero,
    One,
    PBool(String),
    Or(Box<BExp>, Box<BExp>),
    And(Box<BExp>, Box<BExp>),
    Not(Box<BExp>),
}

impl BExp {
    pub fn zero() -> Self {
        Self::Zero
    }

    pub fn one() -> Self {
        Self::One
    }

    pub fn pbool(s: &str) -> Self {
        Self::PBool(s.to_string())
    }

    pub fn or(&self, other: &Self) -> Self {
        Self::Or(Box::new(self.clone()), Box::new(other.clone()))
    }

    pub fn and(&self, other: &Self) -> Self {
        Self::And(Box::new(self.clone()), Box::new(other.clone()))
    }

    pub fn not(&self) -> Self {
        Self::Not(Box::new(self.clone()))
    }

    pub fn test(&self) -> Exp {
        Exp::Test(self.clone())
    }

    pub fn ifte(&self, m: &Exp, n: &Exp) -> Exp {
        Exp::Ifte(self.clone(), Box::new(m.clone()), Box::new(n.clone()))
    }

    pub fn while_(&self, m: &Exp) -> Exp {
        Exp::While(self.clone(), Box::new(m.clone()))
    }

    pub fn sexpr<'b, D>(&'b self, alloc: &'b D) -> DocBuilder<'b, D>
    where
        D: DocAllocator<'b>,
        D::Doc: Clone,
    {
        match self {
            BExp::Zero => alloc.text("0"),
            BExp::One => alloc.text("1"),
            BExp::PBool(x) => alloc.text(x),
            BExp::Or(_, _) => {
                let mut ms = vec![];
                let mut curr = self;
                while let BExp::Or(m, n) = curr {
                    ms.push(m.sexpr(alloc));
                    curr = n;
                }
                ms.push(curr.sexpr(alloc));
                let body = alloc.intersperse(ms, alloc.softline());
                alloc
                    .text("(or")
                    .append(alloc.softline())
                    .append(body)
                    .append(")")
            }
            BExp::And(_, _) => {
                let mut ms = vec![];
                let mut curr = self;
                while let BExp::And(m, n) = curr {
                    ms.push(m.sexpr(alloc));
                    curr = n;
                }
                ms.push(curr.sexpr(alloc));
                let body = alloc.intersperse(ms, alloc.softline());
                alloc
                    .text("(and")
                    .append(alloc.softline())
                    .append(body)
                    .append(")")
            }
            BExp::Not(m) => {
                let m = m.sexpr(alloc);
                alloc
                    .text("(not")
                    .append(alloc.softline())
                    .append(m)
                    .append(")")
            }
        }
    }

    pub fn formatted<'b, D>(&'b self, alloc: &'b D) -> DocBuilder<'b, D>
    where
        D: DocAllocator<'b>,
        D::Doc: Clone,
    {
        match self {
            BExp::Zero => alloc.text("0"),
            BExp::One => alloc.text("1"),
            BExp::PBool(x) => alloc.text(format!("bool({})", &x[1..])),
            BExp::Or(m, n) => {
                let doc = m
                    .formatted(alloc)
                    .append(alloc.softline())
                    .append(alloc.text("||"))
                    .append(alloc.softline())
                    .append(n.formatted(alloc));
                doc.parens()
            }
            BExp::And(m, n) => {
                let doc = m
                    .formatted(alloc)
                    .append(alloc.softline())
                    .append(alloc.text("&&"))
                    .append(alloc.softline())
                    .append(n.formatted(alloc));
                doc.parens()
            }
            BExp::Not(m) => alloc.text("!").append(m.formatted(alloc).parens()),
        }
    }
}

#[derive(Clone)]
pub enum Exp {
    Act(String),
    Seq(Box<Exp>, Box<Exp>),
    Ifte(BExp, Box<Exp>, Box<Exp>),
    Test(BExp),
    While(BExp, Box<Exp>),
}

impl Exp {
    pub fn act(s: &str) -> Self {
        Self::Act(s.to_string())
    }

    pub fn seq(&self, other: &Self) -> Self {
        Self::Seq(Box::new(self.clone()), Box::new(other.clone()))
    }

    pub fn skip() -> Self {
        Self::Test(BExp::One)
    }

    pub fn fail() -> Self {
        Self::Test(BExp::Zero)
    }

    pub fn sexpr<'b, D>(&'b self, alloc: &'b D) -> DocBuilder<'b, D>
    where
        D: DocAllocator<'b>,
        D::Doc: Clone,
    {
        match self {
            Exp::Act(x) => alloc.text(x),
            Exp::Seq(_, _) => {
                let mut ms = vec![];
                let mut curr = self;
                while let Exp::Seq(m, n) = curr {
                    ms.push(m.sexpr(alloc));
                    curr = n;
                }
                ms.push(curr.sexpr(alloc));
                let body = alloc.intersperse(ms, alloc.softline());
                alloc
                    .text("(seq")
                    .append(alloc.softline())
                    .append(body)
                    .append(")")
            }
            Exp::Ifte(b, m, n) => {
                let b = b.sexpr(alloc);
                let m = m.sexpr(alloc);
                let n = n.sexpr(alloc);
                alloc
                    .text("(if")
                    .append(alloc.softline())
                    .append(b)
                    .append(alloc.line())
                    .append(m.indent(2))
                    .append(alloc.line())
                    .append(n.indent(2))
                    .append(")")
            }
            Exp::Test(b) => alloc
                .text("(test")
                .append(alloc.softline())
                .append(b.sexpr(alloc))
                .append(")"),
            Exp::While(b, m) => alloc
                .text("(while")
                .append(alloc.softline())
                .append(b.sexpr(alloc))
                .append(alloc.line())
                .append(m.sexpr(alloc))
                .append(alloc.text(")")),
        }
    }

    pub fn formatted<'b, D>(&'b self, alloc: &'b D) -> DocBuilder<'b, D>
    where
        D: DocAllocator<'b>,
        D::Doc: Clone,
    {
        match self {
            Exp::Act(a) => alloc.text(format!("action({});", &a[1..])),
            Exp::Seq(_, _) => {
                let mut ms = vec![];
                let mut curr = self;
                while let Exp::Seq(m, n) = curr {
                    ms.push(m.formatted(alloc));
                    curr = n;
                }
                ms.push(curr.formatted(alloc));
                alloc
                    .text("{")
                    .append(alloc.line())
                    .append(alloc.intersperse(ms, alloc.line()).indent(4))
                    .append(alloc.line())
                    .append("}")
            }
            Exp::Test(b) => alloc
                .text("assert")
                .append(b.formatted(alloc).parens())
                .append(";"),
            Exp::Ifte(b, m, n) => {
                let b = b.formatted(alloc);
                let m = alloc
                    .text("{")
                    .append(alloc.line())
                    .append(m.formatted(alloc).indent(4))
                    .append(alloc.line())
                    .append("}");
                let n = alloc
                    .line()
                    .append("else")
                    .append(alloc.softline())
                    .append("{")
                    .append(alloc.line())
                    .append(n.formatted(alloc).indent(4))
                    .append(alloc.line())
                    .append("}");
                alloc
                    .text("if")
                    .append(alloc.softline())
                    .append(b.parens())
                    .append(alloc.softline())
                    .append(m)
                    .append(n)
            }
            Exp::While(b, m) => {
                let b = b.formatted(alloc);
                let m = alloc
                    .text("{")
                    .append(alloc.line())
                    .append(m.formatted(alloc).indent(4))
                    .append(alloc.line())
                    .append("}");
                alloc
                    .text("while")
                    .append(alloc.softline())
                    .append(b.parens())
                    .append(alloc.softline())
                    .append(m)
            }
        }
    }
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub body: Exp,
}

impl Function {
    pub fn formatted<'b, D>(&'b self, alloc: &'b D) -> DocBuilder<'b, D>
    where
        D: DocAllocator<'b>,
        D::Doc: Clone,
    {
        let header = alloc
            .nil()
            .append("extern void assert(int);")
            .append(alloc.line())
            .append("extern int bool(int);")
            .append(alloc.line())
            .append("extern void action(int);");
        let body = self.body.formatted(alloc);
        header.append(alloc.line()).append(alloc.line()).append(
            alloc
                .text("void")
                .append(alloc.softline())
                .append(&self.name)
                .append(alloc.nil().parens())
                .append(alloc.softline())
                .append("{")
                .append(alloc.line())
                .append(body.indent(4))
                .append(alloc.line())
                .append("}"),
        )
    }
}
