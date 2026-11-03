# C11.5 계획 PR 변경분 실패 관점 30개

이 검토는 계획·사전 비교·로드맵 메모의 변경분만 판정한다. 구현 또는 Android/iOS runtime 통과를 의미하지 않는다.

| # | 독립 실패 관점 | 검토 결과 |
|---|---|---|
| 1 | 계획이 일반 Root HTML/CSS 전체 지원으로 범위를 넓히는가 | **통과** — 기존 `RuntimeRootHtmlTextPaintV1`의 eligible fixed-track Grid slice만 명시한다. |
| 2 | 이번 계획이 PR #1124의 적격성 검사와 다른 Grid admission을 암묵 도입하는가 | **통과** — container 및 item eligibility를 기존 helper에 고정하고 확대는 별도 후속으로 둔다. |
| 3 | 반복 track, implicit track, named line/area, spanning, dense flow 또는 untyped gap이 기준 scene에 섞이는가 | **통과** — 계획에서 전부 제외하고 단일 fixed-px explicit row/column, finite fixed-px row/column gap, sparse row flow로 고정한다. |
| 4 | `direction`/`writing-mode` 바깥에서 논리 축 또는 물리 정렬 의미를 일반화하는가 | **통과** — static LTR `horizontal-tb`만 허용한다. |
| 5 | anonymous text, replaced element, positioned child가 empty `div` 입력과 혼합되는가 | **통과** — static empty HTML `div`만 item으로 고정하고 다른 종류를 거부 대상으로 기록한다. |
| 6 | item intrinsic/stretch sizing, nonzero edge 또는 item overflow style이 좌표 기대값을 바꾸는가 | **통과** — item 크기를 지정하고 `overflow:visible`, margin/padding/border 0으로 제한하며 `normal`/`stretch`를 거부한다. |
| 7 | 현재 typed layout 경로에 `justify-self`가 없다는 구현 전제가 누락되는가 | **통과** — 코드 대조에서 `LayoutStyle.align_self`만 존재함을 확인했고 계획에 `justify_self: Auto` typed 입력 추가를 반영했다. |
| 8 | 교체 요소의 기존 Taffy `justify_self:start`가 새 초기값 처리로 바뀌는가 | **통과** — 명시값이 없는 교체 요소 fallback을 보존하도록 구현 단계와 회귀 기준을 고쳤다. |
| 9 | `auto`가 item 자체 초기 정렬로 잘못 처리되고 부모 `*-items`를 무시하는가 | **통과** — computed `auto`와 부모의 같은 revision computed default를 명시적으로 연결한다. |
| 10 | `place-self` 첫/둘째 값 또는 단일 값 복제 의미가 뒤집히는가 | **통과** — raw shorthand 파싱 대신 Stylo가 낸 두 computed longhand와 비대칭 scene을 사용한다. |
| 11 | inline/block 축과 item frame 좌표를 서로 바꾸는가 | **통과** — Grid area 기준 x/y 독립 기대값과 비대칭 overflow scene을 기록했다. |
| 12 | `flex-start`/`flex-end` alias가 Flex 컨텍스트 의미로만 적용되는가 | **통과** — Grid의 start/end alias 동작을 사전 관측·규범과 함께 계획한다. |
| 13 | `self-start`/`self-end`가 profile 밖 writing mode 차이를 숨기는가 | **통과** — 두 장면 모두 고정된 동일 writing mode 입력 안에서만 판정한다. |
| 14 | 물리 `left/right`가 잘못된 self property나 RTL에 허용되는가 | **통과** — `justify-self`에만 허용하고 LTR inline 축으로 범위를 제한한다. |
| 15 | safe overflow에서 start fallback이 사라지거나 한 축만 적용되는가 | **통과** — 50×40 item과 40×30 area의 safe center/end 기대 좌표를 `(0,0)`으로 고정한다. |
| 16 | unsafe overflow가 clamp되어 계산 위치가 달라지는가 | **통과** — `(-5,-5)`/`(-10,-10)` layout 원점과 viewport paint clipping을 분리해 확인한다. |
| 17 | at-risk Working Draft overflow keyword를 안정된 일반 적합성으로 주장하는가 | **통과** — Working Draft 관찰 한정 profile이라고 적고 WPT·일반 지원 확대를 명시적으로 제외한다. |
| 18 | 문법상 invalid author declaration을 지원하지 않는 computed value처럼 거부하는가 | **통과** — Chrome의 `auto` cascade fallback을 고정하고 runtime은 알려진 invalid-declaration parser diagnostic만 좁게 허용한다. layout 정렬은 계속 computed value로 판정한다. |
| 19 | 유효하지만 미지원인 baseline/start/stretch 값이 allowlist를 우회하는가 | **통과** — computed value allowlist와 정확한 node/property/value fail-closed 검증을 계획한다. |
| 20 | 다른 Grid owner나 stale style/document revision의 item 값이 적용되는가 | **통과** — item owner 및 revision 검증과 전체 layout 실패 조건을 계획의 필수 assertion으로 둔다. |
| 21 | 일부 item 계산 오류 후 형제 frame이나 scene이 부분 공개되는가 | **통과** — atomic failure와 기존 revision 유지 여부를 검증 단계·성공 조건에 넣었다. |
| 22 | 한 item의 self value가 sibling 위치, source order 또는 root track geometry에 영향을 주는가 | **통과** — sibling 독립성과 node/order/track 불변 조건을 명시한다. |
| 23 | PR #1124의 container `align-items`/`justify-items`/`place-items`나 content alignment가 회귀하는가 | **통과** — 기존 fixed-track container/item alignment 회귀를 다시 실행한다. |
| 24 | 사전 비교가 Spinon 출력이나 WPT 합격 증거로 오인되는가 | **통과** — Chrome 156 DOM 관측은 기준값만 제공하며 구현 결과·적합성 증거가 아니라고 README와 계획에 적었다. |
| 25 | HTML/CSS computed style이 아닌 authored string을 기준으로 shorthand 의미를 판정하는가 | **통과** — 동일 snapshot의 Stylo computed longhand만 구현 입력으로 사용한다. |
| 26 | 계획 범위의 item와 실제 비교 fixture가 달라 runtime admission을 우회하는가 | **수정 후 통과** — 구현 전 점검에서 `X` text node 19개, item `overflow:hidden`, container gap의 `normal` computed value를 발견했다. 자식 없는 빈 `div`, `overflow:visible`, 명시 `0px` row/column gap으로 바꾸고 Agent Browser 관측을 재수집해 eligibility와 비교 입력을 일치시켰다. |
| 27 | 19-scene 사전 비교기가 다른 expected JSON 또는 결과를 읽는가 | **통과** — 로컬 verifier 재실행 결과 `sceneCount=19`, `mismatches=[]`, `pass=true`다. |
| 28 | 이미지 첨부 또는 evidence manifest가 오래된 산출물을 가리키는가 | **통과** — 화면 파일을 PR branch에 포함하고 상대 evidence 링크를 사용하며 SHA-256 manifest를 재생성해 전부 확인했다. |
| 29 | Android AVD 결과가 hardware GPU나 실기기 검증으로 부풀려지는가 | **통과** — 계획은 API 37 ARM64 AVD/SwiftShader와 PixelCopy 표본으로 한정하고 hardware/실기기 주장을 제외한다. |
| 30 | iOS drawable 결과가 compositor 이후/scanout 검증이나 C11 완료로 부풀려지는가 | **통과** — iPhone 17 Pro/iOS 26.2 Simulator의 pre-compositor Metal 표본이라고 제한하고 C11 및 `72/213`은 미완료로 유지한다. |

이 계획 PR에는 구현 코드·simulator 실행 결과·내부 계약 완료 변경이 없다. 계획 검토 20개와 이 변경분 검토 30개는 별도 판정이다.
