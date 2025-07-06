mod kernel1;
mod kernel2;
mod parsing;
mod syntax;
mod testing;

use clap::{Parser, ValueEnum};
use mimalloc::MiMalloc;
use parsing::*;
use pretty::BoxAllocator;
use std::{
    alloc,
    fs::{self, File},
    io::Write,
    path::Path,
};
use syntax::*;

use crate::testing::Generator;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Kernel {
    K1, // Symbolic derivative method
    K2, // Symbolic Thompson's construction
    Formatter,
    Generator,
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
    input3: Option<String>,
    input4: Option<String>,
}

fn main() {
    let args = Args::parse();
    match args.kernel {
        Kernel::K1 => match args.solver {
            Solver::BDD => {
                let input_file = args.input1;
                let file = fs::read_to_string(&input_file).expect("cannot read file");
                let (exp1, exp2, b) = parse(file);
                let mut gkat = BDDGkat::new();
                let mut solver = kernel1::Solver::new();
                let exp1 = gkat.from_exp(&exp1);
                let exp2 = gkat.from_exp(&exp2);
                let result = solver.equiv_iter(&mut gkat, &exp1, &exp2);
                println!("equiv_expected = {}", b);
                println!("equiv_result   = {}", result);
                assert!(b == result);
            }
            Solver::SAT => {
                let input_file = args.input1;
                let file = fs::read_to_string(&input_file).expect("cannot read file");
                let (exp1, exp2, b) = parse(file);
                let mut gkat = SATGkat::new();
                let mut solver = kernel1::Solver::new();
                let exp1 = gkat.from_exp(&exp1);
                let exp2 = gkat.from_exp(&exp2);
                let result = solver.equiv_iter(&mut gkat, &exp1, &exp2);
                println!("equiv_expected = {}", b);
                println!("equiv_result   = {}", result);
                assert!(b == result);
            }
        },
        Kernel::K2 => match args.solver {
            Solver::BDD => {
                let input_file = args.input1;
                let file = fs::read_to_string(&input_file).expect("cannot read file");
                let (exp1, exp2, b) = parse(file);
                let mut gkat = BDDGkat::new();
                let mut solver = kernel2::Solver::new();
                let exp1 = gkat.from_exp(&exp1);
                let exp2 = gkat.from_exp(&exp2);
                let (i, m) = solver.mk_automaton(&mut gkat, &exp1);
                let (j, n) = solver.mk_automaton(&mut gkat, &exp2);
                let result = solver.equiv_iter(&mut gkat, i, j, &m, &n);
                println!("equiv_expected = {}", b);
                println!("equiv_result   = {}", result);
                assert!(b == result);
            }
            Solver::SAT => {
                let input_file = args.input1;
                let file = fs::read_to_string(&input_file).expect("cannot read file");
                let (exp1, exp2, b) = parse(file);
                let mut gkat = SATGkat::new();
                let mut solver = kernel2::Solver::new();
                let exp1 = gkat.from_exp(&exp1);
                let exp2 = gkat.from_exp(&exp2);
                let (i, m) = solver.mk_automaton(&mut gkat, &exp1);
                let (j, n) = solver.mk_automaton(&mut gkat, &exp2);
                let result = solver.equiv_iter(&mut gkat, i, j, &m, &n);
                println!("equiv_expected = {}", b);
                println!("equiv_result   = {}", result);
                assert!(b == result);
            }
        },
        Kernel::Formatter => {
            let input_file = args.input1;
            let file = fs::read_to_string(&input_file).expect("cannot read file");
            let (exp1, exp2, _) = parse(file);
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
            let alloc = BoxAllocator;
            f1.formatted(&alloc).render(90, &mut file1).unwrap();
            f2.formatted(&alloc).render(90, &mut file2).unwrap();
        }
        Kernel::Generator => {
            let exp_max_size = args.input1.parse().unwrap();
            let bexp_max_size: u64 = args.input2.expect("expected input2").parse().unwrap();
            let pbool_max_count: u64 = args.input3.expect("expected input3").parse().unwrap();
            let output_dir = args.input4.expect("expected input2");
            let eq_name = format!("e{}b{}p{}eq", exp_max_size, bexp_max_size, pbool_max_count);
            let ne_name = format!("e{}b{}p{}ne", exp_max_size, bexp_max_size, pbool_max_count);
            let eq_path = Path::new(&output_dir).join(Path::new(&eq_name));
            let ne_path = Path::new(&output_dir).join(Path::new(&ne_name));
            let mut generator = Generator::new(bexp_max_size, exp_max_size, pbool_max_count);
            let alloc = BoxAllocator;
            fs::create_dir(eq_path.clone()).unwrap();
            fs::create_dir(ne_path.clone()).unwrap();
            for i in 0..100 {
                let (m, n) = generator.mk_exp_eq();
                let mut eq_file =
                    File::create(eq_path.join(Path::new(&format!("{eq_name}@{i:02}.txt"))))
                        .unwrap();
                m.sexpr(&alloc).render(90, &mut eq_file).unwrap();
                writeln!(eq_file, "\n").unwrap();
                n.sexpr(&alloc).render(90, &mut eq_file).unwrap();
                writeln!(eq_file, "\n(equiv 1)").unwrap();

                let (m, n) = generator.mk_exp_ne();
                let mut ne_file =
                    File::create(ne_path.join(Path::new(&format!("{ne_name}@{i:02}.txt"))))
                        .unwrap();
                m.sexpr(&alloc).render(90, &mut ne_file).unwrap();
                writeln!(ne_file, "\n").unwrap();
                n.sexpr(&alloc).render(90, &mut ne_file).unwrap();
                writeln!(ne_file, "\n(equiv 0)").unwrap();
            }
        }
    };
}
