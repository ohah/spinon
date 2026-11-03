# C11.1 parsed Root HTML `fit-content(px)` 구현/runtime 실패 검토 20개

검토일: 2026-11-03
범위: 현재 후보의 cascade, style-to-layout admission, Grid/Taffy sizing, Root HTML runtime, Android/iOS capture 변경. 계획 검토와 PR 실제 변경 검토를 대신하지 않는다.

첫 구현 검토에서 Root HTML cascade fixed-repeat 전용 테스트와 fit-content named-area 실패 경계 테스트가 빠진 것을 발견했다. 테스트를 추가하고 관련 suite를 통과시켜 닫았다. 아래 20개 관점에서 남은 구현 결함은 발견하지 못했다.

| # | 실패 경로 / 경계 | 결과와 근거 |
|---:|---|---|
| 1 | Root HTML 이외 cascade profile로 admission이 새지 않는가 | `allow_root_html_fit_content_tracks`는 `RuntimeRootHtmlTextPaintV1`에만 전달된다. 다른 profile의 기존 Grid suite를 유지했다. |
| 2 | px limit가 NaN, infinity, 음수일 때 통과하는가 | cascade conversion과 projection에서 finite non-negative 조건을 적용한다. projection negative matrix가 음수·NaN·±infinity를 거부한다. |
| 3 | 유효한 `0px`가 양수 조건으로 잘못 거부되는가 | style cascade의 양 축 경계 테스트, projection `0px`, Ahem `fit-content(0px)` 결과가 통과한다. |
| 4 | percentage 또는 `calc()` limit가 px로 잘못 투영되는가 | style cascade에서 `%`와 `calc(10px + 5%)`를 거부하고, projection에서도 percentage·calc typed value를 거부한다. |
| 5 | columns와 rows 중 한 축만 처리되는가 | cascade와 projection에서 두 축을 각각 확인하고, runtime frame에서 column 및 row overflow를 확인한다. |
| 6 | fixed integer `repeat()` 확장이 direct track와 다른 허용 규칙을 쓰는가 | cascade test가 `repeat(2, fit-content(40px))`의 두 typed track과 repeat provenance를 확인한다. |
| 7 | `grid-auto-rows` / `grid-auto-columns`에 explicit 허용이 누출되는가 | fit-content opt-in 인자를 auto-track 변환에 전달하지 않는다. `grid-auto-rows`는 cascade에서 `Unsupported`로 남고 layout validator도 auto lists에 새 허용을 주지 않는다. |
| 8 | auto-repeat 또는 named-line 경로가 새 admission을 우회하는가 | Root HTML explicit validator는 여전히 line name과 auto-repeat을 거부한다. 새 mode 선택은 explicit tracks만 검사한다. |
| 9 | Taffy validator가 새 mode에서 percentage fit-content를 받는가 | `RootHtmlFitContentV1`은 finite non-negative `LengthPx`만 허용하며 auto track 검사는 별도 false opt-in을 사용한다. percentage 거부 unit test가 통과한다. |
| 10 | named-area map 크기와 track 수가 어긋나는데 통과하는가 | `areas.rows/columns`와 explicit rows/columns 길이를 각각 대조한다. projection mismatch cases와 runtime 행 수 mismatch가 거부된다. |
| 11 | named-area 트랙에 `fr`, `%`, `minmax()`, `auto`가 섞이거나 fit-content가 하나도 없는가 | 새 area profile은 finite px와 px fit-content만 받고 적어도 하나의 fit-content를 요구한다. projection acceptance/rejection matrix가 확인한다. |
| 12 | 잘못된 area metadata 또는 중복 area 이름이 사용되는가 | parsed area map은 dimensions 및 비어 있지 않은 목록을 확인하고 semantic gate는 빈/NUL/중복 이름을 거부한다. CSS parser가 rectangle/range 유효성을 보장하는 입력 경계를 코드에서 확인했다. |
| 13 | 성공 조건이 `fixed`, `fit` 같은 특정 area 이름에 묶이는가 | projection unit test와 runtime test가 `leading`/`trailing` 임의 이름을 통과시킨다. |
| 14 | DOM id, selector id, child 순서에 성공이 의존하는가 | runtime test가 Grid와 item id를 바꾸고 direct child 순서를 뒤집어도 semantic frame이 유지됨을 확인한다. adapter는 ID로 area를 선택하지 않는다. |
| 15 | 한 area에 중복 배정되거나 빠진 area가 있어도 frame이 발행되는가 | runtime duplicate-placement 및 missing-item controls가 실패하고 `completed` scene이 없다. |
| 16 | 숨은 item, 잘못된 태그, nested child, 익명 non-whitespace text가 허용되는가 | runtime negative matrix가 hidden, non-div, non-empty item을 거부한다. 익명 text는 기존 fail-closed Grid text 경계에서 거부된다. |
| 17 | named-area container가 definite positive px가 아니거나 min/max override가 있는가 | auto/zero width·height 및 min/max 네 속성 override를 runtime에서 거부한다. |
| 18 | 새 named-area 경로의 gap 값이 기존 gap contract과 다른가 | definite axis의 percentage gap은 통과하고 indefinite axis와 기존 Root HTML gap regression suite는 거부/회귀 없이 유지된다. |
| 19 | allocation 실패나 부정 scene이 부분 결과를 남기는가 | 새 area-name/item/usage vector는 `try_reserve_exact` 후 contextual allocation error를 반환한다. runtime rejection matrix는 완료 scene이 없음을 확인한다. 새 vector별 allocator failpoint는 만들지 않았고 이 부분은 정적 코드 검토다. |
| 20 | 앱 표본이 엉뚱한 route/좌표를 확인하거나 stale generation을 읽는가 | Android/iOS capture는 전용 opt-in build flag, run id, surface generation/revision 검사를 사용한다. 최신 API 37 AVD PixelCopy와 iPhone 17 Pro iOS 26.2 Simulator Metal 표본이 각각 7/7 exact이며 화면 PNG도 확인했다. |

최종 focused Rust 결과는 같은 디렉터리의 `layout-ahem-final.log`, `layout-fit-content-rejection-final.log`, `style-cascade-fit-content-final.log`, `style-grid-final.log`, `style-to-layout-root-html-tests.log`, `runtime-root-html-tests.log`, `runtime-fit-content-final.log`, `scoped-clippy.log`에 보관한다. Simulator 환경과 캡처 한계는 `README.md`에 기록한다.
