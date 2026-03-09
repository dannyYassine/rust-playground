# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Environment

The project runs inside Docker. All development commands go through `docker compose`.

```bash
make up        # Start all services (db + app container) with --build
make down      # Stop all services
make watch     # Run cargo watch inside the app container (auto-reloads on change)
make shell     # Open a bash shell in the app container
make logs      # Tail app container logs
make clean     # Stop and remove volumes
```

To run commands directly inside the container:
```bash
docker compose exec app cargo test                        # Run all tests
docker compose exec app cargo test registry               # Run a single test module
docker compose exec app cargo test singleton_register_and_get  # Run a specific test
docker compose exec app cargo build
docker compose exec app cargo clippy
```

Environment variables (`HOST`, `PORT`, `DATABASE_URL`) are provided by `docker-compose.yml`. The database is Postgres 17, automatically started and health-checked before the app container.

## Architecture

### Workspace Structure

This is a Cargo workspace with three crates:
- **`axum-demo`** (`src/`) — the main Axum web application
- **`component`** (`component/`) — Askama component system with `InlineAssets` trait and `render` filter
- **`component-derive`** (`component-derive/`) — proc-macro crate providing `#[derive(Component)]` and `#[derive(Injectable)]`

### Service Registry / Dependency Injection

The core DI pattern lives in `src/services/registry.rs`. `ServiceRegistry` is a type-map built once at startup and stored in `Arc<AppState>`. It supports two registration modes:

- **Singleton** — `builder.register(MyService::new())` — same `Arc` returned on every `get::<T>()`
- **Transient** — `builder.register_type::<T>()` or `builder.register_factory(|_| ...)` — new instance on every `get::<T>()`

`get_or_new::<T>()` constructs `T` via `NewFromContainer` if not registered.

### `#[derive(Injectable)]`

The `Injectable` proc-macro (in `component-derive/src/lib.rs`) auto-generates `NewFromContainer` for a struct by inspecting its field types:

- `field: SomeStruct` → calls `SomeStruct::new_from_container(registry)`
- `field: Arc<SomeStruct>` → calls `registry.get_or_new::<SomeStruct>()`
- `#[inject(registered)] field: Arc<SomeStruct>` → calls `registry.get::<SomeStruct>().expect(...)` — use this for pre-registered third-party types like `PgPool`
- `field: Arc<dyn Trait>` → no impl generated; implement `NewFromContainer` manually

### Service Providers

`ServiceProvider` (trait in `src/services/service_provider.rs`) has `register()` and optional `boot()` methods. Providers are composed via `ApplicationServiceProvider::add_provider::<T>()` and called at startup in `main.rs`.

### Request Extraction

`Extract<T>` (in `src/services/extract.rs`) implements `FromRequestParts` — it resolves `T` from the `ServiceRegistry` via `get_or_new`. `User` also implements `FromRequestParts`, reading the `user_id` cookie and querying the DB via `UserRepo`.

### Routing

`src/routes/mod.rs` merges all sub-routers (counter, dashboard, health, htmx, login, signup, users) into a single `Router<Arc<AppState>>`.

### Templates / Components

Templates use **Askama** (`.html` files in `templates/`). The `component` crate provides:
- `InlineAssets` trait — structs implementing this alongside `Template` can attach scoped CSS/JS
- `#[derive(Component)]` — generates `InlineAssets` impl from `#[component(css = "...", js = "...")]` or file paths (`css_path`, `js_path`)
- `render` filter — use `{{ my_component|render }}` in Askama templates to render a component with its assets

### Use Cases

Business logic lives in `src/use_cases/` as plain structs (e.g. `LoginUseCase`, `SignUpUseCase`, `IncrementCounterUseCase`). They implement `NewFromContainer` (either manually or via `#[derive(Injectable)]`) and are resolved from the registry in route handlers.
