# PR #619 계획 변경 diff 최종 적대 검토 30

## 검토 기준과 변경 경로

검토 범위는 `origin/main...HEAD`이며, base는 `66b74744f33f35567b21709dd38cb5658460bb98`, 최종 HEAD는 `f5d415974eeff46c61f665568c598133deab999b`다. diff에는 아래 11개 경로가 있고 제품 코드 변경은 없다.

1. `plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md`
2. `plan/c11-4-mixed-direct-text-image-intrinsic-row-axis-plan-review-20.md`
3. `spec/STATUS.md`
4. `spec/internal/0059-c11-grid-layout.md`
5. `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md`
6. `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-agent-browser-dom-geometry.json`
7. `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-agent-browser.png`
8. `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-fixture.html`
9. `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-observation-original.txt`
10. `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/pr619-row-axis-image-agent-browser.png`
11. `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/pr619-row-axis-image-verification.json`

기존 수정 검토의 30개 failure view를 모두 다시 확인하고 B1/B2 수정 및 현재 diff를 대조했다. 테스트, 빌드, 브라우저 세션은 실행하지 않았다. 보존된 screenshot은 파일 뷰어로 확인했고, 커밋된 JSON/hash/link와 worktree의 HTML 원본을 읽기 전용으로 검증했다.

## 관점별 결과

