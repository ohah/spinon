import baseConfig from './rspress.config.ts';
import path from 'node:path';

const repositoryRoot = path.resolve(import.meta.dirname, '../..');
const buildRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr1329-c11-4-2026-11-10/pass01');

export default {
  ...baseConfig,
  root: path.join(buildRoot, 'root'),
  outDir: path.join(buildRoot, 'site'),
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
