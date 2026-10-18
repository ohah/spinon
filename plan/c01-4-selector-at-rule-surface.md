# C01.4 Chromium selector·at-rule 표면 기준

**상태:** 계획 실행 완료 · **상태 대장 ID:** C01.4 · **상위:** C01 기준 브라우저와 CSS 기능 inventory

## 목표와 범위

고정 HTML fixture에서 Chromium이 CSS selector를 파싱하고 DOM에 매칭한 결과와 선택된 at-rule의 CSSOM rule tree를 불변 기준 자료로 기록한다. 이 자료는 Chromium 관찰 결과이며 Spinon의 selector·at-rule·cascade 지원을 뜻하지 않는다.

- selector 표본은 type, class, id, universal, attribute, descendant/child/adjacent/general sibling, `:root`, child index, `:not()`, `:is()`, `:where()`, `:has()`, `:lang()`을 포함한다. pseudo-element는 CSS rule parse 여부만 기록하고 DOM element match 결과로 취급하지 않는다. Agent Browser 사전 비교 결과의 expected match ID 순서와 error class를 inventory에 고정한다.
- 각 selector는 고정 fixture의 `data-c01-node` 값 집합으로 match 결과를 기록한다. 잘못된 selector는 예외 이름과 결과 분류를 기록한다. `:is()`의 forgiving selector-list 입력은 별도 행으로 둔다.
- at-rule 표본은 `@media`, `@supports`, `@layer`, `@container`, `@scope`, `@starting-style`, `@keyframes`, `@property`, `@font-face`, `@page`, `@counter-style`를 각각 고립된 stylesheet에서 파싱한다. 사전 비교에서 확인한 retained/dropped, top-level rule count, constructor/type, child count를 inventory에 고정하고 CSSOM의 canonical `cssText`와 nested tree 전체를 snapshot에 저장한다. 외부 자원은 로드하지 않는다.
- invalid at-rule 행은 parse/drop 경계만 기록하며 오류 복구 정책이나 전체 CSS 문법 지원을 일반화하지 않는다.
- selector match 집합은 정확히 비교하고, rule tree도 고정 Chromium 실행에서 수집한 JSON과 깊이·순서·canonical text를 정확히 비교한다. geometry·computed style·paint는 이 slice에 포함하지 않는다.
- Chromium 버전·revision·실행 파일 SHA-256·OS/build·architecture·Node·flags·viewport·DPR·locale·time zone·media·fixture/tool hashes를 기록한다. 기존 reference 디렉터리는 덮어쓰지 않는다.
- Agent Browser로 fixture의 실제 브라우저 화면을 확인하고 screenshot을 보존한다. Agent Browser의 현재 macOS locale/time zone/media가 계획 조건과 다를 수 있으므로 screenshot과 그 관찰은 시각 확인으로만 사용한다. 기준 JSON은 저장소의 고정 Chromium CDP harness가 emulation 조건을 다시 적용해 수집한다. browser version/revision이 다르면 비교 자료를 확정하지 않는다.

## 사전 비교 기준

- Chromium `155.0.8059.85` / CDP `Browser.getVersion.revision` `@0e66b94549a8bd85f03fbdf189b752d6e7b63c32`를 기준으로 고정한다. upstream branch ref `refs/branch-heads/8059@{#1304}`는 보조 provenance로만 기록한다. 로컬 실행 파일이 다르면 최신 버전을 임의 대체하지 않는다.
- CSS viewport `800×600`, DPR `1`, locale/accept-language `en-US`, time zone `UTC`, `prefers-color-scheme: light`, `prefers-reduced-motion: no-preference`, `forced-colors: none`을 고정한다.
- selector 행마다 source text, syntax acceptance, stable matched node IDs, 예외 분류를 수집한다. 단순히 non-empty match인지를 성공 기준으로 삼지 않는다.
- at-rule 행마다 source text, top-level rule count, constructor/type, canonical `cssText`, nested rule shape를 수집한다. rule 없음과 parse exception은 서로 다른 결과로 보존한다.
- Agent Browser screenshot은 oracle fixture 표시 확인용이며 Spinon 앱 화면, CSS 기능 지원, 픽셀 일치의 증거로 표시하지 않는다. Agent Browser가 계획한 locale/time zone/media 조건을 모두 재현하지 않으면 해당 관찰값은 fixed-environment JSON과 섞지 않는다. 이 자료 작업에는 Android/iOS 앱 또는 simulator 검증이 적용되지 않는다.

