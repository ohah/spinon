# C04.12 기본 root HTML 렌더링 사전 비교

새 fixture는 일반 root HTML runtime에서 필요한 실제 body text, heading weight, inline strong, 한글 줄바꿈, light/dark CSS media 환경을 고정한다. 기존 C04.12 `XX`/Ahem fixture와 목적이 다르며 새 fixture는 runtime 구현 전에 Chrome 155에서 관찰했다.

## 실행

- 브라우저: Agent Browser `0.31.1`, HeadlessChrome `155.0.0.0`, macOS ARM host
- viewport: `320×240 CSS px`, DPR 1
- scheme: Agent Browser media emulation으로 light/dark를 각각 선택
- 입력: `tests/fixtures/css/c04/root-html-default-rendering.html`
- 화면: `chrome155-light.png` / `chrome155-dark.png`, 각각 `320×240px`
- 수치: `chrome-observation.json`
- 계획 실패 관점: `plan-review-20.md`
- PR 변경 실패 관점: `pr-delta-review-20.md`

## 관찰값

| 항목 | CSS geometry | 관찰 |
| --- | --- | --- |
| `main` | `(0, 0, 320, 240)` | light `#f8fafc`, dark `#1d2939` |
| `h1` | `(16, 16, 288, 30)` | `700 24px system-ui`, light `#12345a`, dark `#93c5fd` |
| heading text Range | `(16, 16, 124.859375, 28)` | `Spinon 문서` |
| `p` | `(16, 58, 288, 40)` | `16px system-ui`, light `#263342`, dark `#e4e7eb` |
| paragraph text Range | `(16, 58, 272.40625, 38)` | Korean sentence, wraps to two lines |
| `strong` | `(155.453125, 58, 27.6875, 18)` | `700 16px system-ui` |

Chrome's resolved metrics are the host-specific reference, not a promise that iOS and Android use identical system font files. Platform verification records each resolved family and compares each simulator against its platform reference. The screenshots show rendered layout and color; they do not prove app behavior or pixel equality.

## Hashes

`SHA256SUMS` binds the fixture and both captured screenshots; run `shasum -a 256 -c spec/internal/evidence/c04-12-root-html-default-rendering-precomparison-2026-10-18/SHA256SUMS` from the repository root.

## 문서 빌드

`mise exec -- bun install --frozen-lockfile` 뒤 `mise exec -- bun run docs:build`를 실행했다. 현재 checkout 빌드는 560개의 dead-link group으로 실패했으며, build log에서 이 새 plan/evidence 경로는 dead link로 보고되지 않았다. main 기준 build와 별도 비교하지 않았으므로 560은 이 checkout에서 관찰한 값이다. 전체 문서 dead-link 정리는 이 계획 PR에 섞지 않는다.
