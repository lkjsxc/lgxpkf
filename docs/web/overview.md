# Web UI Overview

## Entry Point

- GET / serves the HTML UI.
- GET /{id} serves the note view rendered as Markdown.
- The UI is static HTML and inline JavaScript.

## Global Layout

- Top bar shows LGXPKF (link to /) and an account display.
- Before login the account display is a Login button; after login it shows the account identity and sign-out.
- Background color is set on html/body to prevent white flashes during navigation.

## Authentication

- Google Identity Services provides an ID token.
- ID token is exchanged at POST /auth/google for a session token.
- The session token is stored in local storage and used for API calls.
- Sign-in avoids popup windows by staying in the same tab whenever possible.

## Home Timeline

- Timeline list appears immediately under the top bar.
- Timeline uses a multi-column grid on desktop and a single column on small screens.
- Post button is fixed to the bottom-left and opens a full-screen composer.
- No association or related-note UI is presented on the home view.
- Visual design is a bluish dark mode with bold typography.

## Note Page

- Server assembles prev/next chains recursively, concatenates them into a single markdown document, and renders it.
- Newer version banners, context, and chain sections sit beneath the main note.
- Note title is not duplicated; the body is presented as a continuous document.
- Edit button opens the same composer UI used on the home page.
- Page uses the dark palette with angular surfaces.
