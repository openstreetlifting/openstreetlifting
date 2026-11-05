import { readdir, readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';

const source = new URL('../src/', import.meta.url);
const errors = [];

const arbitrarySize = /\btext-\[(?![\d.]+em\])[^\]]+\]/;
const removedStep = /(?<![\w-])text-(?:2xs|xs|sm|base|lg|xl|\dxl)(?![\w-])/;
const fontSize = /\bfont-size\s*:/;

async function check(directory, prefix = 'src') {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const url = new URL(entry.name + (entry.isDirectory() ? '/' : ''), directory);
    const path = `${prefix}/${entry.name}`;
    if (entry.isDirectory()) {
      await check(url, path);
    } else if (
      /\.(svelte|[cm]?[jt]s|css|html)$/.test(entry.name) &&
      !/\.(test|spec)\./.test(entry.name)
    ) {
      const lines = (await readFile(url, 'utf8')).split('\n');
      lines.forEach((line, index) => {
        if (arbitrarySize.test(line) || removedStep.test(line) || fontSize.test(line)) {
          errors.push(`${path}:${index + 1}: use a font size from the scale in src/app.css.`);
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
  console.log(`Type checks passed: ${fileURLToPath(source)}`);
}
