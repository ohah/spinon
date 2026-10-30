import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const buildRoot = path.join(repositoryRoot, 'build/spinon/c11-4-finite-ratio-tailnet-sync-2026-10-30/pass02');

export default {
  ...baseConfig,
  root: path.join(buildRoot, 'root'),
  outDir: path.join(buildRoot, 'site'),
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
