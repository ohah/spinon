# PR #1142 diff 적대 검토 20개

검토 대상은 base 94afecef7e1ec92bd6eff36cc3dae1eed32a98e9에서 head 1087bc7368a6fe9ec8cdfbf35a87292e6a1f525a까지의 50-file diff다. 앞선 계획 및 구현/runtime 검토를 반복하지 않고 실제 변경 경계, 기존 경로와의 상호작용, artifact, PR 공개 근거를 따로 확인했다.

| # | 적대 관점 | 확인한 근거와 판정 |
| --- | --- | --- |
| 1 | 다른 작업 파일이나 로컬 WIP가 PR diff에 섞였는가 | PR base/head와 로컬 branch SHA를 대조했고 GitHub Files changed는 50개로 commit 요약과 같다. 기존 untracked evidence는 commit과 PR diff에 포함되지 않았다 — 통과. |
| 2 | bounded candidate가 공식 C11 완료처럼 표시되는가 | spec/STATUS.md와 0059 계약은 C11.1/C11 체크를 열어 두고 공식 완료 72/213을 유지한다 — 통과. |
| 3 | 새 placement mode가 다른 computed-style profile에 적용되는가 | 후보 집합 계산과 one-axis projection 호출은 RuntimeRootHtmlTextPaintV1에서만 실행되고 별도 RootHtmlOneAxisAutoPlacementV1 dispatch를 사용한다 — 통과. |
| 4 | CSS 선언 문자열을 다시 읽어 computed placement와 revision 일관성을 우회하는가 | selector는 ComputedStyleSnapshot의 typed Grid/placement 값을 읽는다. 선언 문자열 parser는 추가하지 않았고 RuntimeSession 결과는 현재 fixture revision의 layout snapshot이다 — 통과. |
| 5 | 기존 all-auto Grid가 새 정규화 경로로 바뀌는가 | all-auto placement는 definite axis selector를 통과하지 않는다. 기존-route control projection test가 이를 고정한다 — 통과. |
| 6 | sibling owner의 placement나 occupancy state가 공유되는가 | item을 owner별로 묶고 각 owner마다 axis agreement를 계산한다. sibling row/column Grid fixture에서 좌표가 독립적이다 — 통과. |
| 7 | hidden 자식 종류에 따라 occupancy가 달라지는가 | display:none은 candidate와 layout visible list에서 제외하고 visibility:hidden은 유지한다. projection fixture 및 PixelCopy/Metal 표본이 hidden slot을 확인한다 — 통과. |
| 8 | 같은 Grid에서 row-locked, column-locked 또는 fully definite item이 섞이는가 | selector와 layout validator가 같은 definite axis 및 반대 축 양끝 auto를 요구한다. mixed/full definite rejection projection/runtime fixtures가 전체 scene 거부를 확인한다 — 통과. |
| 9 | named endpoint, span-only 축, 0 이하 span이 numeric definite placement로 오인되는가 | endpoint validator는 named values와 non-positive span을 거부하고 definite axis 판정에는 numeric line을 요구한다. admission unit tests 및 layout tests가 경계를 고정한다 — 통과. |
| 10 | 음수 line, line/span 조합 또는 극단 line 값이 explicit track 바깥으로 확장되는가 | 기존 resolve_axis, checked end arithmetic 및 allow_positive_implicit=false를 사용한다. negative explicit line의 기대 frame과 범위 밖 line rejection test가 있다 — 통과. |
| 11 | overlap 또는 source order가 placement cursor나 paint order를 바꾸는가 | 공용 sparse normalizer가 occupancy를 기록하고 (order, source_order)로 안정 정렬한다. 새 mode는 order 0만 허용한다. span 및 row/column cursor frame tests가 통과했다 — 통과. |
| 12 | item이 더 필요한 implicit row가 만들어져 부분 layout으로 반환되는가 | one-axis dispatch는 normalizer의 expanded_rows가 있으면 scene을 거부한다. fixed implicit-row sentinel 사례는 GridImplicitTrackRequired를 기대한다 — 통과. |
| 13 | line/span이 implicit column을 요구해도 허용되는가 | sparse resolver가 definite column span을 explicit column count와 대조하고 row-locked cursor가 explicit extent를 넘어가면 거부한다. 3-column/4-item rejection test와 runtime fixture가 있다 — 통과. |
| 14 | repeat·line name·area·dense/column flow·잘못된 track 문법이 후보를 우회하는가 | selector와 layout validator 양쪽에서 route 특성을 검사한다. container track은 non-empty finite non-negative px로 제한하고 track 수 상한을 적용한다 — 통과. |
| 15 | indefinite 크기, gap, border/spacing, alignment, RTL/overflow 값이 지원 범위를 넓히는가 | container validator가 definite finite 크기, zero gap/edge, LTR, 기본 정렬 및 visible overflow를 확인하고 다른 값은 property별 오류로 반환한다 — 통과. |
| 16 | text, nested/replaced/positioned item 또는 intrinsic sizing이 sparse placement에 들어오는가 | layout validator는 empty leaf block, auto/fixed dimensions, static position을 요구한다. Root HTML projection은 image Grid item도 별도 capacity gate에서 거부한다 — 통과. |
| 17 | Grid owner 중 하나가 실패한 뒤 다른 owner의 일부 frame이나 stale snapshot이 publish되는가 | normalizer 및 RuntimeSession은 실패를 반환하고 negative runtime test가 diagnostic과 completed=None을 확인한다 — 통과. |
| 18 | Android 기본 앱이 opt-in C11 provider/payload/FFI를 끌어오거나 여러 adapter를 임의 선택하는가 | generic ServiceLoader는 활성 adapter가 정확히 하나일 때만 사용한다. 기본 APK 검사에서 provider, 전용 adapter, fixture payload와 전용 FFI marker가 없고 opt-in provider만 등록된다 — 통과. |
| 19 | Android PixelCopy 또는 iOS Metal이 다른 draw/surface/revision 표본을 성공으로 보고하는가 | Android adapter는 run ID, presented draw report, surface/viewport/density, generation, sample 수·좌표·RGBA를 검사하며 PixelCopy 6/6 exact다. iOS도 compile flag/argument/run ID/sample을 제한하고 drawable 6/6 exact다. iOS 값은 compositor 전 readback이다 — 통과. |
| 20 | PR 문서가 화면, 테스트, CI, 완료 범위를 과장하거나 잘못 연결하는가 | Agent Browser에서 Android PNG 1080×2400과 iOS PNG 1206×2622가 실제 로드되고 0059 heading anchor도 일치한다. PR checks는 0개이며 main required status checks도 설정되지 않았다. 본문 테스트 수 표기 4/4는 evidence log의 6/6에 맞춰 고쳤다. SwiftShader AVD, compositor 전 readback, 미완료 C11 범위를 명시한다 — 수정 후 통과. |

## 판정

코드 결함은 발견하지 않았다. 문서 검토 중 PR 본문의 style-to-layout count가 오래된 4/4였고 저장소 실행 기록은 6/6임을 확인해 본문을 바로잡았다. PR check rollup은 비어 있고 main required status checks도 없으므로 CI 통과를 주장하지 않는다. Focused Rust·시뮬레이터 결과는 위 변경에 포함된 기록을 기준으로 했으며 이번 diff review 과정에서 테스트를 다시 실행하지 않았다. 이 검토는 Android 실기기/hardware GPU, iOS compositor 후 화면, WPT 적합성 또는 전체 C11을 증명하지 않는다.
