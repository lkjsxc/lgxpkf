import { apiJson, apiJsonDecoded } from "./shared/api";
import { escapeHtml, getById, setMessage, setModalState } from "./shared/dom";
import { readStorage } from "./shared/storage";
import { decodeFollowUserIds, decodePostNote } from "./shared/types";

type SessionState = { token: string | null; user: LgxpkfUserProfile | null };

(() => {
  const state: SessionState = { token: readStorage("lgxpkf.session"), user: null };
  const noteId = document.body.dataset.noteId || "", authorId = document.body.dataset.authorId || "", accountNoteId = document.body.dataset.accountNoteId || "";
  let hasNewerVersion = document.body.dataset.hasNewerVersion === "true";
  const editBtn = getById<HTMLButtonElement>("edit-note"), editor = getById<HTMLElement>("editor"), editForm = getById<HTMLFormElement>("edit-form"), editValue = getById<HTMLTextAreaElement>("edit-value"), editStatus = getById<HTMLElement>("edit-status"), closeEditor = getById<HTMLButtonElement>("close-editor"), relatedList = getById<HTMLElement>("related-list"), versionCard = getById<HTMLElement>("version-card"), versionList = getById<HTMLElement>("version-list"), copyBtn = getById<HTMLButtonElement>("copy-link"), copyJsonBtn = getById<HTMLButtonElement>("copy-json"), copyStatus = getById<HTMLElement>("copy-status"), followToggle = getById<HTMLButtonElement>("follow-toggle"), followStatus = getById<HTMLElement>("follow-status"), linkForm = getById<HTMLFormElement>("link-form"), linkTarget = getById<HTMLInputElement>("link-target"), linkKind = getById<HTMLInputElement>("link-kind"), linkStatus = getById<HTMLElement>("link-status");
  const allowedKinds = new Set(["link", "reply", "quote", "parent", "child"]);

  const isAccountNote = (): boolean => Boolean(accountNoteId && accountNoteId === noteId);
  const isOwner = (): boolean => Boolean(state.user && authorId && state.user.user_id === authorId);
  const canEdit = (): boolean => Boolean(state.token) && isOwner() && !isAccountNote();
  const canLink = (): boolean => Boolean(state.token) && isOwner() && !isAccountNote();
  const canCreateVersion = (): boolean => canEdit() && !hasNewerVersion;
  const editLockMessage = (): string => {
    if (canCreateVersion()) return "";
    if (!state.token) return "Sign in at /signin.";
    if (!isOwner()) return "Only the author can edit this note.";
    if (isAccountNote()) return "Editing disabled for this note.";
    if (hasNewerVersion) return "Newer version already exists.";
    return "Editing disabled for this note.";
  };
  const linkLockMessage = (): string => {
    if (canLink()) return "";
    if (!state.token) return "Sign in at /signin.";
    if (!isOwner()) return "Only the author can link this note.";
    if (isAccountNote()) return "Linking disabled for this note.";
    return "Linking disabled for this note.";
  };
  const setSignedIn = (signedIn: boolean): void => {
    const allowEdit = canCreateVersion();
    const allowLink = canLink();
    if (editBtn) editBtn.disabled = !allowEdit;
    if (followToggle) followToggle.disabled = !signedIn;
    if (linkForm) linkForm.querySelectorAll("input, textarea, button").forEach((node) => ((node as HTMLInputElement | HTMLTextAreaElement | HTMLButtonElement).disabled = !allowLink));
    if (editForm) editForm.querySelectorAll("input, textarea, button").forEach((node) => ((node as HTMLInputElement | HTMLTextAreaElement | HTMLButtonElement).disabled = !allowEdit));
    if (editStatus) setMessage(editStatus, allowEdit ? "Idle." : editLockMessage());
    if (linkStatus) setMessage(linkStatus, allowLink ? "" : linkLockMessage());
  };

  const insertVersionItem = (note: LgxpkfNote): void => {
    const id = escapeHtml(note.id || "");
    if (!id) return;
    const summary = escapeHtml(note.value || "Newer version");
    const created = escapeHtml(note.created_at || "");
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
      if (versionCard) versionCard.hidden = false;
    }
  };

  const loadFollowState = async (): Promise<void> => {
    if (!followToggle || !followStatus) return;
    if (!state.token || !state.user) { followToggle.disabled = true; followStatus.textContent = "Sign in to follow."; return; }
    if (!authorId || state.user.user_id === authorId) { followToggle.disabled = true; followStatus.textContent = ""; return; }
    followToggle.disabled = true; followStatus.textContent = "Checking follow...";
    try {
      const payload = await apiJson(`/follows?user=${state.user.user_id}&direction=following`, state.token);
      const followIds = decodeFollowUserIds(payload);
      const following = followIds.includes(authorId);
      followToggle.dataset.following = following ? "true" : "false";
      followToggle.textContent = following ? "Unfollow" : "Follow";
      followStatus.textContent = following ? "Following." : "Not following.";
    } catch (err) {
      setMessage(followStatus, err instanceof Error ? err.message : "Follow check failed.");
    } finally {
      followToggle.disabled = false;
    }
  };

  const applySession = (session: LgxpkfSession): void => { state.token = session.token; state.user = session.user; setSignedIn(Boolean(state.token)); void loadFollowState(); };
  window.addEventListener("lgxpkf:session", (event) => applySession(event.detail));
  const existing = window.lgxpkfSession; if (existing) applySession(existing); else { setSignedIn(Boolean(state.token)); void loadFollowState(); }

  if (copyBtn) copyBtn.addEventListener("click", async () => {
    if (!navigator.clipboard) { setMessage(copyStatus, "Clipboard unavailable."); return; }
    try { await navigator.clipboard.writeText(window.location.href); setMessage(copyStatus, "Link copied."); }
    catch (_) { setMessage(copyStatus, "Copy failed."); }
  });

  if (copyJsonBtn) copyJsonBtn.addEventListener("click", async () => {
    if (!navigator.clipboard) { setMessage(copyStatus, "Clipboard unavailable."); return; }
    if (!noteId) { setMessage(copyStatus, "Missing note id."); return; }
    try {
      const payload = await apiJson(`/notes/${noteId}`, state.token, { headers: {} });
      await navigator.clipboard.writeText(JSON.stringify(payload, null, 2));
      setMessage(copyStatus, "JSON copied.");
    } catch (err) {
      setMessage(copyStatus, err instanceof Error ? err.message : "Copy failed.");
    }
  });

  if (followToggle) followToggle.addEventListener("click", async () => {
    if (!state.token) { setMessage(followStatus, "Sign in at /signin."); return; }
    if (!authorId) { setMessage(followStatus, "Missing author id."); return; }
    const following = followToggle.dataset.following === "true";
    followToggle.disabled = true; setMessage(followStatus, following ? "Unfollowing..." : "Following...");
    try {
      await apiJson("/follows", state.token, { method: following ? "DELETE" : "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ followee_id: authorId }) });
      followToggle.dataset.following = following ? "false" : "true";
      followToggle.textContent = following ? "Follow" : "Unfollow";
      setMessage(followStatus, following ? "Unfollowed." : "Following.");
    } catch (err) {
      setMessage(followStatus, err instanceof Error ? err.message : "Follow failed.");
    } finally { followToggle.disabled = false; }
  });

  if (linkForm) linkForm.addEventListener("submit", async (event) => {
    event.preventDefault();
    if (!canLink()) { setMessage(linkStatus, linkLockMessage()); return; }
    if (!linkTarget || !linkKind) { setMessage(linkStatus, "Link form unavailable."); return; }
    const target = normalizeTarget(linkTarget.value);
    const kind = (linkKind.value || "link").trim().toLowerCase();
    if (!target) { setMessage(linkStatus, "Target note id or URL required."); return; }
    if (!kind) { setMessage(linkStatus, "Association kind required."); return; }
    if (!allowedKinds.has(kind)) { setMessage(linkStatus, kind === "version" ? "Use Edit for versions." : "Unsupported association kind."); return; }
    setMessage(linkStatus, "Linking...");
    try {
      await apiJson("/associations", state.token, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ kind, from_id: noteId, to_id: target }) });
      setMessage(linkStatus, "Link created. Reload to view.");
      linkTarget.value = "";
    } catch (err) {
      setMessage(linkStatus, err instanceof Error ? err.message : "Link failed.");
    }
  });

  if (editForm) editForm.addEventListener("submit", async (event) => {
    event.preventDefault();
    if (!canCreateVersion()) { setMessage(editStatus, editLockMessage()); return; }
    const value = editValue?.value.trim() || "";
    if (!value) { setMessage(editStatus, "Note text required."); return; }
    setMessage(editStatus, "Publishing version...");
    try {
      const root = await apiJsonDecoded(`/notes/${noteId}/versions`, state.token, decodePostNote, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ value }) });
      if (!root) throw new Error("Missing version id");
      insertVersionItem(root);
      hasNewerVersion = true; setSignedIn(Boolean(state.token));
      setMessage(editStatus, "Version published.");
      setModalState(editor, false); document.body.classList.remove("modal-open");
    } catch (err) {
      setMessage(editStatus, err instanceof Error ? err.message : "Edit failed.");
    }
  });

  if (editBtn) editBtn.addEventListener("click", () => {
    if (!canCreateVersion()) { setMessage(editStatus, editLockMessage()); return; }
    setModalState(editor, true); document.body.classList.add("modal-open"); editValue?.focus();
  });
  if (closeEditor) closeEditor.addEventListener("click", () => { setModalState(editor, false); document.body.classList.remove("modal-open"); });
  if (editor) editor.addEventListener("click", (event) => { if (event.target === editor) closeEditor?.click(); });
  document.addEventListener("keydown", (event) => { if (event.key === "Escape" && editor?.classList.contains("open") && closeEditor) closeEditor.click(); });
  if (editValue) {
    editValue.addEventListener("keydown", (event) => {
      if (event.key === "Enter" && event.ctrlKey) {
        event.preventDefault();
        if (editForm?.requestSubmit) editForm.requestSubmit();
        else editForm?.dispatchEvent(new Event("submit", { cancelable: true }));
      }
    });
  }
})();

const normalizeTarget = (value: string): string | null => {
  const trimmed = value.trim();
  if (!trimmed) return null;
  const cleaned = trimmed.split("#")[0].split("?")[0].replace(/\/+$/, "");
  const segment = cleaned.split("/").pop();
  return segment || null;
};
