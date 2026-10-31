# C11.3.3 area-backed typed `grid-auto-*` PR 변경 검토 30개

**검토 대상:** `7bcc6a6641a2d95e607a6f805578a19a5fa84f0a` 기준의 구현·fixture·문서·실행 증거 변경 전체.

**검토 범위:** cascade와 style projection, Grid track 확장/검증, opt-in runtime fixture, Android/iOS 표본 경로, Chrome oracle 고정, 문서 상태와 산출물 링크.

**결과:** 차단 결함 0. 제한 범위 안의 정합성 PASS 27 · 명시적 후속/범위 PARTIAL 3. 이 검토는 PR/CI/병합이나 전체 CSS Grid 적합성 검증이 아니다.

| # | 독립 실패 관점 | 판정 |
|---:|---|---|
| 1 | area가 없는 profile에서 cascade의 typed auto-track gate가 열린다 | PASS — template profile과 computed `template_areas.is_some()`를 함께 요구하고 no-area typed list 회귀를 유지한다. |
| 2 | template area 외부의 explicit `grid-template-*` 문법까지 새 gate가 확장된다 | PASS — 추가 cascade 허용값은 `grid-auto-rows`·`grid-auto-columns` 변환 호출에만 전달된다. |
| 3 | row와 column이 서로 다른 조건으로 typed 값을 거부하거나 허용한다 | PASS — 두 변환에 같은 area/profile gate를 전달하고 축별 projection 및 runtime scene을 둔다. |
| 4 | 빈 auto-track 목록이 허용되어 pattern indexing이 실패한다 | PASS — projection과 layout expansion 모두 nonempty 목록을 요구한다. |
| 5 | unsupported typed value가 일부 track만 보존한 채 부분 성공한다 | PASS — 기존 typed converter 전체를 사용하고 실패하면 computed value를 `Unsupported`로 유지한다. |
| 6 | computed CSS math의 revision·basis가 projection 중 분리된다 | PASS — 기존 `CssMathProjector`와 checked `project_tracks` 경로를 재사용하고 locked fixture hash를 고정한다. |
| 7 | 기존 non-area fixed-px list 지원이 회귀한다 | PASS — 기존 fixed-px 허용 분기를 보존하고 focused cascade/layout 회귀가 통과했다. |
| 8 | area projection에서 authored explicit track이 지워지거나 교체된다 | PASS — tests에서 authored prefix를 보존하고 부족한 track만 area dimension까지 추가한다. |
| 9 | row area 차원이 column 개수나 track list를 조작한다 | PASS — row scene은 definite height와 `minmax(20px, 1fr)`를, column scene은 definite width와 `25%`를 독립 비교한다. |
| 10 | area가 explicit track보다 작을 때 track 수가 줄어든다 | PASS — 기존 authored-longer regression과 39-scene immutable input을 유지한다. |
| 11 | area가 explicit track보다 클 때 생성 수가 잘못 계산된다 | PASS — layout 및 style-to-layout의 양축 tests와 Chrome oracle node frame이 필요한 implicit 수를 고정한다. |
| 12 | fixed/typed list가 area 생성 시 첫 값부터 순서대로 순환하지 않는다 | PASS — projection tests는 typed sequence와 used row/column frame을 확인한다. |
| 13 | area가 소비한 list phase 뒤 positive named-line track이 이전 값부터 재시작한다 | PASS — row·column `[10px, 25%, 8px]` test에서 trailing track frame `(70, 60, 18, 18)`을 확인한다. |
| 14 | named-line typed phase test를 Android/iOS app matrix 결과로 과장한다 | PASS — 문서가 이 조합은 Rust layout test만 통과했고 앱 표면 matrix에는 없다고 분리한다. |
| 15 | signed leading/negative implicit mode에 새 typed sizing이 들어간다 | PASS — expansion mode는 area-backed positive mode만 typed list를 받고, signed mode test는 거부를 확인한다. |
| 16 | area 없는 `TemplateGridImplicitTracks`에 typed list가 열려 numeric placement가 새로 지원된다 | PARTIAL — negative control은 거부를 확인한다. 독립 numeric implicit placement 전체 조합은 이 후보 밖이다. |
| 17 | percentage track이 indefinite/wrong-axis basis를 참조한다 | PASS — definite 100px root의 25% column 결과 25px를 Chrome·layout·surface 표본으로 확인한다. |
| 18 | `minmax(20px, 1fr)`가 남은 공간 대신 전체 grid 크기를 분배한다 | PASS — 90px row scene에서 10px authored row 뒤의 두 implicit row가 각각 40px인지 oracle와 runtime에서 확인한다. |
| 19 | intrinsic keyword·`fit-content()`·`calc()`까지 새 경로의 모든 계산이 앱에서 검증된 것으로 읽힌다 | PARTIAL — 해당 typed grammar는 기존 C11.4 converter를 통과하지만 새 앱 matrix는 `minmax`와 percentage 두 scene만 다룬다. `fit-content()` cap 증거는 별도 min/max 입력을 추가하기 전까지 주장하지 않는다. |
| 20 | C11.4 track-sizing profile 자체의 계산 방식이 변경된다 | PASS — `TrackSizing` mode의 auto-track parser/expansion은 기존 분기를 유지하고 workspace regression/Clippy가 통과했다. |
| 21 | zero area dimension/empty explicit axis에서 잘못된 subtraction이나 빈 template가 통과한다 | PASS — area-backed template mode는 empty authored row/column을 구체 `InvalidGridTrackList`로 거부한다. |
| 22 | track count 증가가 기존 track quota나 checked count를 우회한다 | PASS — `validate_count`, 기존 reservation helper와 profile limits를 재사용한다. |
| 23 | reservation failure 또는 track validation error 뒤 일부 frame이 성공 결과로 남는다 | PASS — 기존 fallible reservation 및 atomic layout error 경로를 유지하고 focused suite가 통과했다. |
| 24 | expanded track 수와 N+1 line-name metadata 수가 불일치한다 | PASS — projection assertions가 rows/columns 각각 `line_names.len() == tracks.len() + 1`을 확인한다. |
| 25 | 신규 fixture FFI가 기본 제품 static archive에 유출된다 | PARTIAL — 검사한 macOS ARM64 Release `spinon-ffi` archive에서 제외를 확인했다. 최종 Android APK/iOS IPA 전체 산출물 검사는 포함하지 않는다. |
| 26 | Rust, Android, iOS의 scene ID 목록이나 matrix summary가 다르다 | PASS — 세 플랫폼 ID 목록, JSON builder, 39 scene·126 node 및 34/5/0 summary 검사와 Node boundary tests를 추가했다. |
| 27 | Chrome oracle 입력/화면의 hash, DPR, viewport 또는 browser version이 바뀌어도 통과한다 | PASS — matrix builder와 Node test가 source/screenshot hash, Chrome 156, DPR 1, viewport와 scene/node order를 고정한다. |
| 28 | Pixel sample이 색상 영역 밖을 읽거나 다른 플랫폼의 sample 색이 서로 달라진다 | PASS — Chrome screenshot에서 sample 위치·RGBA를 검증하고 Android/iOS에서 sample identity와 고정 색을 검사한다. 새 typed scene은 각 7/7 exact다. |
| 29 | 실행 증거 문서가 존재하지 않는 위치로 연결된다 | PASS — 검토 중 runtime README의 최종 Android/iOS run 경로 오류를 찾아 실제 sibling evidence 폴더로 수정했다. 최종 파일 링크를 별도 확인한다. |
| 30 | 제한 slice가 C11/C11.3.3 완료, WPT 또는 공개 CSS 지원으로 표시된다 | PASS — 공식 checkbox는 미완료, 공식 집계 `72/213`을 유지하고 AVD/Metal/WPT/author-admission 한계를 기록한다. |

## 잔여 범위

Android는 API 37 ARM64 AVD/SwiftShader, iOS는 iPhone 17 Pro/iOS 26.2 Simulator의 compositor 제출 전 Metal drawable이다. 실기기·hardware GPU·전체 WPT·fit-content cap·typed named-line 조합의 앱 표면·최종 APK/IPA 전체 fixture 제외는 이 후보의 통과 판정에 포함하지 않는다. 이 PARTIAL들은 계획된 범위 한계이며 현재 제한 후보의 차단 결함은 아니다.
