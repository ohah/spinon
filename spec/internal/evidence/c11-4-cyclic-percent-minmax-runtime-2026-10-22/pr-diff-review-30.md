# C11.4 cyclic percentage `minmax()` PR 변경 실패 관점 30개

계획 검토 20개와 구현/runtime 검토 20개에 재사용하지 않은 PR 변경 관점이다. 코드를 둘러싼 fixture 계약, 플랫폼 adapter, 증거 연결과 공식 상태 변경을 각각 확인했다.

1. **feature 경계** — 신규 scene module은 fixture source tree에서 선언되고 C04 runtime fixture-test feature 경계 안에서만 컴파일된다. 통과.
2. **scene admission** — adapter는 세 명시 ID만 허용하고 임의 ID에 source를 반환하지 않는다. 통과.
3. **기존 scene ID 충돌** — bare cyclic percent scene과 새 minmax scene ID 집합이 겹치지 않는다. 통과.
4. **JS source node count** — 각 source는 root와 item 두 `div`만 만든다. 통과.
5. **stable node identity** — root ID, item ID, return object의 `nodeIds`가 각 scene에 맞고 중복되지 않는다. 통과.
6. **item fixed contribution** — 모든 item의 border-box 크기는 24×16 CSS px이며 margin/padding/border는 0이다. 통과.
7. **root box model** — root도 border-box, margin/padding/border/gap 0으로 고정해 basis에 장식 값을 섞지 않는다. 통과.
8. **maximum/minimum 역할** — `minmax(0px,50%)`의 zero fixed minimum과 percentage maximum을 그대로 전달한다. percentage minimum은 추가하지 않는다. 통과.
9. **cyclic column percentage basis** — root max-content sizing 중 item contribution 24px를 얻고 final 50% basis에서 12px track이 나온다. AVD/iOS trace가 맞다. 통과.
10. **cyclic row percentage basis** — max-content block sizing에서 root height 16px, final percentage row 8px를 얻는다. AVD/iOS trace가 맞다. 통과.
11. **definite width basis** — width 40px control에서 20px column이 나온다. cyclic intrinsic stage 결과와 섞이지 않는다. 통과.
12. **independent second-axis track** — column fixtures의 Rows `[16]`, row fixture의 Columns `[24]`를 별도로 확인한다. 통과.
13. **root frame assertion** — expected root frames 24×16, 24×16, 40×16이 runner와 양 플랫폼 host에 연결되어 있다. 통과.
14. **item frame assertion** — item은 root width 24/40 모두에서 24×16을 유지하며 overflow 여부가 frame equality에 숨지 않는다. 통과.
15. **node order/frame assertion** — root node 1과 item node 2의 순서 및 좌표가 세 platform expectation에 고정된다. 통과.
16. **box count regression** — 새 scene의 box count는 두 개로 고정되어 script와 host가 함께 검사한다. 통과.
17. **track list verification** — runner가 선택 축 한 개만 보지 않고 두 축의 used tracks와 collapsed flags를 확인한다. 통과.
18. **overflow sample remains in item** — x=18/y=12/x=22 표본은 각 item frame 안이고 동시에 해당 track 바깥이다. 통과.
19. **pixel target identity** — 여섯 표본 모두 정확한 scene item node를 가리킨다. 통과.
20. **Android request gate** — C114 ID list와 surface pixel request allowlist에 세 scene이 포함된다. 통과.
21. **Android expected RGB** — 새 sample ID의 red RGB mapping이 Java PixelCopy checker에 포함된다. 초기 누락을 발견해 수정 후 6/6 exact를 확인했다. 통과.
22. **Android coordinate mapping** — AVD density 2.625에서 CSS sample center를 SurfaceView pixel로 변환한 결과와 PixelCopy 로그가 일치한다. 통과.
23. **Android installed artifact identity** — 설치 APK와 로컬 APK SHA-256이 동일하다. 통과.
24. **iOS request gate** — C114 scene Set이 새 IDs를 허용하고 sample tuple checker가 ID/node/좌표/rgba를 모두 검사한다. 통과.
25. **iOS surface transform** — Simulator scale 3에서 Metal sample pixel coordinates와 CSS coordinates가 일치한다. 통과.
26. **iOS compositor boundary** — evidence는 Metal drawable readback까지만 말하며 compositor 제출 이후 pixels로 확대하지 않는다. 통과.
27. **draw/present gate** — 두 플랫폼 모두 scene status뿐 아니라 draw/present marker를 요구하며 세 장면에서 통과했다. 통과.
28. **default archive exclusion** — 기본 release rlib byte scan에서 module marker와 세 new ID가 없다. fixture-test feature만 켠 positive control에는 모두 있다. 통과.
29. **public status semantics** — `STATUS.md`는 하위 fixed-fixture 진행만 덧붙이고 C11.4 checkbox를 열어 둔다. 공식 집계 `72/213`도 그대로다. 통과.
30. **범위 과장과 linkability** — README/plan은 Android AVD·iOS Simulator, border stroke 미검사, iOS pre-compositor, 미지원 CSS 범위를 분리하고 PR 본문 이미지는 immutable raw commit URL을 쓴다. Agent Browser로 PR #472의 이미지 6개를 확인했으며 모두 complete=true다. Android 3개는 1080×2400, iOS 3개는 1206×2622로 자연 크기가 일치한다. [브라우저 확인 기록](./pr-image-browser-check.json). 통과.

**판정:** 변경 diff에서 추가 수정이 필요한 blocker는 발견하지 못했다. 항목 21의 실제 RGB table 누락은 simulator 실행 중 발견해 수정했고 해당 경로와 전체 AVD 행렬을 재검증했다. PR #472 이미지 링크는 Agent Browser에서 모두 로드되고 시뮬레이터 원본 크기로 표시되는 것을 확인했다.
