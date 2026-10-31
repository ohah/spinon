import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
const buildRoot = path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-fit-content-cap-20261031/pass05');

export default {
  ...baseConfig,
  root: path.join(buildRoot, 'root'),
  outDir: path.join(buildRoot, 'site'),
  base: '/spinon/',
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
