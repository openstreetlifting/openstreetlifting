/// <reference types="@sveltejs/kit" />
/// <reference lib="webworker" />

import { build, files, version } from '$service-worker';

const worker = self as unknown as ServiceWorkerGlobalScope;

// Hashed bundles and static files are both immutable for the life of a
// version, so they share one cache that the next version replaces wholesale.
const CACHE = `assets-${version}`;
const PRECACHED = new Set([...build, ...files]);
const ORIGIN = `${worker.location.origin}/`;

let handle: Promise<Cache> | undefined;

function open(): Promise<Cache> {
  handle ??= caches.open(CACHE);
  return handle;
}

worker.addEventListener('install', (event) => {
  event.waitUntil(open().then((cache) => cache.addAll(build)));
});

worker.addEventListener('activate', (event) => {
  event.waitUntil(
    (async () => {
      const keys = await caches.keys();
      await Promise.all(keys.filter((key) => key !== CACHE).map((key) => caches.delete(key)));
      await worker.clients.claim();
    })()
  );
});

async function cacheFirst(request: Request): Promise<Response> {
  const cache = await open();
  const hit = await cache.match(request);
  if (hit) return hit;

  const response = await fetch(request);
  if (response.ok) cache.put(request, response.clone());

  return response;
}

async function networkFirst(request: Request): Promise<Response> {
  const cache = await open();

  try {
    const response = await fetch(request);
    if (response.ok) cache.put(request, response.clone());
    return response;
  } catch (err) {
    const hit = (await cache.match(request)) ?? (await cache.match('/'));
    if (hit) return hit;
    throw err;
  }
}

worker.addEventListener('fetch', (event) => {
  const { request } = event;
  if (request.method !== 'GET') return;
  if (!request.url.startsWith(ORIGIN)) return;

  const url = new URL(request.url);

  if (PRECACHED.has(url.pathname)) {
    event.respondWith(cacheFirst(request));
    return;
  }

  if (request.mode === 'navigate') {
    event.respondWith(networkFirst(request));
  }
});
