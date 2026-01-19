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

## Note Page

- Server assembles prev/next chains recursively, concatenates them into a single markdown document, and renders it.
- Page is styled like a technical blog, with metadata and utility actions.
