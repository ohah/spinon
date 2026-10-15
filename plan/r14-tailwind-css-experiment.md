# R14 · Tailwind 생성 CSS 실험

**상태:** 계획 검토·실행 완료 · **공식 항목:** R14 · **범위:** 작은 Tailwind CSS 생성물을 기존 모바일 변환기 입력 경계와 대조

## 질문과 범위

고정한 Tailwind CLI가 작은 화면 스타일에서 실제로 생성하는 선택자, CSS 변수, cascade layer, Preflight 규칙을 보존한 채 현재 모바일 변환기가 받아들일 수 있는지 확인한다. 이 작업은 CSS 출력·변환 실험이며 Tailwind 유틸리티, Preflight, 반응형 상태, 제품 CSS 파이프라인의 지원 선언이 아니다.

Tailwind CSS와 CLI는 공식 v4.3.3 release를 고정한다. 생성 명령은 공식 Tailwind CLI 문서의 입력 CSS `@import "tailwindcss"`와 명시적 CLI build 형태를 따른다. 프로젝트의 현재 mobile transform oracle은 `spikes/style-layout/compile-css.mjs`의 Lightning CSS 1.33.0 AST visitor 및 fail-closed 변환 규칙이다. 이 spike는 제품 Stylo 런타임으로 연결되지 않았으므로 결과를 제품 렌더링 동작으로 확대하지 않는다.

## 재현 입력

- 작은 정적 HTML fixture에 `flex`, `items-center`, `gap-2`, `p-4`, `rounded-md`, `bg-brand-600`, `text-white`, `font-semibold`, `hover:bg-brand-700`, `sm:p-6`을 직접 적는다. 동적 class 조합이나 미탐지 파일은 사용하지 않는다.
- 입력 CSS는 Tailwind 전체 import와 `--color-brand-600`, `--color-brand-700` 사용자 theme token을 선언한다.
- source 탐지 경로를 fixture 파일 하나로 제한한다. CLI, Tailwind core, Bun, Node, PostCSS analyzer 버전을 정확한 값으로 lockfile에 기록한다. 재현 실행은 이 spike와 기존 `spikes/style-layout/` 양쪽의 frozen lockfile을 설치한다. 후자는 oracle 변환기의 Lightning CSS 1.33.0을 제공한다.
- 원본 generated CSS를 보존하고 SHA-256을 기록한다. 분석기는 PostCSS 8.5.28로 최종 출력만 읽으며, CSS를 정규화·수정해서 변환기에 전달하지 않는다.

## 사전 판정 기준

1. Tailwind CLI가 fixture에 실제로 등장하는 유틸리티만 생성한다. 생성 CSS에 선언되지 않은 별도 동적 class를 넣지 않는다.
2. custom color token이 theme layer의 CSS variable로 나오고, 두 `bg-brand-*` 유틸리티가 그 변수를 참조해야 한다.
3. 최상위 layer 순서 선언 및 theme/base/utilities layer 출력이 확인되어야 한다. class variant 규칙은 해당 media/pseudo 조건을 함께 기록한다.
4. Preflight에서 global margin/padding·box-sizing·border reset, heading reset, list reset, image sizing/display, `[hidden]` 규칙을 각각 확인한다. Preflight는 UA stylesheet와 별도 author origin임을 유지한다.
5. 전체 generated CSS를 모바일 변환기에 넣었을 때 unsupported at-rule·selector·value에서 명시적으로 실패해야 한다. 규칙을 조용히 삭제하거나 성공 JSON을 부분 출력하면 실패다.
6. Tailwind가 만든 단순 `.flex { display:flex }`만 분리한 positive control은 변환되어야 한다. `p-4`의 변수 기반 길이, hover/responsive selector·at-rule, Preflight의 global selector는 각각 거부되어야 한다.
7. 성공/실패·첫 진단·생성 규칙 분류·변수 참조·Preflight 근거를 저장한다. 이는 이 변환기 slice의 결과이며 앱의 iOS/Android runtime·simulator·제품 CSS 경로를 증명하지 않는다.
8. Chromium reference screenshot을 만들면 표준 Chromium이 CSS를 표시한 결과로만 표시하고 Spinon 실행 화면으로 제시하지 않는다. GitHub PR의 이미지 링크가 브라우저에서 실제로 로드되는지도 확인한다.

## 구현·완료 게이트

