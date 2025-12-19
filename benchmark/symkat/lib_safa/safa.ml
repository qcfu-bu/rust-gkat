(*******************************************************************)
(*      This is part of SAFA, it is distributed under the          *)
(*  terms of the GNU Lesser General Public License version 3       *)
(*           (see file LICENSE for more details)                   *)
(*                                                                 *)
(*  Copyright 2014: Damien Pous. (CNRS, LIP - ENS Lyon, UMR 5668)  *)
(*******************************************************************)

open Common
open Automata
open SDFA

module Make(Q: QUEUE) = struct
  let iterations = Stats.counter "iterations" 
  let equiv ?(tracer=fun _ _ _ -> ()) unify a x y =
    let todo = Q.empty () in
    let rec loop () = 
      match Q.pop todo with
	| None -> None
	| Some (w,x,y) ->
	  Stats.incr iterations;
          if a.output_check x y then (
	    tracer `OK x y;
	    Span.iter2 (fun i -> 
	      unify (fun at x y -> 
		Q.push todo ((i,at)::w,x,y))) 
	      (a.t x) (a.t y);
  	    loop ()
	  ) else ( 
	    tracer `CE x y;
	    Some (x, y, w)
	  )
    in 
    unify (fun _ x y -> Q.push todo ([],x,y)) (Bdd.constant a.m x) (Bdd.constant a.m y);
    loop ()

  module Congruence = Congruence.Make(Q)
  let equiv_c ?(tracer=fun _ _ _ -> ()) unify a	x y =
    let todo = Q.empty () in
    let congruence = Congruence.empty() in
    let rec loop () = 
      match Q.pop todo with
	| None -> None
	| Some (w,x,y) ->
	  Stats.incr iterations;
          if a.output_check x y then 
	    if congruence x y todo then (
	      tracer `Skip x y; 
	      loop()
	    ) else ( 
	      tracer `OK x y; 
	      Span.iter2 (fun i -> 
		unify (fun at x y -> 
		  Q.push todo ((i,at)::w,x,y))) 
		(a.t x) (a.t y);
  	      loop ()
	    ) else (
	    tracer `CE x y;
	    Some (x, y, w)
	  )
    in 
    unify (fun _ x y -> Q.push todo ([],x,y)) (Bdd.constant a.m x) (Bdd.constant a.m y);
    loop ()
end
