# rskel - Rust / Svelte Skeleton Application

This project includes a Svelte frontend in Typescript for a starter application that
can be extended for multiplayer web games. The backend is implemented in Rust with
the Rocket web framework and SurrealDB persistence layer.

The project implements login with Google authentication. After login, a count of
visits is displayed which increments each time the page is viewed.