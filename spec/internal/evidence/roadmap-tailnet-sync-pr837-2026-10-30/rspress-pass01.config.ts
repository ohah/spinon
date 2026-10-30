import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr837-2026-10-30/pass01/root'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-pr837-2026-10-30/pass01/site'),
  builderConfig: { ...baseConfig.builderConfig, server: { ...baseConfig.builderConfig?.server, publicDir: false }, output: { ...baseConfig.builderConfig?.output, cleanDistPath: true } },
  markdown: { ...baseConfig.markdown, link: { ...baseConfig.markdown?.link, checkDeadLinks: false }, image: { ...baseConfig.markdown?.image, checkDeadImages: false } },
};
