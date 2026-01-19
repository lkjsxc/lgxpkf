# Web UI Overview

## Entry Point

- GET / serves the HTML UI.
- GET /{id} serves the note view rendered as Markdown.
- The UI is static HTML and inline JavaScript.

## Global Layout

- Top bar shows LGXPKF (link to /) and an account display.
- Before login the account display is a Login button; after login it shows the account identity and sign-out.

## Authentication

- Google Identity Services provides an ID token.
- ID token is exchanged at POST /auth/google for a session token.
- The session token is stored in local storage and used for API calls.

## Home Timeline

- Timeline column is centered.
- Post button is fixed to the bottom-left and opens a full-screen composer.
- No association or related-note UI is presented on the home view.
- Visual design is a bluish dark mode with bold typography.

## Note Page

- Server assembles prev/next chains recursively, concatenates them into a single markdown document, and renders it.
- Newer version banners, context, and chain sections sit beneath the main note.
- Page uses oversized note typography in the dark palette.
