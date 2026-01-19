# Feed API (Timeline)

## GET /feed

- Auth required.
- Query:
  - from (RFC3339, optional)
  - to (RFC3339, optional)
  - limit (1-200, optional, default 50)
- Response: 200 with list of notes for the timeline.
