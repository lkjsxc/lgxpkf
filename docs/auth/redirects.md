# Redirects

## Overview

- Google login uses GIS redirect mode to `/auth/google/redirect`.
- The login URI is built from `PUBLIC_BASE_URL` to prevent host mismatches.

## Configuration

- Set `PUBLIC_BASE_URL` to the canonical origin, including scheme.
- Add `${PUBLIC_BASE_URL}/auth/google/redirect` as an authorized redirect URI.
- Access the app through the same base URL so `state` paths resolve correctly.

## Failure Modes

- `redirect_uri_mismatch` means the computed login URI is not authorized.
