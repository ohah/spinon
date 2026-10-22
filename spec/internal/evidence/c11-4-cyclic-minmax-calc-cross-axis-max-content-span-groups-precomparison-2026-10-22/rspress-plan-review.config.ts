import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/c11-4-c114-max-content-mixed-span-plan-root-20261022'),
  outDir: path.join(repositoryRoot, 'build/spinon/c11-4-c114-max-content-mixed-span-plan-out-pass06-20261022'),
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
