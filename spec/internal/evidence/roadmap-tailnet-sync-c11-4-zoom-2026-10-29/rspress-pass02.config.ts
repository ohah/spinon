import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const evidenceDirectory = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidenceDirectory, '../../../..');
const taskBuildRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-c11-4-zoom-2026-10-29');

export default {
  ...baseConfig,
  root: path.join(taskBuildRoot, 'selected-root-pass02'),
  outDir: path.join(taskBuildRoot, 'selected-site-pass02'),
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
