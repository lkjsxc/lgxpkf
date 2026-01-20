import { apiJson } from "./shared/api";
import { escapeHtml, getById, isTypingTarget, setMessage, setModalState } from "./shared/dom";
import { readStorage } from "./shared/storage";
import { decodeNotes } from "./shared/types";

type SessionState = { token: string | null; user: LgxpkfUserProfile | null };

(() => {
  const state: SessionState = { token: readStorage("lgxpkf.session"), user: null };
  const guestHero = getById<HTMLElement>("guest-hero"), randomBlock = getById<HTMLElement>("random-block"), randomList = getById<HTMLElement>("random-list"), randomStatus = getById<HTMLElement>("random-status"), timelineBlock = getById<HTMLElement>("timeline-block"), timelineTitle = getById<HTMLElement>("timeline-title"), timelineList = getById<HTMLElement>("timeline-list"), timelineStatus = getById<HTMLElement>("timeline-status"), composer = getById<HTMLElement>("composer"), postLink = getById<HTMLAnchorElement>("post-link"), closeBtn = getById<HTMLButtonElement>("close-composer"), noteForm = getById<HTMLFormElement>("note-form"), noteValue = getById<HTMLTextAreaElement>("note-value"), noteStatus = getById<HTMLElement>("note-status"), submitBtn = getById<HTMLButtonElement>("submit");

  const setSignedIn = (signedIn: boolean): void => {
    document.body.dataset.signedIn = signedIn ? "true" : "false";
    if (submitBtn) submitBtn.disabled = !signedIn;
    if (noteValue) noteValue.disabled = !signedIn;
    if (guestHero) guestHero.hidden = signedIn;
    if (randomBlock) randomBlock.hidden = signedIn;
    if (timelineBlock) timelineBlock.hidden = !signedIn;
  };

  const renderNote = (note: LgxpkfNote): HTMLAnchorElement => {
    const card = document.createElement("a");
    const author = note.author?.email || "Unknown";
    card.className = "note";
    card.href = `/${escapeHtml(note.id || "")}`;
    card.innerHTML = `<div class="note-meta"><span>${escapeHtml(note.created_at || "")}</span><span>${escapeHtml(author)}</span></div><div class="note-value">${escapeHtml(note.value || "")}</div>`;
    return card;
  };

  const loadList = async (path: string, list: HTMLElement | null, status: HTMLElement | null, emptyText: string): Promise<void> => {
    if (!list || !status) return;
    status.textContent = "Loading...";
    try {
      const payload = await apiJson(path, state.token);
      const items = decodeNotes(payload);
      list.innerHTML = "";
      if (!items.length) {
        status.textContent = emptyText;
        return;
      }
      status.textContent = "";
      items.forEach((note) => list.appendChild(renderNote(note)));
    } catch (err) {
      status.textContent = err instanceof Error ? err.message : "Timeline failed.";
    }
  };

  const loadFeed = (): Promise<void> => {
    if (timelineTitle) timelineTitle.textContent = "Timeline";
    return loadList("/feed", timelineList, timelineStatus, "Timeline is empty.");
  };

  const loadRandom = async (): Promise<void> => {
    if (!randomList || !randomStatus) return;
    randomStatus.textContent = "Loading random timeline...";
    try {
      const payload = await apiJson("/notes/random?limit=9", null);
      const items = decodeNotes(payload);
      randomList.innerHTML = "";
      if (!items.length) {
        randomStatus.textContent = "No posts yet.";
        return;
      }
      randomStatus.textContent = "";
      items.forEach((note) => randomList.appendChild(renderNote(note)));
    } catch (_) {
      randomStatus.textContent = "Random timeline unavailable.";
    }
  };

  let composeIntent = new URLSearchParams(window.location.search).get("compose") === "1";
  if (composeIntent && window.history.replaceState) window.history.replaceState({}, "", window.location.pathname);

  const openComposer = (): void => {
    if (!state.token) {
      setMessage(noteStatus, "Sign in at /signin.");
      return;
    }
    setMessage(noteStatus, "");
    setModalState(composer, true);
    document.body.classList.add("modal-open");
    noteValue?.focus();
  };

  const maybeOpenComposer = (): void => {
    if (!composeIntent || !state.token) return;
    composeIntent = false;
    openComposer();
  };

  const applySession = (session: LgxpkfSession): void => {
    state.token = session.token;
    state.user = session.user;
    setSignedIn(Boolean(state.token));
    if (state.token) void loadFeed();
    else void loadRandom();
    maybeOpenComposer();
  };

  const bootstrap = (): void => {
    setSignedIn(Boolean(state.token));
    if (state.token) void loadFeed();
    else void loadRandom();
    maybeOpenComposer();
  };

  window.addEventListener("lgxpkf:session", (event) => applySession(event.detail));
  const existing = window.lgxpkfSession;
  if (existing) applySession(existing);
  else bootstrap();

  if (postLink) {
    postLink.addEventListener("click", (event) => {
      event.preventDefault();
      openComposer();
    });
  }

  document.addEventListener("keydown", (event) => {
    if (!postLink || postLink.hidden) return;
    if (event.key !== "n" || event.ctrlKey || event.metaKey || event.altKey || event.shiftKey) return;
    if (isTypingTarget(event.target)) return;
    if (composer?.classList.contains("open")) return;
    event.preventDefault();
    openComposer();
  });

  if (noteValue) {
    noteValue.addEventListener("keydown", (event) => {
      if (event.key === "Enter" && event.ctrlKey) {
        event.preventDefault();
        if (noteForm?.requestSubmit) noteForm.requestSubmit();
        else submitBtn?.click();
      }
    });
  }

  if (closeBtn) {
    closeBtn.addEventListener("click", () => {
      setModalState(composer, false);
      document.body.classList.remove("modal-open");
    });
  }

  if (composer) {
    composer.addEventListener("click", (event) => {
      if (event.target === composer) closeBtn?.click();
    });
  }

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && composer?.classList.contains("open") && closeBtn) closeBtn.click();
  });

  if (noteForm) {
    noteForm.addEventListener("submit", async (event) => {
      event.preventDefault();
      if (!state.token) {
        setMessage(noteStatus, "Sign in at /signin.");
        return;
      }
      const value = noteValue?.value.trim() || "";
      if (!value) {
        setMessage(noteStatus, "Note text required.");
        return;
      }
      setMessage(noteStatus, "Posting...");
      try {
        await apiJson("/notes", state.token, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ value }) });
        if (noteValue) noteValue.value = "";
        setMessage(noteStatus, "Posted.");
        closeBtn?.click();
        void loadFeed();
      } catch (err) {
        setMessage(noteStatus, err instanceof Error ? err.message : "Post failed.");
      }
    });
  }
})();
