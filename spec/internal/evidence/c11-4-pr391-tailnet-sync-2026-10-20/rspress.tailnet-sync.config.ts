import path from 'node:path';
import { defineConfig } from '@rspress/core';
import baseConfig from '../../../../packages/docs/rspress.config.ts';

const run = process.env.SPINON_TAILNET_RUN;
if (!['pass16', 'pass17', 'pass18'].includes(run ?? '')) throw new Error('SPINON_TAILNET_RUN must be pass16, pass17, or pass18');

export default defineConfig({
  ...baseConfig,
  root: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-pr391-tailnet-root-${run}`),
  outDir: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-pr391-tailnet-site-${run}`),
  markdown: { link: { checkDeadLinks: false }, image: { checkDeadImages: false } },
});
