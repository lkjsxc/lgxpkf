export {};

declare global {
  interface Window {
    lgxpkfSession?: LgxpkfSession;
    google?: unknown;
  }

  interface WindowEventMap {
    "lgxpkf:session": CustomEvent<LgxpkfSession>;
  }

  interface LgxpkfUserProfile {
    user_id: string;
    email: string;
    account_note_id?: string | null;
  }

  interface LgxpkfSession {
    token: string | null;
    user: LgxpkfUserProfile | null;
  }

  interface LgxpkfNote {
    id: string;
    value: string;
    created_at: string;
    author?: LgxpkfUserProfile | null;
  }
}
