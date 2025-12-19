(*******************************************************************)
(*      This is part of SAFA, it is distributed under the          *)
(*  terms of the GNU Lesser General Public License version 3       *)
(*           (see file LICENSE for more details)                   *)
(*                                                                 *)
(*  Copyright 2014: Damien Pous. (CNRS, LIP - ENS Lyon, UMR 5668)  *)
(*******************************************************************)

open Common
open Automata

(* polymorphic, generic, determinisation *)
let generic a =
  let m = a.SNFA.m in
  let z = Span.empty (Bdd.constant m Set.empty) in
  let t x = 
    Set.fold (fun i acc -> Span.merge (Bdd.binary m Set.union) (a.SNFA.t i) acc) x z in
  let o x = Set.fold (fun i acc -> a.SNFA.o2 (a.SNFA.o i) acc) x a.SNFA.o0 in
  SDFA.{ m; t; o;
	 output_check=generic_output_check o;
	 state_info=Set.print a.SNFA.state_info }

(* specialised, more efficient determinisation *)
let optimised a =
  let m = a.SNFA.m in
  let t = a.SNFA.t in
  let o = a.SNFA.o in
  let z = Span.empty (Bdd.constant m IntSet.empty) in
  let t x =
    IntSet.fold (fun i acc -> Span.merge (Bdd.binary m IntSet.union) (t i) acc) x z in
  let o x = IntSet.fold (fun i acc -> a.SNFA.o2 (o i) acc) x a.SNFA.o0 in
  SDFA.{ m; t; o;
	 output_check=generic_output_check o;
	 state_info=IntSet.print' a.SNFA.state_info }
