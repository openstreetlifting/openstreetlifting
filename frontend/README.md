# Frontend

The OpenStreetlifting website uses SvelteKit, TypeScript, and Tailwind CSS. It reads competition results and rankings from the backend API.

## Run locally

Use Node.js 24 and pnpm 12, matching CI. Start the [backend](../backend/README.md), then run from `frontend`:

```sh
pnpm install --frozen-lockfile
pnpm dev
```

Open <http://localhost:5173>. The server connects to `http://localhost:8080` by default. To use another API:

```sh
BACKEND_URL=http://localhost:8081 pnpm dev
```

## Configuration

| Variable                                             | Purpose                                                                                |
| ---------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `BACKEND_URL`                                        | API base URL used by the SvelteKit server                                              |
| `PUBLIC_SITE_URL`                                    | Site URL for canonical links and metadata; defaults to `https://openstreetlifting.org` |
| `PUBLIC_ENVIRONMENT`                                 | Environment name; values other than `production` disable indexing when set             |
| `PUBLIC_APP_VERSION`, `PUBLIC_GIT_SHA`               | Version and revision shown on the site                                                 |
| `PUBLIC_UMAMI_SCRIPT_URL`, `PUBLIC_UMAMI_WEBSITE_ID` | Analytics settings; both are needed to enable tracking                                 |

## Checks

```sh
pnpm check
pnpm lint
pnpm test
```

Browser tests use Playwright's Chromium. Install it with `pnpm exec playwright install chromium` if it is missing.

When changing an API contract, update the frontend types and callers alongside the [backend schema](../backend/openapi.json).

## Build

```sh
pnpm build
pnpm preview
```

The project uses the Node adapter. To run the built server directly, use `node build/index.js`; deployment configuration supplies its host, port, and API URL.
