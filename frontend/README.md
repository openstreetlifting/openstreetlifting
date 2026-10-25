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

`pnpm test` runs server and browser tests, as CI does. Browser tests require Playwright's Chromium; install it with `pnpm exec playwright install chromium`. To run only the server tests, use `pnpm exec vitest run --project server`.

## API contract

After changing Rust DTOs or route annotations, run from `frontend`:

```sh
pnpm api:sync
pnpm check
```

Generation uses your current checkout, including uncommitted changes. Commit the [OpenAPI schema](../backend/openapi.json), generated types, and caller changes together. CI checks freshness.

Responses use `null` for missing values and omit unrequested sections. Requested empty collections are `[]`; requested sections without a result are `null`.

`pnpm dev` regenerates types from local OpenAPI. `pnpm api:watch` watches that file for changes.

Deployments use their commit's types. Set `BACKEND_URL` to a compatible API.

## Build

```sh
pnpm build
pnpm preview
```

The project uses the Node adapter. Run the built server with `node build/index.js`. Deployment configuration supplies its host, port, and API URL.
