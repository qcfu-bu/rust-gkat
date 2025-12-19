open Sedlexing
open Parser

exception
  LexError of
    { pos_lnum : int
    ; pos_cnum : int
    }

let blank = [%sedlex.regexp? ' ' | '\t' | "\r" | '\n']
let newline = [%sedlex.regexp? '\r' | '\n' | "\r\n"]
let letter = [%sedlex.regexp? 'a' .. 'z' | 'A' .. 'Z']
let digit = [%sedlex.regexp? '0' .. '9']

(* comments *)
let comment0_begin = [%sedlex.regexp? "//"]
let comment0_end = [%sedlex.regexp? newline]
let comment1_begin = [%sedlex.regexp? "/*"]
let comment1_end = [%sedlex.regexp? "*/"]

(* delimiters *)
let lparen = [%sedlex.regexp? '(']
let rparen = [%sedlex.regexp? ')']

(* identifiers *)
let str = [%sedlex.regexp? (letter | '_'), Star (letter | digit | '_' | '\'')]

(* keywords *)

let rec filter buf =
  match%sedlex buf with
  | Plus blank -> filter buf
  | Plus newline -> filter buf
  | Plus comment0_begin ->
    filter0 buf;
    filter buf
  | Plus comment1_begin ->
    filter1 buf;
    filter buf
  | _ -> ()

and filter0 buf =
  match%sedlex buf with
  | Plus comment0_end -> ()
  | any -> filter0 buf
  | _ -> ()

and filter1 buf =
  match%sedlex buf with
  | Plus comment1_end -> ()
  | any ->
    filter buf;
    filter1 buf
  | _ -> ()

let tokenize buf =
  let _ = filter buf in
  match%sedlex buf with
  (* general *)
  | eof -> EOF
  (* demlimiters *)
  | lparen -> LPAREN
  | rparen -> RPAREN (* identifiers *)
  | '0' -> ZERO
  | '1' -> ONE
  | "or" -> OR
  | "and" -> AND
  | "not" -> NOT
  | "seq" -> SEQ
  | "if" -> IF
  | "test" -> TEST
  | "while" -> WHILE
  | "equiv" -> EQUIV
  | str -> STR (Utf8.lexeme buf)
  (* error *)
  | _ ->
    let pos = fst (lexing_positions buf) in
    raise (LexError { pos_lnum = pos.pos_lnum; pos_cnum = pos.pos_cnum })
