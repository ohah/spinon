import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const repositoryRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
export default {
  ...baseConfig,
  root: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-c11-plan-pr864-2026-10-31/pass02/root'),
  outDir: path.join(repositoryRoot, 'build/spinon/roadmap-tailnet-sync-c11-plan-pr864-2026-10-31/pass02/site'),
  builderConfig: { ...baseConfig.builderConfig, server: { ...baseConfig.builderConfig?.server, publicDir: false }, output: { ...baseConfig.builderConfig?.output, cleanDistPath: true } },
  markdown: { ...baseConfig.markdown, link: { ...baseConfig.markdown?.link, checkDeadLinks: false }, image: { ...baseConfig.markdown?.image, checkDeadImages: false } },
};
