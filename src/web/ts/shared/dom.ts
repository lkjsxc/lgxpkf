export const getById = <T extends HTMLElement>(id: string): T | null =>
  document.getElementById(id) as T | null;

export const setMessage = (node: HTMLElement | null, text: string): void => {
  if (node) {
    node.textContent = text;
  }
};

export const escapeHtml = (value: string): string =>
  value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/\"/g, "&quot;")
    .replace(/'/g, "&#39;");

export const setModalState = (modal: HTMLElement | null, open: boolean): void => {
  if (!modal) {
    return;
  }
  modal.classList.toggle("open", open);
  modal.setAttribute("aria-hidden", open ? "false" : "true");
  if (open) {
    modal.removeAttribute("inert");
  } else {
    modal.setAttribute("inert", "");
  }
  modal.querySelectorAll("button, input, textarea, a").forEach((node) => {
    if (open) {
      node.removeAttribute("tabindex");
    } else {
      node.setAttribute("tabindex", "-1");
    }
  });
};

export const isTypingTarget = (target: EventTarget | null): boolean => {
  if (!target || !(target instanceof HTMLElement)) {
    return false;
  }
  const tag = target.tagName.toLowerCase();
  return tag === "input" || tag === "textarea" || target.isContentEditable;
};