## 사전 비교 결과 반영

Agent Browser Chrome `155.0.8059.85`에서 일반 selector 19개·오류 selector 3개·pseudo-element rule parse 1개와 at-rule 13개를 관찰했다. `@c01-unknown`은 0 rule로 drop됐고 잘못된 media condition은 `CSSMediaRule`로 남았다. 이 차이를 `selector-at-rule-inventory.v1.json`의 expected outcome/count/type에 고정한다. locale/time zone/media가 계획 조건과 다른 Agent Browser 관찰은 사전 기준 선택에만 쓰고, 최종 JSON은 fixed CDP emulation 결과와 대조한다. 상세 조건과 관찰은 `spec/internal/evidence/css-c01-4-selector-at-rule-surface-2026-10-19/precomparison.md`에 둔다.

## 산출물

1. `tests/fixtures/css/c01/selector-at-rule-surface.html`: 외부 요청 없이 고정 node ID와 selector fixture를 제공한다.
2. `tests/fixtures/css/c01/selector-at-rule-inventory.v1.json`: 수동으로 고정한 feature ID, expected match/error, at-rule retention/count/type, selector/rule 입력과 이 slice의 명확한 미포함 범위를 정의한다.
3. `tools/css-reference/selector-at-rule-surface.mjs` 및 `.test.mjs`: exact schema, stable ID, 입력 목록·순서, match ID 소유권, rule tree, digest, reference identity 및 partial 범위를 fail-closed로 검증한다.
4. `tools/css-reference/capture-selector-at-rule-surface.mjs`: pinned Chromium 환경에서 관찰을 수행하고 source/tool/browser provenance를 포함한 새 reference JSON을 원자적으로 기록한다. 같은 reference path가 이미 있으면 실패한다.
5. `tests/fixtures/css/references/<reference-id>/selector-at-rule-surface.json`: immutable oracle 결과를 저장한다.
6. `spec/internal/evidence/css-c01-4-selector-at-rule-surface-2026-10-19/`: 실행 조건·원본 로그·Agent Browser screenshot·fixture/reference/tool hashes·한계를 기록한다.
7. `spec/STATUS.md`, `docs/plans/css-rendering.md`, `tests/fixtures/css/c01/README.md`에 C01.4 부분 범위를 기록한다. C01 부모는 미완료로 둔다.

이 작업에는 Rust/C API, 앱 동작, public API 명세, 제품 CSS 지원 판정이 없다. 계획·실행 기준·도구·원본 근거를 남기며 내부 API 계약은 추가하지 않는다.

## 완료 판정

- 동일 fixture와 고정 환경에서 selector match ID 집합 및 CSSOM rule tree가 reference와 정확히 일치한다.
- malformed/unsupported selector·at-rule 관측은 예외·drop 결과를 성공으로 바꾸지 않는다.
- browser/tool/fixture hash가 snapshot에 연결되고, schema 변조·입력 누락·중복 ID·미지 node ID·reference 경로 충돌은 거부된다.
- 별도의 구현 실패 관점 20개와 저장소 규칙에 따른 PR diff 적대 검토 30개를 기록한다.
- 이 부분 근거만으로 C01 전체나 제품 CSS 기능을 완료로 표시하지 않는다.

## 계획 수정 기록

구현 전 adversarial review에서 확인한 위험과 반영 사항은 [계획 검토 20개](../spec/internal/evidence/css-c01-4-selector-at-rule-plan-review-2026-10-19.md)에 기록한다.
