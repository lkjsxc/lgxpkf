# Network View

## Route

- GET /network

## Behavior

- Fetches a random slice of notes from GET /notes/random.
- Expands a small association graph by requesting /notes/{id}/related.
- Renders nodes and edges in a 2D canvas layout.
- Nodes are clickable and link to the note page.
- Hover highlights nodes and their neighbors.
