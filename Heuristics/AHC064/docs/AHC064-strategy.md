# AHC064: Non-Crossing Railcar Rearrangement Implementation Plan

## Goal

Rearrange 100 railcars distributed across 10 departure tracks into their correct tracks in strict order (cars $10r \dots 10r+9$ in departure track $r$). Minimize the number of turns. Moves between departure and siding tracks must not cross.

## Proposed Strategy

We will use a **Generic Sequential Placement + Greedy Non-Crossing Scheduler** approach.

### 1. Sequential Algorithm (Logical Plan)

We construct a valid sequence of moves ignoring the non-crossing constraints initially, focusing only on capacity and correctness.

- **Turn 1 (Initial Dump)**: Move all 10 cars from each departure track $D_i$ to its corresponding siding track $S_i$. After this, all $D$ tracks are completely empty, and unfinalized cars are exclusively in $S$ tracks.
- **Placing Cars Sequentially**: For each car $c$ from $0$ to $99$:
  - Let its target track be $D_r$ where $r = c / 10$.
  - Find $c$. Since we return unfinalized cars, $c$ will always be in some siding track $S_j$ at some depth $d$.
  - If $d > 0$, we need to temporarily move the $d$ cars above it. We select empty or partially filled departure tracks $D_{buf}$ in a **round-robin fashion** to distribute the buffer load.
  - We issue block moves to shift the $d$ cars to the selected buffers.
  - We move car $c$ directly to $D_r$.
  - We issue block moves to return the $d$ cars from the buffers back to $S_j$. This perfectly preserves their original relative order and keeps unfinalized cars strictly in $S$ tracks.

### 2. Greedy Non-Crossing Scheduler (Parallelization)

To minimize turns, we will compress the purely sequential list of moves into valid parallel turns:

- We track the earliest available time `ready_time` for each track.
- For each sequential move, we find the earliest turn `t >= max(ready_D, ready_S)` where the move does not share a track with, and does not cross, any other move already scheduled in turn `t`.
- We assign the move to turn `t` and update the `ready_time` for its tracks.
- **Why this works**: By respecting the `ready_time` constraints, the scheduler guarantees that all operations on any specific track happen in the *exact same order* as the logical sequential plan. This ensures all capacity invariants ($D \le 15$, $S \le 20$) are strictly maintained, while safely overlapping operations on disjoint tracks to drastically reduce the turn count.

## User Review Required
>
> [!IMPORTANT]
>
> - Does this block-moving and scheduling approach align with the problem constraints?
> - Let me know if you approve this plan or have any specific competitive programming optimizations in mind before I write the code in `AHC064-antigravity-impl.rs`.
>
---

# AHC064 (Version1)

The new approach features:

Lazy Buffering: We no longer force cars to return to their original tracks. We just distribute them to the tracks with the most capacity. This cuts $\sim 33%$ of our physical moves.
Greedy State-Space Generator: Instead of processing cars $0 \dots 99$ linearly, we examine all 10 tracks simultaneously and always extract the most "exposed" needed car first. This naturally minimizes digging depth
---

# AHC064 (Version2)

Magic Extraction: I discovered a mathematical property in the game's block moving mechanics. Moving a block of cars $D \to S$ actually reverses their accessibility order! We can use this to extract buried cars from $D$ tracks in just 2 moves (down from 3), matching the speed of extracting from $S$ tracks.
Distance-Aware Buffer Selection: We will score buffer tracks based on their physical distance $|i - j|$ from the source track. This encourages diagonal, non-crossing moves and mathematically minimizes the number of crossing conflicts our Greedy Scheduler encounters, allowing it to pack even more parallel actions per turn.
Multi-Car Placements: We will peek ahead in the target sequences. Because blocks maintain and reverse order, we frequently end up grouping cars in sequence perfectly by accident. When this happens, we can place multiple finalized cars at once with $k \ge 1$.

---

# AHC064 (Version3)

Algorithm Choice: Greedy State-Space Search with Lookahead Scheduler Simulation.
State Representation: The game state is explicitly tracked (`d` and `s` tracks). We also integrate the `Scheduler` directly into the state, tracking `turns`, `ready_d`, and `ready_s`.
Incremental Improvement: Replaced heuristic distance and depth formulas with an exact Lookahead Scheduler. When choosing buffer tracks or deciding which target car to extract next, we clone the state and scheduler, simulate the physical moves, and evaluate the exact resulting makespan (`turns.len()`). This eliminates guesswork and optimally packs parallel operations by evaluating true dependency footprints.

---

# AHC064 (Version4)

Algorithm Choice: DP-style Beam Search with Lazy/Eager State Branching.
State Representation: `SearchNode` containing `State`, `Scheduler`, `moves`, and `finalized` counts. The Beam Search groups nodes by their total number of finalized cars (`n`).
Incremental Improvement: Upgraded the Greedy Search to a full Beam Search (width = 300) to globally optimize the sequence of extractions. Additionally, we optimized the `Scheduler::simulate` function to be $O(1)$ allocation-free, making millions of simulations computationally viable within the time limit. We also allow the Beam Search to organically decide between an "Eager Turn 1 Dump" and "Lazy Buffer Extraction" by seeding the initial beam with both divergent starting states.

---

# AHC064 (Version5)

Algorithm Choice: Chokudai Search (Time-bounded Priority Beam Search) with State Hashing.
State Representation: `SearchNode` pushed into 101 Priority Queues based on `n` finalized cars. A `HashSet` tracks visited physical track configurations to perfectly deduplicate identical board states reached via different paths.
Incremental Improvement: Upgraded from standard fixed-width Beam Search to a time-bounded Chokudai Search, allowing the algorithm to dynamically explore the state space until the 1.9s time limit expires. This perfectly maximizes the use of competitive programming time constraints. Additionally, fixed a major flaw in greedy buffer selection where small splits were not sufficiently penalized, heavily rewarding block-moves to prevent micro-fragmentation of buffer tracks.