| # | 독립 failure view | 최종 결과 |
|---:|---|---|
| 1 | 변경이 계획 초안을 구현 완료나 runtime 통과로 오인시키는가 | 통과. 계획은 “제한 fixture 계획 초안”이며 runtime 및 Android/iOS는 구현 후 gate로 남긴다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:9`). |
| 2 | 새 행 축 slice를 선행 #611 column slice와 합쳐 범위를 과장하는가 | 통과. fixed rows에서 column을 검사한 #611과 새 intrinsic row 후속을 구분한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:9,13`). |
| 3 | 선행 계획의 intrinsic-row 제외 범위와 충돌하는가 | 통과. 제외됐던 row axis를 별도 후속으로 명시하고 내부 계약에도 후속 후보로 기록한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:13`; `spec/internal/0059-c11-grid-layout.md:496,498-500`). |
| 4 | direct text element와 anonymous raw-text Grid item을 혼동하는가 | 통과. placement가 가능한 direct `<div>`를 쓰고 anonymous Grid text는 명시적으로 제외한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:15-17`). |
| 5 | 정확한 fixture scope가 public CSS/general author input으로 넓어지는가 | 통과. opt-in ID, exact shape, fail-closed rejection, 공개 API/support 제외를 고정한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:17,56-57`). |
| 6 | C11.4 checkbox 또는 aggregate 완료 수가 바뀌는가 | 통과. STATUS와 0059는 미완료 및 `72/213`을 유지하고 C11 checkbox도 `[ ]`다 (`spec/STATUS.md:259,275`; `spec/internal/0059-c11-grid-layout.md:500`). |
| 7 | 표준 근거에서 CRD와 Working Draft의 역할을 혼동하는가 | 통과. CSS Grid 2025 CRD를 계산 근거, CSS Sizing 2026 WD를 용어 참고로 구분한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:21`; evidence `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:29`). 외부 링크는 브라우저 금지 조건에 따라 live fetch하지 않았다. |
| 8 | fixed columns를 먼저 계산하고 그 inline size로 row를 평가하는 순서가 누락되는가 | 통과. CSS Grid 열→행 sizing 의존성과 50px inline size 사용을 명시한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:21,34`). |
| 9 | Ahem font readiness가 보장되지 않아 다른 wrap을 관찰하는가 | 통과. font readiness/hash를 precondition으로 두며 관측 JSON에 Ahem loaded, `checkAhem=true`, 80px canvas width가 있다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis-plan-review-20.md:7`; evidence `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:5`). |
| 10 | `XX XXXXX` 줄바꿈과 Ahem glyph advance 계산이 잘못되는가 | 통과. 20px/50px 단어와 10px line-height로 두 line box, text block contribution 20px을 독립 계산한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:34`; evidence `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:31`). |
| 11 | `min-content`와 `max-content` 결과가 다르다고 잘못 주장하는가 | 통과. 둘 다 20px이며 이 입력으로 두 알고리즘 경로를 구별할 수 없다고 계획과 evidence에 공개한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:32,45`; evidence `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:35`). |
| 12 | PNG decode/natural size가 불확실하거나 image contribution이 잘못되는가 | 통과. decode 성공 뒤 `24×16px` natural size와 16px block contribution을 요구하며 관측 JSON도 동일 값을 기록한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:30,34`; `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-agent-browser-dom-geometry.json` `imageNaturalSize`). |
| 13 | row sizing의 contribution aggregation 기대값이 틀리는가 | 통과. 두 span-1 item의 같은-kind contribution 최댓값 `max(20,16)=20px`를 독립 산출한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:34-41`). |
| 14 | implicit placement가 발생하거나 item이 다른 cell/row로 이동하는가 | 통과. text는 column 1/row 1, image는 column 2/row 1로 모든 scene에서 명시한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:15,29-31`; `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-fixture.html:12-19`). |
| 15 | DOM order reverse가 item placement까지 바꾸는가 | 통과. 순서만 바꾸고 개별 placement를 고정한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:31`; `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-fixture.html:16-19`; `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-agent-browser-dom-geometry.json` `cases[].sourceOrder`). |
| 16 | root 폭, 열 합계, auto block-size가 기대 frame을 흔드는가 | 통과. content width 74px, columns 50px+24px, auto height와 root 74×20px를 고정한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:27-28,36-41`). |
| 17 | 다른 writing mode/direction이 row 축과 coordinates를 바꾸는가 | 통과. `horizontal-tb`, `ltr` 고정 및 관측 `writingMode`를 기록한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:15,28`; `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-agent-browser-dom-geometry.json` `cases[].writingMode`). |
| 18 | gap·margin·padding·border가 root, track 또는 item geometry에 더해지는가 | 통과. 모든 edge와 gap을 0으로 고정한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:27-30`; fixture HTML `:7,12-14`). |
| 19 | alignment/stretch가 예상 text/image 크기를 바꾸는가 | 통과. text는 stretch/start, image는 start/natural-size로 제한한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:29-30`; fixture HTML `:12-13`). |
| 20 | 절대 위치 track probe가 sizing contribution으로 들어오는가 | 통과. probe는 out-of-flow이며 sizing item이 아니고 runtime에는 in-flow text/image 두 item만 둔다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:43`; evidence `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:16`). |
| 21 | Browser absolute root와 runtime normal-flow root를 부당하게 같은 조건으로 취급하는가 | 통과. Browser absolute 배치는 scene을 쌓기 위한 것이고 runtime은 normal flow, root-relative CSS px로 비교한다고 설명한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:28`). |
| 22 | Browser version, viewport, DPR, font/image readiness가 부족한가 | 통과. HeadlessChrome 156/macOS ARM/1280×633/DPR 1 및 ready 상태가 JSON/evidence에 기록됐다 (evidence `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:5`; `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-agent-browser-dom-geometry.json` `data.result.capture`). |
| 23 | 기존 관측과 DOM 재관측이 scene별로 불일치하는가 | 통과. 네 장면 모두 columns/rows, source order, root/text/image frames, natural size가 동일하다 (`spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-observation-original.txt:20-255`; `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/chrome156-row-axis-agent-browser-dom-geometry.json` `cases`). |
| 24 | 저장 파일 hash 또는 embedded asset bytes가 README 주장과 다른가 | 통과. README 여섯 SHA가 실제 파일과 일치하고 HTML embedded Ahem·PNG bytes도 저장소 asset과 같다 (evidence `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:39-46,61-64`). |
| 25 | Screenshot 화면이 geometry/범위 주장과 어긋나거나 pixel equality로 과장되는가 | 통과. 저장된 1280×633 화면에서 네 scene layout을 볼 수 있으며 README는 이를 배치 참고로 제한한다 (`spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:10,37`). glyph/PNG pixel equality는 주장하지 않는다. |
| 26 | 관측 JSON을 실제 사용한 HTML input과 연결할 수 있는가 | 부분 통과. JSON origin은 `/tmp/spinon-c11-row-axis-chrome-oracle.html`; 검토 시 해당 파일 hash는 tracked fixture hash `ab34be0125b128a1d47f0a1cc703b67e31435f3f16a48f293d9d198b94acd12a`와 같았지만 JSON에 input hash 자체는 없다. N2로 기록한다. |
| 27 | Agent Browser session/process cleanup 주장이 보존 evidence로 independently verifiable한가 | 부분 통과. README는 session close와 해당 PID/profile 미존재를 기록하지만 종료 stdout/process inventory 원본은 포함하지 않는다 (`spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:48-50`). N3로 기록한다. |
| 28 | PR 본문 screenshot의 링크, tracked membership, 크기·visibility가 확인되는가 | 통과. PR PNG/JSON 둘 다 HEAD tree와 11-file diff에 있다. 두 SHA가 README와 일치하고 JSON은 URL #619, `complete=true`, natural `1280×633`, rendered `782×387`, `visibleInViewport=true`, `.js-comment-body.markdown-body` 내부를 기록한다 (`spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:52-64`). 저장된 PR screenshot에서도 comment body의 image가 보인다. |
| 29 | 새 scene이 실제 simulator host adapter·capture gate에 연결되는가 | 통과. B1 수정으로 Java/Swift adapters의 allowlist, per-scene frames/tracks/contributions/trace 기대값 및 새 row-only capture/evidence 경로가 plan gate와 file ownership에 구체화됐다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:59-63,70-78`). |
| 30 | 이전 20-view plan review 수정 기록이 현재 PR status와 모순되는가 | 통과. 수정 기록은 B1 소유권 변경을 설명하며, B2에 대해서는 commit `f5d41597`에서 두 이미지 파일이 commit tree에 포함되고 README hashes와 일치함을 명시한다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis-plan-review-20.md:30-31`). |

