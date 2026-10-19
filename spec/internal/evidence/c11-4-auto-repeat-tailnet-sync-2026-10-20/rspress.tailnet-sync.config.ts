import path from 'node:path';
import { defineConfig } from '@rspress/core';
import baseConfig from '../../../../packages/docs/rspress.config.ts';

const run = process.env.SPINON_TAILNET_RUN;
if (!['pass10'].includes(run)) {
  throw new Error('SPINON_TAILNET_RUN must be pass10');
}

export default defineConfig({
  ...baseConfig,
  root: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-auto-repeat-tailnet-root-${run}`),
  outDir: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-auto-repeat-tailnet-site-${run}`),
  markdown: {
    link: { checkDeadLinks: false },
    image: { checkDeadImages: false },
  },
});
