'use strict';
const assert = require('node:assert/strict');
const { trace, cross } = require('./model.js');
const points = coords => coords.map(([x,y],i)=>({x,y,id:String(i)}));
const keys = arr => arr.map(p=>`${p.x},${p.y}`).sort();

for (const [input, expected] of [
  [[], []], [[[2,3]], [[2,3]]], [[[2,3],[2,3]], [[2,3]]],
  [[[0,0],[1,1],[2,2],[3,3]], [[0,0],[3,3]]],
  [[[1,0],[1,2],[1,4]], [[1,0],[1,4]]],
  [[[0,0],[4,0],[4,4],[0,4],[2,2],[2,0]], [[0,0],[4,0],[4,4],[0,4]]],
  [[[0,0],[5,0],[2,4],[2,1]], [[0,0],[5,0],[2,4]]]
]) assert.deepEqual(keys(trace(points(input)).hull), keys(points(expected)));

let seed=41;
const rand=()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/2**32;};
for(let trial=0;trial<300;trial++) {
  const coords=new Map();
  while(coords.size<12) {const x=Math.floor(rand()*10),y=Math.floor(rand()*10);coords.set(`${x},${y}`,[x,y]);}
  const input=points([...coords.values()]), result=trace(input);
  assert.equal(result.events.at(-1).action,'finish');
  const hull=result.hull;
  // Geometric oracle: every directed edge must leave all points to its left.
  for(let i=0;i<hull.length;i++) {
    const a=hull[i],b=hull[(i+1)%hull.length],c=hull[(i+2)%hull.length];
    assert.ok(cross(a,b,c)>0);
    assert.ok(input.every(p=>cross(a,b,p)>=0));
  }
  // Each push must leave a strictly convex chain.
  for(const event of result.events) {
    if(event.action==='push') {
      const stack=event.phase==='Lower'?event.lower:event.upper;
      for(let i=2;i<stack.length;i++)assert.ok(cross(stack[i-2],stack[i-1],stack[i])>0);
    }
    if(event.action==='check')assert.equal(event.value,cross(...event.triple));
  }
}
const repeated=trace(points([[2,2],[5,4],[8,7],[11,12],[14,16],[17,3],[21,11],[26,5]]));
assert.ok(repeated.events.some((e,i,arr)=>e.action==='pop'&&arr[i+1]?.action==='check'&&arr[i+2]?.action==='pop'));
console.log('Passed 7 edge cases, 300 randomized geometry/trace checks, and repeated-pop coverage.');
