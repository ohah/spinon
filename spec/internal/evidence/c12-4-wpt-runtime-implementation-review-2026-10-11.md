# C12.4 · WPT 후보 런타임 회귀 20개 실패 관점 검토

**검토 대상:** WPT 후보에서 도출한 Rust paint-order 회귀와 제한 profile 거부 검사
**실행 근거:** [대응 결과](c12-4-wpt-candidate-runtime-2026-10-11.md)

| # | 공격 관점 | 검토 결과 |
|---:|---|---|
| 1 | 원문 WPT revision이 바뀌었는데 이전 결과를 재사용하는가 | 다섯 원문을 pinned commit에서 다시 가져와 저장소 SHA-256과 각각 일치시켰다. |
| 2 | 원문 hash 일치를 WPT 실행 성공이라고 표현하는가 | 표현을 분리했다. 문서는 hash 확인과 공식 harness 미실행을 함께 기록한다. |
| 3 | 양수 `z-index` 테스트가 tree order만 검사하는가 | 값 1·2·3의 서로 다른 level을 사용하고 rank `[root, 1, 2, 3]`을 검사한다. |
| 4 | 음수 자손이 부모 context 배경보다 먼저 그려지는 오류를 놓치는가 | 별도 level-0 부모와 level-negative child 입력에서 실제 snapshot rank를 검사한다. |
| 5 | opacity stacking context를 일반 box로 평탄화해도 테스트가 통과하는가 | style-to-render helper가 runtime admission을 건너뛸 수 있음을 발견했다. opacity 검사는 실제 `BlockFlexStacking` 요청에 두고 snapshot 게시 전 실패를 확인한다. |
| 6 | opacity 실패가 속성 이름 없는 임의 오류로 발생하는가 | 실패 코드 `unsupported_inline_property`, node ID 2, 속성 `opacity`를 정확히 비교한다. |
| 7 | WPT opacity 장면을 지원 완료로 오인하는가 | 지원이 아니라 명시적 fail-closed라고 기록하고 WPT pass 집계에서 제외한다. |
| 8 | `overflow`·scroll 동작을 opacity 거부 결과로 덮어쓰는가 | 해당 동작을 실행하지 않았고 미검증 경계로 남겼다. |
| 9 | static Flex item의 `z-index` 예외를 일반 static Block 규칙으로 처리하는가 | `display:flex`의 직접 static item 두 개와 서로 다른 integer level을 검사한다. |
| 10 | static Flex item의 자손 큰 level이 형제 item context를 벗어나는가 | 첫 item의 absolute 자손에 9999를 두고 형제 item level 2의 rank를 비교한다. |
| 11 | `order`가 HostDocument를 재정렬하는가 | fixture의 NodeId는 삽입 순서 그대로 유지하며 결과 paint rank를 그 ID로 확인한다. |
| 12 | 같은 positive level의 tie가 source order만 따라가 `order`를 잃는가 | order -1과 0 두 item의 같은 level rank를 검사한다. |
| 13 | 큰 Flex `order`가 작은 z-index level을 덮는가 | z=0, order=100 item을 z=1 items 아래에 둬 phase 우선순위를 검증한다. |
| 14 | fixture root가 빠져 자식 paint rank만 우연히 맞는가 | 모든 direct paint test가 실제 root node를 포함한 전체 NodeId 배열을 비교한다. |
| 15 | parent-child 관계가 WPT 축약 중 잘못 연결되는가 | nested negative/context 및 Flex item subtree 입력을 서로 다른 부모 index로 구성했다. |
| 16 | 화면 geometry 변화가 rank test를 가릴 수 있는가 | Rust rank 회귀와 별도 WGPU readback을 함께 둔다. WPT-derived WGPU 테스트는 세 표본 RGBA를 정확히 비교하고, 이는 전체 geometry oracle을 뜻하지 않는다. |
| 17 | 색상 오차가 paint rank 통과로 둔갑하는가 | WGPU 표본을 세 지점에 고정해 색상 평균이나 다른 표본으로 실패를 가리지 않는다. mobile screenshot은 여전히 pixel oracle로 쓰지 않는다. |
| 18 | Flex WPT의 미지원 border·전체 행렬을 통과로 간주하는가 | border와 전체 조합은 명시적으로 제외했다. 좁은 paint subset 결과로만 기록했다. |
| 19 | Android/iOS 실행을 이 Rust 회귀 결과로 대체하는가 | 대체하지 않는다. 플랫폼 근거는 기존 10-node 실행이며 새 회귀는 Rust host pipeline 단위다. |
| 20 | 이 subset 결과 때문에 C12.4/WPT 완료로 체크되는가 | 공식 WPT, 26-scene mobile runtime, iOS surface readback 및 추가 실패 검증이 남아 상태 체크를 미완료로 유지한다. |

## 조치

런타임 profile 검사를 우회하는 테스트 helper의 한계를 분리하고 opacity 실패 검사를 실제 runtime request 경로에 추가했다. 네 개 paint 의미는 별도 style-to-render 회귀로 고정했다. 공식 WPT 실행기와 전체 C12.4 장면 실행은 이 검토 범위에 포함되지 않는다.

## 후속 플랫폼 검증

이 검토의 “iOS surface readback 미완료”는 당시 상태다. 이후 Android `PixelCopy` 21 scene·22 sample과 iOS Metal surface reverse Flex 8 sample을 각각 실제 표면에서 비교해 모두 exact match했다. iOS Simulator 26-scene geometry matrix도 26/26을 확인했다. 실제 앱 worker depth와 시뮬레이터 동시 session memory도 별도 evidence에 기록했다. 공식 WPT runner는 여전히 Chrome oracle에만 연결되어 있고 Spinon WPT 적합성으로 바뀌지 않았다. hardware GPU 및 global allocator OOM 복구는 미확정이다. [최신 통합 근거](./c12-4-stacking-runtime-2026-10-11/README.md)를 참조한다.
