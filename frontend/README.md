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

## Languages

The frontend supports English and French. The server uses the `osl-language` cookie when set, then the browser’s `Accept-Language` preference, with English as the fallback. Selecting a language in the header automatically saves the choice for one year and reloads the same path, filters, and fragment. The dropdown requires JavaScript; server rendering still uses the saved or browser language without it.

Locales share the existing URLs and canonical links. This keeps existing athlete and competition links stable; French does not have a separately indexable URL or `hreflang` alternate. HTML, page-data responses, and the web manifest vary by language and use private caching. The service worker caches static assets only, so offline document fallback is no longer available.

Translations live in `src/lib/i18n/en.ts` and `fr.ts`. English source messages are the translation IDs. French must cover every ID, and interpolation parameters are checked at call sites. Use `useI18n()` inside components and `i18n.t('Message with {value}', { value })` for text. Use the `Message` component for sentences with inline snippets; translation files contain no HTML. The root layout provides a separate instance for each request, following [SvelteKit’s guidance on shared state](https://svelte.dev/docs/kit/state-management#Avoid-shared-state-on-the-server).

Pass `i18n.locale` to date, number, and country formatters. Dates use UTC to preserve competition calendar dates across time zones. Keep API filter values, event codes, athlete names, competition names, and published divisions unchanged. French movement names are **muscle-up**, **traction**, **dips**, and **squat**; **RIS**, **All4**, and event codes such as **MPDS** retain their names.

When adding a language, extend `Locale`, provide its messages and formatters, add it to the picker, and extend the negotiation, catalog, SSR, and calculator tests.