- `spikes/tailwind-generated-css/`에 pinned CLI fixture, lockfile, 생성 CSS analyzer와 재현 명령을 둔다. 생성 파일은 해당 spike의 ignore된 output 아래에 둔다.
- 실행 전에 계획의 실패 관점 20개를 검토한다. 구현 뒤에는 별도의 새 20개 구현·실행 관점을 작성한다.
- 원본 생성 CSS와 converter raw stdout/stderr를 확인한 뒤 analyzer·positive/negative control을 실행한다. 실행 전 예상치를 실행 결과로 바꾸지 않는다.
- R14 완료 표시는 이 작은 생성 CSS의 구조 분석과 제한 변환기의 통과/거부 판정에만 적용한다. CSS API 지원, 모든 utility, Tailwind 플러그인/directive, dark mode, 임의값·동적 class, Android/iOS 제품 렌더링은 미완료로 남긴다.
- PR 변경을 계획·실행 근거와 대조해 서로 다른 20개 PR 관점으로 검토한다. 기존 프로젝트 규칙에 따라 문서 PR에서 실행하지 않은 제품·플랫폼 검증은 통과로 쓰지 않는다.

## 계획 적대 검토 · 구현 전

| # | 실패 관점 | 계획에 둔 방어·판정 | 결과 |
|---:|---|---|---|
| 1 | Tailwind 버전 drift로 다른 CSS를 비교 | 공식 v4.3.3과 CLI 패키지를 정확히 고정 | 반영 |
| 2 | fixture 밖 파일에서 우연히 class를 탐지 | source 입력을 fixture 하나로 제한 | 반영 |
| 3 | class가 있지만 실제 HTML에는 없어 tree-shaken | 모든 목표 class를 정적 HTML에 직접 둠 | 반영 |
| 4 | 비표준 class를 Tailwind 지원으로 오해 | 생성 성공과 mobile transform 결과를 별도 판정 | 반영 |
| 5 | custom token 선언만 확인하고 utility 참조를 놓침 | theme variable 정의와 두 utility의 var 참조를 함께 검사 | 반영 |
| 6 | theme token의 CSS variable 생성이 utility 생성과 다름 | 생성 CSS 양쪽 연결을 독립 확인 | 반영 |
| 7 | `@layer` 선언을 삭제한 파일로 변환 성공을 주장 | 원본 전체 CSS를 첫 판정 대상으로 유지 | 반영 |
| 8 | 규칙 순서를 무시해 cascade 우선순위 손실 | 최상위 layer order와 각 rule의 layer를 기록 | 반영 |
| 9 | PostCSS analyzer가 의미 변환을 섞음 | analyzer는 읽기 전용이며 입력 원본 hash를 확인 | 반영 |
| 10 | Preflight를 UA stylesheet로 합침 | reset을 author origin으로 별도 기록 | 반영 |
| 11 | reset 중 일부만 있어도 전체 Preflight 통과 | 여섯 범주의 독립 sentinel을 사전 고정 | 반영 |
| 12 | hidden/image reset이 빠진 채 핵심 reset만 보고 | `[hidden]` 및 replaced/image sentinel 포함 | 반영 |
| 13 | selector 목록을 세지 않아 global reset 위험 누락 | global, pseudo, attribute, class selector 유형 분류 | 반영 |
| 14 | 변수 기반 길이가 px-only converter를 통과했다고 오해 | `p-4`를 독립 fail-closed negative control로 둠 | 반영 |
| 15 | 단순 양성 대조가 없어 전부 거부하는 converter를 정상으로 봄 | `.flex` positive control 요구 | 반영 |
| 16 | pseudo variant를 단순 class로 평탄화 | hover selector와 조건을 함께 확인·거부 요구 | 반영 |
| 17 | responsive media 조건을 누락 | `sm:p-6`의 at-rule과 selector를 별도 확인 | 반영 |
| 18 | 첫 오류만 보고 뒤의 미지원 구조를 놓침 | analyzer로 원본 전체 layer/rule/variable inventory 보존 | 반영 |
| 19 | Chromium 캡처를 Spinon 동작으로 오해 | reference-only 표기를 강제하고 runtime 증거에서 제외 | 반영 |
| 20 | 조사 완료를 Tailwind/CSS API 지원 완료로 표시 | R14 한정 상태 문구와 미완료 범위를 같이 기록 | 반영 |

계획 검토 결과: 서로 다른 20개 실패 관점을 검토했고 기준을 계획에 반영했다. 계획 검토와 구현 후 실행 검토는 별도로 유지한다.

실행 결과와 변환 진단은 [R14 실행 근거](../spec/internal/evidence/r14-tailwind-css-2026-10-16/README.md)에 기록했다. Tailwind 4.3.3 출력 전체는 첫 `layer-statement`에서 거부됐고, 단순 `.flex` positive control만 통과했다. 이는 계획에서 정한 fail-closed 결과이며 제품 기능 완료 판정은 아니다.
