# Physics Engine Documentation

This document summarizes the custom physics engine implemented in this repository, detailing how it addresses classic simulation problems like tunneling, stability, and performance.

## 1. Integration & Time-stepping
The engine uses **Semi-implicit Euler integration** with a high-frequency **substepping** approach to ensure simulation accuracy.
*   **Substepping:** The `Universe::tick` loop executes 100 substeps per frame (with a `dt` of 0.01). This effectively solves the **tunneling problem**, preventing fast-moving objects from skipping over each other between frames.
*   **Deterministic Loop:** In each substep, forces are calculated, velocities are updated, collisions are resolved, and finally, positions are updated.
*   **Reference:** `src/universe.rs`

## 2. Collision Detection (Spatial Partitioning)
To optimize performance and avoid $O(n^2)$ complexity, the engine utilizes a **Grid-based Broad Phase**.
*   **Hierarchical Grids:** The world is partitioned using a `MultiGrid` system that manages `UniverseGrid` instances, which in turn contain individual `GridCell`s.
*   **Neighbor Tracking:** Each `GridCell` pre-calculates and stores references to items in its 8 surrounding cells. This allows for $O(1)$ local proximity queries, as an object only needs to check its cell's `neighbors` list to find potential colliders.
*   **Infinite World Support:** Grids are dynamically loaded and dropped based on the player's position, keeping the active memory footprint low.
*   **Reference:** `src/grid.rs`, `src/multigrid.rs`

## 3. Collision Resolution & Stability
The engine employs an **Impulse-based Narrow Phase** with several stabilization techniques to maintain physical integrity.
*   **Impulse Method:** Collisions are resolved by calculating a scalar impulse based on relative velocity, mass (`koko`), and `elasticity`. This handles momentum transfer between dynamic objects and static boundaries.
*   **Baumgarte Stabilization:** To prevent objects from "sinking" into each other due to numerical drift, the engine applies **Positional Correction**. It moves overlapping objects apart by a fraction of their penetration depth, using a "slop" factor to prevent jitter.
*   **Velocity Clamping:** All velocities are strictly clamped to a maximum range (`[-1.0, 1.0]`) to prevent "explosions" or extreme instability.
*   **Collision Dampening:** The engine tracks `collision_stats`. If an object undergoes more than 1000 collisions in a single tick, its velocity is zeroed to stabilize the cluster.
*   **Reference:** `src/inertia.rs`, `src/v2.rs`

## 4. Handling "Resting Contact" (Sleeping)
To further optimize performance, the engine implements a "sleeping" mechanism for stationary objects.
*   **Static Transition:** Objects with velocity below a certain threshold are marked as **static** (`koko = 0`).
*   **Activity Filtering:** Static objects are removed from the `moving_cells` list, meaning they are ignored during force calculations and position updates until they are "unstuck" by an external event.
*   **Reference:** `src/universe.rs`, `src/inertia.rs`

## 5. Summary Table
| Problem | Solution | Primary Files |
| :--- | :--- | :--- |
| **Tunneling** | 100x Substepping | `universe.rs` |
| **Stability** | Baumgarte Stabilization & Velocity Clamping | `inertia.rs`, `v2.rs` |
| **Performance** | Grid-based Spatial Partitioning | `grid.rs`, `multigrid.rs` |
| **Energy Gain** | Collision count dampening | `universe.rs`, `inertia.rs` |
| **Static Objects**| Mass-zeroing (`koko = 0`) & Sleeping | `universe.rs`, `inertia.rs` |
