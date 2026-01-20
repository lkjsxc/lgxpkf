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

- The sign-in view stays on the dark palette to match the rest of the site.
- The Google button uses the white outline theme (`outline`) for a clean light appearance.
