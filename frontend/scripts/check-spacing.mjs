import { readdir, readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';

const source = new URL('../src/', import.meta.url);
const errors = [];

const SCALE = new Set(['0', '0.5', '1', '1.5', '2', '3', '4', '6', '8', '12', '16']);
const PROPERTIES =
  'p|px|py|pt|pb|pl|pr|ps|pe|m|mx|my|mt|mb|ml|mr|ms|me|gap|gap-x|gap-y|space-x|space-y';
const numeric = new RegExp(`(?<![\\w-])-?(?:${PROPERTIES})-(\\d+(?:\\.\\d+)?)(?![\\w.-])`, 'g');
const arbitrary = new RegExp(`(?<![\\w-])-?(?:${PROPERTIES})-\\[(?![\\d.]+em\\])[^\\]]+\\]`);
const RESERVED = new Set([
  'block',
  'flex',
  'flow',
  'grid',
  'inline',
  'table',
  'contents',
  'hidden',
]);

async function check(directory, prefix = 'src') {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const url = new URL(entry.name + (entry.isDirectory() ? '/' : ''), directory);
    const path = `${prefix}/${entry.name}`;
    if (entry.isDirectory()) {
      await check(url, path);
    } else if (/\.(svelte|[cm]?[jt]s)$/.test(entry.name) && !/\.(test|spec)\./.test(entry.name)) {
      const lines = (await readFile(url, 'utf8')).split('\n');
      lines.forEach((line, index) => {
        const offScale = [...line.matchAll(numeric)].some((match) => !SCALE.has(match[1]));
        if (offScale || arbitrary.test(line)) {
          errors.push(`${path}:${index + 1}: use a spacing value from src/app.css.`);
        }
      });
    }
  }
}

const theme = await readFile(new URL('app.css', source), 'utf8');
for (const [, name] of theme.matchAll(/--spacing-([\w-]+):/g)) {
  if (RESERVED.has(name)) {
    errors.push(`src/app.css: --spacing-${name} would turn inline-${name} into a width.`);
  }
}

await check(source);
if (errors.length) {
  console.error(errors.join('\n'));
  process.exitCode = 1;
} else {
  console.log(`Spacing checks passed: ${fileURLToPath(source)}`);
}
