import { escapeHtml } from "./dom";

const notePath = (id: string): string => `/${encodeURIComponent(id)}`;

const noteUrl = (id: string): string => `${window.location.origin}${notePath(id)}`;

const flashLabel = (button: HTMLButtonElement | null, text: string): void => {
  if (!button) return;
  const original = button.dataset.label || button.textContent || "";
  button.dataset.label = original;
  button.textContent = text;
  window.setTimeout(() => {
    button.textContent = button.dataset.label || original;
  }, 1600);
};

const shareNote = async (id: string, button: HTMLButtonElement | null): Promise<void> => {
  const url = noteUrl(id);
  if (navigator.share) {
    try {
      await navigator.share({ title: "lgxpkf note", url });
      flashLabel(button, "Shared");
      return;
    } catch (err) {
      if (err instanceof Error && err.name === "AbortError") return;
    }
  }
  if (navigator.clipboard) {
    await navigator.clipboard.writeText(url);
    flashLabel(button, "Copied");
    return;
  }
  window.location.assign(url);
};

export const renderNoteCard = (note: LgxpkfNote): HTMLElement => {
  const id = note.id || "";
  const card = document.createElement("article");
  const author = escapeHtml(note.author?.email || "Unknown");
  const created = escapeHtml(note.created_at || "");
  const value = escapeHtml(note.value || "");
  card.className = "note";
  card.dataset.noteId = id;
  card.innerHTML = `
    <div class="note-head">
      <div class="note-author">
        <span class="account-icon" aria-hidden="true">G</span>
        <span class="note-author-text">${author}</span>
      </div>
      <div class="note-actions">
        <button class="note-action" data-note-action="reply" type="button">Reply</button>
        <button class="note-action" data-note-action="share" type="button">Share</button>
      </div>
    </div>
    <a class="note-link" href="${notePath(id)}">
      <div class="note-meta"><span>${created}</span></div>
      <div class="note-value">${value}</div>
    </a>
  `;
  return card;
};

export const bindNoteCardActions = (container: HTMLElement | null): void => {
  if (!container) return;
  container.addEventListener("click", (event) => {
    const target = event.target as HTMLElement | null;
    const button = target?.closest<HTMLButtonElement>("[data-note-action]") || null;
    if (!button) return;
    const card = button.closest<HTMLElement>(".note");
    const noteId = card?.dataset.noteId || "";
    if (!noteId) return;
    const action = button.dataset.noteAction;
    if (action === "reply") {
      window.location.assign(`${notePath(noteId)}#link-form`);
      return;
    }
    if (action === "share") {
      void shareNote(noteId, button);
    }
  });
};
