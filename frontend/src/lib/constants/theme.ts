// The page background, duplicated in `src/app.css` as `--color-canvas` and in
// `src/app.html` as `theme-color`, because neither a Tailwind `@theme` block
// nor a static template can import a module. `pnpm lint:colors` keeps the
// copies in step; this one is the source every importable file reads.
export const CANVAS = '#141415';
