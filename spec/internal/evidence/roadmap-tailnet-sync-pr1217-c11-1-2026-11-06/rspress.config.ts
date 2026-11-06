import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const repositoryRoot = path.resolve(evidence, '../../../..');
const buildRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr1217-c11-1-2026-11-06/pass01');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'spec'),
  outDir: path.join(buildRoot, 'site'),
  builderConfig: {
    ...baseConfig.builderConfig,
    output: { ...baseConfig.builderConfig?.output, cleanDistPath: true },
  },
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
