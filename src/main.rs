mod kernel1;
mod kernel2;
mod parsing;
mod syntax;

use clap::{Parser, ValueEnum};
use mimalloc::MiMalloc;
use parsing::*;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use syntax::*;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Kernel {
    K1, // Symbolic derivative method
    K2, // Symbolic Thompson's construction
    Formatter,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Solver {
    BDD, // CUDD
    SAT, // MiniSat2 via LogicNG
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Kernel::K1)]
    kernel: Kernel,
    #[arg(short, long, value_enum, default_value_t = Solver::BDD)]
    solver: Solver,
    input1: String,
    input2: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input_file = args.input1;
    let file = fs::read_to_string(&input_file).expect("cannot read file");
    let (exp1, exp2, b) = parse(file);
    match args.kernel {
        Kernel::K1 => match args.solver {
            Solver::BDD => {
                let mut gkat = BDDGkat::new();
                let mut solver = kernel1::Solver::new();
                let exp1 = gkat.from_exp(exp1);
                let exp2 = gkat.from_exp(exp2);
                let result = solver.equiv_iter(&mut gkat, &exp1, &exp2);
                println!("equiv_expected = {}", b);
                println!("equiv_result   = {}", result);
                assert!(b == result);
            }
            Solver::SAT => {
                let mut gkat = SATGkat::new();
                let mut solver = kernel1::Solver::new();
                let exp1 = gkat.from_exp(exp1);
                let exp2 = gkat.from_exp(exp2);
                let result = solver.equiv_iter(&mut gkat, &exp1, &exp2);
                println!("equiv_expected = {}", b);
                println!("equiv_result   = {}", result);
                assert!(b == result);
            }
        },
        Kernel::K2 => match args.solver {
            Solver::BDD => {
                let mut gkat = BDDGkat::new();
                let mut solver = kernel2::Solver::new();
                let exp1 = gkat.from_exp(exp1);
                let exp2 = gkat.from_exp(exp2);
                let (i, m) = solver.mk_automaton(&mut gkat, &exp1);
                let (j, n) = solver.mk_automaton(&mut gkat, &exp2);
                let result = solver.equiv_iter(&mut gkat, i, j, &m, &n);
                println!("equiv_expected = {}", b);
                println!("equiv_result   = {}", result);
                assert!(b == result);
            }
            Solver::SAT => {
                let mut gkat = SATGkat::new();
                let mut solver = kernel2::Solver::new();
                let exp1 = gkat.from_exp(exp1);
                let exp2 = gkat.from_exp(exp2);
                let (i, m) = solver.mk_automaton(&mut gkat, &exp1);
                let (j, n) = solver.mk_automaton(&mut gkat, &exp2);
                let result = solver.equiv_iter(&mut gkat, i, j, &m, &n);
                println!("equiv_expected = {}", b);
                println!("equiv_result   = {}", result);
                assert!(b == result);
            }
        },
        Kernel::Formatter => {
            let output_dir = args.input2.expect("expected input2");
            let stem = Path::new(&input_file)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let file_name1 = format!("{}@1.c", stem);
            let file_name2 = format!("{}@2.c", stem);
            let mut file1 =
                File::create(Path::new(&output_dir).join(Path::new(&file_name1))).unwrap();
            let mut file2 =
                File::create(Path::new(&output_dir).join(Path::new(&file_name2))).unwrap();
            let f1 = Function {
                name: stem.to_string(),
                body: exp1,
            };
            let f2 = Function {
                name: stem.to_string(),
                body: exp2,
            };
            write!(file1, "{:?}", f1).unwrap();
            write!(file2, "{:?}", f2).unwrap();
        }
    };
}
