import path from 'node:path';
import { defineConfig } from '@rspress/core';
import baseConfig from '../../../../packages/docs/rspress.config.ts';

const run = process.env.SPINON_TAILNET_RUN;
if (run !== 'pass1') throw new Error('SPINON_TAILNET_RUN must be pass1');

export default defineConfig({
  ...baseConfig,
  root: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-3-3-pr393-tailnet-root-${run}`),
  outDir: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-3-3-pr393-tailnet-site-${run}`),
  markdown: { link: { checkDeadLinks: false }, image: { checkDeadImages: false } },
});
