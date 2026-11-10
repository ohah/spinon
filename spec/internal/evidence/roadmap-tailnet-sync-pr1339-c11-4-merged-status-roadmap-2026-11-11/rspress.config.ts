import baseConfig from '../../../../packages/docs/rspress.config.ts';
import path from 'node:path';

const repositoryRoot = path.resolve(import.meta.dirname, '../../../../');
const pass = process.env.SPINON_DOC_PASS ?? 'pass01';
if (!/^pass[0-9]+$/.test(pass)) throw new Error(`invalid SPINON_DOC_PASS: ${pass}`);
const buildRoot = path.join(repositoryRoot, 'build/spinon/roadmap-postmerge-pr1338-c11-4-three-fit-row-span-2026-11-11', pass);
const priorRspackConfig = baseConfig.builderConfig?.tools?.rspack;
const rspackConfigs = Array.isArray(priorRspackConfig)
  ? priorRspackConfig
  : priorRspackConfig
    ? [priorRspackConfig]
    : [];
const rspackTools = [
  ...rspackConfigs,
  (config) => {
    config.resolve = { ...config.resolve, preferRelative: true };
  },
];

export default {
  ...baseConfig,
  root: path.join(buildRoot, 'root'),
  outDir: path.join(buildRoot, 'site'),
  builderConfig: {
    ...baseConfig.builderConfig,
    tools: { ...baseConfig.builderConfig?.tools, rspack: rspackTools },
    server: { ...baseConfig.builderConfig?.server, publicDir: false },
    output: { ...baseConfig.builderConfig?.output, cleanDistPath: true },
  },
  markdown: {
    ...baseConfig.markdown,
    link: { ...baseConfig.markdown?.link, checkDeadLinks: false },
    image: { ...baseConfig.markdown?.image, checkDeadImages: false },
  },
};
