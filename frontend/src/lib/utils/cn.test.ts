import { describe, expect, it } from 'vitest';
import { cn } from './cn.js';

describe('cn', () => {
  it('keeps a named font size next to a text color', () => {
    expect(cn('text-meta', 'text-muted')).toBe('text-meta text-muted');
  });

  it('lets a later named font size replace an earlier one', () => {
    expect(cn('text-meta', 'text-body')).toBe('text-body');
  });

  it('lets a named spacing value replace a numeric one', () => {
    expect(cn('p-3', 'p-panel')).toBe('p-panel');
    expect(cn('px-panel', 'px-4')).toBe('px-4');
  });
});
