import baseConfig from '../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../..');
const buildRoot = path.join(repositoryRoot, 'build/spinon/roadmap-c11-4-text-transform-pr954-tailnet-postmerge-20261101');

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
