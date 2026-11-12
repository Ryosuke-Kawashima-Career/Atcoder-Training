(() => {
  'use strict';
  const $ = id => document.getElementById(id);
  const NS = 'http://www.w3.org/2000/svg';
  const colors = { lower: '#c9ef83', upper: '#8ecbd9', candidate: '#ffc28c', text: '#e8eeeb', muted: '#9cb1a3', grid: '#2a3930', pop: '#f2a8a0' };
  const presets = {
    lesson: [[2,4],[4,12],[6,6],[8,16],[10,9],[12,3],[14,14],[16,7],[18,16],[20,4],[23,12],[26,6],[14,10],[7,10]],
    pops: [[2,2],[5,4],[8,7],[11,12],[14,16],[17,3],[21,11],[26,5]],
    collinear: [[3,3],[6,4],[9,5],[12,6],[15,7],[18,8],[21,9],[24,10]],
    triangle: [[3,3],[25,3],[14,16],[10,6],[16,7],[14,11],[7,3],[19,3]]
  };
  const code = [
    'points.sort_by_key(|p| (p.x, p.y));',
    'let (mut lower, mut upper) = (vec![], vec![]);',
    'for &p in &points {',
    '  while lower.len() >= 2 && cross_last(&lower, p) <= 0 {',
    '    lower.pop();',
    '  }',
    '  lower.push(p); }',
    'for &p in points.iter().rev() {',
    '  while upper.len() >= 2 && cross_last(&upper, p) <= 0 {',
    '    upper.pop();',
    '  }',
    '  upper.push(p); }',
    'lower.pop(); upper.pop();',
    'lower.extend(upper); // hull'
  ];
  const names = { sort: 'First, put the points in order.', candidate: 'Meet the next candidate.', check: 'Which way does the chain turn?', pop: 'Step back. Test again.', push: 'Keep this point for now.', join: 'Two chains become one boundary.', finish: 'The outer edge, complete.' };
  let points = [], state, index = 0, timer = null, idCounter = 0;
  function idFor(i) { return String.fromCharCode(65 + i % 26) + (i >= 26 ? Math.floor(i / 26) : ''); }
  function load(coords) {
    points = coords.map(([x, y], i) => ({ x, y, id: idFor(i) }));
    idCounter = points.length;
    rebuild();
  }
  function randomSet() {
    const coords = [], used = new Set();
    while (coords.length < 16) {
      const x = 2 + Math.floor(Math.random() * 25), y = 2 + Math.floor(Math.random() * 15), key = `${x},${y}`;
      if (!used.has(key)) { used.add(key); coords.push([x,y]); }
    }
    return coords;
  }
  function rebuild() { pause(); state = ConvexHullModel.trace(points); index = 0; $('timeline').max = state.events.length - 1; render(); }
  const px = p => 64 + p.x * 24;
  const py = p => 502 - p.y * 24;
  function svg(tag, attrs, parent = $('plot'), text) {
    const el = document.createElementNS(NS, tag);
    for (const [key, value] of Object.entries(attrs)) el.setAttribute(key, value);
    if (text !== undefined) el.textContent = text;
    parent.appendChild(el); return el;
  }
  function path(points, attrs, close = false) {
    if (!points.length) return;
    svg(close ? 'polygon' : 'polyline', { points: points.map(p => `${px(p)},${py(p)}`).join(' '), fill: 'none', 'stroke-linejoin': 'round', 'stroke-linecap': 'round', ...attrs });
  }
  function renderPlot(e) {
    const plot = $('plot');
    plot.replaceChildren();
    svg('title', { id: 'plot-title' }, plot, 'Convex hull construction');
    svg('desc', { id: 'plot-desc' }, plot, `${e.phase}: ${e.text} ${points.length} input points. Click to add a point. Focus a point and press Delete to remove it.`);
    for (let x = 0; x <= 28; x += 2) {
      svg('line', { x1:px({x}), y1:70, x2:px({x}), y2:502, stroke:colors.grid, 'stroke-width':1 });
      svg('text', { x:px({x}), y:526, fill:colors.muted, 'font-size':11, 'text-anchor':'middle' }, plot, x);
    }
    for (let y = 0; y <= 18; y += 2) {
      svg('line', { x1:64, y1:py({y}), x2:736, y2:py({y}), stroke:colors.grid, 'stroke-width':1 });
      svg('text', { x:47, y:py({y})+4, fill:colors.muted, 'font-size':11, 'text-anchor':'end' }, plot, y);
    }
    svg('text', {x:755,y:506,fill:colors.muted,'font-size':13}, plot, 'x');
    svg('text', {x:60,y:48,fill:colors.muted,'font-size':13}, plot, 'y');
    if ($('preview').checked) path(state.hull, { stroke: '#64796b', 'stroke-width':1.5, 'stroke-dasharray':'6 6' }, state.hull.length > 2);
    const complete = e.action === 'join' || e.action === 'finish';
    if (complete) path(e.hull, { stroke:colors.lower, 'stroke-width':2.8, fill:e.hull.length > 2 ? '#c9ef8310' : 'none' }, e.hull.length > 2);
    else {
      path(e.lower, { stroke:colors.lower, 'stroke-width':2.5 });
      path(e.upper, { stroke:colors.upper, 'stroke-width':2.5 });
      if (e.triple) path(e.triple, { stroke:colors.candidate, 'stroke-width':1.8, 'stroke-dasharray':'6 5', fill:'#ffc28c0d' }, true);
      else if (e.candidate && e.action === 'candidate') {
        const stack = e.phase === 'Lower' ? e.lower : e.upper;
        if (stack.length) path([stack[stack.length-1], e.candidate], {stroke:colors.candidate,'stroke-width':1.5,'stroke-dasharray':'5 5'});
      }
    }
    const inLower = new Set(e.lower.map(p=>p.id)), inUpper = new Set(e.upper.map(p=>p.id));
    const inHull = new Set((e.hull || []).map(p=>p.id));
    for (const p of state.points) {
      let fill = '#7d9385';
      if (complete) { if (inHull.has(p.id)) fill = colors.lower; }
      else { if (inLower.has(p.id)) fill=colors.lower; if (inUpper.has(p.id)) fill=colors.upper; }
      if (e.candidate?.id === p.id && !complete) fill=colors.candidate;
      if (e.removed?.id === p.id) fill=colors.pop;
      const g = svg('g', {class:'point',tabindex:'0',role:'button','aria-label':`${p.id}, x ${p.x}, y ${p.y}. Activate to remove.`, 'data-id':p.id});
      svg('title', {}, g, `${p.id} (${p.x}, ${p.y}) · click to remove`);
      svg('circle', {cx:px(p),cy:py(p),r:13,fill:'transparent'},g);
      if (e.candidate?.id === p.id && !complete) svg('rect', {x:px(p)-9,y:py(p)-9,width:18,height:18,fill:'none',stroke:colors.candidate,'stroke-width':1.2,transform:`rotate(45 ${px(p)} ${py(p)})`},g);
      svg('circle', {cx:px(p),cy:py(p),r:4.5,fill,stroke:'#17201f','stroke-width':1.5},g);
      svg('text', {x:px(p)+10,y:py(p)-11,fill,'font-size':12,'font-family':'ui-monospace,monospace','pointer-events':'none'},g,p.id);
      if (e.removed?.id === p.id) {
        svg('line',{x1:px(p)-7,y1:py(p)-7,x2:px(p)+7,y2:py(p)+7,stroke:colors.pop,'stroke-width':1.5,'pointer-events':'none'},g);
        svg('line',{x1:px(p)-7,y1:py(p)+7,x2:px(p)+7,y2:py(p)-7,stroke:colors.pop,'stroke-width':1.5,'pointer-events':'none'},g);
      }
    }
  }
  function renderStack(id, stack) {
    $(id).replaceChildren();
    if (!stack.length) { $(id).textContent = 'empty'; return; }
    for (const p of stack) { const span=document.createElement('span'); span.className='stack-item'; span.textContent=p.id; span.title=`(${p.x}, ${p.y})`; $(id).appendChild(span); }
  }
  function render() {
    const e = state.events[index];
    $('event-title').textContent=names[e.action]; $('event-text').textContent=e.text;
    $('action-badge').textContent=e.action.toUpperCase();
    $('phase-text').textContent=e.phase === 'Lower' ? 'Lower · left → right' : e.phase === 'Upper' ? 'Upper · right → left' : e.phase;
    $('step-text').textContent=`Step ${index+1} / ${state.events.length}`;
    $('timeline').value=index; $('point-count').textContent=`${points.length} / 40 points`;
    $('back').disabled=index===0; $('restart').disabled=index===0 && !timer;
    $('next').disabled=index===state.events.length-1; $('finish').disabled=index===state.events.length-1;
    $('play').innerHTML=timer ? 'Pause <span aria-hidden="true">Ⅱ</span>' : index===state.events.length-1 ? 'Replay <span aria-hidden="true">↺</span>' : 'Play <span aria-hidden="true">▶</span>';
    if (e.triple) {
      const [a,b,c]=e.triple;
      $('turn-formula').textContent=`(${b.x} − ${a.x}) × (${c.y} − ${a.y}) − (${b.y} − ${a.y}) × (${c.x} − ${a.x})`;
      $('turn-value').textContent=e.value>0 ? `+${e.value}` : e.value;
      $('turn-value').style.color=e.value>0 ? colors.lower : colors.candidate;
      $('turn-label').textContent=`${a.id} → ${b.id} → ${c.id} · ${e.value>0 ? 'Left turn / keep' : e.value<0 ? 'Right turn / pop' : 'Collinear / pop'}`;
    } else {
      $('turn-formula').textContent='(Bₓ − Aₓ)(Cᵧ − Aᵧ) − (Bᵧ − Aᵧ)(Cₓ − Aₓ)';
      $('turn-value').textContent='—'; $('turn-value').style.color=colors.muted;
      $('turn-label').textContent=e.action==='finish' ? 'Hull complete' : 'Waiting for a turn test';
    }
    renderStack('lower-stack',e.lower); renderStack('upper-stack',e.upper);
    $('code').replaceChildren();
    code.forEach((line,i)=>{const span=document.createElement('span'); span.className='code-line'+(i===e.line?' active':'');const no=document.createElement('span');no.className='line-no';no.textContent=i+1;span.append(no,document.createTextNode(line));$('code').appendChild(span);});
    renderPlot(e);
  }
  function pause() { if (timer) clearInterval(timer); timer=null; }
  function play() {
    if (timer) {pause();render();return;}
    if (index===state.events.length-1) index=0;
    timer=setInterval(()=>{if(index<state.events.length-1)index++;if(index===state.events.length-1)pause();render();},Number($('speed').value));
    render();
  }
  $('play').onclick=play;
  $('back').onclick=()=>{pause();index=Math.max(0,index-1);render();};
  $('next').onclick=()=>{pause();index=Math.min(state.events.length-1,index+1);render();};
  $('restart').onclick=()=>{pause();index=0;render();};
  $('finish').onclick=()=>{pause();index=state.events.length-1;render();};
  $('timeline').oninput=()=>{pause();index=Number($('timeline').value);render();};
  $('speed').onchange=()=>{if(timer){pause();play();}};
  $('preview').onchange=render;
  $('preset').onchange=()=>load($('preset').value==='random'?randomSet():presets[$('preset').value]);
  $('shuffle').onclick=()=>{$('preset').value='random';load(randomSet());};
  function remove(id) { points=points.filter(p=>p.id!==id);rebuild(); }
  $('plot').onclick=e=>{
    const target=e.target.closest('[data-id]');
    if(target){remove(target.getAttribute('data-id'));return;}
    const location=$('plot').createSVGPoint(); location.x=e.clientX;location.y=e.clientY;
    const local=location.matrixTransform($('plot').getScreenCTM().inverse());
    const x=Math.round((local.x-64)/24), y=Math.round((502-local.y)/24);
    if(x<0||x>28||y<0||y>18||points.length>=40||points.some(p=>p.x===x&&p.y===y))return;
    points.push({x,y,id:idFor(idCounter++)});rebuild();
  };
  $('plot').onkeydown=e=>{
    const target=e.target.closest('[data-id]');
    if(target&&['Enter',' ','Delete','Backspace'].includes(e.key)){e.preventDefault();remove(target.getAttribute('data-id'));$('next').focus();}
  };
  document.addEventListener('keydown',e=>{
    if(['INPUT','SELECT','BUTTON','SUMMARY'].includes(document.activeElement.tagName)||document.activeElement.closest('[data-id]'))return;
    if(e.key==='ArrowRight'){$('next').click();e.preventDefault();}
    if(e.key==='ArrowLeft'){$('back').click();e.preventDefault();}
  });
  load(presets.lesson);
})();
