# Frontend

The OpenStreetlifting website uses SvelteKit, TypeScript, and Tailwind CSS. It reads competition results and rankings from the backend API.

## Run locally

Install Node.js 24 and pnpm 12, matching CI. Start the [backend](../backend/README.md), then run from `frontend`:

```sh
pnpm install --frozen-lockfile
pnpm dev
```

Open <http://localhost:5173>. The SvelteKit server connects to the API at `http://localhost:8080` by default. To use another API URL:

```sh
BACKEND_URL=http://localhost:8081 pnpm dev
```

## Configuration

| Variable                                             | Purpose                                                                                |
| ---------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `BACKEND_URL`                                        | API base URL used by the SvelteKit server; defaults to `http://localhost:8080`         |
| `PUBLIC_SITE_URL`                                    | Site URL for canonical links and metadata; defaults to `https://openstreetlifting.org` |
| `PUBLIC_ENVIRONMENT`                                 | Environment name; any nonempty value other than `production` disables search indexing  |
| `PUBLIC_APP_VERSION`, `PUBLIC_GIT_SHA`               | Version and Git revision shown on the site                                             |
| `PUBLIC_UMAMI_SCRIPT_URL`, `PUBLIC_UMAMI_WEBSITE_ID` | Umami script URL and website ID; both are required to enable analytics                 |

## Checks

From `frontend`, run:

```sh
pnpm check
pnpm lint
pnpm test
```

`pnpm test` runs server and browser tests. Browser tests require Playwright's Chromium; install it with `pnpm exec playwright install chromium`. To run only the server tests, as CI does, use `pnpm exec vitest run --project server`.

## API contract

Rust API types and route annotations generate [OpenAPI](../backend/openapi.json). `openapi-typescript` generates `src/lib/api/schema.generated.ts`, including enum values, and `openapi-fetch` uses it to type API requests and responses. Keep generated files committed and edit the Rust definitions when changing the contract.

After changing backend types or routes, run from `frontend`:

```sh
pnpm api:sync
pnpm check
```

`api:sync` compiles the API, exports OpenAPI, and regenerates TypeScript from your current checkout, including uncommitted changes. It requires Rust but does not start the API or connect to a database. Update Utoipa annotations alongside handler signatures so the exported contract describes the HTTP API.

After pulling a schema change, `pnpm api:generate` regenerates TypeScript without compiling Rust. `pnpm dev` also runs it before starting. To regenerate whenever the local OpenAPI file changes, run `pnpm api:watch` in another terminal.

`pnpm check` verifies that generated types match the committed schema and checks frontend callers. CI also checks that OpenAPI matches the Rust definitions. Contract changes, generated files, and affected callers belong in the same pull request.

Every command reads files in the current branch. Feature development uses that branch's local contract; it never downloads a schema from `main` or a running environment. Integration and production images use their commit's checked-in types, with `BACKEND_URL` selecting the running API. Deploy compatible backend and frontend versions together; types do not change at runtime.

## Build

```sh
pnpm build
pnpm preview
```

The project uses the Node adapter. Run the built server with `node build/index.js`. Deployment configuration supplies its host, port, and API URL.
