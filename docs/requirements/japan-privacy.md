# Japan Privacy and Legal Baseline

## Applicable Laws and Guidance

- Act on the Protection of Personal Information (APPI) and PPC guidelines.
- Telecommunications Business Act obligations for service providers.
- Unauthorized Computer Access Law and related security expectations.

## Product Implications

- Publish a privacy notice covering data categories, purpose, retention, and contact.
- Notify users of data collection at sign-in (Google subject + email).
- Keep purpose limitation: auth, posting, timeline, follow graph.
- Document cross-border transfers and safeguards if hosting outside Japan.
- Provide access, correction, and deletion request handling.
- Maintain incident response with PPC/user notification process.
- Disclose local storage usage for session tokens.
- Clarify minimum age policy or guardian consent when relevant.
- Define retention windows and deletion procedures for dormant accounts.

## System Controls

- Minimize stored data; never store raw Google ID tokens.
- Audit log auth and write operations without secrets.
- Encrypt in transit; support at-rest encryption where feasible.
- Enforce least-privilege access and monitor privileged actions.
