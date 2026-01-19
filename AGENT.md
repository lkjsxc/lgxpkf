# rule

- This entire project is to be developed solely by an LLM, and there is no need to consider readability from a user’s perspective.

- In the documentation, directly under each directory there must be one README.md that also serves as a table of contents, along with multiple directories or multiple md files.

- Structure everything recursively and make use of directories to form a tree-like structure.

- Construct the documents so that each documentation file is within 300 lines.

- Construct the source code so that each source file is within 200 lines.

- There is absolutely no need to consider backward compatibility.

- Commit to git frequently.

- Please verify builds, etc., using docker compose.

# Code style

- Functional programming.

- Best practices.

- Performance-oriented.

# Content

- Use Rust.

- Asynchronous processing with tokio.

- Users can log in with a Google account.

- Use PostgreSQL.

- Do not use a web framework; implement everything in Rust starting from the HTTP server.

## note

- Users can post notes.

- A note is a struct and has at least the following members.

    - value: All of the user’s posted content is stored here. Size is 1024 bytes or less. Immutable.

    - id: A 32-byte unique ID. Immutable.

    - created_at: The time it was created. Immutable.

    - auther: Information about the user who created it. Immutable.

- There is something that associates notes with each other. It has a kind and the IDs of two notes. It is used for things like new versions of a note, large pieces of information created by linking multiple notes, or simply replies or quotes.

- A note can be accessed from a web browser by specifying its URL in base32.