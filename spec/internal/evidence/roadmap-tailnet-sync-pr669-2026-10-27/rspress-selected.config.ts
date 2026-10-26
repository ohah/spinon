import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr669-20261027-root-pass02'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr669-20261027-site-pass02'),
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
