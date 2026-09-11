import { readdir, readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';

const source = new URL('../src/', import.meta.url);
const theme = await readFile(new URL('app.css', source), 'utf8');
const canvas = theme.match(/--color-canvas:\s*(#[\da-f]+);/i)?.[1];
const errors = [];

// Files allowed to spell the canvas out, because they cannot import it: a
// Tailwind `@theme` block and a static template. Each entry names the value it
// pins and how to read it back, so a file cannot be exempted without also
// being checked.
const PINNED = [
  ['app.html', 'theme-color', /<meta name="theme-color" content="([^"]+)"/],
  ['lib/constants/theme.ts', 'CANVAS', /export const CANVAS = '(#[\da-f]+)';/i],
];

const pinned = await Promise.all(PINNED.map(([path]) => readFile(new URL(path, source), 'utf8')));

PINNED.forEach(([path, name, pattern], index) => {
  if (pinned[index].match(pattern)?.[1] !== canvas) {
    errors.push(`src/${path}: ${name} must match --color-canvas in src/app.css.`);
  }
});

const exempt = new Set(['src/app.css', ...PINNED.map(([path]) => `src/${path}`)]);

const palette =
  /\b(?:slate|gray|zinc|neutral|stone|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)-(?:50|[1-9]00|950)\b/;
const monochrome =
  /\b(?:bg|text|border|ring|outline|fill|stroke|shadow|divide|decoration|from|via|to)-(?:white|black)\b/;
const literal = /#[\da-f]{3,8}\b|\b(?:rgb|hsl|oklch|oklab|lab|lch)a?\(/i;

async function check(directory, prefix = 'src') {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const url = new URL(entry.name + (entry.isDirectory() ? '/' : ''), directory);
    const path = `${prefix}/${entry.name}`;
    if (entry.isDirectory()) {
      await check(url, path);
    } else if (
      /\.(svelte|[cm]?[jt]s|css|html)$/.test(entry.name) &&
      !/\.(test|spec)\./.test(entry.name) &&
      !exempt.has(path)
    ) {
      const lines = (await readFile(url, 'utf8')).split('\n');
      lines.forEach((line, index) => {
        if (
          palette.test(line) ||
          monochrome.test(line) ||
          (literal.test(line) && !line.trimStart().startsWith('mask-image:'))
        ) {
          errors.push(`${path}:${index + 1}: use a semantic color from src/app.css.`);
        }
      });
    }
  }
}

await check(source);
if (errors.length) {
  console.error(errors.join('\n'));
  process.exitCode = 1;
} else {
  console.log(`Shared colors checked in ${fileURLToPath(source)}`);
}
