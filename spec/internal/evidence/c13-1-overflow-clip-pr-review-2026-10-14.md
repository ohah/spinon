# C13.1 overflow clip PR 변경 관점 30개 사전 검토 · 2026-10-14

계획 PR #157의 head를 기반으로 하는 `feat/c13-1-overflow-clip` 변경분을 PR 게시 전에 검토했다. 구현 검토 20개와 분리해 저장소 계약, crate/ABI 경계, Android 검증 harness, 범위 선언을 확인했다. PR 공개 뒤 base/head와 자동 검사 상태를 마지막에 보충한다. 검토는 이 변경을 C13.1 또는 C13 완료로 승격하지 않는다.

| # | 변경 관점 | 확인과 결과 |
| --- | --- | --- |
| 1 | 작업 범위가 상위 C13 전체로 넓어짐 | 변경은 제한 block/flex overflow clip profile과 fixture에 한정한다. scroll state·gesture·scrollbar·snap 구현은 추가하지 않았다. |
| 2 | 계획 PR 변경이 기능 PR diff에 중복됨 | feature branch는 `origin/docs/c13-overflow-scroll-clipping-plan`에서 갈라져 계획 문서 diff를 반복하지 않는다. 게시할 PR base는 계획 PR #157의 head branch다. |
| 3 | 상태 대장이 미완료 기능을 완료로 표시 | `spec/STATUS.md`의 C13 체크는 미완료 상태를 유지하고 남은 iOS/WPT/float/runtime-worker 관문을 쓴다. |
| 4 | 내부 계약 버전이 단순 구현 추가로 상승 | 계약 0060은 `0.1.0-draft`로 유지한다. 출시 정책 변경은 없다. |
| 5 | 사용자용 CSS 지원으로 오해되는 표현 | 내부 계약 제목을 “제품 CSS 지원 선언 아님”으로 명확히 했다. opt-in fixture 경계도 문서화했다. |
| 6 | Rust 전달용 공개 타입과 제품 API를 혼동 | `CssClip`/`CssAxisClip`의 crate 공개 가시성은 데이터 전달 경계다. 문서는 이를 앱 작성자용 CSS 지원으로 취급하지 않는다. |
| 7 | 기존 `LayoutStyle` 생성 경로의 누락 | 신규 overflow fields는 기본값 `Visible`을 설정하고 기존 struct update 호출이 모두 workspace에서 컴파일·테스트됐다. 외부 소비자의 필드 직접 생성은 API 검토 전에 미출시 내부 계약으로 남긴다. |
| 8 | `LayoutOutput` 결과 추가가 기존 출력 소비를 깨뜨림 | used box edge map을 output 생성 지점에서 채우며 workspace의 기존 소비자가 모두 통과했다. |
| 9 | Stylo에서 CSS 문자열을 다시 파싱함 | overflow와 margin은 Stylo computed 값에서 typed enum/px로 snapshot에 복사한다. renderer는 CSS 문자열 파서를 추가하지 않았다. |
| 10 | stylesheet cascade feature가 silently ignored | 고정 author feature allowlist에서 미지원 transform/logical overflow 및 layering/registered-property 입력을 거부한다. positive/negative cascade tests 통과. |
| 11 | 다른 runtime profile에서 clip 의미가 활성화 | 새 profile selector만 overflow projection과 clip lineage를 사용한다. 기본 workspace tests 통과. |
| 12 | Taffy 의존 동작이 버전 상향에 의존 | Cargo.lock의 Taffy 0.14 source에서 Clip non-scroll-container semantics를 확인했다. 의존성 버전 변경은 없다. |
| 13 | hidden/scroll/auto를 실제 scroll 지원으로 기술 | 내부 계약은 이를 현재 padding-box paint clip으로만 취급하고 scroll container state/API가 없다고 명시한다. |
| 14 | `overflow-clip-margin`이 hidden에도 적용 | 계산은 computed 축이 `Clip`일 때만 margin edge를 확장한다. hidden/scroll/auto에는 padding edge를 쓴다. |
| 15 | half-open edge와 empty intersection 혼동 | CSS axis clip은 finite ordered interval을 사용하며 disjoint 교차는 빈 interval로 축약되어 quad와 hit가 제외된다. |
| 16 | parent clip과 target clip 순서 뒤바뀜 | DOM parent map에서 target self를 제외한 lineage를 사용한다. nested fixture와 pixel controls를 확보했다. |
| 17 | absolute containing block escape가 global skip으로 확대 | skip 조건은 해당 absolute descendant의 owner가 현재 clip lineage에 없을 때만 적용한다. 전용 unit test가 있다. |
| 18 | fixed viewport 전제가 transform 지원과 충돌 | transform 등 ancestor containing-block 입력은 허용되지 않는다. 현재 C12 fixed viewport 가정으로 제한하고 문서에 남은 교차 검증을 기록한다. |
| 19 | hit-test와 GPU paint 구현이 하나의 clip을 공유 | render box에 paint/hit clip을 각각 보존한다. WGPU geometry는 paint clip만 사용한다. |
| 20 | runtime hit-test를 DOM event 전달 완료로 표현 | method는 box query만 제공한다. 고정 Chrome 155 `elementFromPoint()` 11점과 runtime query 결과는 별도 delta test에서 11/11 일치한다. S05 event dispatch와 전체 hit-test matrix는 미완료로 분리했다. |
| 21 | clip ancestry 할당 실패가 panic 또는 partial publish | fallible reserve 오류 stage와 같은 reserve 재사용 테스트가 있다. 실제 worker queue/retry는 PR 완료 조건으로 주장하지 않는다. |
| 22 | Android fixture ABI가 기본 FFI에 유입 | default와 opt-in static library의 C13 symbol 및 `c131-root` string positive/negative control을 matching Rust LLVM `llvm-nm`과 `strings`로 확인했다. |
| 23 | Cargo/Gradle/C/C++/JNI/Java feature flag가 서로 다름 | Cargo feature, shell environment, BuildConfig, C define/header, JNI route가 같은 opt-in 이름을 사용하고 API 37 AVD runtime 실행으로 연결했다. |
| 24 | 앱 시작 intent가 기본 화면 경로를 바꿈 | C13 route는 `spinon_c131_overflow_clip` intent와 feature flag가 동시에 활성화될 때 선택한다. 기본 경로는 opt-in false다. |
| 25 | surface 교체 또는 오래된 callback이 PNG를 게시 | PixelCopy callback 전에 generation, renderer, size, closing 상태를 검사한다. output 경로는 고유 run ID를 사용한다. |
| 26 | 캡처 표본의 물리 pixel 변환이 CSS 기준과 다름 | API 37 AVD density `2.625`; log의 physical sample 좌표에서 PNG 값과 Chrome CSS 좌표 sample을 대조해 11/11 exact. |
| 27 | 캡처가 저장 전 bitmap만 검사하거나 hash가 다른 파일을 가리킴 | SurfaceView capture는 PNG 저장 후 재열어 표본을 읽고 SHA-256을 기록한다. 저장된 PNG를 별도 추출해 log sample과 다시 확인했다. |
| 28 | Chrome oracle가 시스템 또는 terminal browser에서 바뀜 | 고정 agent-browser 0.31.1/Chrome 155 revision을 사용했다. terminal-browser와 실기기는 사용하지 않았다. |
| 29 | 성공 검증이 한 패키지 테스트만 포함 | feature-on 변경 8개 package, locked workspace suite, workspace Clippy, rustfmt, diff check, fixture module boundary 5/5, Android build와 AVD evidence를 보존했다. iOS/WPT는 실행하지 않았고 남은 범위에 썼다. |
| 30 | PR 게시·required check·merge 상태를 혼동 | [PR #158](https://github.com/ohah/spinon/pull/158)은 base `docs/c13-overflow-scroll-clipping-plan`, head `feat/c13-1-overflow-clip`인 draft OPEN 상태다. GitHub는 `MERGEABLE`을 반환했지만 review decision과 status checks는 비어 있다. 그래서 merge/auto-merge 없이 draft를 유지한다. |

## PR 공개 후 상태

[PR #158](https://github.com/ohah/spinon/pull/158)은 2026-10-14 관찰 시 OPEN draft이며 base `docs/c13-overflow-scroll-clipping-plan`, head `feat/c13-1-overflow-clip`이다. mergeability 값은 `MERGEABLE`, `reviewDecision`은 비어 있고 `gh pr checks`는 “no checks reported”를 반환했다. required CI가 확인되지 않았으므로 merge와 auto-merge는 설정하지 않았다. GitHub 상태는 변할 수 있으며 이 기록은 C13.1 완료 판정과 별도다.

## Hit-test delta 재검토

후속 delta는 browser query 좌표/target, Rust fixture tree 동기화, visible·clip·margin·nested axis·positioned escape fallback을 대상으로 별도 실패 관점 20개로 확인했다. 결과는 [hit-test delta 구현 검토](c13-1-overflow-clip-hit-test-review-2026-10-14.md)에 기록했다. 변경은 테스트와 근거 문서뿐이며 제품 runtime 구현 동작을 확장하지 않는다.
