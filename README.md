# hello-askama

Небольшой веб-проект на Rust с использованием Axum и Askama.

## Стек

- Rust (edition 2024)
- Axum
- Askama
- Tailwind CSS
- htmx

## Запуск в разработке

1. Установить зависимости фронтенда:

```bash
npm install
```

2. Запустить сборку статики в watch-режиме:

```bash
npm run dev
```

3. В отдельном терминале запустить приложение:

```bash
cargo run
```

## Продакшен-сборка статики

```bash
npm run build
```

## Структура

- `src/` — backend (маршруты, конфиг, UI-компоненты)
- `templates/` — шаблоны Askama
- `assets/` — исходники стилей/изображений
- `static/` — собранная статика
