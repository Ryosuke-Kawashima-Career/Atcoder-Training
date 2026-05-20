# Strategy for AHC063

## Current Algorithm (Greedy Target Color Matching)

### Path Selection

- **Approach**: The solver extracts the target board size ($N$) and required target color sequence length ($M$).
- Since incorrect colors result in a massive penalty (-$10,000$ points) compared to turn counts (-$1$ point), the strategy exclusively relies on finding the next valid *required target color* and pathfinding toward it using a basic **Breadth-First Search (BFS)** without passing over other colors.
- **Rules of Collision**: Empty cells (`0`) and the selected targeted food (`color`) are reachable. Any other food or the snake's current physical coordinates are treated as blocked.
- **Biting**: Not yet implemented. The solver takes wide avoidance on its own body.

### Chain of Thought

1. Initialize state (`Grid`, `Snake Deque`, `Targets`).
2. While length $k < m$: Pull $d_k$ as expected Target.
3. Perform $O(N^2)$ BFS tracking parents & shortest move sequence `dist[cx][cy]`.
4. Apply reverse move reconstruction and feed instructions into snake simulator to increment length or walk aimlessly safely on map until stuck or max turns.
5. Terminates when length matched or algorithm is trapped.

## Future Vision & Adjustments

- **Auto-Biting Mechanics**: Right now we are completely ignoring the 'biting' rules, keeping our unmatching initial size-5 `$c_0 \cdots c_4$` snake (`1, 1, 1, 1, 1`) intact. We can dramatically lower positional error sequence mismatch by performing a localized U-turn 'bite' at index $h=1$ to drop the excess 1s and re-start mapping perfectly over `d`.
- **Beam Search**: Replace naive shortest-path BFS with randomized branch exploration of multiple move combinations mapping 10-20 moves ahead, rewarding state branches with higher exact food extraction and lesser trapping potential. (Since $M \approx N^2/2$, dense regions trap snake easily).
- **Food Relocation**: Biting tail to drop unneeded consumed food acting as *transporter* blocks.

## Resources & Reason of Use

- Use `std::collections::VecDeque` to manipulate $O(1)$ shifts of grid simulation head prepending & tail dropping.
- Utilized BFS directly instead of Dijkstra as edge distance cost remains uniform = 1 turn.
