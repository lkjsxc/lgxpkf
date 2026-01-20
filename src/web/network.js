(() => {
  const canvas = document.getElementById("network-canvas");
  if (!canvas) { return; }
  const ctx = canvas.getContext("2d");
  const state = { nodes: [], edges: [], hover: null, nodeIndex: new Map() };
  const rand = (min, max) => min + Math.random() * (max - min);
  const clamp = (value, min, max) => Math.min(max, Math.max(min, value));
  const fetchJson = async (url) => {
    const res = await fetch(url);
    if (!res.ok) { throw new Error("Network load failed"); }
    return res.json();
  };
  const resize = () => {
    const { clientWidth, clientHeight } = canvas;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = clientWidth * dpr;
    canvas.height = clientHeight * dpr;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  };
  const addNode = (map, note) => {
    if (map.has(note.id)) { return map.get(note.id); }
    const node = {
      id: note.id,
      label: note.author && note.author.email ? note.author.email : "note",
      x: rand(60, canvas.clientWidth - 60),
      y: rand(60, canvas.clientHeight - 60),
      vx: 0,
      vy: 0,
    };
    map.set(note.id, node);
    return node;
  };
  const buildGraph = async () => {
    const seed = await fetchJson("/notes/random?limit=24");
    const map = new Map();
    seed.forEach((note) => addNode(map, note));
    const edgeSet = new Set();
    const related = await Promise.all(seed.map((note) => fetchJson(`/notes/${note.id}/related`).catch(() => null)));
    const edges = [];
    related.forEach((resp) => {
      if (!resp) { return; }
      const center = addNode(map, resp.center);
      resp.related.forEach((entry) => {
        const other = addNode(map, entry.note);
        const key = center.id < other.id ? `${center.id}:${other.id}` : `${other.id}:${center.id}`;
        if (edgeSet.has(key)) { return; }
        edgeSet.add(key);
        edges.push({ a: center.id, b: other.id });
      });
    });
    state.nodes = Array.from(map.values());
    state.edges = edges;
    state.nodeIndex = new Map(state.nodes.map((node) => [node.id, node]));
  };
  const simulate = () => {
    const nodes = state.nodes;
    const edges = state.edges;
    const repulsion = 9000;
    const spring = 0.002;
    const damping = 0.86;
    for (let i = 0; i < nodes.length; i += 1) {
      const a = nodes[i];
      for (let j = i + 1; j < nodes.length; j += 1) {
        const b = nodes[j];
        const dx = a.x - b.x;
        const dy = a.y - b.y;
        const dist = Math.hypot(dx, dy) || 1;
        const force = repulsion / (dist * dist);
        const fx = (dx / dist) * force;
        const fy = (dy / dist) * force;
        a.vx += fx;
        a.vy += fy;
        b.vx -= fx;
        b.vy -= fy;
      }
    }
    edges.forEach((edge) => {
      const a = state.nodeIndex.get(edge.a);
      const b = state.nodeIndex.get(edge.b);
      if (!a || !b) { return; }
      const dx = b.x - a.x;
      const dy = b.y - a.y;
      const dist = Math.hypot(dx, dy) || 1;
      const target = 160;
      const pull = (dist - target) * spring;
      const fx = (dx / dist) * pull;
      const fy = (dy / dist) * pull;
      a.vx += fx;
      a.vy += fy;
      b.vx -= fx;
      b.vy -= fy;
    });
    nodes.forEach((node) => {
      node.vx *= damping;
      node.vy *= damping;
      node.x = clamp(node.x + node.vx, 30, canvas.clientWidth - 30);
      node.y = clamp(node.y + node.vy, 30, canvas.clientHeight - 30);
    });
  };
  const render = () => {
    ctx.clearRect(0, 0, canvas.clientWidth, canvas.clientHeight);
    ctx.lineWidth = 1;
    ctx.strokeStyle = "rgba(106,227,255,0.18)";
    state.edges.forEach((edge) => {
      const a = state.nodeIndex.get(edge.a);
      const b = state.nodeIndex.get(edge.b);
      if (!a || !b) { return; }
      ctx.beginPath();
      ctx.moveTo(a.x, a.y);
      ctx.lineTo(b.x, b.y);
      ctx.stroke();
    });
    state.nodes.forEach((node) => {
      const hovered = state.hover && state.hover.id === node.id;
      ctx.fillStyle = hovered ? "#f0b35a" : "#6ae3ff";
      ctx.beginPath();
      ctx.arc(node.x, node.y, hovered ? 6 : 4, 0, Math.PI * 2);
      ctx.fill();
    });
    if (state.hover) {
      ctx.fillStyle = "#e7eef8";
      ctx.font = "12px sans-serif";
      ctx.fillText(state.hover.label, state.hover.x + 10, state.hover.y - 10);
    }
  };
  const tick = () => {
    simulate();
    render();
    requestAnimationFrame(tick);
  };
  const pickNode = (x, y) => {
    let nearest = null;
    let best = 18;
    state.nodes.forEach((node) => {
      const dist = Math.hypot(node.x - x, node.y - y);
      if (dist < best) { best = dist; nearest = node; }
    });
    return nearest;
  };
  canvas.addEventListener("mousemove", (event) => {
    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;
    state.hover = pickNode(x, y);
  });
  canvas.addEventListener("click", () => {
    if (state.hover) { window.location.assign(`/${state.hover.id}`); }
  });
  window.addEventListener("resize", resize);
  resize();
  buildGraph().then(() => requestAnimationFrame(tick)).catch(() => {
    ctx.fillStyle = "#93a2bb";
    ctx.font = "14px sans-serif";
    ctx.fillText("Network unavailable.", 24, 48);
  });
})();
