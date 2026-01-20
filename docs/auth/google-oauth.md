# Google OAuth

## Flow

- Client obtains Google ID token.
- Server validates token with Google public keys.
- Server extracts subject and email.
- Client submits policy acceptance alongside the token.

## Validation

- Verify issuer and audience.
- Check signature and expiry.
- Enforce email verification.

## Redirect Handling

- GIS redirect mode uses `/auth/google/redirect`.
- The login URI is derived from `PUBLIC_BASE_URL`.
- Add `${PUBLIC_BASE_URL}/auth/google/redirect` to authorized redirect URIs.
- `redirect_uri_mismatch` indicates the base URL or authorized list is wrong.
- Redirect state includes policy acceptance and the desired post-login path.
