# C11.1 `aspect-ratio` 값 확장 PR diff 검토 20개

이 검토는 구현/runtime 검토 및 PR의 30관점 적대 검증과 별개로, 제출 diff의 책임 경계·추적성·유지보수성을 검토한다.

| # | diff 질문 | 확인 및 판정 |
|---:|---|---|
| 1 | 새 문법이나 공개 CSS API까지 범위가 넓어졌는가 | cascade allowlist, Root HTML projection, 고정 Grid admission에 한정했다. 일반 CSS/Grid API 변경은 없다. 통과. |
| 2 | ratio string을 parse해 계산하는 중복 수치 경로가 생겼는가 | `layout_aspect_ratio` typed 값이 수치 원본이고 serialization은 numeric/auto 구분에만 쓴다. 통과. |
| 3 | projection와 layout의 유효 범위가 서로 달라졌는가 | 둘 다 기존 topology/owner/size 조건을 지키고 finite-positive ratio를 요구한다. 차이가 나는 80px 예외도 0.5로 일치한다. 통과. |
| 4 | fixed dimension gate가 표면상만 넓고 단위/방향이 뒤집혔는가 | 60px fixed width와 30px fixed height, 단일 80px/0.5 조건을 명시해 fixture 값과 대조했다. 통과. |
| 5 | ratio가 없는 control도 unintended transfer를 받는가 | ratio-free scene은 transfer가 없으며 text auto-min 측정만 수행한다. 주 matrix에서 기존 control을 보존했다. 통과. |
| 6 | automatic minimum 로직이 Grid 모듈의 candidate 계약을 우회하는가 | `text_item_targets`가 기존 `is_supported`를 재사용하고, helper는 해당 set에 든 NodeId만 처리한다. 통과. |
| 7 | target lookup을 반복해 node 수에 비례한 중첩 scan을 도입했는가 | layout tree를 만들기 전에 set을 한 번 만들도록 수정했다. helper의 비대상 fast path는 set membership이다. 통과. |
| 8 | intrinsic size 계산을 public type에 새로 노출했는가 | helper와 setter는 crate/private 범위이며 공개 API·C ABI·FFI는 바뀌지 않았다. 통과. |
| 9 | TextLayoutEngine 소유권을 불필요하게 바꾸었는가 | 기존 cloned engine을 `RefCell`로 유지하고 tree 생성 중 intrinsic query에 재사용한다. 이후 callback cache도 같은 clone을 쓴다. 통과. |
| 10 | callback과 선계산이 서로 다른 node cache key를 쓰는가 | `engine_ids[&external_id]`에 intrinsic을 저장하고 target은 real postorder node다. 통과. |
| 11 | finite 검사 위치가 한 진입 경로만 보호하는가 | transfer는 projection/layout에서, 절대 frame 끝점은 공통 frame collector에서 검사한다. 통과. |
| 12 | 공통 frame 검사 변경이 새로 진단할 실패가 모호한가 | `NonFiniteFrame(NodeId)` 기존 오류를 재사용하며 x/y, width/height, right/bottom을 검사한다. 통과. |
| 13 | Taffy style mutation이 authored min-width/min-height를 덮어쓰는가 | candidate admission의 기존 min/max 조건이 none이어야 하고 자동 최소값만 setter로 넣는다. 통과. |
| 14 | negative controls가 성공 fixture와 입력을 공유해 서로 오염되는가 | 각 control이 독립 `Fixture`를 만들고 자기 property 오류를 단언한다. 통과. |
| 15 | 비교 행렬이 ratio syntax와 numeric geometry를 혼동하는가 | `1.5` 및 `3/2` scene은 다른 serialization, 같은 typed ratio·frame을 확인한다. 통과. |
| 16 | runtime test가 main fixture의 IDs/classes와 깨지기 쉽게 어긋났는가 | source는 `include_str!`로 versioned fixture를 사용하고 ID/class로 각각 14 target을 찾아 geometry를 확인한다. 통과. |
| 17 | simulator runner가 APK/app bundle에 fixture bytes가 들어갔는지 검증하는가 | Android APK zip과 iOS app의 `index.html`을 fixture bytes와 직접 비교한다. 통과. |
| 18 | simulator runner가 실제 app verifier를 확인하지 않고 화면만 캡처하는가 | bundle `root-app.js`의 required marker, app boot/layout, draw-present 로그를 검사한다. 통과. |
| 19 | 증거 문서가 draw-present를 pixel/compositor 증거라고 확대하는가 | README가 `NodeId` geometry, PixelCopy, Metal readback, compositor 검증 한계를 분명히 적는다. 통과. |
| 20 | 문서·해시·상태 대장이 구현 범위를 잘못 공표하는가 | 내부 plan과 증거는 범위를 제한하고 공식 C11.1/C11 완료를 표시하지 않는다. merge 뒤 contract/status/Tailnet 동기화는 별도 후속으로 남긴다. 통과. |

**판정:** diff의 계산 source, candidate 경계, 오류 전파, test/evidence 경계가 같은 제한 slice를 가리킨다. 구현/runtime 및 PR 적대 검토와 별도로 본 검토표 20개를 완료했다.
