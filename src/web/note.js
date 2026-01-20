(() => {
  const el = (id) => document.getElementById(id);
  const state = { token: localStorage.getItem("lgxpkf.session"), user: null };
  const noteId = document.body.dataset.noteId || "";
  const authorId = document.body.dataset.authorId || "";
  const accountNoteId = document.body.dataset.accountNoteId || "";
  const editBtn = el("edit-note");
  const editor = el("editor");
  const editForm = el("edit-form");
  const editValue = el("edit-value");
  const editStatus = el("edit-status");
  const closeEditor = el("close-editor");
  const relatedList = el("related-list");
  const versionCard = el("version-card");
  const versionList = el("version-list");
  const copyBtn = el("copy-link");
  const copyJsonBtn = el("copy-json");
  const copyStatus = el("copy-status");
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
  const isAccountNote = () => accountNoteId && accountNoteId === noteId;
  const canEdit = () => Boolean(state.token) && !isAccountNote();
  const api = async (path, options = {}) => {
    const headers = options.headers || {};
    if (state.token) { headers.Authorization = `Bearer ${state.token}`; }
    const response = await fetch(path, { ...options, headers });
    const data = await response.json().catch(() => ({}));
    if (!response.ok) { throw new Error(data.message || "Request failed"); }
    return data;
  };
  const setSignedIn = (signedIn) => {
    if (editBtn) { editBtn.disabled = !canEdit(); }
    if (followToggle) { followToggle.disabled = !signedIn; }
    if (linkForm) { linkForm.querySelectorAll("input, textarea, button").forEach((node) => { node.disabled = !signedIn; }); }
    if (editForm) { editForm.querySelectorAll("input, textarea, button").forEach((node) => { node.disabled = !canEdit(); }); }
  };
  const insertVersionItem = (note) => {
    if (!note) { return; }
    const id = esc(note.id || "");
    if (!id) { return; }
    const summary = esc(note.value || "Newer version");
    const created = esc(note.created_at || "");
    const html = `<span class="related-kind">Newer version</span><span class="related-text">${summary}</span><span class="related-meta">${created}</span><span class="related-cite">Citation: ${id}</span>`;
    if (relatedList && !relatedList.querySelector(`a[href="/${id}"]`)) {
      const item = document.createElement("a");
      item.className = "related-item related-item-version";
      item.href = `/${id}`;
      item.innerHTML = html;
      relatedList.prepend(item);
    }
    if (versionList && !versionList.querySelector(`a[href="/${id}"]`)) {
      const entry = document.createElement("a");
      entry.className = "related-item related-item-version";
      entry.href = `/${id}`;
      entry.innerHTML = html;
      versionList.prepend(entry);
      if (versionCard) { versionCard.hidden = false; }
    }
  };
  const loadFollowState = async () => {
    if (!followToggle || !followStatus) { return; }
    if (!state.token || !state.user) { followToggle.disabled = true; followStatus.textContent = "Sign in to follow."; return; }
    if (!authorId || state.user.user_id === authorId) { followToggle.disabled = true; followStatus.textContent = ""; return; }
    followToggle.disabled = true; followStatus.textContent = "Checking follow...";
    try {
      const data = await api(`/follows?user=${state.user.user_id}&direction=following`);
      const edges = Array.isArray(data.edges) ? data.edges : [];
      const following = edges.some((edge) => edge.user && edge.user.user_id === authorId);
      followToggle.dataset.following = following ? "true" : "false";
      followToggle.textContent = following ? "Unfollow" : "Follow";
      followStatus.textContent = following ? "Following." : "Not following.";
    } catch (err) { followStatus.textContent = err.message; } finally { followToggle.disabled = false; }
  };
  const applySession = (session) => { state.token = session.token; state.user = session.user; setSignedIn(Boolean(state.token)); loadFollowState(); };
  window.addEventListener("lgxpkf:session", (event) => applySession(event.detail));
  const existing = window.lgxpkfSession;
  if (existing) { applySession(existing); } else { setSignedIn(Boolean(state.token)); loadFollowState(); }
  if (copyBtn) {
    copyBtn.addEventListener("click", async () => {
      if (!navigator.clipboard) { copyStatus.textContent = "Clipboard unavailable."; return; }
      try { await navigator.clipboard.writeText(window.location.href); copyStatus.textContent = "Link copied."; }
      catch (_) { copyStatus.textContent = "Copy failed."; }
    });
  }
  if (copyJsonBtn) {
    copyJsonBtn.addEventListener("click", async () => {
      if (!navigator.clipboard) { copyStatus.textContent = "Clipboard unavailable."; return; }
      if (!noteId) { copyStatus.textContent = "Missing note id."; return; }
      try {
        const payload = await api(`/notes/${noteId}`, { headers: {} });
        await navigator.clipboard.writeText(JSON.stringify(payload, null, 2));
        copyStatus.textContent = "JSON copied.";
      } catch (err) { copyStatus.textContent = err.message || "Copy failed."; }
    });
  }
  if (followToggle) {
    followToggle.addEventListener("click", async () => {
      if (!state.token) { followStatus.textContent = "Sign in at /signin."; return; }
      if (!authorId) { followStatus.textContent = "Missing author id."; return; }
      const following = followToggle.dataset.following === "true";
      followToggle.disabled = true; followStatus.textContent = following ? "Unfollowing..." : "Following...";
      try {
        await api("/follows", { method: following ? "DELETE" : "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ followee_id: authorId }) });
        followToggle.dataset.following = following ? "false" : "true";
        followToggle.textContent = following ? "Follow" : "Unfollow";
        followStatus.textContent = following ? "Unfollowed." : "Following.";
      } catch (err) { followStatus.textContent = err.message; } finally { followToggle.disabled = false; }
    });
  }
  if (linkForm) {
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
        linkStatus.textContent = "Link created. Reload to view."; linkTarget.value = "";
      } catch (err) { linkStatus.textContent = err.message; }
    });
  }
  if (editForm) {
    editForm.addEventListener("submit", async (event) => {
      event.preventDefault();
      if (!canEdit()) { editStatus.textContent = "Editing disabled for this note."; return; }
      const value = editValue.value.trim();
      if (!value) { editStatus.textContent = "Note text required."; return; }
      editStatus.textContent = "Publishing version...";
      try {
        const post = await api("/notes", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ value }) });
        const root = post.root || {};
        if (!root.id) { throw new Error("Missing version id"); }
        await api("/associations", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ kind: "version", from_id: noteId, to_id: root.id }) });
        insertVersionItem(root); editStatus.textContent = "Version published.";
        setModalState(editor, false); document.body.classList.remove("modal-open");
      } catch (err) { editStatus.textContent = err.message; }
    });
  }
  if (editBtn) {
    editBtn.addEventListener("click", () => {
      if (!canEdit()) { editStatus.textContent = "Editing disabled for this note."; return; }
      setModalState(editor, true); document.body.classList.add("modal-open"); editValue.focus();
    });
  }
  if (closeEditor) { closeEditor.addEventListener("click", () => { setModalState(editor, false); document.body.classList.remove("modal-open"); }); }
  if (editor) { editor.addEventListener("click", (event) => { if (event.target === editor && closeEditor) { closeEditor.click(); } }); }
  document.addEventListener("keydown", (event) => { if (event.key === "Escape" && editor && editor.classList.contains("open") && closeEditor) { closeEditor.click(); } });
})();
