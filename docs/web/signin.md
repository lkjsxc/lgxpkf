# Sign-in Page

## Purpose

- Centralized sign-in experience at `/signin`.
- Policy acceptance is collected before account creation.
- Google Identity Services is loaded only on the sign-in page.

## Flow

- Signed-out users are directed to `/signin?next={path}`.
- Sign-in page stores policy acceptance in local storage.
- On success, the session token is saved and the user is redirected to `next`.

## Theme

- The sign-in view is always rendered in dark mode.
- The Google button theme is forced to `filled_black` to avoid light-theme re-renders.
