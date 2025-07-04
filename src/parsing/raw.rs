use std::fmt::Debug;

use pretty::{BoxAllocator, DocAllocator, DocBuilder};

#[derive(Clone)]
pub enum BExp {
    Zero,
    One,
    PBool(String),
    Or(Box<BExp>, Box<BExp>),
    And(Box<BExp>, Box<BExp>),
    Not(Box<BExp>),
}

impl Debug for BExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alloc = BoxAllocator;
        let doc = self.pretty(&alloc);
        doc.render_fmt(90, f)
    }
}

impl BExp {
    fn pretty<'b, D>(&'b self, alloc: &'b D) -> DocBuilder<'b, D>
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
                    .pretty(alloc)
                    .append(alloc.softline())
                    .append(alloc.text("||"))
                    .append(alloc.softline())
                    .append(n.pretty(alloc));
                doc.parens()
            }
            BExp::And(m, n) => {
                let doc = m
                    .pretty(alloc)
                    .append(alloc.softline())
                    .append(alloc.text("&&"))
                    .append(alloc.softline())
                    .append(n.pretty(alloc));
                doc.parens()
            }
            BExp::Not(m) => alloc.text("!").append(m.pretty(alloc).parens()),
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

impl Debug for Exp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alloc = BoxAllocator;
        let doc = self.pretty(&alloc);
        doc.render_fmt(90, f)
    }
}

impl Exp {
    fn pretty<'b, D>(&'b self, alloc: &'b D) -> DocBuilder<'b, D>
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
                    ms.push(m.pretty(alloc));
                    curr = n;
                }
                ms.push(curr.pretty(alloc));
                alloc
                    .text("{")
                    .append(alloc.line())
                    .append(alloc.intersperse(ms, alloc.line()).indent(4))
                    .append(alloc.line())
                    .append("}")
            }
            Exp::Test(b) => alloc
                .text("assert")
                .append(b.pretty(alloc).parens())
                .append(";"),
            Exp::Ifte(b, m, n) => {
                let b = b.pretty(alloc);
                let m = alloc
                    .text("{")
                    .append(alloc.line())
                    .append(m.pretty(alloc).indent(4))
                    .append(alloc.line())
                    .append("}");
                let n = alloc
                    .line()
                    .append("else")
                    .append(alloc.softline())
                    .append("{")
                    .append(alloc.line())
                    .append(n.pretty(alloc).indent(4))
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
                let b = b.pretty(alloc);
                let m = alloc
                    .text("{")
                    .append(alloc.line())
                    .append(m.pretty(alloc).indent(4))
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

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alloc = BoxAllocator;
        let doc = self.pretty(&alloc);
        doc.render_fmt(90, f)
    }
}

impl Function {
    fn pretty<'b, D>(&'b self, alloc: &'b D) -> DocBuilder<'b, D>
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
        let body = self.body.pretty(alloc);
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
