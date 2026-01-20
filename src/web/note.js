(() => {
  const el = (id) => document.getElementById(id);
  const state = { token: localStorage.getItem("lgxpkf.session"), user: null };
  const noteId = document.body.dataset.noteId || "";
  const authorId = document.body.dataset.authorId || "";
  const accountInfo = el("account-info");
  const editBtn = el("edit-note");
  const editor = el("editor");
  const editForm = el("edit-form");
  const editValue = el("edit-value");
  const editStatus = el("edit-status");
  const closeEditor = el("close-editor");
  const relatedList = el("related-list");
  const copyBtn = el("copy-link");
  const copyStatus = el("copy-status");
  const followRow = el("follow-row");
  const followToggle = el("follow-toggle");
  const followStatus = el("follow-status");
  const linkForm = el("link-form");
  const linkTarget = el("link-target");
  const linkKind = el("link-kind");
  const linkStatus = el("link-status");
  const esc = (value) => String(value).replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/\"/g, "&quot;").replace(/'/g, "&#39;");
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
    editBtn.disabled = !signedIn;
    if (accountInfo) { accountInfo.hidden = !signedIn; }
    followRow.hidden = true;
    [linkForm, editForm].forEach((form) => {
      if (!form) { return; }
      form.querySelectorAll("input, textarea, button").forEach((node) => { node.disabled = !signedIn; });
    });
  };
  const api = async (path, options = {}) => {
    const headers = options.headers || {};
    if (state.token) { headers.Authorization = `Bearer ${state.token}`; }
    const response = await fetch(path, { ...options, headers });
    const data = await response.json().catch(() => ({}));
    if (!response.ok) { throw new Error(data.message || "Request failed"); }
    return data;
  };
  const insertVersionItem = (note) => {
    if (!note || !relatedList) { return; }
    const id = esc(note.id || "");
    if (!id || relatedList.querySelector(`a[href="/${id}"]`)) { return; }
    const summary = esc(note.value || "New version");
    const created = esc(note.created_at || "");
    const item = document.createElement("a");
    item.className = "related-item related-item-version";
    item.href = `/${id}`;
    item.innerHTML = `<span class="related-kind">VERSION (newer)</span><span class="related-text">${summary}</span><span class="related-meta">${created}</span><span class="related-cite">Citation: ${id}</span>`;
    relatedList.prepend(item);
  };
  const loadFollowState = async () => {
    if (!followRow || !followToggle || !followStatus) { return; }
    if (!state.token || !state.user || !authorId || state.user.user_id === authorId) { followRow.hidden = true; return; }
    followRow.hidden = false;
    followToggle.disabled = true;
    followStatus.textContent = "Checking follow...";
    try {
      const data = await api(`/follows?user=${state.user.user_id}&direction=following`);
      const edges = Array.isArray(data.edges) ? data.edges : [];
      const following = edges.some((edge) => edge.user && edge.user.user_id === authorId);
      followToggle.dataset.following = following ? "true" : "false";
      followToggle.textContent = following ? "Unfollow" : "Follow";
      followStatus.textContent = following ? "Following." : "Not following.";
    } catch (err) {
      followStatus.textContent = err.message;
    } finally {
      followToggle.disabled = false;
    }
  };
  const applySession = (session) => {
    state.token = session.token;
    state.user = session.user;
    setSignedIn(Boolean(state.token));
    if (state.token && state.user) { loadFollowState(); }
  };
  window.addEventListener("lgxpkf:session", (event) => applySession(event.detail));
  const existing = window.lgxpkfSession;
  if (existing) { applySession(existing); } else { setSignedIn(Boolean(state.token)); }
  copyBtn.addEventListener("click", async () => {
    if (!navigator.clipboard) { copyStatus.textContent = "Clipboard unavailable."; return; }
    try {
      await navigator.clipboard.writeText(window.location.href);
      copyStatus.textContent = "Link copied.";
    } catch (_) {
      copyStatus.textContent = "Copy failed.";
    }
  });
  followToggle.addEventListener("click", async () => {
    if (!state.token) { followStatus.textContent = "Sign in at /signin."; return; }
    if (!authorId) { followStatus.textContent = "Missing author id."; return; }
    const following = followToggle.dataset.following === "true";
    followToggle.disabled = true;
    followStatus.textContent = following ? "Unfollowing..." : "Following...";
    try {
      await api("/follows", { method: following ? "DELETE" : "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ followee_id: authorId }) });
      followToggle.dataset.following = following ? "false" : "true";
      followToggle.textContent = following ? "Follow" : "Unfollow";
      followStatus.textContent = following ? "Unfollowed." : "Following.";
    } catch (err) {
      followStatus.textContent = err.message;
    } finally {
      followToggle.disabled = false;
    }
  });
  linkForm.addEventListener("submit", async (event) => {
    event.preventDefault();
    if (!state.token) { linkStatus.textContent = "Sign in at /signin."; return; }
    const target = linkTarget.value.trim();
    const kind = (linkKind.value || "link").trim();
    if (!target) { linkStatus.textContent = "Target note id required."; return; }
    if (!kind) { linkStatus.textContent = "Association kind required."; return; }
    linkStatus.textContent = "Linking...";
    try {
      await api("/associations", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ kind, from_id: noteId, to_id: target }) });
      linkStatus.textContent = "Link created. Reload to view.";
      linkTarget.value = "";
    } catch (err) {
      linkStatus.textContent = err.message;
    }
  });
  editForm.addEventListener("submit", async (event) => {
    event.preventDefault();
    if (!state.token) { editStatus.textContent = "Sign in at /signin."; return; }
    const value = editValue.value.trim();
    if (!value) { editStatus.textContent = "Note text required."; return; }
    editStatus.textContent = "Publishing version...";
    try {
      const post = await api("/notes", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ value }) });
      const root = post.root || {};
      if (!root.id) { throw new Error("Missing version id"); }
      await api("/associations", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ kind: "version", from_id: noteId, to_id: root.id }) });
      insertVersionItem(root);
      editStatus.textContent = "Version published.";
      setModalState(editor, false);
      document.body.classList.remove("modal-open");
    } catch (err) {
      editStatus.textContent = err.message;
    }
  });
  editBtn.addEventListener("click", () => {
    if (!state.token) { editStatus.textContent = "Sign in at /signin."; return; }
    setModalState(editor, true);
    document.body.classList.add("modal-open");
    editValue.focus();
  });
  closeEditor.addEventListener("click", () => { setModalState(editor, false); document.body.classList.remove("modal-open"); });
  editor.addEventListener("click", (event) => { if (event.target === editor) { closeEditor.click(); } });
  document.addEventListener("keydown", (event) => { if (event.key === "Escape" && editor.classList.contains("open")) { closeEditor.click(); } });
})();
