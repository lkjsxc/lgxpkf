(() => {
  const el = (id) => document.getElementById(id);
  const policyVersion = "2025-02-01";
  const consentKey = "lgxpkf.policy_acceptance.v1";
  const clientId = document.body.dataset.clientId || "";
  const loginUri = document.body.dataset.loginUri || "";
  const consentBox = el("policy-consent");
  const policyStatus = el("policy-status");
  const signinButton = el("signin-button");
  const signinStatus = el("signin-status");
  const sanitizePath = (value) => value && value.startsWith("/") && !value.startsWith("//") && !value.includes("://") ? value : "/";
  const nextParam = new URLSearchParams(window.location.search).get("next");
  const nextPath = sanitizePath(nextParam);
  const readConsent = () => {
    const raw = localStorage.getItem(consentKey);
    if (!raw) { return null; }
    try {
      const data = JSON.parse(raw);
      return data && data.accepted === true && data.version === policyVersion ? data : null;
    } catch (_) { return null; }
  };
  const writeConsent = (accepted) => {
    if (!accepted) { localStorage.removeItem(consentKey); return null; }
    const data = { accepted: true, version: policyVersion, agreed_at: new Date().toISOString() };
    localStorage.setItem(consentKey, JSON.stringify(data));
    return data;
  };
  const updateConsentUi = () => {
    const accepted = Boolean(readConsent());
    if (consentBox) { consentBox.checked = accepted; }
    if (policyStatus) { policyStatus.textContent = accepted ? "Consent recorded." : "Consent required before sign-in."; }
  };
  const ensureScript = () => {
    if (window.google && google.accounts && google.accounts.id) { initGoogle(); return; }
    if (document.getElementById("gsi-script")) { return; }
    const script = document.createElement("script");
    script.id = "gsi-script";
    script.src = "https://accounts.google.com/gsi/client";
    script.async = true;
    script.defer = true;
    script.onload = () => initGoogle();
    script.onerror = () => { if (signinStatus) { signinStatus.textContent = "Google sign-in failed to load."; } };
    document.head.appendChild(script);
  };
  const buildState = () => JSON.stringify({ path: nextPath, policy_acceptance: readConsent() });
  const initGoogle = () => {
    if (!clientId) { if (signinStatus) { signinStatus.textContent = "Missing GOOGLE_CLIENT_ID configuration."; } return; }
    if (!loginUri) { if (signinStatus) { signinStatus.textContent = "Missing PUBLIC_BASE_URL configuration."; } return; }
    if (!readConsent() || !signinButton) { return; }
    google.accounts.id.initialize({ client_id: clientId, ux_mode: "redirect", login_uri: loginUri, state: buildState() });
    google.accounts.id.renderButton(signinButton, { theme: "outline", size: "large", text: "signin_with" });
    signinButton.hidden = false;
  };
  const maybeRedirectSignedIn = async () => {
    const token = localStorage.getItem("lgxpkf.session");
    if (!token) { return; }
    try {
      const response = await fetch("/auth/me", { headers: { Authorization: `Bearer ${token}` } });
      if (!response.ok) { return; }
      window.location.replace(nextPath);
    } catch (_) { return; }
  };
  updateConsentUi();
  maybeRedirectSignedIn();
  if (consentBox) {
    consentBox.addEventListener("change", () => {
      writeConsent(consentBox.checked);
      updateConsentUi();
      if (consentBox.checked) { ensureScript(); }
      if (!consentBox.checked) { signinButton.hidden = true; }
    });
  }
  if (readConsent()) { ensureScript(); }
})();
