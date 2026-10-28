# C11.4 fit-content span-2 · 구현 PR diff 실패 관점 30개

이 검토는 계획의 기대값 목록과 runtime 20개 표를 반복하지 않고, 최종 파일 간 연결, feature/artifact 경계, capture·문서 일관성과 PR 표시를 검토한다. 구현 PR을 열고 이미지 URL을 Agent Browser에서 확인한 뒤 마지막 항목을 확정한다.

| # | diff에서 확인한 실패 경로 | 확인과 결과 |
|---:|---|---|
| 1 | FFI dispatcher가 새 scene 모듈을 빠뜨려 native에서 generic source로 실행한다. | `c11_4_grid_track_sizing_runtime_source`가 새 exact route를 호출하고 FFI test가 pinned ID만 허용한다. 통과. |
| 2 | 새 `fit_content_spanning.rs`가 runtime fixture tree 밖에 있어 기본 제품에도 포함된다. | 모듈은 기존 `runtime_gpu/fixtures/c11_4.rs` 아래에 등록됐고 boundary test 17/17 및 static archive scan이 경계를 확인한다. 통과. |
| 3 | Trace report가 fixture-test 조건 없이 제품 JSON/ABI를 바꾼다. | report module과 diagnostics field는 기존 fixture-test feature 경계 아래에 있다. Default static archive marker scan은 0개다. 통과. |
| 4 | Runtime observation field가 clone, diagnostics initialization 또는 reporting path 중 한 곳에서 빠진다. | `RuntimeGridTrackSizingDiagnostics` 선언·clone·fixture collection·FFI report가 연결되고 feature build가 통과한다. 통과. |
| 5 | Exact scene source가 CSS 스타일과 runtime expected values 사이에서 어긋난다. | FFI source, runtime fixture validator, Rust tests, Android/iOS matchers와 capture scene이 동일한 60×300 root/두 fit-content row/200px child를 사용한다. 통과. |
| 6 | Root의 implicit rows가 explicit `grid-template-rows`로 조용히 바뀐다. | CSSOM property와 typed Grid input의 explicit rows `[]`, auto rows `FitContent(40px)`를 각각 검사한다. 통과. |
| 7 | `overflow:visible` author declaration이 initial value와 같은 값이라 admission 범위를 넓힌다. | Fixture source에는 선언이 없고 Rust source boundary test도 이를 검사한다. 명시적 선언 mutation은 projection 단계에서 거부된다. 통과. |
| 8 | Computed overflow JSON이 Rust Debug 값 `Some(Visible)`로 남아 platform matcher에서 실패한다. | 최종 report는 semantic CSS keyword `visible`을 내보내며 Android/iOS strict matcher와 capture test가 같은 문자열을 사용한다. 통과. |
| 9 | Typed `GridContainerInput`은 맞지만 DOM placement나 style은 달라진다. | Root, item, child computed styles, item span placement, tree identity와 typed input을 함께 exact 검사한다. 통과. |
| 10 | Mutation matrix가 root axis, fit limit, gap, minimum, child 또는 topology를 빠뜨린다. | Rust matrix가 root width/height, row limit/gap, item minimum, explicit overflow, child height, writing mode, 추가 direct item을 거부한다. 통과. |
| 11 | Growth limit이 `40px`을 넘거나 unlimited track이지만 base만 보고 통과한다. | Rust validator와 stage report는 growth limit 및 `infinitelyGrowable`을 보존한다. Dedicated mutations가 limit/flexible 변화를 거부한다. 통과. |
| 12 | Minimum/MinContent contribution이 추가되거나 바뀌어도 MaxContent 한 건만 찾고 통과한다. | Rust, Android, iOS, capture verifier가 세 contribution의 순서·값·source·item·span을 exact 검사한다. 통과. |
| 13 | Validation 코드가 500줄 상한을 넘어 책임이 섞인다. | 준비/관찰, trace/style validator, validator tests를 분리해 각 신규 Rust 파일이 500줄 이하다. 통과. |
| 14 | Module split 뒤 기존 boundary test가 validator의 새 위치를 검사하지 않는다. | `fixture-module-boundary.test.mjs`가 base module과 `validation.rs`를 함께 읽으며 final 17/17이다. 통과. |
| 15 | Report payload가 row trace를 누락하거나 다른 root trace를 선택한다. | Report는 selected root의 Rows pass, stages, track states, contributions, frames와 revisions를 내보낸다. 양 플랫폼 runtime logs와 verifier가 일치한다. 통과. |
| 16 | Android parser가 다른 section이나 malformed report를 성공으로 해석한다. | Matcher는 diagnostics JSON object와 필수 nested values를 fail-closed로 파싱하고 예외 시 false를 반환한다. 통과. |
| 17 | iOS parser가 같은 필드를 다르게 비교하거나 JSON null growth limit을 누락으로 착각한다. | Swift matcher는 `NSNull`인 초기 row limit과 이후 `40px`을 분리하고 exact values를 검사한다. Simulator 1/1 통과. |
| 18 | Platform matcher가 row stages를 count만 확인하고 sequence나 gutter state를 무시한다. | Android, iOS와 capture verifier가 여섯 stage, 다섯 internal entries, gutters와 collapsed boundary를 순서대로 검사한다. 통과. |
| 19 | Root frames와 style snapshot이 다른 generation/document/environment에 속한다. | Report revision, layout input revision, native summary와 presentation identity를 연결한다. 양 플랫폼에서 exact identity가 matched다. 통과. |
| 20 | Draw callback이 같은 scene의 오래된 frame 또는 다른 surface generation에서 온다. | Android/iOS expected presentation identity, generation, sequence를 draw/present에서 비교하고 capture 결과의 `drawPresented`가 true다. 통과. |
| 21 | Pixel sample이 item 내부에 놓여 overflow를 증명하지 못한다. | Child 표본 `(30,80)`은 80px item 끝에서 child가 계속 보이는 위치이고 노랑 RGBA가 Android PixelCopy/iOS Metal에서 exact다. 통과. |
| 22 | Child 이후 배경 또는 root 외부가 잘못 좌표화돼 overflow 범위를 오판한다. | `(30,200)` root background와 `(60,120)` outside-root samples의 CSS→device coordinates/RGBA를 둘 다 검증한다. 통과. |
| 23 | Android screenshot만으로 surface pixels 또는 hardware GPU를 증명한다고 서술한다. | README는 PixelCopy와 emulator AVD 범위만 주장하고 hardware GPU/physical device를 미검증으로 둔다. 통과. |
| 24 | iOS screenshot을 compositor 이후 OS 화면이나 기기 결과로 서술한다. | README와 contract는 Metal drawable 제출 전 표본으로 제한한다. 통과. |
| 25 | Capture runner catalog가 신규 scene을 생략하거나 scene count만 바뀐다. | Exact scene ID, one selected scene, evidence directory, total catalog count 539 및 unique IDs를 boundary test가 확인한다. 통과. |
| 26 | Runtime log verifier가 geometry만 보고 raw style/trace mutation을 놓친다. | Capture verifier는 profile, typed input, revisions, frames, stage bases/growth limits와 contribution vector를 대조한다. 통과. |
| 27 | Product와 fixture archive 검사에서 같은 archive hash를 잘못 비교한다. | Default 빌드 hash/0 marker 결과를 먼저 저장하고 fixture positive-control을 별도 build/hash/scan한다. APK/IPA exclusion으로 확대하지 않는다. 통과. |
| 28 | README 이미지나 증거 링크가 경로 이동 뒤 깨진다. | 최종 README relative links는 새 Android/iOS run directory의 `result.json`, `simulator.json`, runtime logs, screenshots를 가리킨다. 존재 검사를 완료한다. 통과. |
| 29 | 구현 결과가 공식 상태 대장에 CSS 전체 지원이나 C11.4 완료로 올라간다. | `spec/STATUS.md`, 계획 및 내부 계약은 공식 `72/213`과 C11.4 미완료를 유지하고 제한/한계를 분리한다. 통과. |
| 30 | PR 본문 이미지 URL이 잘못돼 review 화면에서 동작 screenshot이 안 보인다. | PR [#768](https://github.com/ohah/spinon/pull/768) 화면을 Agent Browser에서 확인했다. Android와 iOS screenshot image가 `complete=true`로 로드됐고 natural dimensions는 각각 `1080×2400`, `1206×2622`였다. 전용 browser session을 닫은 뒤 프로세스 검색 결과도 비어 있었다. 통과. |

PR #768에는 두 simulator screenshot을 raw GitHub image로 삽입했다. 검토 시점에 labels는 모두 설정돼 있었고 `mergeStateStatus`는 `CLEAN`이었다. `gh pr checks --required`는 `no checks reported`를 반환했고 main branch protection에는 required status checks가 설정돼 있지 않았다. Rebase and merge 뒤 최종 merge SHA와 check 상태를 후속 문서 동기화에 기록한다.
