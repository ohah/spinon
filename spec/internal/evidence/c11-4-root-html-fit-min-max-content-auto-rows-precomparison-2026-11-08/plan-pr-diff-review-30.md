# C11.4 four-member intrinsic row-list plan PR diff review · 30 checks

**검토일:** 2026-11-08

**검토 대상:** 계획 PR diff (`plan/c11-4-grid-track-sizing.md`, `plan/c11-css-grid.md`, 새 C11.4 계획·계획 검토·Chrome 사전 비교 자료, `spec/STATUS.md`, `spec/public/roadmap.html`, 고정 HTML fixture)

**기준 HEAD:** `96bc8e2c387d521c888ffad254ffef9ca0c9c5cf` (`origin/main`, PR #1278 병합)
**판정:** 아래 30개 관점에서 계획 diff를 검토했다. 기능 구현 검토와 실제 PR diff 검토는 별도 관문으로 남겨 둔다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 계획을 C11 완료 또는 공개 CSS 지원으로 오인 | PASS — 상위 C11 체크를 미완료로 유지하고 제한 구현 계획으로 표시한다. |
| 2 | C11.4 체크 상태가 계획 PR에서 완료로 바뀜 | PASS — C11.4는 계속 `[ ]`다. |
| 3 | 공식 완료 집계가 잘못 승격 | PASS — `78/214`를 그대로 기록한다. |
| 4 | 공식 완료 수와 별도 C11.4 진행 카드를 같은 수로 취급 | PASS — roadmap 안내가 두 지표를 구분한다. |
| 5 | 현재 계획 상태를 제품 구현 시작으로 표현 | PASS — 계획 검토·oracle 완료와 구현 미착수를 명시한다. |
| 6 | PR #1277에서 이미 구현한 세 member 범위를 다시 계획 | PASS — 여섯 기존 순서는 직전 병합으로 기록하고 네 member 확장을 다음 단계로 지정한다. |
| 7 | 임의 `<track-size>` 문법 전체를 열도록 범위가 넓어짐 | PASS — 이 profile의 길이 4·네 종류 각 1회만 허용한다. |
| 8 | 24개 순열 중 특정 순서가 빠지거나 중복 | PASS — 4! 순서 공간과 96-scene matrix를 고정하고 각 order 문자열을 oracle에서 보존한다. |
| 9 | 중복·누락·추가 track member가 성공하는 허용 목록 | PASS — 계획이 중복·누락·extra 및 다른 track function의 거부를 요구한다. |
| 10 | fit-content cap에 negative/non-finite/math 입력이 들어옴 | PASS — direct finite non-negative `px`/`%`만 열고 `calc()` 등을 거부한다. |
| 11 | percentage cap의 basis가 indefinite 또는 viewport 기준으로 처리 | PASS — definite 100px block-size basis를 fixture와 계획에 고정한다. |
| 12 | computed `20%`가 computed `20px`로 정규화되어 authored identity 손실 | PASS — 기하 등가와 computed track value 보존을 별도 assertion으로 둔다. |
| 13 | max-content가 min-content로 잘못 typed되어 고정 geometry만 통과 | PASS — computed serialization과 typed projection에서 enum identity를 직접 확인한다. |
| 14 | 동일한 min/max payload를 알고리즘 동등성 증거로 과장 | PASS — fixture가 두 contribution을 구별하지 못한다는 한계를 README·계획에 명시한다. |
| 15 | CSS reference가 재현 불가능한 브라우저·viewport로 기술 | PASS — HeadlessChrome 157, macOS arm64, viewport 1280×633, DPR 1을 기록한다. |
| 16 | scene 총량 산식이 24 order × 2 cap × 2 alignment와 불일치 | PASS — 총 96 scene이며 verifier 결과가 96/96이다. |
| 17 | verifier가 Chrome 관측값을 그대로 기대값으로 재사용 | PASS — 별도 계산 로직이 고정 payload와 Grid 규칙에서 기대 row/frame을 산출한다. |
| 18 | `start` case의 explicit/implicit row 순서 또는 총합이 틀림 | PASS — `[10,20,10,12,6,20]px`, 합계 78px와 남은 22px를 명시한다. |
| 19 | `stretch`가 min/max/fit-content track에도 여유 공간을 배분 | PASS — auto max-sizing track만 늘어야 한다는 기대를 고정한다. |
| 20 | 선두 auto와 5번째 row 반복에서 균등 배분이 잘못 계산 | PASS — 두 auto track이 각각 11px 추가 공간을 받는 경우를 독립 검사한다. |
| 21 | list cycling이 첫 implicit row가 아닌 explicit row 뒤 잘못된 offset에서 시작 | PASS — explicit row 뒤 첫 implicit track부터 시작하고 5번째 implicit row에서 첫 member 반복을 고정한다. |
| 22 | cycle size assertion이 서로 다른 item contribution을 비교 | PASS — pass02에서 첫·다섯 번째 item payload를 둘 다 20px로 맞춘다. |
| 23 | 최초 잘못된 fixture 관측(pass01)이 최종 결과에 섞임 | PASS — pass01은 superseded로 보존하되 96/96 집계에서 제외한다. |
| 24 | overflow, nested Grid, named tracks, spanning 등 무관한 topology로 admission 확장 | PASS — 기존 단일 owner/direct child/fixed shape와 fail-closed negatives를 유지한다. |
| 25 | 거부된 revision이 partial layout/draw를 내거나 session을 오염 | PASS — 구현 gate에서 무발행과 동일 RuntimeSession 회복을 요구한다. |
| 26 | Android AVD/iOS Simulator smoke를 실기기·GPU·compositor 증거로 확대 | PASS — 해당 확대를 명시적으로 금지하고 표면 readback을 주장하지 않는다. |
| 27 | Chrome 캡처를 Spinon 동작 화면으로 PR 본문에 오표기 | PASS — 이미지는 Chrome oracle reference이며 앱 runtime 증거가 아니라고 분류한다. |
| 28 | 계획 검토 20개를 구현/runtime 20개 또는 PR review로 재사용 | PASS — 별도 구현/runtime 20개와 PR diff 30개를 새로 수행하도록 관문에 적었다. |
| 29 | 문서 연결·근거 해시가 누락되거나 변경 상태가 깨짐 | PASS — fixture SHA-256 `fb1dea45dfe225df0e871ec376d1c7ee91cbc8bffd531d5b7ef662408af4d369`를 재계산했고 oracle verifier가 96/96 PASS를 반환했다. 새 evidence README는 이번 RSPress link scan에서 dead-link 보고 대상이 아니었다. |
| 30 | 전체 문서 빌드 실패를 계획 변경의 green 증거로 표현 | PASS — `bun run docs:build`는 repository-wide dead-link errors 1,480건으로 종료 코드 1이다. 이 실행은 이전 baseline과의 차이를 측정하지 않았다. 전체 빌드 성공으로 주장하지 않으며 새 계획의 이미지 링크는 RSPress 오류 목록에 나타나지 않았다. |

## 검증 명령 결과

- `git diff --check`: 통과.
- `node --check spec/internal/evidence/c11-4-root-html-fit-min-max-content-auto-rows-precomparison-2026-11-08/verify-chrome-observation.mjs`: 통과.
- Independent Chrome observation verifier: `PASS`, 96/96.
- 고정 HTML fixture SHA-256: 계획에 적힌 hash와 일치.
- `mise exec -- bun run docs:build`: 실패. 1,480개 repository-wide dead-link 오류가 출력되며, 이전 baseline과의 차이 측정 없이 전체 docs build green으로 보고하지 않는다.
- Agent Browser lifecycle: oracle capture 완료 뒤 세션을 닫고 browser process 종료를 기록했다. 실제 앱 runtime 검증은 구현 PR 관문으로 남아 있다.
