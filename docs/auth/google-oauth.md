# Google OAuth

## Flow

- Client obtains Google ID token.
- Server validates token with Google public keys.
- Server extracts subject and email.

## Validation

- Verify issuer and audience.
- Check signature and expiry.
- Enforce email verification.
