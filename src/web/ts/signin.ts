import { getById } from "./shared/dom";
import { readStorage, writeStorage } from "./shared/storage";
import { isRecord } from "./shared/types";

type GoogleIdConfig = {
  client_id: string;
  ux_mode: "redirect";
  login_uri: string;
  state: string;
};

type GoogleButtonConfig = {
  theme: "filled_black" | "outline";
  size: "large" | "medium" | "small";
  text: "signin_with";
  shape: "pill" | "rectangular";
};

type GoogleIdentity = {
  initialize: (config: GoogleIdConfig) => void;
  renderButton: (element: HTMLElement, options: GoogleButtonConfig) => void;
};

type GoogleAccounts = { accounts: { id: GoogleIdentity } };

const isFunction = (value: unknown): value is (...args: unknown[]) => unknown =>
  typeof value === "function";

const isGoogleIdentity = (value: unknown): value is GoogleIdentity =>
  isRecord(value) && isFunction(value.initialize) && isFunction(value.renderButton);

const getGoogleAccounts = (): GoogleAccounts | null => {
  const google = window.google;
  if (!isRecord(google)) return null;
  const accounts = google.accounts;
  if (!isRecord(accounts)) return null;
  const id = accounts.id;
  if (!isGoogleIdentity(id)) return null;
  return { accounts: { id } };
};

(() => {
  const policyVersion = "2025-02-01";
  const consentKey = "lgxpkf.policy_acceptance.v1";
  const clientId = document.body.dataset.clientId || "";
  const loginUri = document.body.dataset.loginUri || "";
  const consentBox = getById<HTMLInputElement>("policy-consent");
  const policyStatus = getById<HTMLElement>("policy-status");
  const signinButton = getById<HTMLDivElement>("signin-button");
  const signinStatus = getById<HTMLElement>("signin-status");

  const sanitizePath = (value: string | null): string =>
    value && value.startsWith("/") && !value.startsWith("//") && !value.includes("://")
      ? value
      : "/";

  const nextParam = new URLSearchParams(window.location.search).get("next");
  const nextPath = sanitizePath(nextParam);

  type PolicyAcceptance = { accepted: boolean; version: string; agreed_at: string };

  const readConsent = (): PolicyAcceptance | null => {
    const raw = readStorage(consentKey);
    if (!raw) return null;
    try {
      const data = JSON.parse(raw) as PolicyAcceptance;
      return data && data.accepted === true && data.version === policyVersion ? data : null;
    } catch (_) {
      return null;
    }
  };

  const writeConsent = (accepted: boolean): PolicyAcceptance | null => {
    if (!accepted) {
      writeStorage(consentKey, null);
      return null;
    }
    const data = { accepted: true, version: policyVersion, agreed_at: new Date().toISOString() };
    writeStorage(consentKey, JSON.stringify(data));
    return data;
  };

  const updateConsentUi = (): void => {
    const accepted = Boolean(readConsent());
    if (consentBox) consentBox.checked = accepted;
    if (policyStatus) {
      policyStatus.textContent = accepted
        ? "Consent recorded."
        : "Consent required before sign-in.";
    }
  };

  const getButtonTheme = (): "filled_black" | "outline" =>
    window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "filled_black"
      : "outline";

  const initGoogle = (): void => {
    if (!clientId) {
      if (signinStatus) signinStatus.textContent = "Missing GOOGLE_CLIENT_ID configuration.";
      return;
    }
    if (!loginUri) {
      if (signinStatus) signinStatus.textContent = "Missing PUBLIC_BASE_URL configuration.";
      return;
    }
    if (!readConsent() || !signinButton) return;
    const google = getGoogleAccounts();
    if (!google) return;
    google.accounts.id.initialize({
      client_id: clientId,
      ux_mode: "redirect",
      login_uri: loginUri,
      state: JSON.stringify({ path: nextPath, policy_acceptance: readConsent() }),
    });
    google.accounts.id.renderButton(signinButton, {
      theme: getButtonTheme(),
      size: "large",
      text: "signin_with",
      shape: "pill",
    });
    signinButton.hidden = false;
  };

  const ensureScript = (): void => {
    if (getGoogleAccounts()) {
      initGoogle();
      return;
    }
    if (document.getElementById("gsi-script")) return;
    const script = document.createElement("script");
    script.id = "gsi-script";
    script.src = "https://accounts.google.com/gsi/client";
    script.async = true;
    script.defer = true;
    script.onload = () => initGoogle();
    script.onerror = () => {
      if (signinStatus) signinStatus.textContent = "Google sign-in failed to load.";
    };
    document.head.appendChild(script);
  };

  const maybeRedirectSignedIn = async (): Promise<void> => {
    const token = readStorage("lgxpkf.session");
    if (!token) return;
    try {
      const response = await fetch("/auth/me", { headers: { Authorization: `Bearer ${token}` } });
      if (!response.ok) return;
      window.location.replace(nextPath);
    } catch (_) {
      return;
    }
  };

  updateConsentUi();
  void maybeRedirectSignedIn();

  if (consentBox) {
    consentBox.addEventListener("change", () => {
      writeConsent(consentBox.checked);
      updateConsentUi();
      if (consentBox.checked) ensureScript();
      if (!consentBox.checked && signinButton) signinButton.hidden = true;
    });
  }

  if (readConsent()) ensureScript();
})();
