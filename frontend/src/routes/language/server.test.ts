import { describe, expect, it, vi } from 'vitest';
import { POST } from './+server';

function event(locale: string, returnTo: string, protocol = 'https:') {
  const form = new FormData();
  form.set('locale', locale);
  form.set('returnTo', returnTo);
  return {
    request: new Request(`${protocol}//openstreetlifting.org/language`, {
      method: 'POST',
      body: form,
    }),
    url: new URL(`${protocol}//openstreetlifting.org/language`),
    cookies: { set: vi.fn() },
  };
}

it('saves an HTTP-only preference and returns to the current page, filters and fragment', async () => {
  const request = event('fr', '/competitions?event=PD&page=2#results');
  await expect(POST(request as unknown as Parameters<typeof POST>[0])).rejects.toMatchObject({
    status: 303,
    location: '/competitions?event=PD&page=2#results',
  });
  expect(request.cookies.set).toHaveBeenCalledWith('osl-language', 'fr', {
    path: '/',
    maxAge: 31536000,
    httpOnly: true,
    sameSite: 'lax',
    secure: true,
  });
});

it('rejects unsupported languages before writing a cookie', async () => {
  const request = event('de', '/');
  await expect(POST(request as unknown as Parameters<typeof POST>[0])).rejects.toMatchObject({
    status: 400,
  });
  expect(request.cookies.set).not.toHaveBeenCalled();
});

describe('return paths', () => {
  it.each(['https://example.com/', '//example.com/', 'javascript:alert(1)', 'http://['])(
    'keeps %s on the same origin',
    async (returnTo) => {
      await expect(
        POST(event('en', returnTo) as unknown as Parameters<typeof POST>[0])
      ).rejects.toMatchObject({ status: 303, location: '/' });
    }
  );
});
