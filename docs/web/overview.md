# Web UI Overview

## Entry Point

- GET / serves the HTML UI.
- The UI is static HTML and inline JavaScript.

## Authentication

- Google Identity Services provides an ID token.
- ID token is exchanged at POST /auth/google for a session token.
- The session token is stored in local storage and used for API calls.

## Features

- Note posting, timeline browsing, association creation, related note browsing.
