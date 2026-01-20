(() => {
  const el = (id) => document.getElementById(id);
  const state = { token: localStorage.getItem("lgxpkf.session"), user: null };
  const signinLink = el("signin-link");
  const postLink = el("post-link");
  const accountToggle = el("account-toggle");
  const accountLabel = el("account-label");
  const accountMenu = el("account-menu");
  const accountEmail = el("account-email");
  const signoutBtn = el("account-signout");
  const shortLabel = (value) => value.length > 26 ? `${value.slice(0, 12)}...${value.slice(-8)}` : value;
  const updateSigninLink = () => {
    if (!signinLink) { return; }
    const next = encodeURIComponent(`${window.location.pathname}${window.location.search}`);
    signinLink.href = `/signin?next=${next}`;
  };
  const setMenuOpen = (open) => {
    if (!accountMenu || !accountToggle) { return; }
    accountMenu.hidden = !open;
    accountToggle.setAttribute("aria-expanded", open ? "true" : "false");
  };
  const setSignedIn = (signedIn) => {
    if (signinLink) { signinLink.hidden = signedIn; }
    if (postLink) { postLink.hidden = !signedIn; }
    if (accountToggle) { accountToggle.hidden = !signedIn; }
    if (!signedIn) { setMenuOpen(false); }
    if (signedIn && state.user) {
      const label = state.user.email || "Account";
      if (accountLabel) { accountLabel.textContent = shortLabel(label); }
      if (accountEmail) { accountEmail.textContent = label; }
    }
  };
  const dispatchSession = () => {
    const payload = { token: state.token, user: state.user };
    window.lgxpkfSession = payload;
    window.dispatchEvent(new CustomEvent("lgxpkf:session", { detail: payload }));
  };
  const api = async (path, options = {}) => {
    const headers = options.headers || {};
    if (state.token) { headers.Authorization = `Bearer ${state.token}`; }
    const response = await fetch(path, { ...options, headers });
    const data = await response.json().catch(() => ({}));
    if (!response.ok) { throw new Error(data.message || "Request failed"); }
    return data;
  };
  const clearSession = () => {
    localStorage.removeItem("lgxpkf.session");
    state.token = null;
    state.user = null;
  };
  const hydrate = async () => {
    if (!state.token) { setSignedIn(false); dispatchSession(); return; }
    try {
      const data = await api("/auth/me", { headers: {} });
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
  hydrate();
  if (accountToggle) {
    accountToggle.addEventListener("click", (event) => {
      event.stopPropagation();
      const open = accountMenu && accountMenu.hidden;
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
    if (!accountMenu || accountMenu.hidden) { return; }
    if (accountMenu.contains(event.target) || (accountToggle && accountToggle.contains(event.target))) { return; }
    setMenuOpen(false);
  });
  document.addEventListener("keydown", (event) => { if (event.key === "Escape") { setMenuOpen(false); } });
})();
