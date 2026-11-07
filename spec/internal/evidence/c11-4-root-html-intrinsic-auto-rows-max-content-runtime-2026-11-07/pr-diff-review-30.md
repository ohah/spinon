# `grid-auto-rows:max-content` PR diff 적대 검토 30개 · 2026-11-07

계획 검토 및 구현/runtime 검토와 구분해, PR에 들어갈 구현·문서·실행 근거 diff를 서로 다른 실패 경로로 다시 확인했다. 기존 회귀를 포함한 focused 결과는 실행 README에 기록하고, 이 표는 변경 diff의 계약 경계와 배포 근거를 추적한다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 다른 computed-style profile까지 `max-content`가 허용됨 | 신규 auto-row 경로는 `RuntimeRootHtmlTextPaintV1` 조건 안에 있으며 다른 profile의 projection 분기는 변경하지 않았다. |
| 2 | Grid가 아닌 Root HTML 노드가 intrinsic placement route에 들어옴 | route 후보는 `grid_layout`이 존재하는 element를 사용하며 기존 Grid owner 검증을 유지한다. |
| 3 | `max-content`의 cascade identity가 `min-content`로 바뀜 | projection은 typed `ComputedCssGridTrackSize::MaxContent`를 `GridTrackSize::MaxContent`로 보존한다. cascade와 layout RuntimeSession assertion이 이를 각각 확인한다. |
| 4 | `[max-content]` 이외의 auto-row 목록이 통과함 | helper는 정확한 singleton slice만 candidate로 인식한다. max/min 혼합 및 max/길이 혼합 negative control이 거부된다. |
| 5 | 빈 auto-row list 또는 기본 `auto`가 max-content 경로로 오인됨 | intrinsic route의 track-list pattern과 기존 `Auto` 분기를 분리했다. `auto` sizing 동작은 기존 경로에 남는다. |
| 6 | 0·음수·무한 explicit row가 bounded max-content를 통과함 | explicit row shape는 유한하고 양수인 단일 fixed-px track으로 제한된다. |
| 7 | 여러 explicit row track이 max-content로 허용됨 | fixed-track capacity predicate와 공통 track 검증이 단일 row 조건을 확인한다. |
| 8 | intrinsic 또는 indefinite explicit column이 고정-track 경로에 들어옴 | max-content fixed-track candidate는 유한하고 양수인 단일 fixed-px column을 요구한다. intrinsic-column max-content 조합 negative control이 RuntimeSession에서 거부된다. |
| 9 | fixed-repeat group이 단일 고정 track처럼 취급됨 | row 및 column fixed-repeat group count가 0인지 capacity predicate에서 검사한다. |
| 10 | intrinsic column과 max-content가 기존 min-content 전용 결합 계약을 넓힘 | intrinsic-column 결합 predicate가 `root_html_has_intrinsic_implicit_min_content_rows`를 별도로 요구해 max-content는 차단한다. |
| 11 | 비영 gap이 implicit row의 계산에 조용히 합산됨 | max-content RuntimeSession negative control의 `row-gap:1px`이 `grid-auto-rows` 오류로 거부된다. 후보의 row/column gap 계약은 0이다. |
| 12 | `grid-auto-columns`가 implicit column을 만들어냄 | intrinsic candidate와 layout contract 모두 `auto_columns`만 허용한다. implicit-column reservation 경로는 추가하지 않았다. |
| 13 | column-flow 또는 dense placement가 row-sparse candidate를 우회함 | projection validator는 `GridAutoFlow::Row`를 요구하며 layout intrinsic contract의 기존 flow 제한도 유지한다. |
| 14 | areas나 auto-repeat과의 결합이 추적되지 않은 track을 생성함 | `template_areas`, columns/rows auto-repeat 및 intrinsic profile의 제한이 유지된다. |
| 15 | 별도 Grid sizing mode의 입력 목록에 `MaxContent`가 확산됨 | layout의 intrinsic placement mode에서만 min/max singleton을 허용한다. 변경하지 않은 배치 mode는 기존 검증을 그대로 사용한다. |
| 16 | item capacity가 fixed explicit row와 intrinsic implicit row의 수를 잘못 계산함 | capacity admission이 이 max-content fixed-track shape를 승인하도록 보완됐고, 세 direct child RuntimeSession positive case가 complete layout까지 간다. |
| 17 | sparse auto-placement 정규화가 projection보다 넓은 track 목록을 허용함 | normalizer는 intrinsic mode에서 min/max track variant만 받아들이고, 상위 계약이 singleton과 max-content fixed shape를 제한한다. |
| 18 | one-axis container validation이 max-content를 generic auto track으로 재거부함 | intrinsic mode의 shared validator가 exact singleton min/max를 통과시키며 다른 track shape는 기존 validation으로 남긴다. |
| 19 | Taffy가 max-content를 min-content처럼 sizing함 | Layout trace의 최종 row base sizes `[10,20,34]`가 독립 기대값과 일치하고 typed track identity도 별도로 확인된다. |
| 20 | 고정 높이 자식만으로 text max-content 결과를 흉내 냄 | parsed Root HTML fixture의 direct-text item은 40px inline size에서 실제 두 줄로 wrap한다. Runtime test는 glyph와 painted line count를 확인한다. |
| 21 | track trace가 explicit row만 측정하고 implicit rows를 놓침 | 최종 row sizing stage의 비-gutter·비-collapsed tracks 전체를 읽어 세 행의 크기를 검증한다. |
| 22 | 마지막 item의 overflow가 Grid 경계에서 잘리거나 frame과 paint가 어긋남 | 마지막 frame은 `[0,30,40,34]`, visible bottom은 64px로 독립 기대값과 일치하며 40px Grid 아래 overflow를 보존한다. |
| 23 | unsupported mixed shape가 다른 property 오류로 보고됨 | mixed auto-row 목록과 intrinsic-column 조합 negative controls는 `grid-auto-rows`에 귀속된 RuntimeSession 오류를 확인한다. |
| 24 | 환경 변경 뒤 실패 결과가 이전 완료 snapshot을 partial result로 노출함 | dark media rule로 mixed tracks를 적용한 요청은 `Failed`와 `completed=None`을 확인한다. |
| 25 | 실패 후 같은 RuntimeSession이 stale 결과로 복구됨 | 동일 session의 light→dark reject→light 회복을 수행하고 새 ready snapshot과 마지막 item frame을 확인한다. |
| 26 | paint text source와 layout snapshot이 서로 다른 문서 revision에서 결합됨 | RuntimeSession positive assertion이 document, render-tree, style revision identity를 완료 snapshot과 대조한다. |
| 27 | Android 검증 artifact와 시험한 fixture source가 다름 | fixture sha가 source, Android APK asset 및 설치 APK 근거에서 일치한다. API 37 ARM64 AVD이며 SwiftShader 사용을 기록했다. |
| 28 | iOS 검증이 다른 fixture 또는 기본 route 밖의 경로를 사용함 | source hash와 설치 bundle을 대조하고 iPhone 17 Pro/iOS 26.2 Simulator 기본 route의 ready 상태를 기록했다. |
| 29 | simulator screenshot을 GPU pixel readback이나 일반 지원으로 확대 해석함 | 실행 README는 screenshots를 visual smoke로 한정하며 PixelCopy, Metal drawable readback, compositor-after pixels, 실기기, hardware GPU 및 WPT를 제외한다. |
| 30 | candidate가 공식 C11.4/C11 완료나 CI 통과로 게시됨 | `STATUS.md`는 `78/214`와 incomplete 상태를 보존한다. PR #1264는 enhancement/area: layout 라벨로 열렸고 현재 check rollup은 비어 있으므로 CI 통과를 기록하지 않는다. simulator 증거만으로 완료를 표시하지 않는다. |

검토 과정에서 이번 diff에서 추가 수정이 필요한 미해결 결함은 확인하지 못했다. PR이 생성된 뒤에는 실제 PR head와 CI rollup을 별도로 확인한다.
