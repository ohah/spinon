import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';

const repositoryRoot = path.resolve(import.meta.dirname, '../../../..');
const base = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-c11-root-html-percentage-2026-11-02/pass02');

export default {
  ...baseConfig,
  root: path.join(base, 'root'),
  outDir: path.join(base, 'site'),
  builderConfig: {
    ...baseConfig.builderConfig,
    server: { ...baseConfig.builderConfig?.server, publicDir: false },
    output: { ...baseConfig.builderConfig?.output, cleanDistPath: true },
  },
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
