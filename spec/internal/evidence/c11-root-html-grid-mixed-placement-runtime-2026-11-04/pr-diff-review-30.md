# C11.1 mixed explicit/one-axis placement PR diff 적대 검토 30개

**대상:** PR [#1147](https://github.com/ohah/spinon/pull/1147)의 실제 `main...feat/c11-root-html-grid-mixed-placement-impl` diff, 제한 runtime 입력과 시뮬레이터 근거.
**검토 head:** `eb006631c`.
**방법:** 앞선 구현/runtime 20개와 구현 diff 20개 문서에 기대지 않고, 구현 진입·placement 정규화·호스트 경계·artifact 및 주장 범위에서 서로 다른 실패 조건을 대조했다.

| # | 실패 관점 | 판정 |
| ---: | --- | --- |
| 1 | 새 placement mode가 Root HTML 이외 profile에도 적용되는가 | Selector 실행과 mode 전환은 `RuntimeRootHtmlTextPaintV1`로 제한된다. 다른 profile의 computed Grid는 기존 projection 경로를 유지한다. |
| 2 | authored CSS 문자열을 재해석해 computed placement와 다른 mode를 고르는가 | Selector는 `ComputedCssGridItemPlacement` typed endpoints를 사용하고 authored text 파싱을 추가하지 않는다. |
| 3 | sibling Grid owner가 item 축이나 occupancy 분류를 공유하는가 | `grid_item_owners`로 owner별 item 목록을 만든다. style integration 테스트는 sibling owner 간 격리를 확인한다. |
| 4 | 중첩 descendant가 직접 item처럼 candidate cardinality에 들어오는가 | selector 입력은 runtime Grid tree가 소유자로 연결한 direct item set이다. nested Grid item은 별도 owner이며 layout admission은 nested item을 거부한다. |
| 5 | fully definite만 또는 partial만 있는 owner가 mixed mode를 선택하는가 | Selector와 layout validator 모두 fully definite와 one-axis item 각각 하나 이상을 요구한다. 두 불완전 조합의 focused tests가 있다. |
| 6 | 여러 partial item이 row-lock과 column-lock을 섞어 같은 owner에 들어오는가 | Selector의 단일 `axis`와 validator의 `grid_locked_axis`가 축 변경을 거부한다. |
| 7 | partial item의 반대 축 endpoint 한쪽만 `auto`인 입력이 통과하는가 | `placement_kind`는 opposite axis 양 끝이 모두 `Auto`인 경우만 one-axis로 분류한다. layout validator도 같은 조건을 다시 검사한다. |
| 8 | line 없이 span만 둔 축이 definite axis로 잘못 분류되는가 | `has_line`이 numeric line을 요구하므로 span-only axis는 fully definite/one-axis 어느 쪽에도 속하지 않아 거부된다. |
| 9 | named line, named span 또는 area placement가 numeric mode에 유입되는가 | selector numeric allowlist와 layout endpoint validator가 named forms를 거부하며 candidate는 area owner를 제외한다. |
| 10 | zero span 또는 unsigned 범위 밖 span이 occupancy 크기로 사용되는가 | layout endpoint 검사와 `resolve_axis`의 positive-span 변환에서 0 및 변환 불가 span을 오류로 반환한다. |
| 11 | reversed/equal numeric line 쌍의 span을 mode별로 다르게 계산하는가 | mixed mode는 공통 sparse normalizer의 `resolve_axis`를 사용하며 line 정규화와 기존 explicit line 변환을 공유한다. |
| 12 | 음수 line이 implicit negative track을 조용히 만들거나 잘못된 track을 참조하는가 | `allow_positive_implicit=false` 경로에서 negative line은 explicit track count 기준으로 해석되고 범위 밖은 거부된다. |
| 13 | DOM 뒤쪽 fully definite item이 앞쪽 partial item 이후에 예약되는가 | sparse normalizer는 모든 definite rectangle을 먼저 occupancy에 넣고 그 뒤 row/column partial pass를 수행한다. |
| 14 | row-locked partial의 cursor가 같은 행 blocker를 건너뛰지 않는가 | cursor는 definite row별로 따로 시작하고 occupancy search는 blocker 다음 빈 열을 찾는다. 수정된 row fixture의 Android/iOS 표본이 blocker와 후속 item을 따로 관측한다. |
| 15 | column-locked partial이 definite row/column blocker를 통과해 겹치는가 | column-locked pass는 fixed column에서 충돌 없는 row를 찾아 배치하고 source order 및 cursor 진행을 유지한다. focused spans/reference frames가 이 경로를 검사한다. |
| 16 | 두 축 span의 교차 셀이 아닌 시작점만 검사해 overlap을 허용하는가 | occupancy는 start/end 사각형 전체를 차지한다. row/column span blocker 장면과 independent frame 기대값이 이를 대조한다. |
| 17 | 한 행에서 배치 공간이 모자라면 auto row를 늘리고 결과 일부를 반환하는가 | sparse 결과의 expanded rows가 mixed branch에서 발견되면 placements를 output에 합치기 전에 오류를 반환한다. |
| 18 | fully definite line이 explicit column 수 밖에서 implicit column을 만들 수 있는가 | `resolve_axis`가 explicit bound를 넘는 definite line을 거부하고 별도 layout/style integration test가 확인한다. |
| 19 | partial span 또는 blocker 조합이 explicit column extent 밖으로 밀려나는가 | sparse placement의 fixed-column span bound 및 occupancy placement 실패가 오류로 끝나며 implicit-column test가 회귀를 막는다. |
| 20 | 빈 track, 비유한/음수 track, track cap 초과가 candidate admission을 우회하는가 | layout mode validator가 두 explicit track list의 비어 있음·finite/non-negative px·`GRID_MAX_TRACKS`를 검사한다. |
| 21 | 컨테이너 intrinsic/minmax/percentage dimension이나 nonzero box edge가 제한 profile에 들어오는가 | definite finite width/height, fixed-px explicit tracks, zero gap/edges 및 지원 alignment 조건 밖 입력은 layout validation에서 fail-closed한다. |
| 22 | item content나 intrinsic sizing이 빈 block의 auto size 의미를 바꾸는가 | layout admission은 empty static direct block와 no text/nested content를 요구한다. Root HTML style projection의 `validate_root_html_grid_item_capacity`는 `img` Grid item을 별도로 거부하므로 replaced image도 mixed route에 들어오지 않는다. |
| 23 | positioned item 또는 `order`가 DOM source order와 다른 placement를 만들 수 있는가 | style projection과 layout validator는 nonzero `order`를 거부하고 layout validator는 non-static positioning을 거부한다. |
| 24 | `display:none`과 `visibility:hidden`이 동일하게 occupancy에서 제거되는가 | hidden subtree는 display suppression으로 구성되고 visibility는 별도 유지된다. style integration과 runtime 표본에서 두 의미를 분리한다. |
| 25 | 실패 Grid가 expanded track, partial placement 또는 stale snapshot을 publish하는가 | placement normalization은 local result를 완성한 뒤 `Result`로 반환한다. mixed branch error 시 결과 object를 publish하지 않으며 Root HTML fail-fast의 문서 단위 한계도 문서화했다. |
| 26 | mixed selector가 기존 all-auto, fully explicit 또는 one-axis route를 덮어쓰는가 | 후보 조건은 상호 배타적이다. 런타임 fixture에는 all-auto/full explicit controls가 있고 기존 one-axis regression module 7/7이 별도 기록돼 있다. |
| 27 | Android adapter가 임의 run ID나 다른 장면의 sample table을 실행하는가 | adapter는 기존 one-axis와 고정 mixed run ID만 허용하고 run별 scene/sample set을 선택한다. mixed scene은 final log에 `c11mix19-20261104`로 기록됐다. |
| 28 | Android PixelCopy가 다른 크기 surface나 density 좌표를 exact 성공으로 오인하는가 | draw report의 status/presentation, surface 크기, viewport·density 비율, sample 수와 각 좌표/RGBA를 함께 검사한다. final AVD log는 19/19 exact다. |
| 29 | Android 기본 APK 또는 iOS 기본 app에 fixture adapter/payload가 포함되는가 | Android default DEX scan은 새 adapter/run/sample marker 0건이고 opt-in은 모두 포함한다. iOS default binary scan도 새 marker 0건, opt-in scan은 positive control로 포함을 확인한다. |
| 30 | iOS run ID가 이전 one-axis 장면을 mixed scene으로 오인하거나 compositor 이후 parity로 과장되는가 | parser는 고정 mixed run ID를 별도 scene/sample set에 연결한다. 19/19는 opt-in Metal drawable readback으로만 기록하고 compositor 제출 전 경계를 명시한다. |

## PR 및 evidence 경계

- PR base는 `main`, head는 `feat/c11-root-html-grid-mixed-placement-impl`이며 mergeability는 `clean`이다. PR labels는 `enhancement`, `area: layout`, `area: android`, `area: ios`다.
- Chrome 156 Agent Browser 재실행 JSON/PNG hash는 저장된 observation/화면과 같고 독립 기대값 검증기는 9 scenes·19 visible frames에서 최대 0 CSS px 차이를 기록한다.
- PR 본문 Android/iOS 이미지 링크를 Agent Browser로 열어 각각 1080×2400과 1206×2622 PNG가 실제 로드되는 것을 확인했다. 확인 뒤 Agent Browser session을 닫았고 active session은 남지 않았다.
- `gh pr checks 1147` 및 commit check-runs는 모두 비어 있다. `main`에는 required status checks와 required approval이 설정되어 있지 않다. 따라서 CI 통과를 주장하지 않는다. 보호 설정의 linear history에 맞춰 병합할 때는 Rebase and merge만 사용한다.
- focused run 기록은 placement layout 9/9, Root HTML integration 8/8, selector 3/3이다. 전체 `spinon-layout` 실행은 431/432였고 C15 `host_document_collects_direct_text_and_inherited_spans_in_source_order` assertion 하나가 실패했다. 이 검토는 이를 통과로 세지 않으며 base에서의 재현 여부도 확인하지 않았다.
- 상태 대장 C11.1/C11 checkbox와 공식 완료 `72/213`은 그대로다. 이 제한 candidate로 상위 항목을 완료 처리하지 않는다.

**판정:** 실제 PR diff를 30개의 별도 failure path로 검토했다. 이 검토에서 추가 blocking 구현 결함은 확인하지 않았다. PR check은 보고되지 않았으며, 전체 package suite의 단일 C15 실패는 별도 미해결 검증 상태로 남긴다.

## 검토 후 문서 delta

30관점 검토 대상 구현 diff의 head는 `eb006631c`였다. 그 뒤 `65eb3cdd4`에서 이 검토 기록과 runtime README·계획·상태 대장·내부 계약의 링크 및 PR 상태 요약만 추가·갱신했다. 이 후속 delta를 별도로 대조했다. Rust/Android/iOS 구현, fixture와 측정값은 바뀌지 않았고, 수정된 문서는 72/213 및 미완료 상태와 일치하며 `git diff --check eb006631c..HEAD`를 통과했다.
