import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
const pass = process.env.SPINON_TAILNET_PASS;
if (!['pass26-initial', 'pass26-final'].includes(pass ?? '')) {
  throw new Error('SPINON_TAILNET_PASS must be pass26-initial or pass26-final');
}

export default {
  ...baseConfig,
  root: path.join(repo, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-spans-tailnet-root-' + pass + '-selected'),
  outDir: path.join(repo, 'build/spinon/c11-4-cyclic-minmax-calc-overlapping-spans-tailnet-site-' + pass + '-retry'),
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
