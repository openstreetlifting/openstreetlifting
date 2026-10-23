import { execFileSync } from 'node:child_process';
import { watchFile } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import openapiTS, { astToString } from 'openapi-typescript';
import { format, resolveConfig } from 'prettier';

const schema = new URL('../../backend/openapi.json', import.meta.url);
const output = new URL('../src/lib/api/schema.generated.ts', import.meta.url);

if (process.argv.includes('--sync')) {
  const contents = execFileSync(
    'cargo',
    ['run', '--locked', '-p', 'osl_api', '--', '--dump-openapi'],
    {
      cwd: new URL('../../backend/', import.meta.url),
      env: { ...process.env, SQLX_OFFLINE: process.env.SQLX_OFFLINE ?? 'true' },
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'inherit'],
    }
  );
  await writeFile(schema, contents);
}

async function generate() {
  const ast = await openapiTS(schema, { enumValues: true });
  const contents = await format(astToString(ast, { formatOptions: { removeComments: true } }), {
    ...(await resolveConfig(output)),
    parser: 'typescript',
  });

  if (process.argv.includes('--check')) {
    if ((await readFile(output, 'utf8')) !== contents) {
      throw new Error('API types are stale. Run pnpm api:generate and commit the generated file.');
    }
  } else {
    await mkdir(new URL('.', output), { recursive: true });
    await writeFile(output, contents);
  }
}

await generate();

if (process.argv.includes('--watch')) {
  watchFile(schema, { interval: 500 }, () => generate().catch(console.error));
}
