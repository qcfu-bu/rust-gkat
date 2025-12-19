%token EOF

%token LPAREN  // (
%token RPAREN  // )

%token <string>STR

%token ZERO
%token ONE
%token NOT 
%token OR
%token AND
%token SEQ
%token IF
%token TEST
%token WHILE
%token EQUIV

%{ open Kat %}

%start <expr * expr * bool>main

%%

let parens(P) ==
  | LPAREN; ~ = P; RPAREN; <>

let bexp :=
  | ZERO; { Bot }
  | ONE;  { Top }
  | s = STR; { Prd (Hashtbl.hash s) }
  | LPAREN; NOT; b = bexp; RPAREN; { Neg b } 
  | LPAREN; OR;  b = bexp; bs = bexp+; RPAREN; { 
    let opt =
      List.fold_right 
        (fun b1 opt -> 
          match opt with
          | None -> Some b1
          | Some b2 -> Some (Dsj (b1, b2)))
        (b :: bs) None
    in
    match opt with
    | Some b -> b
    | None -> assert false }
  | LPAREN; AND; b = bexp; bs = bexp+; RPAREN; { 
    let opt =
      List.fold_right 
        (fun b1 opt -> 
          match opt with
          | None -> Some b1
          | Some b2 -> Some (Cnj (b1, b2)))
        (b :: bs) None
    in
    match opt with
    | Some b -> b
    | None -> assert false }

let exp :=
  | s = STR; { Var (Hashtbl.hash s) }
  | LPAREN; SEQ; m = exp; ms = exp+; RPAREN; { 
    let opt =
      List.fold_right 
        (fun b1 opt -> 
          match opt with
          | None -> Some b1
          | Some b2 -> Some (Dot (b1, b2)))
        (m :: ms) None
    in
    match opt with
    | Some b -> b
    | None -> assert false }
  | LPAREN; IF; b = bexp; m = exp; n = exp; RPAREN; { 
      Pls (Dot (Tst b, m), Dot (Tst (Neg b), n))
    }
  | LPAREN; TEST; b = bexp; RPAREN; { Tst b }
  | LPAREN; WHILE; b = bexp; m = exp; RPAREN; { 
      Dot (Str (Dot (Tst b, m)), Tst (Neg b))
    }

let main :=
  | m = exp; n = exp; LPAREN; EQUIV; ZERO; RPAREN; EOF; { (m, n, false) }
  | m = exp; n = exp; LPAREN; EQUIV; ONE;  RPAREN; EOF; { (m, n, true) }