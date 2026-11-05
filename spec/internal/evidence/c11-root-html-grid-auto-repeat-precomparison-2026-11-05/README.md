# Parsed Root HTML Grid auto-repeat 사전 비교 · 2026-11-05

## 범위

이 자료는 `RuntimeRootHtmlTextPaintV1` 기본 parsed Root HTML 경로에 `repeat(auto-fill|auto-fit, …)` 제한 입력을 추가할지 판단하기 위한 사전 비교다. 구현 후 결과가 아니며 제품 지원 선언이나 C11 완료 근거가 아니다.

## 입력과 관찰

- Fixture: [`root-html-grid-auto-repeat-precomparison.html`](../../../../tests/fixtures/css/c11/root-html-grid-auto-repeat-precomparison.html)
- Fixture SHA-256: `996904bc7f92cb16d47614392db0fc9ad9b83e2bd684491b68b539f980405d8d`
- Browser: Agent Browser CLI 0.31.1, `HeadlessChrome/156.0.8078.80`, viewport 1280×1200, device scale factor 1.
- Observation: [`chromium-observation.json`](./chromium-observation.json)
- Screenshot: [`chromium-agent-browser.png`](./chromium-agent-browser.png), 1280×1200.
- Machine comparison: [`comparison.json`](./comparison.json), 13/13 cases pass; comparator source is [`compare.mjs`](./compare.mjs). A first comparator run incorrectly assumed every case used a 10px gap; the failed result is preserved in [`comparison-failed-fixed-gap-assertion.json`](./comparison-failed-fixed-gap-assertion.json). The corrected comparator reads each case's gap expectation.

## Normative basis

Independent expectations use [CSS Grid Layout Module Level 1, 2025 CRD §7.2.3.2](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#auto-repeat), [§11.7](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-flex-tracks), and [§11.7.1](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#algo-find-fr-size).

For a repeated two-track minimum pattern of 40px and 30px with a 10px gap, two repetitions need 140px of track minima and three gutters (30px), totaling 170px. Therefore 169px chooses one repetition and 170px chooses two. With four `1fr` tracks at 200px, the content space after gutters is 170px and each track is 42.5px. At 169px with one repetition, the two flex tracks share 159px and each is 79.5px. At the exact two-repeat boundary, the 40px minima freeze first and the two remaining tracks resolve to 30px each. `auto-fit` keeps the repeat count, then collapses empty tracks and adjacent gutters: the two occupied tracks are 95px each, one occupied track is 200px, and an empty grid has four 0px tracks. A separate 2px case uses a 0.5px minimum: §7.2.3.2 floors minimum track size to the UA 1px value for repeat-count calculation, so two tracks are repeated, while §11 sizing uses the original minimum and produces 1px per 1fr track.

## Recorded expectation correction

The first hand-derived file, [`initial-derived-expected-before-fr-audit.json`](./initial-derived-expected-before-fr-audit.json), incorrectly distributed free space on top of each track minimum. That would predict 47.5px/37.5px at 200px and 84.5px/74.5px at 169px. The CSS Grid §11.7.1 algorithm instead computes an `fr` size from the available space and freezes only tracks whose hypothetical size is below their base size. The initial file is retained as the failed expectation; corrected independent expectations are in [`normative-expected.json`](./normative-expected.json). Chrome observations matched the corrected oracle on all 13 cases.

## Evidence hashes

- Initial expectation SHA-256: `e1456874e5836a55b2571ef4e6b31ac8991c2a181a035270c20c723d4f2ce40a`
- Corrected expectation SHA-256: `b562c2f43c2a6cd4f975e3dd2b27b3b82a8c2c9cb5fa90427e98010f262db428`
- Chrome observation SHA-256: `9b73ae92fd2da4a10b5d668c77e070383aae9d17e1172f531668eba1edf75f63`
- Comparison SHA-256: `6b67300d50b1dd7a616c9caf3f381568a56f0af1fa124e952ebbdccd57202d4c`
- Comparator SHA-256: `135c1719dd2776bf73fb6f6c4c59e2aeb797380ebf925b94b88ff1a97da23b46`
- First comparator failure SHA-256: `5b7fb889a41a18eaaea439bd38f2f847931654102fa9f6b25e4f3779f27a8eed`
- Screenshot SHA-256: `21aa0eaad64a34167c6ac232da844762727f38e579a4c1eab48e763fcdbe9d72`

This precomparison only fixes parsed CSS expectations. Root HTML integration, simulator geometry, pixel evidence, resource caps, failure atomicity, and unsupported-input behavior remain to be tested in the implementation candidate.
