import path from 'node:path';
import baseConfig from '../../../../packages/docs/rspress.config.ts';

export default {
  ...baseConfig,
  root: path.resolve(import.meta.dirname, '../../../../build/spinon/c11-4-image-ratio-span-tailnet-root-20261019'),
  outDir: path.resolve(import.meta.dirname, '../../../../build/spinon/c11-4-image-ratio-span-tailnet-site-pass-2-20261019'),
  markdown: {
    link: { checkDeadLinks: false },
    image: { checkDeadImages: false },
  },
};
