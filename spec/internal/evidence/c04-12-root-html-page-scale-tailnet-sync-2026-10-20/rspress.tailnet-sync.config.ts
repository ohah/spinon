import path from 'node:path';
import { defineConfig } from '@rspress/core';
import baseConfig from '../../../../packages/docs/rspress.config.ts';

const run = process.env.SPINON_TAILNET_RUN;
if (!['pass01', 'pass02', 'pass03', 'pass04', 'pass05', 'pass06', 'pass07', 'pass08'].includes(run ?? '')) {
  throw new Error('SPINON_TAILNET_RUN must be pass01, pass02, pass03, pass04, pass05, pass06, pass07, or pass08');
}

export default defineConfig({
  ...baseConfig,
  root: path.resolve(import.meta.dirname, `../../../../build/spinon/c04-12-page-scale-tailnet-root-${run}`),
  outDir: path.resolve(import.meta.dirname, `../../../../build/spinon/c04-12-page-scale-tailnet-site-${run}`),
  markdown: {
    link: { checkDeadLinks: false },
    image: { checkDeadImages: false },
  },
});
