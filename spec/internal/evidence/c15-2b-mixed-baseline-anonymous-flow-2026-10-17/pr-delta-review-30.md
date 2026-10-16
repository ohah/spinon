# C15.2b PR 변경 실패 관점 검토 30개

이 검토는 구현 검토 20개와 관점을 분리해 public/internal interface, 기존 layout 경로, 문서·증거와 주장 범위를 살폈다. PR에 포함될 diff와 저장된 실행 로그를 대상으로 했다.

| # | PR diff 관점 | 확인 근거와 결과 |
| --- | --- | --- |
| 1 | 기존 paragraph API 호출자가 run map을 넘기지 않아도 되는가 | 기존 method는 빈 `InlineRunStyle` map으로 새 내부 호출에 위임한다. workspace test 954개가 통과한다. |
| 2 | 새 run style이 공개 CSS cascade처럼 오해될 이름·계약인가 | crate type는 내부 `InlineRunStyle`; 계약 0070은 고정 profile이라고 명시하고 cascade/fallback/loading을 제외한다. |
| 3 | `ParagraphInput`의 새 field가 이전 기본 값 경로를 비우는가 | 기존 paragraph fixture는 `inline_run_styles` 빈 map을 가지며 기본 run 값으로 계산된다. text-layout 17/17이 통과한다. |
| 4 | `LayoutOutput` 새 flow field의 소비 위치가 문서화됐는가 | `text_flow_layouts[owner]`의 item index/frame/snapshot을 계약 0070과 공개 doc comment에 기록했다. |
| 5 | 공개 struct field 추가가 외부 struct-literal 사용자에게 미치는가 | workspace 검색에서 직접 `LayoutOutput` literal 소비는 발견되지 않았다. 다만 downstream source compatibility는 이 저장소에서 증명할 수 없고, 현재 계약은 출시 전 내부 0.1.0이다. |
| 6 | run map helper가 외부 제품 API로 노출되는가 | HostDocument profile과 text-flow enum은 `pub(crate)`이며 style-to-layout runtime projection으로 연결하지 않았다. |
| 7 | 새 snapshot이 다른 layout revision과 섞일 수 있는가 | 기존 `LayoutTextSnapshot` revision stamp를 재사용하며 최종 output 구성 시 같은 `LayoutInput` revision을 넣는다. |
| 8 | final layout 이후 다른 width의 measure snapshot을 반환하는가 | 기존 exact content-width 검증과 auto-height 검증이 flow synthetic node에도 적용된다. 회귀 suite 통과. |
| 9 | synthetic node가 external NodeId와 충돌할 수 있는가 | Taffy index는 기존 postorder 길이 뒤에서 할당되고 별도 map에만 보관한다. `checked_taffy_id`와 overflow error 경로를 확인했다. |
| 10 | synthetic node 수 계산이 정수 overflow를 일으키는가 | 각 증가에 `checked_add`, Taffy ID와 order 변환에 checked conversion을 사용해 `TooManyNodes`로 닫는다. |
| 11 | display:flex의 order sort가 source-ordered text flow를 바꾸는가 | flow owner에서는 원본 items를 구성하고 flex sort를 건너뛴다. 일반 flex 노드는 기존 sort 경로를 유지한다. |
| 12 | absolute/fixed child가 원본 flow와 viewport root 양쪽에 들어가는가 | flow는 out-of-flow direct child를 거부하고, validation 전에 일반 reparent를 실행하지 않는다. negative fixture가 확인한다. |
| 13 | display:none child가 visible-child equality를 깨뜨리거나 flow에 들어가는가 | validation은 hidden 목록을 제외한 child만 비교하고 HostDocument는 hidden subtree를 제외 수집한다. hidden source regression 통과. |
| 14 | hidden descendant style 누락이 조용히 무시되는가 | hidden layout node 수집 중 누락 style은 `MissingStyle`로 전파된다. HostDocument negative fixture가 이를 확인한다. |
| 15 | text source node가 여러 paragraph owner에 중복 귀속될 수 있는가 | source insertion set이 중복 ID를 `DuplicateTextParagraph`로 거부한다. 기존 duplicate ownership 검사 통과. |
| 16 | 잘못된 UTF-16 입력이 로그/geometry에서 손실되는가 | 명시적 `InvalidTextEncoding`이 유지되고 source UTF-16/paragraph UTF-8 mapping 계약이 그대로다. supplementary scalar 회귀가 통과한다. |
| 17 | C15.2a에서 지원했던 span depth가 새 splitter로 낮아지는가 | direct inline spans는 기존 collector로 전달하고 최대 2단계 정책·border/padding fixture를 유지한다. focused 16/16 통과. |
| 18 | 빈 paragraph input이 flow owner의 public segment를 부풀리는가 | 빈 paragraph는 synthetic node 없이 일반 빈 paragraph 결과 경로를 쓴다. flow item 생성은 non-empty source에 한한다. |
| 19 | source-order item index가 Taffy child ordinal과 혼동되는가 | 결과 `item_index`는 원본 `flow.items` 순번이며 Block 사이의 anonymous paragraph 인덱스를 보존한다. fixture는 0/1/2를 확인한다. |
| 20 | frame 좌표가 content-box와 border-box 기준 사이에서 모호한가 | 계약·evidence는 owner border-box 기준을 명시하고 inset fixture에서 offset 이중 적용을 막는다. |
| 21 | Chrome oracle screenshot을 제품 screenshot으로 표시하는가 | README와 파일 설명은 CSS fixture/Chrome DOM oracle이며 Spinon renderer 출력이 아니라고 명시한다. |
| 22 | oracle 대조가 test-only marker 오프셋 때문에 거짓 exact 비교를 하는가 | README는 baseline marker가 Rust에 없으므로 x 시작 좌표 비교에서 제외하고 width/vertical baseline만 대조한다고 설명한다. |
| 23 | fixture/font identity가 재현 가능한가 | DOM fixture, Ahem/Roboto, oracle screenshot SHA-256을 기록하고 선행 precomparison README를 연결했다. |
| 24 | 허용 오차가 사후 튜닝처럼 보이는가 | 구현 전 비교 표에 `0.05px` 허용 오차와 구체 geometry를 고정했다. |
| 25 | 실패 경로가 성공값과 같은 oracle 범위로 과장되는가 | nested Block, duplicate, unknown ID, out-of-flow, missing font, invalid metric negative control을 별도 열거한다. |
| 26 | public conformance 문서가 C15 완료를 표시하는가 | `spec/STATUS.md`의 C15는 `[ ]`이며 conformance 문구도 projection/painter/simulator 앱의 미검증을 분명히 한다. |
| 27 | 내부 계약 숫자 버전이 구현 slice 때문에 올라가는가 | 계약 0070의 0.1.0 고정 원칙을 유지하고 문서 내용만 갱신했다. |
| 28 | Rust target check를 앱 simulator 성공이라고 주장하는가 | Android/iOS 로그 이름과 README에서 Rust cross-target compile로 한정하고 app launch/화면은 미검증으로 분리했다. |
| 29 | 미실행 검증이 통과했다고 적혀 있는가 | workspace/focused/Clippy/rustfmt 로그를 각각 저장했다. simulator app/pixel, overflow/clip은 미확인으로 적었다. |
| 30 | PR 변경 중 새 결함 발견 뒤 재확인이 빠졌는가 | run baseline y, parent inset, out-of-flow reparent 관점을 수정 후 재실행했으며 focused 16/16, text-layout 17/17, workspace 954/0/4를 기록했다. |

## 남은 제한

`LayoutOutput` 공개 struct에 field가 추가돼 downstream에서 직접 struct literal을 만들던 외부 Rust crate의 source compatibility는 저장소 내부 검사만으로 확인할 수 없다. 이 저장소의 계약은 출시 전 내부 0.1.0이며 workspace 소비자 회귀는 통과했지만, 외부 downstream compile-test는 실행하지 않았다. C15 runtime paint 및 simulator app 검증은 이 diff에 포함되지 않는다.
