# C11.4 direct text/image row-axis 계획 실패 관점 20개

이 검토는 새 구현을 시작하기 전에 계획의 범위, 규범 기대, Chrome oracle 경계와 검증 방법을 서로 다른 실패 관점에서 반박한다. 구현 코드와 PR diff 검토를 대신하지 않는다.

1. **#611과 같은 범위로 오인될 수 있음** — 선행 계획은 intrinsic column 및 fixed rows라고 고정했고, intrinsic row는 제외 범위로 명시했다. 이 계획은 row-axis 후속으로 이름과 경계를 분리한다.
2. **원시 text node를 익명 Grid item으로 착각할 수 있음** — CSS placement를 명시하려면 direct child `div` text item을 사용한다. Anonymous raw text Grid item을 다루지 않는다고 범위에 적었다.
3. **fixed columns가 실수로 intrinsic하게 될 수 있음** — 두 열을 `50px 24px`로 고정하고 폭 합이 root content width `74px`와 일치해야 한다.
4. **root auto width가 geometry를 불명확하게 할 수 있음** — root content width를 definite `74px`로 고정하고 content box 기반 좌표를 사용한다.
5. **root height가 track보다 큰 definite 값으로 덮일 수 있음** — block size는 auto이고 단일 explicit row의 intrinsic result로 결정되도록 고정한다.
6. **열 pass 후 행 pass 순서를 놓칠 수 있음** — CSS Grid §11.1에 따라 fixed columns가 결정된 뒤 50px text available inline size로 row contribution을 산정한다고 썼다.
7. **Ahem 미로드가 다른 line break를 만들 수 있음** — Ahem 10px/10px, font readiness 및 asset hash를 oracle precondition으로 둔다.
8. **space를 glyph 폭 계산에서 빠뜨리거나 줄바꿈으로 오인할 수 있음** — Ahem sequence의 unwrapped width 80px와 `XX`/`XXXXX`의 20/50px를 계산하고 space를 soft wrap opportunity로 적었다.
9. **50px text column에서 1줄로 잘못 가정할 수 있음** — `XX XXXXX`의 second word만으로 50px이 필요하므로 첫 단어 뒤에서 줄바꿈되어 높이가 20px임을 명시했다.
10. **min-content width와 block contribution을 섞을 수 있음** — 50px inline min-content width는 column과 관련되고, row 기대값은 text block-size 20px에서 온다고 분리했다.
11. **min/max-content row가 서로 다른 결과라고 과장할 수 있음** — 이 block-axis text의 두 기여가 같고 image는 16px이므로 두 row 모두 20px이라고 적었다.
12. **PNG가 아직 decode되지 않아 natural size가 0일 수 있음** — decode 완료와 자연 크기 `24×16px` 확인을 fixture readiness 조건으로 요구한다.
13. **image가 늘어나 row 전체 높이를 차지할 수 있음** — replaced item은 auto width/height와 자연 크기를 사용하고 geometry acceptance는 24×16px이다.
14. **row에서 image/text가 서로 다른 cell로 자동 배치될 수 있음** — 모든 scene에서 text column1, image column2 및 같은 row1을 명시 배치한다.
15. **DOM reverse가 item의 column을 바꿀 수 있음** — reverse는 source order만 바꾸고 class-bound explicit placements를 유지한다.
16. **gap이나 box edges가 20px/74px을 오염시킬 수 있음** — row/column gap, margins, padding, border를 모두 0으로 고정한다.
17. **정렬의 stretch가 기대 item frame을 바꿀 수 있음** — 두 item의 block alignment를 start로 두며 text inline size는 stretch로 definite 50px column과 맞춘다. image는 start-aligned natural size를 유지한다.
18. **Chrome 관찰을 규범 계산으로 승격할 수 있음** — 새 Chrome 156 row-axis 4-scene observation은 독립 CSS 계산과 별도 표기한다. Browser-only absolute `track-probe`는 out-of-flow measurement helper이며 sizing item이 아니다. Chrome 결과가 CSS 규범을 결정하지 않으며 geometry가 다르면 구현 전 원인을 조사하도록 둔다. PR 본문 이미지 확인 자료는 README에서 별도 링크/hash로 연결한다.
19. **하나의 고정 scene 성공을 일반 HTML 지원으로 확대할 수 있음** — 전용 opt-in IDs, Android Java/iOS Swift scene allowlists, scene별 frame/track/trace assertion owner와 별도 row capture/evidence 경로를 계획한다. 입력은 fail-closed로 유지하고 simulator evidence와 C11.4 미완료를 구분한다.
20. **계획 검토를 구현/runtime 또는 PR diff 검토에 재사용할 수 있음** — 각 단계에서 새 관점 검토를 별도로 요구하고, 공식 완료 `72/213`을 변경하지 않는다.

1차 계획 검토 시점에는 누락을 찾지 못했다. 이후 PR diff 검토에서 host adapter 및 capture ownership 누락이 확인되어 아래 수정 기록에 반영했다.

## 계획 검토 수정 기록 · PR #619 diff feedback

- **B1 수정:** 계획의 구현 gate와 파일 경계에 Android `platforms/android/app/src/main/java/dev/spinon/bootstrap/C0410RuntimeGpuDemo.java`, iOS `platforms/ios/Sources/C0410RuntimeGpuDemo.swift`의 scene allowlist, root/item frame, column/row track, intrinsic contribution 및 trace 기대값 책임을 명시했다. 기존 #611 capture script/evidence는 재사용하지 않고, `spec/internal/evidence/c11-4-direct-mixed-text-image-row-axis-runtime-2026-10-25/capture-direct-mixed-row-axis.mjs`와 그 디렉터리의 run evidence를 새 경로로 사용한다. **관점 19와 20을 수정 후 다시 읽어 확인했다:** host adapter와 capture gate가 모두 구현 완료 조건에 연결되고 일반 CSS 지원을 확대하지 않는다.
- **B2 확인:** precomparison README에는 `pr619-row-axis-image-agent-browser.png` 및 `pr619-row-axis-image-verification.json` 링크와 SHA-256이 있다. 실제 파일 hash도 각각 `f770eed5db65c25743981419237bc0a61539352ccada9007783b0da057796d65` 및 `8977ba0bd7a3aa9ae8febfbea1444388d9ebb34230d14f3c26817512eb2f1049`로 README와 일치한다. **관점 18과 20을 수정 후 다시 확인했다:** README 경로/hash는 유효하지만 두 파일은 현재 untracked라 아직 PR tree에 포함되지 않았다. 이 수정 기록은 staged/PR membership을 주장하지 않는다.

검토 종료 시점의 계획은 `min-content`/`max-content` 기대값이 모두 20px이라는 한계, direct `<div>` text item topology, Chrome-vs-runtime evidence 경계를 유지한다. 수정 계획 review는 implementation/runtime review 또는 PR diff review를 대체하지 않는다.
