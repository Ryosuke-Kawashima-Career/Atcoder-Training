(function (root) {
  'use strict';
  const cross = (a, b, c) => (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);

  function trace(input) {
    const points = [...new Map(input.map(p => [`${p.x},${p.y}`, { ...p }])).values()]
      .sort((a, b) => a.x - b.x || a.y - b.y);
    const lower = [], upper = [], events = [];
    function emit(phase, action, text, line, extra = {}) {
      events.push({ phase, action, text, line, lower: [...lower], upper: [...upper], ...extra });
    }
    emit('Sort', 'sort', 'Sort by x, then by y. The lower chain scans left to right; the upper chain scans back.', 0);
    if (points.length < 2) {
      emit('Complete', 'finish', points.length ? 'One point is already its own convex hull.' : 'Add at least three points to explore a polygon.', 13, { hull: [...points] });
      return { points, events, hull: [...points] };
    }
    for (const [phase, order, stack, loopLine] of [
      ['Lower', points, lower, 2], ['Upper', [...points].reverse(), upper, 7]
    ]) {
      for (const candidate of order) {
        emit(phase, 'candidate', `Consider ${candidate.id} (${candidate.x}, ${candidate.y}). Check the last two points on the ${phase.toLowerCase()} stack.`, loopLine, { candidate });
        while (stack.length >= 2) {
          const a = stack[stack.length - 2], b = stack[stack.length - 1];
          const value = cross(a, b, candidate), triple = [a, b, candidate];
          emit(phase, 'check', value > 0
            ? `A left turn (+${value}). Keep ${b.id}; this turn belongs on the chain.`
            : value === 0 ? `Collinear (0). Remove ${b.id}; keep only the endpoints of a straight edge.`
              : `A right turn (${value}). Remove ${b.id}; it makes the chain bend inward.`, loopLine + 1, { candidate, triple, value });
          if (value > 0) break;
          stack.pop();
          emit(phase, 'pop', `Pop ${b.id}. Check the same candidate again against the new end of the stack. This is why the code uses while.`, loopLine + 2, { candidate, triple, value, removed: b });
        }
        stack.push(candidate);
        emit(phase, 'push', `Push ${candidate.id} onto the ${phase.toLowerCase()} stack.`, loopLine + 4, { candidate });
      }
    }
    lower.pop(); upper.pop();
    const hull = [...lower, ...upper];
    emit('Join', 'join', 'Remove the last point of each chain: each endpoint appears in both chains. Join lower + upper.', 12, { hull });
    emit('Complete', 'finish', hull.length === 2
      ? 'All points are collinear. The hull is the segment between the two extreme points.'
      : `${hull.length} vertices form the convex hull, in counterclockwise order. Every input point is on or inside it.`, 13, { hull });
    return { points, events, hull };
  }
  const api = { cross, trace };
  if (typeof module !== 'undefined' && module.exports) module.exports = api;
  else root.ConvexHullModel = api;
})(typeof globalThis !== 'undefined' ? globalThis : this);
