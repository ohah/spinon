import path from 'node:path';
import baseConfig from '../../../../packages/docs/rspress.config.ts';

const run = process.env.SPINON_TAILNET_RUN;
if (!['pass10', 'pass11', 'pass12', 'pass13', 'pass14'].includes(run)) {
  throw new Error('SPINON_TAILNET_RUN must be pass10, pass11, pass12, pass13, or pass14');
}

export default {
  ...baseConfig,
  root: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-cyclic-grid-gap-tailnet-root-${run}`),
  outDir: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-cyclic-grid-gap-tailnet-site-${run}`),
  markdown: {
    link: { checkDeadLinks: false },
    image: { checkDeadImages: false },
  },
};
