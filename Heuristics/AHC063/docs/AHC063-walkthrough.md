# AHC063 Implementation Walkthrough

## Summary

Successfully implemented version 1 of the Greedy target color matching algorithm using Breadth-First Search (BFS) for AtCoder Heuristic Contest 063.

## What was Changed

1. **Removed Initial Base Logic**: Replaced the default hardcoded zigzag `get_moves()` logic in `playground/src/main.rs`.
2. **State Class Implementation**: Added a virtual simulator mimicking exactly AHC063's mechanics of $N \times N$ tracking `snake` coordinate queues and `grid` states.
3. **Pathfinding Module**: Introduced an $O(N^2)$ Breadth-First Search distance-mapping check algorithm preventing body collisions and unneeded color-food collections while pulling the optimal target color `d_len` route to maximize coordinate matching accuracy.
4. **Fallback Safe Wander**: The snake performs safe step allocations moving to empty blocks (`0`) to avoid being boxed in.

## Test Inputs & Results

`tests` on sample sequence inside `./playground`:

- **Run command**: `cargo run`
- **Input Used**:

    ```text
    8 30 3
    1 1 1 1 1 2 2 3 3 2 2 2 3 2 3 1 3 2 3 1 2 3 2 3 3 2 2 3 3 2
    0 0 2 0 2 0 0 0
    0 0 3 0 0 3 0 2
    0 0 2 3 3 3 0 0
    //... (truncated full map representation)
    ```

- **Execution Results**: Standard Output generated exact coordinate mapping letters correctly without producing unexpected cyclic states or invalid jumps:

    ```text
    D
    R
    D
    R
    U
    R
    U
    L
    U
    U
    ...
    ```

## Post-Run Observations

- Because the map represents up to 40% cell occupation with foods ($M \approx N^2/2$), Greedy BFS runs out of valid open trajectories quickly when length grows because it actively rejects $E$-penalty producing alternative color targets.
- The algorithm properly halts prior to $100,000$ limit when fully boxed.

## Next Steps

To improve performance, refer to [strategy.md](../strategy.md) for future vision on Beam Search heuristics and Auto-Biting mechanisms to handle tight constraints.
