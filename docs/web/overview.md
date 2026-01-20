# Web UI Overview

## Entry Point

- GET / serves the HTML UI.
- GET /{id} serves the note view rendered as Markdown.
- The UI is static HTML and inline JavaScript.

## Global Layout

- Top bar shows LGXPKF (link to /) and an account display.
- Before login the account display is a Login button; after login it shows the account identity and sign-out only.
- Background color is a solid, darker tone on html/body to avoid uneven brightness and prevent white flashes.
- Cards and meta text enforce wrapping to prevent overflow beyond borders.

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
- Timeline cards omit the note id (metadata shows time and author only).

## Note Page

- Server assembles prev/next chains recursively, concatenates them into a single markdown document, and renders it.
- Newer version banners, context, and chain sections sit beneath the main note.
- Sections are vertically ordered: note, content, associations, chain, link note.
- Associations and chain lists use multi-column grids (about three columns on desktop).
- Note title is not duplicated; the body is presented as a continuous document.
- Edit button opens the same composer UI used on the home page.
- Page uses the dark palette with reduced corner roundness.

## Performance + Metadata

- HTML includes meta descriptions plus Open Graph/Twitter metadata.
- DNS preconnect hints are used for Google auth and font assets.
