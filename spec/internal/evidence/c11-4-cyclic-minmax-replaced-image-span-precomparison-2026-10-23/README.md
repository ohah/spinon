# C11.4 cyclic `minmax(calc())`와 replaced image multi-span 사전 비교

## 입력과 실행 경계

이 사전 비교는 저장소의 [CSS fixture source](https://github.com/ohah/spinon/blob/main/tests/fixtures/css/c11/grid-track-sizing-c114-cyclic-minmax-replaced-image-precomparison.html) 하나를 Agent Browser Chrome에서 열어 측정했다. 3열 × 2행 Grid의 모든 track은 `minmax(calc(10% + 2px), max-content)`, gap은 2px, border는 2px다. Root `align-items:start`와 item `justify-self:auto;align-self:auto`를 사용한다. A/B/C 세 `<img>`는 모두 같은 PNG를 사용하며 `width:auto`, CSS height 8/12/16px로 natural 3:2 비율을 유지한다. Forward/reverse variants는 DOM order만 바꾼다.

PNG natural dimension은 24×16px이고 SHA-256은 `5d8c9509a0fec2288f02611a8ccc22a23a90033d1a96b894b439525398bce4a2`다. Agent Browser 0.31.1의 HeadlessChrome 156.0.8078.48에서 viewport 1280×633 CSS px, DPR 1을 사용했고 각 image decode 완료 후 관측했다.

## CSS Grid 2025 CRD 독립 기대값

[§7.2.1 track sizing functions](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#track-sizes), [§11.5 resolving intrinsic track sizes](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#resolve-intrinsic-track-sizes), [§11.5.1 distributing extra space across spanned tracks](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#distributing-extra-space-across-spanned-tracks)를 기준으로 기대값을 산출했다. Chrome 출력값으로부터 역산하지 않았다.

Intrinsic min-content sizing에서 cyclic percentage 성분은 intrinsic pass에서 auto처럼 처리되고 2px 길이 성분은 남는다. 초기 column bases `[2,2,2]`에서 A span-2 width 12px은 track마다 3px, B span-2 width 18px은 공유 group 안에서 track마다 6px planned increase를 제안한다. Per-track maximum 후 bases는 `[5,8,8]`이다. C의 24px span-3 width는 25px tracks+gutters 안에 들어와 추가 증가를 만들지 않는다. Rows의 `[2,2]` 초기 basis와 gap 2px에서 A/B/C heights 8/12/16px은 track별 1/3/5px increase를 제안하므로 rows는 `[7,7]`이다. Border box는 29×20px다.

80×80px definite content box에서는 양 축의 10%+2px minimum이 10px이다. 모든 image가 각 span의 available size 안에 들어오므로 columns `[10,10,10]`, rows `[10,10]`, border box 84×84px다. item identity별 frame과 case 값은 [spec-derived-expectations.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/spec-derived-expectations.json)에 담았다.

## Chrome 156 Agent Browser 관측

| Scene | Columns | Rows | Root border box | A/B/C root-relative frame |
| --- | --- | --- | --- | --- |
| `intrinsic-forward` / `intrinsic-reverse` | `[5,8,8]` | `[7,7]` | `29×20px` | A `(2,2,12,8)`, B `(9,2,18,12)`, C `(2,2,24,16)` |
| `definite-forward` / `definite-reverse` | `[10,10,10]` | `[10,10]` | `84×84px` | A `(2,2,12,8)`, B `(14,2,18,12)`, C `(2,2,24,16)` |

이미지 natural dimension, DOM item order, root 기준 frame과 computed placement를 포함한 전체 관측값은 [chrome156-observation.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/chrome156-observation.json)에 기록했다. 독립 산술은 [spec-derived-expectations.json](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/spec-derived-expectations.json)에 있고, browser screenshot은 [chrome-agent-browser-precomparison.png](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/chrome-agent-browser-precomparison.png)로 보존했다.

![Chrome 156 Agent Browser 사전 비교: replaced images와 cyclic minmax multi-span](https://raw.githubusercontent.com/ohah/spinon/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/chrome-agent-browser-precomparison.png)

계획 자체의 20개 검토와 별도로, 이 변경의 failure path 20개는 [계획 PR diff 검토](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/plan-pr-diff-review-20.md)에 남겼다.

## 문서 빌드와 로드맵 확인

RSPress 2.0.22 selected-root build는 새 계획, 계획 검토, 사전 비교 README와 PR diff 검토 페이지를 생성했다. selected config는 저장소의 오래된 dead link를 이 slice의 빌드 오류와 혼동하지 않도록 dead-link/image 검사를 끄므로 이 결과는 page compile과 route 생성만 증명한다. 재현 script, config, selected root file hash manifest와 build log는 아래 evidence에 있다.

기본 전체 `docs:build`는 계속 실패한다. 최종 실행은 774개 기존 문서/evidence 경로의 dead-link 오류를 보고했고, 이번 계획·사전 비교 evidence 경로는 그 목록에 없었다. 이는 전체 저장소 링크 검사가 통과했다는 의미가 아니다. [Agent Browser roadmap source observation](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/roadmap-browser-verification.json)은 로컬 `spec/public/roadmap.html`에서 새 card와 5개 href를 확인한다. Tailnet이 제공하는 live 경로는 계획 PR 병합 뒤 sync 단계에서 다시 확인한다.

- [selected-root preparation manifest](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/selected-root-preparation-pass08.json)
- [selected-root preparation script](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/prepare-selected-root-pass08.mjs)
- [selected RSPress config](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/rspress-pass08.config.ts) · [build log](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/selected-build-pass08.log)
- [build·roadmap 검증 요약 JSON](https://github.com/ohah/spinon/blob/main/spec/internal/evidence/c11-4-cyclic-minmax-replaced-image-span-precomparison-2026-10-23/docs-build-summary-pass08.json)

## 한계

이 evidence는 Chrome browser layout 관측과 spec-derived expectation만 담는다. Spinon runtime, Android/iOS 앱 draw/present, PixelCopy, Metal drawable, 기본 artifact 제외, WPT, image decoding 오류, arbitrary CSS admission, 실기기/hardware GPU, iOS compositor 이후 pixels와 전체 C11.4를 검증하지 않았다. 구현 gate는 [집중 계획](https://github.com/ohah/spinon/blob/main/plan/c11-4-cyclic-minmax-replaced-image-span.md)에 별도로 적었다.
