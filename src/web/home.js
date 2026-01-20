(() => {
  const el = (id) => document.getElementById(id);
  const state = { token: localStorage.getItem("lgxpkf.session"), user: null };
  const view = document.body.dataset.view || "home";
  const guestHero = el("guest-hero");
  const randomBlock = el("random-block");
  const randomList = el("random-list");
  const randomStatus = el("random-status");
  const timelineBlock = el("timeline-block");
  const timelineTitle = el("timeline-title");
  const timelineList = el("timeline-list");
  const timelineStatus = el("timeline-status");
  const composer = el("composer");
  const openBtn = el("open-composer");
  const closeBtn = el("close-composer");
  const noteForm = el("note-form");
  const noteValue = el("note-value");
  const noteStatus = el("note-status");
  const submitBtn = el("submit");
  const esc = (value) => String(value).replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/\"/g, "&quot;").replace(/'/g, "&#39;");
  const setMessage = (node, text) => { if (node) { node.textContent = text; } };
  const setModalState = (modal, open) => {
    if (!modal) { return; }
    modal.classList.toggle("open", open);
    modal.setAttribute("aria-hidden", open ? "false" : "true");
    if (open) { modal.removeAttribute("inert"); } else { modal.setAttribute("inert", ""); }
    modal.querySelectorAll("button, input, textarea, a").forEach((node) => {
      if (open) { node.removeAttribute("tabindex"); } else { node.setAttribute("tabindex", "-1"); }
    });
  };
  const setSignedIn = (signedIn) => {
    submitBtn.disabled = !signedIn;
    noteValue.disabled = !signedIn;
    openBtn.hidden = !signedIn;
    if (view === "home") {
      guestHero.hidden = signedIn;
      randomBlock.hidden = signedIn;
      timelineBlock.hidden = !signedIn;
    } else {
      guestHero.hidden = true;
      randomBlock.hidden = true;
      timelineBlock.hidden = false;
    }
  };
  const api = async (path, options = {}) => {
    const headers = options.headers || {};
    if (state.token) { headers.Authorization = `Bearer ${state.token}`; }
    const response = await fetch(path, { ...options, headers });
    const data = await response.json().catch(() => ({}));
    if (!response.ok) { throw new Error(data.message || "Request failed"); }
    return data;
  };
  const renderNote = (note) => {
    const card = document.createElement("a");
    card.className = "note";
    card.href = `/${esc(note.id)}`;
    card.innerHTML = `<div class="note-meta"><span>${esc(note.created_at)}</span><span>${esc(note.author.email)}</span></div><div class="note-value">${esc(note.value)}</div>`;
    return card;
  };
  const loadList = async (path, list, status, emptyText) => {
    if (!list || !status) { return; }
    status.textContent = "Loading...";
    try {
      const items = await api(path);
      list.innerHTML = "";
      if (!items.length) { status.textContent = emptyText; return; }
      status.textContent = "";
      items.forEach((note) => list.appendChild(renderNote(note)));
    } catch (err) {
      status.textContent = err.message;
    }
  };
  const loadFeed = () => { timelineTitle.textContent = "Timeline"; return loadList("/feed", timelineList, timelineStatus, "Timeline is empty."); };
  const loadMyPosts = () => {
    timelineTitle.textContent = "My posts";
    if (!state.user) { setMessage(timelineStatus, "Sign in to view your posts."); return Promise.resolve(); }
    return loadList(`/notes?author=${state.user.user_id}`, timelineList, timelineStatus, "No posts yet.");
  };
  const loadRandom = async () => {
    if (!randomList || !randomStatus) { return; }
    randomStatus.textContent = "Loading random timeline...";
    try {
      const response = await fetch("/notes/random?limit=9");
      if (!response.ok) { throw new Error("Random timeline failed"); }
      const items = await response.json();
      randomList.innerHTML = "";
      if (!items.length) { randomStatus.textContent = "No posts yet."; return; }
      randomStatus.textContent = "";
      items.forEach((note) => randomList.appendChild(renderNote(note)));
    } catch (_) {
      randomStatus.textContent = "Random timeline unavailable.";
    }
  };
  const applySession = (session) => {
    state.token = session.token;
    state.user = session.user;
    setSignedIn(Boolean(state.token));
    if (view === "home") {
      if (state.token) { loadFeed(); } else { loadRandom(); }
    } else {
      timelineTitle.textContent = "My posts";
      if (state.token) { loadMyPosts(); } else { setMessage(timelineStatus, "Sign in to view your posts."); }
    }
  };
  const bootstrap = () => {
    setSignedIn(Boolean(state.token));
    if (view === "home") {
      if (state.token) { loadFeed(); } else { loadRandom(); }
    } else {
      timelineTitle.textContent = "My posts";
      if (!state.token) { setMessage(timelineStatus, "Sign in to view your posts."); }
    }
  };
  window.addEventListener("lgxpkf:session", (event) => applySession(event.detail));
  const existing = window.lgxpkfSession;
  if (existing) { applySession(existing); } else { bootstrap(); }
  openBtn.addEventListener("click", () => {
    if (!state.token) { setMessage(noteStatus, "Sign in at /signin."); return; }
    setModalState(composer, true);
    document.body.classList.add("modal-open");
    noteValue.focus();
  });
  closeBtn.addEventListener("click", () => { setModalState(composer, false); document.body.classList.remove("modal-open"); });
  composer.addEventListener("click", (event) => { if (event.target === composer) { closeBtn.click(); } });
  document.addEventListener("keydown", (event) => { if (event.key === "Escape" && composer.classList.contains("open")) { closeBtn.click(); } });
  noteForm.addEventListener("submit", async (event) => {
    event.preventDefault();
    if (!state.token) { setMessage(noteStatus, "Sign in at /signin."); return; }
    const value = noteValue.value.trim();
    if (!value) { setMessage(noteStatus, "Note text required."); return; }
    setMessage(noteStatus, "Posting...");
    try {
      await api("/notes", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ value }) });
      noteValue.value = "";
      setMessage(noteStatus, "Posted.");
      closeBtn.click();
      if (view === "me") { loadMyPosts(); } else { loadFeed(); }
    } catch (err) {
      setMessage(noteStatus, err.message);
    }
  });
})();
