import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');

export default {
  ...baseConfig,
  root: path.join(repo, 'build/spinon/c11-4-spanning-text-tailnet-sync-root-pass35-selected'),
  outDir: '/Volumes/ZTSBuild/c11-4-spanning-text-tailnet-sync-site-pass35',
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
