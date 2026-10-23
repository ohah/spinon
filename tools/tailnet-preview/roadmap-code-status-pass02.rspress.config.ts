import baseConfig from '../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

export default {
  ...baseConfig,
  root: path.join(repo, 'build/spinon/roadmap-code-status-selected-root-20261023-pass02'),
  outDir: path.join(repo, 'build/spinon/roadmap-code-status-selected-site-20261023-pass02'),
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
