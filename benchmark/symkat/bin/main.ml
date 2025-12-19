(*******************************************************************)
(*  This is part of SymbolicKAT, it is distributed under the       *)
(*  terms of the GNU Lesser General Public License version 3       *)
(*           (see file LICENSE for more details)                   *)
(*                                                                 *)
(*  Copyright 2014: Damien Pous. (CNRS, LIP - ENS Lyon, UMR 5668)  *)
(*******************************************************************)

open Sedlexing
open Common
open Automata
open Lib_symkat
open Ppx_yojson_conv_lib.Yojson_conv.Primitives

(* generic symbolic algorithm*)
module Symb = Safa.Make(Queues.BFS) 

(* quiet or not, stats or not *)
let quiet = ref !Sys.interactive
let stats = ref false

(* recording traces or not *)
let trace = ref false

(* do we use Hopcroft and Karp's version? *)
let hk = ref false

(* do we put expressions in strict star form first? *)
let ssf = ref false

(* which construction to use *)
let construction = ref `Antimirov
let set x () = construction := x

(* do we use up-to congruence *)
let congruence = ref false

(* hypotheses to exploit *)
let hypotheses = ref ""

let hexprs hyps x y = 
  let x,y,z = Hypotheses.eliminate hyps x y in
  if !ssf then Kat.ssf x, Kat.ssf y, Kat.ssf z else x,y,z

let hexpr x = let x,_,_ = hexprs [] x (Kat.Tst Kat.Bot) in x

let equiv_ e a exclude x y =
  let tracer = if !trace then (
    Trace.clear();
    let pp = SDFA.trace ~exclude
      Format.pp_print_int Format.pp_print_int (Bdd.print_formula 0) a
    in
    pp y; pp x;
    Some (fun k x y ->
      let x = Bdd.tag (Bdd.constant a.SDFA.m x) in
      let y = Bdd.tag (Bdd.constant a.SDFA.m y) in
      match k with
	| `CE -> Trace.ce x y
	| `OK -> Trace.ok x y
	| `Skip -> Trace.skip x y
    ) 
  ) else None 
  in
  match 
    if !hk then e ?tracer (Bdd.unify_dsf !trace) a x y 
    else e ?tracer (Bdd.unify_naive !trace) a x y 
  with
    | None -> None
    | Some (x,y,w) -> 
      let b,c = a.SDFA.o x, a.SDFA.o y in
      Some (List.rev (Bdd.witness (Bdd.xor b c)), w)

(* Antimirov' construction *)
let antimirov e x y =
  let a,f = SNFA.reindex (Antimirov.nfa()) in
  equiv_ e (Determinisation.optimised a) IntSet.empty
    (f (Antimirov.split x)) (f (Antimirov.split y))

(* Brzozowski's construction *)
let brzozowski e x y = 
  equiv_ e (Brzozowski.dfa()) Kat.zer x y

(* Ilie and Yu's construction *)
let ilie_yu e x y =
  let a,x,y = IlieYu.enfa x y in
  let a = Determinisation.optimised (Epsilon.remove a) in
  equiv_ e a IntSet.empty (IntSet.singleton x) (IntSet.singleton y)

(* equivance check *)
let equiv x y = 
  match !construction with
    | `Antimirov -> antimirov (if !congruence then Symb.equiv_c else Symb.equiv) x y
    | `IlieYu -> ilie_yu (if !congruence then Symb.equiv_c else Symb.equiv) x y
    | `Brzozowski -> 
      if !congruence then failwith "up-to congruence cannot be used with Brzozowski's automaton"
      else brzozowski Symb.equiv x y

(* full comparison *)
let compare ?(hyps=[]) x y = 
  Stats.reset(); 
  let x',y',z' = hexprs hyps x y in
  (x',y'),
  match equiv x' y' with
    | None -> `E
    | Some w -> Stats.reset(); Trace.save(); match equiv z' y' with
	| None -> `L w
	| Some w1 -> Stats.reset(); match equiv z' x' with
	    | None -> `G w
	    | Some w2 -> Trace.restore(); `D (w1,w2)

(* exported equivance check *)
let equiv ?(hyps=[]) x y = 
  let x',y',_ = hexprs hyps x y in
  equiv x' y'

exception Timeout

let run_with_memory_limit limit f =
  let limit_memory () =
    let mem = Gc.(quick_stat ()).heap_words in
    if mem > limit / (Sys.word_size / 8) then raise Out_of_memory
  in
  let alarm = Gc.create_alarm limit_memory in
  Fun.protect f ~finally:(fun () ->
      Gc.delete_alarm alarm;
      Gc.compact ())

let delayed_fun f x timeout =
  let _ =
    Sys.set_signal Sys.sigalrm (Sys.Signal_handle (fun _ -> raise Timeout))
  in
  ignore (Unix.alarm timeout);
  try
    let r = f x in
    ignore (Unix.alarm 0);
    r
  with e ->
    ignore (Unix.alarm 0);
    raise e

type equiv_result =
  | Done of bool
  | Unsupported
  | Failed
  | Timeout
  | OutOfMemory
[@@deriving yojson]

(* entry point, for standalone program *)
let main =
  let src_opt = ref None in
  let specs = [] in
  Printexc.record_backtrace true;
  let anon str = src_opt := Some str in
  Arg.parse specs anon "";
  match !src_opt with
  | Some src_file ->
    let src_ch = open_in src_file in
    let m, n, b = Parse.parse (Utf8.from_channel src_ch) in
    let result = 
      try
        delayed_fun
          (fun () ->
            run_with_memory_limit
              (4 * 1024 * 1024 * 1024)
              (fun () ->
                try
                  let b = match equiv m n with
                    | None -> true
                    | Some _ -> false
                  in
                  Done b
                with _ -> Failed))
          () 120
      with
      | Timeout -> Timeout
      | Out_of_memory -> OutOfMemory
    in
    print_endline
      Yojson.Safe.(to_string @@ yojson_of_equiv_result result)
  | None -> Format.eprintf "input expected@."
