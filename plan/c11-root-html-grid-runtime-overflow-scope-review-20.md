# C11.1 visible overflow scope refinement 검토 20개

검토 대상: [Root HTML Grid runtime 계획](./c11-root-html-grid-runtime.md)의 item overflow 조건 수정, 고정 source fixture, Chrome 156 관측, Android/iOS Simulator 관측.

기존 계획의 “item 크기와 assigned track 크기가 다르면 fail-closed” 조건은 source-order 역전 양성 통제와 맞지 않았다. item 폭을 유지한 채 다른 column으로 이동하는 CSS Grid 동작을 보존하도록 조건을 정리했고, root 바깥까지 나가는 fixed-size item을 별도 fixture로 고정했다.

| # | 실패 관점 | 계획 검토 및 판정 |
|---:|---|---|
| 1 | item track overflow와 container overflow를 혼동 | track보다 item이 커도 item frame을 clamp하지 않는다. root 밖 extent를 별도 입력으로 기록한다. 통과 |
| 2 | item overflow로 root 크기가 바뀜 | main은 120×70px 고정이며 기대 JSON과 Chrome 관측이 같다. 통과 |
| 3 | item 폭의 오른쪽 끝을 놓침 | item-b 끝 x=121px과 scroll overflow right를 Chrome에서 확인한다. 통과 |
| 4 | item 높이의 아래쪽 끝을 놓침 | item-a 끝 y=71px과 scroll overflow bottom을 Chrome에서 확인한다. 통과 |
| 5 | item frame을 track 경계로 잘라냄 | Rust runtime expected frame은 41×71px·81×70px이며 Chrome과 같다. 통과 |
| 6 | source-order 역전이 CSS 크기를 바꿈 | item intrinsic/fixed dimensions는 유지하고 auto-placement 위치만 바꾼다. 통과 |
| 7 | 겹친 item의 paint order를 혼동 | root HTML child source order가 유지되고 후속 green item이 겹치는 영역 위에 그려진다. 통과 |
| 8 | visible overflow를 clipping 지원으로 과장하거나 authored clip을 무시 | overflow clip/scroll은 계획에서 계속 제외하며 non-visible overflow를 Grid container/item에서 거부한다. 통과 |
| 9 | root item 수가 implicit row를 생성 | explicit capacity 초과 item은 `grid-auto-rows` 오류로 partial scene 없이 실패한다. 통과 |
| 10 | auto row sizing을 implicit track support로 오인 | non-default `grid-auto-rows` author 선언은 Root HTML cascade gate가 거부한다. 통과 |
| 11 | flexible track이 고정 px로 조용히 변환 | `1fr`은 fixed-px validator가 명시적으로 거부한다. 통과 |
| 12 | percentage track의 viewport basis가 고정값으로 오인 | percentage track은 Root HTML fixed-px gate에서 거부한다. 통과 |
| 13 | repeat/named lines/areas를 고정 목록으로 오인 | repeat, line name, area는 기존 Root HTML profile gate에서 거부한다. 통과 |
| 14 | author placement/order가 source order를 우회 | explicit placement·nonzero order는 cascade gate가 거부한다. 통과 |
| 15 | non-default auto-flow가 row로 무시 | column flow는 Root HTML cascade gate가 거부한다. 통과 |
| 16 | gap·edge·position이 overflow 좌표와 결합 | nonzero gap/padding/margin/border와 non-static position negative controls가 실패한다. 통과 |
| 17 | writing axis와 inline direction을 미검증 상태로 허용 | non-horizontal writing mode와 `direction:rtl`은 fail-closed하고 Root HTML Grid 입력은 horizontal LTR로 제한한다. 통과 |
| 18 | whitespace 처리로 anonymous text item 수를 바꿈 | CSS Grid §6에 따라 whitespace-only text는 `white-space:pre`여도 item에서 제외하고 non-whitespace direct text는 거부한다. 통과 |
| 19 | simulator 화면을 exact frame oracle로 취급 | exact frame은 Rust assertion, Chrome DOM rect는 독립 oracle, simulator 화면은 보조 증거다. 통과 |
| 20 | fixture build나 부분 slice가 제품 ABI/공식 상태를 오염 | Android/iOS fixture-test feature는 0이며 binary marker 검사와 `72/213`·parent C11 미완료 상태를 함께 확인한다. 통과 |

**PASS 20 / PARTIAL 0 / FAIL 0.** Chrome 156 geometry와 Rust runtime frame 통제가 refinement를 뒷받침한다. CSS Grid §6의 anonymous whitespace, horizontal LTR gate, Android API 37 AVD와 iOS 26.2 Simulator의 동일 overflow fixture를 기본 Root HTML route로 확인했다. Simulator screenshot은 pixel readback이나 compositor 이후 확인이 아니다.
