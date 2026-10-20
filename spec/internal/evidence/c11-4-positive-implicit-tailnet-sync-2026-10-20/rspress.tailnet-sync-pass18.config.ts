import path from 'node:path';
import baseConfig from '../../../../packages/docs/rspress.config.ts';

const run = process.env.SPINON_TAILNET_RUN;
if (run !== 'pass18') {
  throw new Error('SPINON_TAILNET_RUN must be pass18');
}

export default {
  ...baseConfig,
  root: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-positive-implicit-tailnet-root-${run}`),
  outDir: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-positive-implicit-tailnet-site-${run}`),
  markdown: {
    link: { checkDeadLinks: false },
    image: { checkDeadImages: false },
  },
};
