import path from 'node:path';
import { defineConfig } from '@rspress/core';
import baseConfig from '../../../../packages/docs/rspress.config.ts';

export default defineConfig({
  ...baseConfig,
  root: path.resolve(import.meta.dirname, '../../../../build/spinon/c11-4-tailnet-root-post349-20261019'),
  outDir: path.resolve(import.meta.dirname, '../../../../build/spinon/c11-4-tailnet-site-post349-pass4-20261019'),
  markdown: {
    link: { checkDeadLinks: false },
    image: { checkDeadImages: false },
  },
});
