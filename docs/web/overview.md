# Web UI Overview

## Entry Point

- GET / serves the HTML UI.
- GET /{id} serves the note view rendered as Markdown.
- The UI is static HTML and inline JavaScript.

## Global Layout

- Top bar shows lgxpkf, a Post button (hidden while signed out), and the account display.
- Before login the account area shows a Sign in link; after login it shows the account display and menu only.
- Account display is hydrated after session validation to avoid flicker for logged-in users.
- Background color is a solid, darker tone on html/body to avoid uneven brightness and prevent white flashes.
- Cards and meta text enforce wrapping to prevent overflow beyond borders.
- Policy links are visible on the sign-in page.

## Authentication

- Google Identity Services provides an ID token.
- ID token is exchanged at POST /auth/google for a session token.
- The session token is stored in local storage and used for API calls.
- Sign-in avoids popup windows by staying in the same tab whenever possible.

## Home Timeline

- Timeline list appears immediately under the top bar.
- Timeline uses a multi-column grid on desktop and a single column on small screens.
- Post button lives next to the brand and opens a full-screen composer.
- Post button is hidden entirely while signed out.
- No association or related-note UI is presented on the home view.
- Visual design is a bluish dark mode with bold typography.
- Timeline cards omit the note id (metadata shows time and author only).
- Signed-out users see a hero and a random timeline on the root page.

## Note Page

- Server assembles prev/next chains recursively, concatenates them into a single markdown document, and renders it.
- Newer versions appear between the Note and Content sections and again under Associations.
- Sections are vertically ordered: note, version panel, content, associations, chain, link note.
- Associations and chain lists use multi-column grids (about three columns on desktop).
- Note title is not duplicated; the body is presented as a continuous document.
- Edit is available from the action row at the bottom of the note card (except for account bootstrap notes).
- Page uses the dark palette with reduced corner roundness.

## Network View

- GET /network renders a 2D canvas of notes and their associations.
- Nodes are positioned by a light force layout with simple interaction on hover.

## Performance + Metadata

- HTML includes meta descriptions plus Open Graph/Twitter metadata.
- DNS preconnect hints are used for Google auth and font assets.
