import { apiJson } from "./shared/api";
import { getById } from "./shared/dom";
import { readStorage, writeStorage } from "./shared/storage";

(() => {
  const storageKey = "lgxpkf.session";
  const state: LgxpkfSession = { token: readStorage(storageKey), user: null };
  const signinLink = getById<HTMLAnchorElement>("signin-link");
  const postLink = getById<HTMLAnchorElement>("post-link");
  const accountToggle = getById<HTMLButtonElement>("account-toggle");
  const accountLabel = getById<HTMLSpanElement>("account-label");
  const accountMenu = getById<HTMLDivElement>("account-menu");
  const accountEmail = getById<HTMLDivElement>("account-email");
  const signoutBtn = getById<HTMLButtonElement>("account-signout");

  const shortLabel = (value: string): string =>
    value.length > 26 ? `${value.slice(0, 12)}...${value.slice(-8)}` : value;

  const updateSigninLink = (): void => {
    if (!signinLink) {
      return;
    }
    const next = encodeURIComponent(`${window.location.pathname}${window.location.search}`);
    signinLink.href = `/signin?next=${next}`;
  };

  const setMenuOpen = (open: boolean): void => {
    if (!accountMenu || !accountToggle) {
      return;
    }
    accountMenu.hidden = !open;
    accountToggle.setAttribute("aria-expanded", open ? "true" : "false");
  };

  const setSignedIn = (signedIn: boolean): void => {
    document.body.dataset.signedIn = signedIn ? "true" : "false";
    if (signinLink) {
      signinLink.hidden = signedIn;
    }
    if (postLink) {
      postLink.hidden = !signedIn;
    }
    if (accountToggle) {
      accountToggle.hidden = !signedIn;
    }
    if (!signedIn) {
      setMenuOpen(false);
    }
    if (signedIn && state.user) {
      const label = state.user.email || "Account";
      if (accountLabel) {
        accountLabel.textContent = shortLabel(label);
      }
      if (accountEmail) {
        accountEmail.textContent = label;
      }
    }
  };

  const dispatchSession = (): void => {
    const payload: LgxpkfSession = { token: state.token, user: state.user };
    window.lgxpkfSession = payload;
    window.dispatchEvent(new CustomEvent("lgxpkf:session", { detail: payload }));
  };

  const clearSession = (): void => {
    writeStorage(storageKey, null);
    state.token = null;
    state.user = null;
  };

  const hydrate = async (): Promise<void> => {
    if (!state.token) {
      setSignedIn(false);
      dispatchSession();
      return;
    }
    try {
      const data = await apiJson<{ user: LgxpkfUserProfile }>("/auth/me", state.token, {
        headers: {},
      });
      state.user = data.user;
      setSignedIn(true);
      dispatchSession();
    } catch (_) {
      clearSession();
      setSignedIn(false);
      dispatchSession();
    }
  };

  updateSigninLink();
  void hydrate();

  if (accountToggle) {
    accountToggle.addEventListener("click", (event) => {
      event.stopPropagation();
      const open = accountMenu ? accountMenu.hidden : false;
      setMenuOpen(Boolean(open));
    });
  }

  if (signoutBtn) {
    signoutBtn.addEventListener("click", () => {
      clearSession();
      setSignedIn(false);
      dispatchSession();
      window.location.assign("/");
    });
  }

  document.addEventListener("click", (event) => {
    if (!accountMenu || accountMenu.hidden) {
      return;
    }
    if (
      accountMenu.contains(event.target as Node) ||
      (accountToggle && accountToggle.contains(event.target as Node))
    ) {
      return;
    }
    setMenuOpen(false);
  });

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      setMenuOpen(false);
    }
  });
})();
