------------------------------- MODULE GridSystem -------------------------------

(* 
A TLA+ model for:

grids: identified by their position (whcih never changes)

- each grid can be either stored, or missing

- each grid can be either loaded, or not loaded

  - each loaded Grid can be either dirty, unmodified, or pristine



players: identified by their id (never changes)

- each player has a current position (same type as grid position)


initial state:

- all grids are missing, and not loaded

actions:

- player moves to new position, and there is a loaded grid at that position

- a missing grid is loaded, and marked pristine

- a stored grid is loaded, and marked unmodified

- a loaded grid which is pristine or unmodified is marked dirty

- a loaded grid which is dirty or unmodified is stored

- a loaded grid which is pristine or unmodified is set "not loaded"

- a loaded grid which is dirty is stored and set unmodified

*)
EXTENDS Naturals

(* -- Constants -- *)
CONSTANTS GridPos, (* Set of all possible grid positions *)
          PlayerId  (* Set of all possible player identifiers *)

(* -- Variables -- *)
VARIABLES grid_store_status,  (* Function: GridPos -> {"stored", "not_stored"} *)
          grid_load_status,   (* Function: GridPos -> {"loaded", "not_loaded"} *)
          grid_dirty_status,  (* Function: GridPos -> {"dirty", "unmodified", "pristine"} *)
          player_positions    (* Function: PlayerId -> GridPos *)

(* -- Type Invariant -- *)
TypeOK ==
    /\ (grid_store_status  \in [GridPos -> {"stored", "not_stored"}])
    /\ (grid_load_status   \in [GridPos -> {"loaded", "not_loaded"}])
    /\ (grid_dirty_status  \in [GridPos -> {"dirty", "unmodified", "pristine", "unknown"}])
    /\ (player_positions   \in [PlayerId -> GridPos])

(* -- Invariant -- *)
(* Invariant: If a grid is loaded its dirty_status can't be "unknown" *)
KnownDirtyStatus == 
    \A pos \in GridPos :
        (grid_load_status[pos] = "loaded") => (grid_dirty_status[pos] \in {"dirty", "unmodified", "pristine"})
  

(* -- Initial State -- *)
Init ==
    /\ grid_store_status = [g_pos \in GridPos |->"not_stored" ]
    /\ grid_load_status = [g_pos \in GridPos |-> "not_loaded"]
    /\ grid_dirty_status = [g_pos \in GridPos |-> "unknown"]
    /\ player_positions = [p_id \in PlayerId |-> CHOOSE pos \in GridPos: TRUE]

(* -- Actions -- *)

(* Player moves to new position, and there is a loaded grid at that position *)
PlayerMoves(player_id, new_pos) ==
    /\ player_id \in PlayerId
    /\ new_pos \in GridPos
    /\ grid_load_status[new_pos] = "loaded"
    /\ player_positions' = [player_positions EXCEPT ![player_id] = new_pos]
    /\ UNCHANGED <<grid_store_status, grid_load_status, grid_dirty_status>>

(* A missing grid is loaded and marked pristine *)
LoadMissingGrid(pos) ==
    /\ pos \in GridPos
    /\ grid_store_status[pos] = "not_stored"
    /\ grid_load_status[pos] = "not_loaded"
    /\ grid_load_status' = [grid_load_status EXCEPT ![pos] = "loaded"]
    /\ grid_dirty_status' = [grid_dirty_status EXCEPT ![pos] = "pristine"]
    /\ UNCHANGED <<grid_store_status, player_positions>>

(* A stored grid is loaded and marked unmodified *)
LoadStoredGrid(pos) ==
    /\ pos \in GridPos
    /\ grid_store_status[pos] = "stored"
    /\ grid_load_status[pos] = "not_loaded"
    /\ grid_load_status' = [grid_load_status EXCEPT ![pos] = "loaded"]
    /\ grid_dirty_status' = [grid_dirty_status EXCEPT ![pos] = "unmodified"]
    /\ UNCHANGED <<grid_store_status, player_positions>>

(* A loaded grid which is pristine or unmodified is marked dirty *)
MarkDirty(pos) ==
    /\ pos \in GridPos
    /\ grid_load_status[pos] = "loaded"
    /\ (grid_dirty_status[pos] = "pristine" \/ grid_dirty_status[pos] = "unmodified") (* Internal disjunction kept for clarity *)
    /\ grid_dirty_status' = [grid_dirty_status EXCEPT ![pos] = "dirty"]
    /\ UNCHANGED <<grid_store_status, grid_load_status, player_positions>>

(* A loaded grid which is dirty or unmodified is stored (persistently) *)
StoreGrid(pos) ==
    /\ pos \in GridPos
    /\ grid_load_status[pos] = "loaded"
    /\ grid_dirty_status[pos] \in {"dirty", "unmodified"}
    /\ grid_store_status' = [grid_store_status EXCEPT ![pos] = "stored"]
    /\ grid_dirty_status' = [grid_dirty_status EXCEPT ![pos] = "unmodified"]
    /\ UNCHANGED <<grid_load_status, player_positions>>

(* A loaded grid which is pristine or unmodified is set "not loaded" (unloaded from memory) *)
UnloadPristineUnmodifiedGrid(pos) ==
    /\ pos \in GridPos
    /\ grid_load_status[pos] = "loaded"
    /\ grid_dirty_status[pos] \in {"pristine", "unmodified"}
    /\ grid_load_status' = [grid_load_status EXCEPT ![pos] = "not_loaded"]
    /\ UNCHANGED <<grid_store_status, grid_dirty_status, player_positions>>

(* A loaded grid which is dirty is stored (persistently) and set unmodified (after storing) *)
StoreAndUnloadDirtyGrid(pos) ==
    /\ pos \in GridPos
    /\ grid_load_status[pos] = "loaded"
    /\ grid_dirty_status[pos] = "dirty"
    /\ grid_store_status' = [grid_store_status EXCEPT ![pos] = "stored"]
    /\ grid_dirty_status' = [grid_dirty_status EXCEPT ![pos] = "unmodified"]
    /\ UNCHANGED <<grid_load_status, player_positions>>

(* -- Next State -- *)
Next ==
    \/ \E p_id \in PlayerId, new_pos \in GridPos : PlayerMoves(p_id, new_pos)
    \/ \E pos \in GridPos : LoadMissingGrid(pos)
    \/ \E pos \in GridPos : LoadStoredGrid(pos)
    \/ \E pos \in GridPos : MarkDirty(pos)
    \/ \E pos \in GridPos : StoreGrid(pos)
    \/ \E pos \in GridPos : UnloadPristineUnmodifiedGrid(pos)
    \/ \E pos \in GridPos : StoreAndUnloadDirtyGrid(pos)

(* -- Fairness (Optional, but good practice for liveness properties) -- *)
(* To ensure all enabled actions eventually happen, add liveness properties
   such as weak fairness (WF_vars(Next)) or strong fairness (SF_vars(Next))
   to the Spec. For this model, no specific liveness properties were requested.
*)

====