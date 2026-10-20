import path from 'node:path';
import baseConfig from '../../../../packages/docs/rspress.config.ts';

const run = process.env.SPINON_TAILNET_RUN;
if (!['pass6', 'pass7', 'pass8', 'pass9', 'pass11', 'pass12', 'pass13', 'pass14', 'pass15', 'pass16'].includes(run)) {
  throw new Error('SPINON_TAILNET_RUN must be pass6, pass7, pass8, pass9, pass11, pass12, pass13, pass14, pass15, or pass16');
}

export default {
  ...baseConfig,
  root: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-image-tailnet-root-${run}`),
  outDir: path.resolve(import.meta.dirname, `../../../../build/spinon/c11-4-image-tailnet-site-${run}`),
  markdown: {
    link: { checkDeadLinks: false },
    image: { checkDeadImages: false },
  },
};
