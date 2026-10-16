# C15.2b 구현 실패 관점 검토 20개

대상은 mixed font run geometry와 HostDocument의 anonymous inline/block flow 구현이다. 각 항목은 별도 불변 조건을 확인한다. 확인 결과는 해당 fixture, 오류 경로, 또는 코드 경계를 적었다.

| # | 실패 관점 | 확인 근거와 결과 |
| --- | --- | --- |
| 1 | 유한하지 않거나 0 이하인 inline font size가 shaping에 전달되는가 | `run_styles_fail_closed_for_missing_fonts_and_invalid_metrics`의 NaN 입력은 `InvalidStyle`로 거부된다. `InlineRunStyle::validate`도 0 이하를 거부한다. |
| 2 | line-height가 유한하지 않거나 0 이하일 때 line box가 오염되는가 | 같은 negative fixture에서 무한 line-height를 거부한다. 유효성 검사는 engine 입력 전에 실행된다. |
| 3 | 명시했지만 registry에 없는 family가 조용히 기본 폰트로 바뀌는가 | `MissingFontFamily`를 반환한다. mixed-font fixture는 Ahem/Roboto를 각각 명시 등록한다. |
| 4 | run style이 없는 기존 caller가 새 기본값 때문에 달라지는가 | 기본 `ParagraphStyle`을 run base로 사용하며 기존 paragraph 입력 API는 빈 run-style map을 전달한다. 전체 workspace 954개가 통과한다. |
| 5 | 중첩 span에서 바깥 값이 안쪽 명시 값을 덮는가 | source ancestor를 바깥에서 안쪽 순으로 적용해 가장 안쪽 owner 값이 우선한다. mixed fixture는 span 경계의 run family/size를 함께 확인한다. |
| 6 | 한 source의 run style이 다음 source로 새어 나가는가 | source UTF-8 range와 owner ancestry로 각 run을 분리한다. mixed fixture에서 Ahem/Roboto fragment의 family·size·baseline을 각각 확인한다. |
| 7 | 서로 다른 font ascent가 공통 paragraph baseline 좌표에 잘못 더해지는가 | mixed oracle의 baseline 19px, Roboto y=0px, Ahem y=11px와 비교한다. run fragment top을 고칠 때 line top 이중 가산을 제거했다. |
| 8 | 낮은 run line-height가 높은 sibling run의 line box를 줄이는가 | 같은 줄의 10px/10px Ahem과 20px/24px Roboto가 line-height 24px를 유지하는 focused fixture가 통과한다. |
| 9 | wrapped source fragment의 y가 line top을 두 번 더하는가 | C15.2a 줄별 fragment 회귀와 C15.2b baseline fixture를 재실행해 paragraph 좌표 변환을 확인했다. 발견했던 line-top 이중 가산을 `run_baseline - run_box_ascent`로 수정했다. |
| 10 | 잘못된 UTF-16이 replacement character로 조용히 변환되는가 | `push_text`는 `String::from_utf16` 실패를 `InvalidTextEncoding(node)`로 전달한다. 기존 malformed input 회귀가 통과한다. |
| 11 | 빈 text node가 0-height anonymous paragraph를 만드는가 | 빈 source는 ownership은 기록하되 shaping source 목록에는 넣지 않는다. 빈 문자열 engine 경계 테스트가 line을 만들지 않음을 확인한다. |
| 12 | display:none inline subtree가 visible flow item 또는 source로 들어가는가 | hidden subtree는 별도 제외 수집 경로를 사용하며, HostDocument 회귀에서 숨김 text는 source와 inline owner에서 빠진다. |
| 13 | 직접 지원하지 않는 visible element가 text로 뭉뚱그려지는가 | 직접 child는 HTML span 또는 display:block만 허용한다. 기존 non-span negative fixture와 nested inline subtree 검사가 이를 거부한다. |
| 14 | span 안의 Block이 direct flow block인 것처럼 승격되는가 | inline subtree walker는 span/text 이외 topology를 거부한다. `inline_span_rejects_nested_block_topology`가 이를 확인한다. |
| 15 | direct Block 경계에서 앞/뒤 inline source가 합쳐지거나 역전되는가 | anonymous paragraph를 Block 경계마다 flush한다. positive fixture는 item index 0/1/2, block y=10px, trailing segment y=20px를 확인한다. |
| 16 | 연속 Block 사이에 내용 없는 synthetic paragraph가 끼는가 | `push_anonymous_paragraph`는 source가 비었으면 item을 만들지 않는다. source 없음이 Taffy child count를 늘리지 않는 구현 경계를 확인했다. |
| 17 | 같은 Block이 flow item에 중복되어 레이아웃되거나 absolute reparent와 충돌하는가 | flow validation이 duplicate와 absolute/fixed direct child를 Taffy 구성 전에 거부한다. 각 negative fixture가 통과한다. |
| 18 | flow가 graph 외 ID, 빠진 visible child, 또는 순서가 다른 child를 받을 수 있는가 | graph 존재와 visible direct-child 전체 일치/순서를 검증한다. graph 밖 ID 및 duplicate는 negative fixture로 확인했고 누락·순서 불일치는 같은 equality guard가 닫는다. |
| 19 | parent border/padding이 synthetic paragraph frame에 두 번 더해지는가 | 추가 inset 변형에서 frame을 owner border-box 기준으로 확인한다. 발견된 inset 중복 가산을 제거했고 parent `30×40px` oracle이 유지된다. |
| 20 | 내부 synthetic node가 HostDocument ID/frame 수로 노출되는가 | synthetic Taffy ID는 `engine_ids` 외부 map에 넣지 않고 결과는 owner의 `text_flow_layouts`에만 모은다. 외부 frame 개수와 flow owner 결과를 fixture에서 확인한다. |

## 판정

검토 중 line fragment y 이중 가산, owner inset 중복 가산, out-of-flow Block의 reparent 중복 가능성을 결함으로 처리했다. 수정 뒤 해당 baseline, inset, out-of-flow 검사를 다시 실행했고 관련 focused suite 16/16 및 text-layout suite 17/17이 통과했다. runtime paint와 clip은 검토 대상 구현이 제공하지 않으므로 검증 완료로 간주하지 않았다.
