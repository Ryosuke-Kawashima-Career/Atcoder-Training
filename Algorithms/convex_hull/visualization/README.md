# Convex hull algorithm lab

Open `index.html` in a browser. No installation, server, or network connection is required.

- Play, pause, step backward/forward, or scrub the full construction.
- Switch between the walkthrough, repeated pops, collinear points, a triangle, and random data.
- Click the coordinate plane to add integer points; click a point to remove it. Editing restarts the trace. Maximum: 40 points.
- Focus a point and press Enter, Space, or Delete to remove it. Use the labeled controls or Left/Right arrow keys to step.
- See the cross product, candidate triangle, both stacks, and highlighted corrected Rust pseudocode at every step.
- Toggle the final hull guide to compare the current chain with the completed result.

The app demonstrates the intended **corrected** Andrew monotone-chain algorithm, rather than reproducing the mistakes in `../convex_hull.rs`. The source file is left unchanged. `cross_last` in the walkthrough means `cross(stack[stack.len()-2], stack[stack.len()-1], p)`; it is an abbreviation for readability.

The `<= 0` turn condition removes intermediate collinear points, leaving only the extreme hull vertices. Empty, single-point, two-point, and fully collinear inputs are supported. App coordinates are small integers so JavaScript arithmetic is exact; Rust implementations for large coordinates should use an appropriately wide integer type for cross products.

## Verify

```sh
node model.test.js
```

The tests cover edge cases, repeated pops, counterclockwise convexity, containment of input points, stack invariants, and turn values on 300 seeded random inputs.
