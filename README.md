# Symbolic GKAT Equivalence 
This repository implements symbolic
[GKAT](https://dl.acm.org/doi/10.1145/3371129) equivalence algorithms.

## Building
Requires the `cargo` build tool for the [Rust](https://www.rust-lang.org/)
programming language.

To build the equivalence checker:
``` sh
cargo build --release
```
The resulting executable can be found at `target/release/rust-gkat`.

## Usage
`rust-gkat` offers 2 equivalence checking kernels backed by 2 satisfiability solver backends.

- kernel `k1`: symbolic derivative method (default)
``` sh
rust-gkat -k k1 ./input/test00.txt
```

- kernel `k2`: symbolic thompson's construction
``` sh
rust-gkat -k k2 ./input/test00.txt
```

- solver `bdd`: use Binary Decision Diagrams (CUDD) for boolean satisfiability checking (default)
``` sh
rust-gkat -s bdd ./input/test00.txt
```

- solver `sat`: use a SAT solver (MiniSat2) for boolean satisfiability checking
``` sh
rust-gkat -s sat ./input/test00.txt
```

Kernels and solvers can be mixed freely.

## Input Format
Each input file consists of 3 s-expressions. The first 2 s-expressions are the
GKAT expressions for equivalence testing. The final `(equiv ...)` marks whether
these 2 expressions are expected to be equivalent or not.

```
<const> ::= 0 | 1

<bexp> ::= <const> | identifier
         | ( and <bexp> {<bexp>}+ )
         | ( or  <bexp> {<bexp>}+ )
         | ( not <bexp> )

 <exp> ::= identifier 
         | ( test <bexp> )
         | ( seq <exp> {<exp>}+ )
         | ( if <bexp> <exp> <exp> )
         | ( while <bexp> <exp> )

<format> ::= <exp> <exp> ( equiv <const> )
```

For n-ary syntax such as `(and A B C)`, it is parsed right-associatively into
binary form as `(and A (and B C))`.

Sample from `input/test10.txt`:
```
(seq
  (seq
    (seq
      (seq (seq p7 p10) p9) (if b66 (if (or b79 b78 b68) p8 p1) (seq p6 p1)))
    (seq (if (not b25) p5 p22) p3 p0 p7) (seq p6 p2)
    (if (and b60 b18 b99) (seq p3 p98) p6))
  (seq (seq p9 p8) p2) (seq p25 p5)
  (if (and (or b82 b42) (or b42 b82)) (if (and b52 b75) p6 p1) p20))

(seq
  (seq
    (seq
      (seq (seq p7 p10) p9)
      (if b66 (if (or (or b79 b78) b68) p8 p1) (seq p6 p1)))
    (seq (seq (if (not b25) p5 p22) (seq p3 p0) p7) p6 p2)
    (if (and (and b60 b18) b99) (seq p3 p98) p6))
  (seq (seq p9 p8) p2) (seq p25 p5)
  (if (and (and b52 b75) (or b82 b42)) p6
    (if (and (or b82 b42) (or b42 b82)) p1 p20)))

(equiv 1)
```

## Performance and Evaluation
### Benchmarking rust-gkat
We provide a set of benchmarks for evaluating the performance of `rust-gkat`.
These benchmarks follow a simple naming scheme describing the expression pairs
inside. For example, the benchmark `e250b5p10eq` contains expressions which have
approximately 250 primitive actions (`e250`), a maximum boolean expression size
of 5 (`b5`), 10 possible boolean variables (`p10`) and are known to be
equivalent (`eq`). Benchmarks with the suffix `ne` have expression pairs which
are known to be non-equivalent. 

One use the following command to run `rust-gkat` on a particular dataset:
``` sh
make -f rsgkat.make [dataset] kernel=[k1|k2] solver=[bdd|sat]
```
For example, `make e250b5p10eq kernel=k1 solver=bdd`
runs `rust-gkat` on all expression pairs contained in dataset `e250b5p10eq`
using kernel `k1` and solver `bdd`.

### Benchmarking SymKAT
We benchmark against a patched version of [SymKAT](https://perso.ens-lyon.fr/damien.pous/symbolickat/) (sk) that allows for checking larger expressions.
The source code for this modified version can be found in the `benchmark/symkat` directory.

To build SymKAT, navigate to the `benchmark/symkat` directory and run:
``` sh
opam switch create . 5.0.0   # Create a local opam switch
eval $(opam env)             # Set environment variables for the local switch
dune build --profile release # Build SymKAT in release mode
```

To run SymKAT on a particular dataset, run the following command from the project root:
``` sh
make -f symkat.make [dataset]
```

### Results
We evaluate the performance of `rust-gkat` in terms of time and memory usage. We
also compare `rust-gkat` with a patched [SymKAT](https://perso.ens-lyon.fr/damien.pous/symbolickat/) (sk). 
The following table lists the total time and peak memory used for each benchmark.

#### Benchmark Total Time Usage (seconds)
| Benchmark      | Time (k1-bdd) | Time (k2-bdd) | Time (k1-sat) | Time (k2-sat) | Time (sk) |
| -------------- | ------------- | ------------- | ------------- | ------------- | --------- |
| e250b5p10ne    | 0.19          | 0.20          | 0.17          | 0.17          | 5.80      |
| e250b5p10eq    | 0.20          | 0.19          | 0.18          | 0.20          | 3.21      |
| e500b5p50ne    | 0.22          | 0.22          | 0.22          | 0.23          | 37.66     |
| e500b5p50eq    | 0.23          | 0.22          | 0.25          | 0.29          | 14.21     |
| e1000b10p100ne | 0.26          | 0.29          | 0.34          | 0.40          | timeout   |
| e1000b10p100eq | 0.31          | 0.30          | 0.39          | 0.49          | 102.38    |
| e2000b20p200ne | 1.35          | 2.59          | 0.69          | 0.82          | timeout   |
| e2000b20p200eq | 0.81          | 0.98          | 0.67          | 0.89          | timeout   |
| e3000b30p200ne | 1.52          | 26.45         | 1.30          | 1.62          | timeout   |
| e3000b30p200eq | 9.61          | 28.75         | 1.15          | 1.69          | timeout   |
| degenerate     | 185.62        | 416.42        | 0.48          | 0.63          | timeout   |

#### Benchmark Peak Memory Usage (megabytes)
| Benchmark      | Mem (k1-bdd) | Mem (k2-bdd) | Mem (k1-sat) | Mem (k2-sat) | Mem (sk) |
| -------------- | ------------ | ------------ | ------------ | ------------ | -------- |
| e250b5p10ne    | 15.26        | 14.61        | 6.84         | 6.79         | 114.01   |
| e250b5p10eq    | 15.77        | 14.76        | 7.06         | 7.04         | 113.06   |
| e500b5p50ne    | 15.92        | 15.38        | 7.66         | 7.01         | 524.79   |
| e500b5p50eq    | 17.20        | 15.40        | 7.02         | 7.02         | 371.11   |
| e1000b10p100ne | 17.46        | 21.38        | 7.88         | 7.56         | timeout  |
| e1000b10p100eq | 19.54        | 17.39        | 9.12         | 7.66         | 4809.88  |
| e2000b20p200ne | 237.48       | 284.40       | 13.15        | 11.69        | timeout  |
| e2000b20p200eq | 54.36        | 52.83        | 15.74        | 13.10        | timeout  |
| e3000b30p200ne | 112.39       | 1211.66      | 21.23        | 19.64        | timeout  |
| e3000b30p200eq | 207.17       | 228.57       | 21.25        | 17.48        | timeout  |
| degenerate     | 628.53       | 1212.66      | 19.34        | 19.56        | timeout  |
