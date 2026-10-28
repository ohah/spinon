# C11.4 `fit-content()` automatic-minimum floor 계획 적대 검토 20개

이 검토는 계획·사전 비교가 구현에 들어가기 전에 입력, 독립 계산, 범위와 검증 경계의 서로 다른 실패 경로를 살핀다. 구현/runtime review나 계획 PR diff review를 대체하지 않는다.

1. **비슷한 Grid WPT 파일을 pin할 수 있음 — PASS.** 계획은 정확한 `css/css-grid/grid-definition/fit-content-track-sizing.html` 경로, revision `c999c58338ee1d223df5ad62e48be1202ced0d37`, SHA-256을 함께 고정한다.
2. **다른 assertion의 줄을 검증 대상으로 삼을 수 있음 — PASS.** 대상 DOM 31–34행과 “above the argument” assertion 90–92행을 별도로 적었다. `nl` 출력과 Agent Browser의 test title을 대조했다.
3. **현재 upstream 파일이 pin과 다른데 같다고 가정할 수 있음 — PASS.** pinned checkout과 이번 `wpt.live` fetch 결과 모두 SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`다.
4. **Chrome upstream pass를 Spinon 적합성으로 부풀릴 수 있음 — PASS.** HeadlessChrome 156의 13/13을 browser baseline이라고 명시했고 Spinon runtime 미실행 상태와 분리했다.
5. **browser가 target assertion보다 먼저 캡처되어 부분 실행일 수 있음 — PASS.** Agent Browser는 `13 Pass`를 기다린 뒤 target computed rows, root height 및 item style을 읽고 1280×5266 full-page screenshot을 저장했다.
6. **WPT 대상 DOM의 콘텐츠 크기가 계획에서 바뀔 수 있음 — PASS.** 두 direct grid item과 그 안의 10px/100px block child를 각각 고정한다.
7. **item의 기본 automatic minimum이 사라질 수 있음 — PASS.** main scene은 item에 `min-height`나 scrollable overflow를 추가하지 않고 computed `min-height:auto`, `overflow:visible`을 요구한다.
8. **fit-content argument를 fixed max track처럼 잘못 적용할 수 있음 — PASS.** CRD §6.6은 fit-content argument가 content-based minimum을 fixed max처럼 clamp하지 않는다고 밝힌다.
9. **fit-content 최소값을 잘못 정의할 수 있음 — PASS.** CRD §11.2의 공식 `max(minimum, min(limit, max-content))`와 §11.5의 minimum contribution을 계획과 JSON에 분리했다.
10. **첫 row가 40px cap으로 잘못 늘어날 수 있음 — PASS.** 첫 item의 minimum/max-content 10px과 40px limit에서 독립 계산은 `max(10,min(40,10))=10px`이다.
11. **두 번째 row의 100px minimum이 40px cap으로 잘못 잘릴 수 있음 — PASS.** 두 번째 item의 content-based minimum 100px이 limit보다 크므로 `max(100,min(40,100))=100px`이다.
12. **root height·alignment와 row size를 같은 값으로 오해할 수 있음 — PASS.** definite 300px root height, `align-content:normal`, `[10,100]px` Rows를 별도 oracle로 고정한다. Fit-content row가 auto max-track stretch 대상이라는 잘못된 전제를 두지 않는다.
13. **WPT의 implicit auto-row 입력이 explicit rows로 바뀔 수 있음 — PASS.** `grid-template-rows:none`과 `grid-auto-rows:fit-content(40px)`를 구현 입력 계약에 둔다.
14. **root width deviation이 숨겨질 수 있음 — PASS.** WPT auto width를 deterministic runtime용 60px로 조정한다고 공개하고, row sizing 및 동일 60px explicit column만 비교한다.
15. **minimum-zero control이 main case의 원인을 분리하지 못할 수 있음 — PASS.** control에서만 두 번째 item에 `min-height:0`을 적용하고 100px child는 유지하며 Rows `[10,40]`을 예상한다.
16. **control의 content overflow를 track size로 혼동할 수 있음 — PASS.** 두 번째 control track은 40px, child는 100px, visible overflow는 60px로 각각 구별해 기록한다.
17. **scene 명칭이 기존 allowlist와 충돌할 수 있음 — PASS.** 현재 구현 디렉터리에서 `fit-content-auto-min-floor-wpt`와 `fit-content-auto-min-floor-zero-control` ID가 없다.
18. **frame·track·style이 서로 다른 document revision에서 합쳐질 수 있음 — PASS.** 수용 기준은 computed style, typed input, frame, trace, draw/present가 같은 scene revision에 결합될 것을 요구한다.
19. **fixture helper가 제품 artifact에 포함될 수 있음 — PASS.** 기본·product artifact negative control과 fixture-test positive control을 모두 요구하며 현재 API/support를 새로 선언하지 않는다.
20. **한 assertion slice가 C11.4 완료로 오인될 수 있음 — PASS.** 다른 12개 WPT assertion, 일반 CSS admission, 실기기/hardware GPU, 전체 C11.4와 공식 `72/213`은 범위 밖이며 미완료로 명시했다.

**판정:** 계획 source, 규범 기대값, target/control 구분과 검증 범위에서 수정이 필요한 실패 관점은 발견하지 못했다. 계획 PR diff는 별도 30개 관점으로 다시 검토한다.
