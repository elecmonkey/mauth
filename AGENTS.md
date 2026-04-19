# AGENTS.md

## Login Frontend

`packages/login` is a separate login-side frontend.

### Tech Stack

- Svelte 5
- runes mode
- TypeScript
- Vite

### Rules

- do not downgrade to legacy syntax for Svelte 4
- in runes mode, prefer modern event attributes such as `onclick`
- use runes such as `$state`, `$derived`, and `$effect`

## Admin Frontend

`packages/admin` is the backoffice admin frontend application of this repository.

### Tech Stack

- React
- React Router
- TanStack Query
- native `fetch` based HTTP wrapper
- MUI
- TypeScript
- Vitest

### Preferred Directory Structure

The frontend package should gradually converge to a structure like this:

```text
packages/admin/src/
├── contracts/      # API contract definitions and schema validation
├── api/            # business API functions
├── http/           # fetch wrapper, request/response transformation
├── types/          # shared types and utility types
├── query/          # TanStack Query hooks
├── pages/          # route-level page components
├── components/     # reusable UI and business components
├── stores/         # client-side state, session state, UI state if needed
├── router/         # React Router configuration
├── layout/         # app shells and layout-level components
├── theme/          # MUI theme and design tokens
└── main.tsx / App.tsx
```

### Layering Rules

#### 1. Contract Layer

The `contracts/` layer defines the API contract:

- request params
- query params
- request body
- response shape

Use schema validation (Zod) for contracts when practical.

The contract layer is the source of truth for frontend/backend data expectations.

#### 2. HTTP Layer

The `http/` layer is the only place that should know how network requests are executed.

- centralize base URL handling
- centralize headers, auth header injection and error normalization
- centralize key transformation if backend uses `snake_case`

UI code must not call `fetch` directly.

#### 3. API Layer

The `api/` layer wraps concrete backend endpoints and business actions. It should primarily expose plain async functions for the `query/` layer to consume.

- call the HTTP layer
- apply contract validation
- one exported function per backend action
- functions return parsed business data, not raw `Response`

#### 4. Query Layer

The `query/` layer wraps the API layer with TanStack Query.

- define query keys
- define `useQuery` / `useInfiniteQuery` / `useMutation`
- handle invalidation
- handle cache boundaries
- keep async state logic out of pages

Pages and components should consume `query/` hooks, avoid directly calling API functions for server state.
Pages should not import `useQuery` or `useMutation` from TanStack Query directly. Those hooks belong in the `query/` layer.

#### 5. Store Layer

The `stores/` layer is for client-side state that is not just remote server state.

#### 6. Page Layer

The `pages/` layer contains route-level components.

Pages should not become very large. If a page starts doing too much, extract smaller pieces into `components/`.

#### 7. Component Layer

The `components/` layer contains reusable UI and business components.

### Data Flow

Frontend data flow should generally be:

```text
page/component -> query -> api -> http -> backend
```

And the contract layer supports `api` and `query` with shared schemas and types.

This means:

- pages do not call `fetch` directly
- pages do not manually build backend URLs
- pages do not manually normalize backend error payloads
- pages do not manually convert `snake_case` to `camelCase`

### Naming Rules

- Use `camelCase` for variables, props, functions, and object properties in frontend code.
- Use `PascalCase` for React components, page components, layout components, and type names.
- Avoid `any`. Prefer explicit types or generic utility types.
- Query hook names should follow `useXxxQuery`, `useXxxMutation`, or another clear TanStack Query style.
- API function names should use verb + noun, for example:
  - `loginAdmin`
  - `fetchAdminUsers`
  - `createAdminUser`
  - `changeMyPassword`

### API Case Convention

Frontend code should stay `camelCase`.

If the backend exposes `snake_case`, that conversion should be handled in the HTTP layer rather than leaking through the rest of the app.

### Component Size Rule

Do not write oversized components. If a page or component starts handling too many responsibilities, split it into **page container**, **feature components**, **dialogs**, **tables**, **forms**, **cards**, **utility hooks**.

### Testing

Use Vitest for frontend unit tests.
