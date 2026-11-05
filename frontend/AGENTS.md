# Frontend design system

`src/app.css` holds every token value. `pnpm lint` enforces colors, font sizes and spacing, and CI runs the same checks.

## Font sizes

Pick the size by its role:

| Class           | Use for                                  |
| --------------- | ---------------------------------------- |
| `text-micro`    | field labels, chart ticks                |
| `text-meta`     | captions, dates, controls, table headers |
| `text-body`     | default reading text                     |
| `text-lead`     | introductions                            |
| `text-subtitle` | section titles on phones                 |
| `text-heading`  | section titles                           |
| `text-title`    | page titles on phones, large figures     |
| `text-display`  | page titles                              |

Prefer `<Text variant="…">`, whose variants come from `TEXT` in `src/lib/constants/typography.ts`. Add a variant when a new role appears. `em` sizes are for text that scales with its parent, such as a superscript.

## Spacing

Named values carry layout rhythm and already adapt at the `sm` breakpoint, so they need no `sm:` pair:

| Class suffix             | Use for                                  |
| ------------------------ | ---------------------------------------- |
| `gutter`, `gutter-dense` | page side margins (dense: ranking table) |
| `shell`, `shell-dense`   | page top and bottom padding              |
| `panel`                  | padding inside a panel                   |
| `tile`                   | gap between cards in a grid              |
| `section`                | gap between sections of a page           |
| `chapter`                | gap between major blocks of a long page  |
| `icon`                   | input padding that clears an icon        |
| `control`                | height of fields, buttons and page links |

Inside a component, use the numeric scale in `scripts/check-spacing.mjs`. That check covers padding, margin and gap. Size every field, button, pagination link and segmented control with `h-control` (32px) so they line up; keep `size-11` for icon-only buttons such as the menu and social links.

## Components

Exported from `#lib/components/ui/index.js`:

- `Text`: a role-based text element, with `as` for the tag.
- `Stack`: a vertical flow, with `gap` limited to the scale and the `tile`, `section` and `chapter` names.
- `Panel`: a bordered surface with `panel` padding.

`Table` draws the divider after frozen columns from its `frozenColumns` prop, so frozen cells need no border of their own.

## Gotchas

- A new token goes in both `src/app.css` and `src/lib/utils/cn.ts`. Without the second, `cn()` treats `text-foo` as a color and drops one of the two classes.
- Tailwind generates `inline-<spacing>` as a width, so a spacing token named after a display value breaks that display class: `--spacing-grid` collapsed every `inline-grid`. The spacing check rejects these names.
- A spacing token sharing a name with a `--container-*` value can also change `max-w-<name>`, so give each its own name.
- Declare a `{#snippet}` outside `<Text>`, `<Stack>` and `<Panel>`. Directly inside, Svelte passes it as a prop.

## Verifying a visual change

Lint, types and tests all passed with the `inline-grid` bug above. After changing tokens or shared components, render the affected pages in Playwright at 320, 375, 640, 768, 1440 and 2560px, and check for page overflow and collapsed controls.
