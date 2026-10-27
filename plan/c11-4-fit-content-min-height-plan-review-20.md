# C11.4 `min-height` / `fit-content()` WPT 계획 · 적대적 검토 20개

계획 검토는 구현 전에 입력과 기대값의 별도 실패 경로를 확인했다. 이 목록은 구현/runtime review나 PR diff review가 아니다.

1. **WPT 파일 pin이 다른 revision을 가리킬 수 있음 — PASS.** 계획과 `expected.json`은 revision `c999c58338ee1d223df5ad62e48be1202ced0d37` 및 SHA-256 `828160951bba50f6ce1c09a69b804480df10d5e622c92a047452f432cb790736`를 함께 고정한다.
2. **잘못된 assertion을 대상으로 삼을 수 있음 — PASS.** 고정 원본 87–88행이 min-height case이고 27행은 그 DOM임을 각각 직접 연결한다.
3. **Chrome 관측을 기대값 산출에 역사용할 수 있음 — PASS.** 행 10px는 §11.2·§11.5·§11.8에서 별도로 계산하며 브라우저 pass는 보조 관찰로 분리한다.
4. **upstream 13/13을 Spinon 적합성으로 과장할 수 있음 — PASS.** 계획과 사전비교 README가 Chrome testharness 결과를 Spinon WPT pass와 명시적으로 분리한다.
5. **min-height가 free space에 미치는 영향을 반대로 설명할 수 있음 — PASS.** §11.8에 따라 definite min-height를 stretch 계산에 사용한다고 적고, “min-height는 free space를 만들지 않는다”는 주장은 하지 않는다.
6. **fit-content track을 auto maximum track으로 오분류할 수 있음 — PASS.** §11.2의 max-content 처리와 §11.8 auto-max-track eligibility를 구분해 fit-content가 확장 대상에서 빠짐을 고정한다.
7. **growth limit을 40px로 잘못 잡을 수 있음 — PASS.** child max-content 10px와 argument 40px를 비교해 `min(10,40)=10px`로 계산한다.
8. **grid item의 기본 minimum이 달라질 수 있음 — PASS.** WPT의 item `min-height:auto`와 `overflow:visible`을 유지하고 child 10px contribution을 검사한다.
9. **root를 300px definite height로 바꿔 test 의미를 잃을 수 있음 — PASS.** fixture는 `height:auto; min-height:300px`를 정확히 확인한다.
10. **root 폭 조정이 row 기대값을 바꿀 수 있음 — PASS.** 60px 고정 열과 단일 10px block child로 inline contribution을 배제하고, 폭 차이는 block-axis 비교에서만 무관하다고 한정한다.
11. **root used height와 row track size를 혼동할 수 있음 — PASS.** root 300px과 row 10px을 별도 관측·기대로 기록한다.
12. **implicit track을 explicit row로 대체할 수 있음 — PASS.** `grid-template-rows:none`, `grid-auto-rows:fit-content(40px)` 및 한 implicit row를 입력 경계로 고정한다.
13. **정렬 단계가 실행되지 않는 대조군으로 바뀔 수 있음 — PASS.** `align-content:normal`을 고정하고 Rows trace에 `StretchAutoTracks { applied:true }`를 요구한다.
14. **`applied:true`를 track 성장의 증거로 오독할 수 있음 — PASS.** 계획은 alignment stage 실행 여부와 row base 변화 여부를 분리하고 전후 base `[10px]`을 요구한다.
15. **scene 또는 node identity 충돌이 생길 수 있음 — PASS.** 전용 route/root/item/child ID를 정하고 기존 ID와 겹치지 않도록 한다.
16. **fixture가 다른 DOM을 통과시키거나 unknown scene을 허용할 수 있음 — PASS.** 단일 item-child tree를 강제하고 unknown ID source 거부와 변형 입력 거부를 수용 기준으로 둔다.
17. **style과 trace가 다른 document revision에서 합쳐질 수 있음 — PASS.** computed style, typed Grid input, frames, trace를 하나의 source/style/environment revision에 묶는다.
18. **fixture ABI가 기본 제품 artifact에 섞일 수 있음 — PASS.** default/product archive에서 marker·symbol·payload 부재와 fixture-test positive control 포함을 별도로 검사한다.
19. **스크린샷을 surface pixel 또는 hardware GPU 근거로 확대할 수 있음 — PASS.** screenshot을 보조 자료로 제한하고 simulator 결과와 실기기·hardware GPU를 분리한다.
20. **단일 WPT slice로 C11.4나 전체 13건을 완료 처리할 수 있음 — PASS.** 다른 12 assertions, 일반 CSS admission, `spec/STATUS.md`, official `72/213`은 명시적으로 범위 밖에 둔다.

**판정:** 계획의 source, 규범 산술, runtime boundary와 미지원 범위에서 수정이 필요한 FAIL은 발견하지 못했다. 계획 PR diff review는 이 검토와 별도로 새 20관점으로 수행한다.
