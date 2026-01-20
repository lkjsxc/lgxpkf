(() => {
  const canvas = document.getElementById("network-canvas") as HTMLCanvasElement | null;
  if (!canvas) return;
  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  type Node = { id: string; label: string; x: number; y: number; vx: number; vy: number };
  type Edge = { a: string; b: string };
  type RelatedResponse = { center?: LgxpkfNote; related?: Array<{ note?: LgxpkfNote }> };

  const state: { nodes: Node[]; edges: Edge[]; hover: Node | null; nodeIndex: Map<string, Node> } = {
    nodes: [],
    edges: [],
    hover: null,
    nodeIndex: new Map(),
  };
  const rand = (min: number, max: number): number => min + Math.random() * (max - min);
  const clamp = (value: number, min: number, max: number): number => Math.min(max, Math.max(min, value));

  const fetchJson = async (url: string): Promise<unknown> => {
    const res = await fetch(url);
    if (!res.ok) throw new Error("Network load failed");
    return res.json();
  };

  const normalizeNotes = (payload: unknown): LgxpkfNote[] => (Array.isArray(payload) ? (payload as LgxpkfNote[]) : []);

  const resize = (): void => {
    const { clientWidth, clientHeight } = canvas;
    const dpr = window.devicePixelRatio || 1;
    canvas.width = clientWidth * dpr;
    canvas.height = clientHeight * dpr;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  };

  const addNode = (map: Map<string, Node>, note: LgxpkfNote): Node => {
    if (map.has(note.id)) return map.get(note.id) as Node;
    const width = Math.max(canvas.clientWidth, 160);
    const height = Math.max(canvas.clientHeight, 160);
    const node = {
      id: note.id,
      label: note.author?.email || "note",
      x: rand(60, width - 60),
      y: rand(60, height - 60),
      vx: 0,
      vy: 0,
    };
    map.set(note.id, node);
    return node;
  };

  const buildGraph = async (): Promise<void> => {
    const seed = normalizeNotes(await fetchJson("/notes/random?limit=24"));
    const map = new Map<string, Node>();
    seed.forEach((note) => addNode(map, note));
    const edgeSet = new Set<string>();
    const related = await Promise.all(seed.map((note) => fetchJson(`/notes/${note.id}/related`).catch(() => null)));
    const edges: Edge[] = [];
    related.forEach((resp) => {
      const data = resp as RelatedResponse | null;
      if (!data?.center) return;
      const center = addNode(map, data.center);
      const relatedItems = Array.isArray(data.related) ? data.related : [];
      relatedItems.forEach((entry) => {
        if (!entry?.note) return;
        const other = addNode(map, entry.note);
        const key = center.id < other.id ? `${center.id}:${other.id}` : `${other.id}:${center.id}`;
        if (edgeSet.has(key)) return;
        edgeSet.add(key);
        edges.push({ a: center.id, b: other.id });
      });
    });
    state.nodes = Array.from(map.values());
    state.edges = edges;
    state.nodeIndex = new Map(state.nodes.map((node) => [node.id, node]));
  };

  const simulate = (): void => {
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
      if (!a || !b) return;
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

  const render = (): void => {
    ctx.clearRect(0, 0, canvas.clientWidth, canvas.clientHeight);
    ctx.lineWidth = 1;
    ctx.strokeStyle = "rgba(106,227,255,0.18)";
    state.edges.forEach((edge) => {
      const a = state.nodeIndex.get(edge.a);
      const b = state.nodeIndex.get(edge.b);
      if (!a || !b) return;
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

  const tick = (): void => {
    simulate();
    render();
    window.requestAnimationFrame(tick);
  };

  const pickNode = (x: number, y: number): Node | null => {
    let nearest: Node | null = null;
    let best = 18;
    state.nodes.forEach((node) => {
      const dist = Math.hypot(node.x - x, node.y - y);
      if (dist < best) {
        best = dist;
        nearest = node;
      }
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
    if (state.hover) window.location.assign(`/${state.hover.id}`);
  });

  window.addEventListener("resize", resize);
  resize();
  buildGraph()
    .then(() => window.requestAnimationFrame(tick))
    .catch(() => {
      ctx.fillStyle = "#93a2bb";
      ctx.font = "14px sans-serif";
      ctx.fillText("Network unavailable.", 24, 48);
    });
})();