## 최종 판정 및 notes

### Blocking findings

없음. 이전 blocker B1은 adapter/capture ownership이 계획 gate와 file boundary에 들어가 해소됐다. B2 PNG/JSON도 HEAD tree에 포함되고 README hash table과 일치한다.

### Non-blocking notes

- **N1 · min/max 분별 한계:** min-content와 max-content 결과가 모두 20px이므로 이 matrix는 두 알고리즘의 오동작을 서로 구별하지 못한다. 계획이 이 한계를 명시했으며 일반 Grid 적합성으로 확대하지 않는다 (`plan/c11-4-mixed-direct-text-image-intrinsic-row-axis.md:45`; evidence `spec/internal/evidence/c11-4-row-axis-mixed-text-image-precomparison-2026-10-25/README.md:35`).
- **N2 · 관측 input hash join:** 현재 임시 HTML과 tracked fixture의 bytes/hash는 검토 시점에 일치했지만, 그 SHA는 관측 JSON에 저장되지 않았다. 다음 evidence 갱신 때 `inputHtmlSha256`을 JSON에 넣으면 checkout 밖에서도 source-to-observation을 입증하기 쉽다.
- **N3 · 브라우저 종료 근거:** 종료/PID 상태는 README에 요약되어 있으나 원본 close/process inventory는 diff에 없다. 세션 종료 주장을 더 감사 가능하게 만들려면 sanitized close result를 남길 수 있다.

### Final PR screenshot rendering report

커밋된 `pr619-row-axis-image-verification.json`은 `https://github.com/ohah/spinon/pull/619`에서 alt text `Chrome 156 Agent Browser row-axis fixture`인 이미지를 확인한 결과다. `complete=true`, natural size `1280×633`, 표시 크기 `782×387`, `visibility:visible`, `opacity:1`, viewport 교차 `true`, comment body 내부를 기록한다. 이미지 source는 PR 기준 commit `2591b8528f3107d5efba6dd640ca468df082e823`의 tracked `chrome156-row-axis-agent-browser.png`이며 해당 경로가 그 commit tree에 있었음을 확인했다. 커밋된 `pr619-row-axis-image-agent-browser.png`도 1280×633이고 화면에서 PR body와 렌더된 image가 확인된다. 이 diff-only 재검토에서는 Browser를 실행하지 않고 저장된 JSON/PNG만 확인했으며, raw screenshot link의 별도 live browser validation은 이 범위에 포함하지 않았다.

README의 6개 screenshot/fixture hash는 실제 파일과 모두 일치했고 embedded Ahem/PNG도 저장소 자산과 byte-identical이다. 이전 원본 관측과 재관측의 4 scene geometry도 동일했다. 수정되지 않은 plan/status/0059 본문과 다른 untracked evidence는 건드리지 않았다.
