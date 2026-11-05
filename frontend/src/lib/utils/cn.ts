import { clsx, type ClassValue } from 'clsx';
import { extendTailwindMerge } from 'tailwind-merge';

const twMerge = extendTailwindMerge({
  extend: {
    theme: {
      text: ['micro', 'meta', 'body', 'lead', 'subtitle', 'heading', 'title', 'display'],
      spacing: [
        'gutter',
        'gutter-dense',
        'shell',
        'shell-dense',
        'panel',
        'tile',
        'section',
        'chapter',
        'icon',
      ],
    },
  },
});

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}
