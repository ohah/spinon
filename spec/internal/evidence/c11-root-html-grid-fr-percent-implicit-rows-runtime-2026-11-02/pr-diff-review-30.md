# PR #1027 구현 변경 diff 실패 관점 검토 30개

대상: PR [#1027](https://github.com/ohah/spinon/pull/1027), merge base `3f7bfa71ae67a1f61cb8a5154f0cb74137bad9fc`, 구현 commit `0b075e3845262a3624e4bb03fa06ef3ab6451c28`. 실제 변경 소스·상태/계약 문서·evidence를 각각 대조했다. 구현과 실행 경로에 대한 별도 신규 20개 검토는 [implementation-runtime-review-20.md](implementation-runtime-review-20.md)에 있다.

| # | 별도 diff 관점 | 확인 결과 |
|---:|---|---|
| 1 | 공개 API나 ABI가 추가되어 지원 범위가 제품 API처럼 보이는가 | 새 public type, FFI, fixture feature 또는 ABI는 없다. 기본 Root HTML route의 내부 Grid mode 안에서만 조합한다. `crates/spinon-layout/src/grid.rs`, runtime evidence README. 통과. |
| 2 | 변경이 기존의 다른 computed-style profile까지 전파되는가 | Root HTML explicit validator와 `RootHtmlImplicitRowsV1` contract 조건만 넓혔다. named-area/track-sizing profile admission은 변경하지 않았다. `grid_values.rs:120-165`, `grid.rs:1049-1104`. 통과. |
| 3 | Track admission이 direct px/%/fr 이외의 grammar를 실수로 포함하는가 | 두 축에 finite direct `LengthPx | Percentage | Fraction`만 남긴다. `_`·`Auto`·`MinMax`·`FitContent`는 계속 거부된다. Projection과 layout validator를 대조했다. 통과. |
| 4 | 음수, NaN, infinity가 projection 또는 layout에서 통과하는가 | 양 단계 모두 `is_finite()`와 `>= 0.0`을 검사한다. 기존 invalid-value controls와 신규 composed cases를 확인했다. 통과. |
| 5 | `%` normalization이 25%를 25배 크기로 해석하는가 | Computed percentage `0.25`를 typed value에 그대로 전달하고 definite 120px axis에서 30px/90px가 나온다. Chrome reference와 projection/runtime expectation 일치. 통과. |
| 6 | `fr` factor가 definite root axis의 자유 공간 분배와 분리되는가 | Fraction track typed conversion은 기존 Taffy path를 사용한다. rows `1fr 2fr` + 15px implicit row에서 y=`0,15,45`를 Chrome·layout·runtime이 공유한다. 통과. |
| 7 | line-name metadata가 direct track 경로를 우회하는가 | 비어 있지 않은 각 line-name group은 projection/layout에서 실패한다. 이름 없는 direct track 요구가 유지된다. 통과. |
| 8 | auto-repeat이 implicit-row 제한 mode에 섞이는가 | repeat metadata가 양 단계에서 거부된다. `repeat(2,40px)` negative control은 runtime scene을 내지 않는다. 통과. |
| 9 | named-area template과 fixed implicit row를 혼용할 수 있는가 | projection/layout에서 `template_areas`를 명시적으로 거부하고 runtime negative matrix가 이를 확인한다. 통과. |
| 10 | column fraction의 definite width 검사 누락 | 기존 fraction-axis validator의 width/min/max/aspect-ratio 검사를 그대로 호출한다. `root_html_grid_tracks.rs:17-58`. 통과. |
| 11 | row fraction의 definite height 검사 누락 | 기존 validator가 row fraction에 height/min/max 조건을 적용한다. `root_html_grid_tracks.rs:35-47`; 별도 row scene exact test. 통과. |
| 12 | percentage axis가 반대 축 크기를 basis로 사용 | column→width, row→height 입력이 기존 axis validator에 연결된다. `root_html_grid_tracks.rs:61-102`; Chrome 156 네 scene 기대와 일치. 통과. |
| 13 | percentage basis·gap·fixed track 합산 중 f32 overflow | 각 percentage resolved value와 고정 track/gap 합을 finite f32 한도와 비교한다. `root_html_grid_tracks.rs:127-217`; regression tests 통과. 통과. |
| 14 | `grid-auto-rows`가 list, %, fr, intrinsic으로 넓어지는가 | mode는 단일 `LengthPx`만 허용한다. implicit list/%/fr/minmax negative runtime cases는 failed layout/no scene이다. 통과. |
| 15 | non-default auto-flow나 dense placement가 의도치 않게 켜지는가 | `grid-auto-flow:row`만 허용한다. column/dense 입력은 기존 failure diagnostics로 거부한다. 통과. |
| 16 | auto columns를 사용해 implicit column이 생기는가 | `grid-auto-columns:auto`를 요구한다. non-auto value는 layout contract와 Root HTML negative matrix 양쪽에서 차단한다. 통과. |
| 17 | gap이 percentage sizing 또는 sparse placement에서 중복 적용/누락 | 고정 px gap 5px는 percentage columns 사이, explicit/implicit row 배치에 각각 한 번 반영된다. Chrome expected/JSON 및 runtime frame 일치. 통과. |
| 18 | row placement가 hidden node나 source reorder를 잘못 처리 | hidden item은 occupancy/paint에서 제외되고 source reorder는 frame ownership을 바꾼다. 기존 Root HTML runtime controls를 다시 실행한 log에 포함한다. 통과. |
| 19 | ID/class 변경으로 배치 geometry가 바뀌는가 | runtime test는 class와 ID rename 후 semantic color/frame을 확인해 selector naming에 의존하지 않는다. `root_html_grid_implicit_rows.rs:137-151`. 통과. |
| 20 | visible overflow item이 root edge에서 frame-clamp되는가 | percentage scene의 second item `(35,0,100,10)`은 root right `120px`를 `15px` 넘는다. Rust frame, Android/iOS screenshot과 Chrome observation의 한계가 함께 기록된다. 통과. |
| 21 | capacity preflight의 두 번째 admission guard가 px/%/fr 조합을 거부 | 기존 px-only guard가 통합 런타임에서 발견되어 direct finite non-negative px/%/fr을 같은 제한 capacity에 반영하도록 수정했다. `style_values.rs:1097-1129`; 20 runtime tests 통과. 통과. |
| 22 | explicit capacity·implicit capacity 연산에서 `usize` 곱셈 overflow | 기존 explicit 곱과 새 implicit 곱 모두 `checked_mul`을 쓴다. Root HTML resource-limit test는 expansion 전에 실패한다. 통과. |
| 23 | 10,000-track ceiling이 제거되거나 allocation 이후로 밀리는가 | 전용 capacity guard는 `GRID_MAX_TRACKS`를 사용하고 layout 쪽 limit regression도 green이다. 지원 범위에는 10,000-track 초과 거부가 남는다. 통과. |
| 24 | 테스트가 fixture 문자열을 여러 번 바꿔 의도와 다른 input을 실행 | 신규 test helper `replace_once`는 각 token의 출현이 정확히 1회인지 검사하고 네 scene마다 dimensions/tracks/gaps/item size를 명시한다. 통과. |
| 25 | 신규 tests가 기존 test module과 이름/호출 경로 충돌 | grid, style projection, runtime tests를 각 책임별 child module로 이동했고 `#[path]` 선언을 확인했다. 새 Rust files 모두 500줄 아래다. 통과. |
| 26 | 앱 결과가 fixture-only adapter에서 나와 기본 경로 증거를 오염 | Android/iOS build flags는 runtime GPU만 켜고 fixture flags는 0이다. 로그에 기본 Root HTML boot marker와 ready layout/draw가 있다. 통과. |
| 27 | 시뮬레이터에 올라간 HTML이 candidate와 다른 버전 | candidate, Android APK `assets/index.html`, 설치 iOS bundle `index.html` SHA-256이 모두 `3b65d6be…d5296cc7e`다. 통과. |
| 28 | 빌드 과정에서 repository bootstrap source/generated input이 남아 변경 | 임시로 바꾼 5개 source/generated bootstrap 파일을 capture 전 백업과 byte-for-byte 비교했고 hashes를 manifest에 남겼다. PR diff에는 backup copy를 포함하지 않았다. 통과. |
| 29 | 문서가 후보 결과를 완료 API/제품 지원 또는 green CI로 잘못 선언 | STATUS와 contract/plan은 구현 후보로만 기록하며 checkbox와 `72/213`을 유지한다. PR은 check rollup 0이므로 CI 통과 주장 없음; WPT·surface readback 한계 명시. 통과. |
| 30 | PR 본문의 동작 화면 링크가 깨지거나 diff review 기록이 실제 범위를 가리키지 않음 | Agent Browser에서 PR #1027 body의 Android 1080×2400·iOS 1206×2622 image 두 개가 모두 로드됨을 확인했다. 주소는 브랜치명 대신 구현 commit SHA에 고정했다. 이 파일은 30개 별도 관점을 번호로 기록하고 실행 evidence README에 구현/runtime 20개와 연결한다. 통과. |

**결과:** 30개 관점에서 병합을 막는 diff 결함을 찾지 못했다. 초기 통합 시험에서 발견한 capacity preflight 누락은 이 PR의 수정과 runtime regression으로 닫았다. PR check rollup은 비어 있으므로 CI 통과로 취급하지 않는다.
