# hello-askama

A small Rust web project built with Axum and Askama.

## Stack

- Rust (edition 2024)
- Axum
- Askama
- Tailwind CSS
- htmx

## Development Run

1. Install frontend dependencies:

```bash
npm install
```

2. Start static asset build in watch mode:

```bash
npm run dev
```

3. In a separate terminal, run the app:

```bash
cargo run
```

## Production Static Build

```bash
npm run build
```

## Structure

- `src/` - backend (routes, config, UI components)
- `templates/` - Askama templates
- `assets/` - source styles/images
- `static/` - compiled static assets
