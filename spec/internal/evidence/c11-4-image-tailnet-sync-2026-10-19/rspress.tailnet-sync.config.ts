import path from 'node:path';
import { defineConfig } from '@rspress/core';
import baseConfig from '../../../../packages/docs/rspress.config.ts';

const run = process.env.SPINON_TAILNET_RUN;
if (!run || !/^pass3$|^pass4$|^pass5$/.test(run)) {
  throw new Error('SPINON_TAILNET_RUN must be pass3, pass4, or pass5');
}

export default defineConfig({
  ...baseConfig,
  root: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-image-tailnet-root-${run}`),
  outDir: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-image-tailnet-site-${run}`),
  markdown: {
    link: { checkDeadLinks: false },
    image: { checkDeadImages: false },
  },
});
