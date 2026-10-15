# C13.3 style invalidation PR 변경 관점 30개 · 2026-10-15

## 검토 범위

`origin/main` 대비 구현·FFI·Android/iOS adapter·fixture·캡처 도구·상태·명세·계획·로드맵 diff를 실제 코드와 실행 근거에 대조했다. 구현 실패 관점 20개 및 계획 검토 20개와 별도 기록이다. 아래 항목은 작업자 자체의 서로 다른 실패 경로 검토이며 독립 reviewer 승인을 뜻하지 않는다.

## 적대 관점

| # | 실패 관점 | 판정 및 근거 |
|---:|---|---|
| 1 | 새 document generation에서 이전 style epoch가 이어지는가 | **통과.** generation 교체는 `StyleRevision::INITIAL`로 초기화하고 기존 key/cache의 generation 대조도 유지한다. workspace tests 통과. |
| 2 | style revision 증가 overflow 뒤 document snapshot만 부분 적용되는가 | **통과.** 다음 revision을 checked 계산한 뒤 snapshot을 교체한다. overflow는 오류로 반환한다. |
| 3 | environment-only update가 style revision을 잘못 올리는가 | **통과.** environment와 author style epoch는 분리되고, 환경 변경은 별도 cache key로 무효화한다. |
| 4 | 최초/full invalidation이 subtree revision으로 오분류되는가 | **통과.** `Full`은 full calculation을 수행하고 subtree adjacency reuse 경로를 사용하지 않는다. workspace cascade tests 통과. |
| 5 | style mutation이 worker request에 반영되지 않거나 이전 작업 결과가 새 snapshot을 덮는가 | **통과.** typed revision이 요청 key에 들어가고 publish 전 최신 request key와 비교한다. current-key tests 통과. |
| 6 | 계산 cache가 style revision을 생략해 이전 computed output을 반환하는가 | **통과.** calculation cache key에 style revision을 포함하고 origin 계산 key도 대조한다. cache tests 통과. |
| 7 | style-only cache reuse가 한 단계보다 큰 revision 간격을 숨기는가 | **통과.** `Subtrees`는 정확히 한 epoch 증가만 허용하고 `Unchanged`는 동일 epoch만 허용한다. |
| 8 | incremental cascade가 다른 revision output을 base로 재사용하는가 | **통과.** 유효한 동일/직전 revision만 재사용하고 나머지는 full fallback한다. |
| 9 | profile calculation이나 empty-scene layout이 `INITIAL`을 다시 주입하는가 | **통과.** 계산 profile과 빈 scene 경로가 요청 revision을 사용하며 snapshot key 일관성을 확인한다. |
| 10 | style 계산 결과의 문서·tree·revision 조합이 서로 다른 key로 게시되는가 | **통과.** 계산 완료와 layout 생성 경계에서 요청 key를 대조한다. FFI feature test 포함 workspace 통과. |
| 11 | geometry-neutral style revision에서 active gesture가 불필요하게 끊기는가 | **통과.** 활성 session은 최신 document/tree/style revision을 추적하면서 같은 geometry를 허용한다. scenario 8·10 양 플랫폼 통과. |
| 12 | session이 앞선 revision이나 generation 변경을 받아 오래된 입력을 실행하는가 | **통과.** generation 불일치와 revision 후퇴는 취소하며 환경 revision 변경도 별도 사유로 취소한다. |
| 13 | style revision 증가가 없는 구조적 document/tree 교체를 style update로 오인하는가 | **통과.** document/tree projection 변경과 style epoch 유지가 함께 오면 `DocumentOrTreeChanged`로 취소한다. |
| 14 | 외부 scroll update가 geometry-neutral style update와 섞여 기존 입력을 계속 허용하는가 | **통과.** scroll-state revision이 session 예상값과 다르면 `ExternalScrollUpdate`로 종료한다. |
| 15 | 목표 render box 제거 또는 owner metadata 누락을 pan 지속으로 해석하는가 | **통과.** geometry key 생성 실패는 `GeometryChanged` 취소다. |
| 16 | frame은 같지만 owner chain·scrollport·range가 바뀐 geometry를 놓치는가 | **통과.** geometry key에 target frame, owner ancestry, scrollport, max offset을 포함한다. scenario 9에서 실제 range 변경을 검출했다. |
| 17 | hit-test/paint clip 또는 hit order 변경을 geometry-neutral로 오해하는가 | **통과.** clip layer의 paint/hit clip과 scroll ancestors 및 target hit order를 key에 포함한다. |
| 18 | 기존 offset 복구 전 scroll-relative clip을 비교해 style-only 변경을 geometry 변경으로 잘못 취소하는가 | **통과.** offset 복구 뒤 geometry를 비교한다. scenario 8·10 scroll revision은 그대로이고 9만 증가했다. |
| 19 | 새 geometry에서 offset이 새 maximum을 넘어가거나 stale offset을 보존하는가 | **통과.** snapshot offset 적용 경계가 새 range로 clamp하고 scenario 9는 X offset/range 66을 기록했다. |
| 20 | fixture scenario ID 8·9·10이 서로 다른 mode/policy/source에 매핑되는가 | **통과.** Rust enum mapping test와 양 플랫폼 runtime report가 8/9/10을 확인한다. |
| 21 | policy-only fixture가 paint/geometry 변경을 놓친 채 통과하는가 | **통과.** geometry unchanged, scroll revision 동일, active pan 지속 및 다음 gesture rejection을 함께 assert한다. |
| 22 | geometry fixture가 cancel 뒤 새 pan을 허용하지 않거나 잘못된 scroll range를 보고하는가 | **통과.** 현재 pan의 `GeometryChanged`, range/offset 66, 새 gesture pan 시작을 양 플랫폼에서 확인했다. |
| 23 | paint fixture가 scroll state를 변경하거나 inner border 바깥에 색을 그리는가 | **통과.** scroll revision은 같고 inner exact pixel만 변하며 바깥 `(200,100)`은 `[27,52,65,255]`다. screenshot validator는 색 누출을 거부한다. |
| 24 | native style update가 처리되기 전에 후속 Move/End가 옛 snapshot에 적용되는가 | **통과.** Android/iOS adapter가 barrier 동안 Move와 terminal을 보류하고 style update 뒤 순서대로 제출한다. |
| 25 | 스타일 업데이트 실패 시 보류한 Move가 새 policy 성공 입력처럼 다시 실행되는가 | **통과.** 실패 경로는 취소 terminal을 제출하고 deferred Move를 재생하지 않는다. |
| 26 | app background가 barrier completion보다 먼저 gesture ID를 지워 deferred cancel을 stale event로 만드는가 | **통과, 수정 후 source 검사.** iOS background 경로는 barrier가 pending이면 gesture identity를 보존하고 completion이 deferred cancel을 제출한 뒤 reset한다. 이 timing은 simulator run에서 별도 주입하지 않았으므로 background interruption 자체는 검증 범위 밖이다. |
| 27 | Android JNI/iOS Objective-C FFI가 fixture-off 빌드에서 새 ABI를 노출하는가 | **통과.** C 선언과 bridge는 전용 feature macro 아래 있고 기본 `.so`/`.app`에는 symbol이 없다. artifact positive/negative controls 기록. |
| 28 | Android Java report 검사 상수가 DEX에 style fixture marker/payload를 유출하는가 | **통과.** 완성 문자열은 런타임 조립; module boundary 검사와 기본 APK의 `.so`·DEX·APK 정확 문자열 scan에서 새 항목이 없다. opt-in 실행 report는 marker를 포함한다. |
| 29 | iOS Swift/ObjC bridge flag 또는 build script feature가 Rust feature와 불일치하는가 | **통과.** iOS build flag, native Cargo feature, C header, Swift condition이 같은 C13.3 opt-in을 사용한다. default/opt-in `.app`를 검사했다. |
| 30 | 캡처 검증기가 중복 log·truncated payload를 잘못 승인하거나 화면 밖 색 누출을 놓치는가 | **통과.** 동일 stable report만 합치고 가장 긴 원문을 보존한다. 서로 다른 report, revision/gesture assertion 위반, border 바깥 pixel 위반은 거부한다. JS 14/14 및 simulator matrix 6/6 통과. |

## 결론과 범위

선언한 C13.3 style invalidation slice에서 차단 결함은 발견되지 않았다. 발견된 revision 전달, offset 복구 순서, Android 중복 로그, 기본 DEX 문자열 누출, iOS background/barrier gesture identity 경로는 수정하고 관련 관점을 다시 확인했다. 최신 [Simulator matrix](20261015T112615633Z-3abc0b5e/README.md)와 [기본/opt-in artifact 검사](artifact-boundary/README.md)를 참조한다.

이 검토는 Chrome parity, public DOM event/CSSOM, 실기기, hardware GPU, compositor/display scanout 또는 C13.3 전체 완료를 판단하지 않는다. Chrome 155 active-touch policy observation과의 계약 차이는 계획에 남아 있다.
