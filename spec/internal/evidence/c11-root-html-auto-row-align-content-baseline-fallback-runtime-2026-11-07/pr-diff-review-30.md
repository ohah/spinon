# C11.4 `align-content:baseline` 실제 PR diff 적대 검토 30개

구현/runtime 검토와 별도로 실제 PR 변경분의 제품 경계, 테스트 유지보수, 문서 상태와 시뮬레이터 evidence wiring을 다시 검토했다. 계획 단계 리뷰 표를 재사용하지 않았다.

| # | 변경 경계 | 적대 확인 | 판정 |
|---:|---|---|---|
| 1 | `spec/public/roadmap.html` 진행 카드 | 실제 작업이 보이도록 generic “후속 slice 계획 중”을 feature title로 교체했다. | PASS |
| 2 | Roadmap card identity | 별도 상위 status ID가 없는 작업이므로 `C11.4` badge를 유지한다. | PASS |
| 3 | Roadmap latest note | #1260 이후 이 slice가 진행 중이라고 link하고 계획 경로를 제공한다. | PASS |
| 4 | Official status source | C11.4 전체를 완료로 쓰지 않고 `spec/STATUS.md`를 수정하지 않았다. | PASS |
| 5 | Official completion count | `78/214` 그대로를 계획·계약·로드맵 문맥에서 유지한다. | PASS |
| 6 | Internal contract version | 0059 숫자 버전 `0.1.0`을 유지한다. | PASS |
| 7 | Contract scope | fixed top-level `RootHtmlAutoRowStretchV1`만 지원한다고 한정한다. | PASS |
| 8 | Contract error claims | nested baseline group·일반 CSS·WPT·physical device·hardware GPU를 지원으로 확대하지 않는다. | PASS |
| 9 | Parser boundary | baseline 동작을 위해 일반 parser 또는 CSS grammar를 넓히지 않았다. | PASS |
| 10 | API/ABI boundary | 새 public API·FFI symbol·fixture ABI가 추가되지 않았다. | PASS |
| 11 | Style projection allowlist | computed string `baseline` 하나만 추가하고 `last baseline`과 modifier 조합은 열지 않는다. | PASS |
| 12 | Existing layout profile | `FirstBaseline` gate가 Root auto-row profile에서만 열리는지 확인했다. | PASS |
| 13 | Legacy Grid profile | 이전 intrinsic track sizing profile의 negative test가 살아 있다. | PASS |
| 14 | General baseline validator | Grid 특례가 다른 display의 validator 경로를 skip하지 않는다. | PASS |
| 15 | Parent Flex topology | direct Flex parent rejection을 실제 Layout error로 단언한다. | PASS |
| 16 | Parent Grid topology | direct Grid parent에 실제 grid container를 만들어 validator rejection을 단언한다. | PASS |
| 17 | Parentless/top-level topology | fixed Root HTML route의 no-sharing-context positive fixture를 유지한다. | PASS |
| 18 | Auto-row trace | `StretchAutoTracks { applied:false }`와 `[10,10,10]px` breadth를 따로 확인한다. | PASS |
| 19 | Gutter accounting | positive zero gap과 overflow 46px gutter 둘을 track trace에서 구분한다. | PASS |
| 20 | Existing rejection regressions | 지원되기 시작한 baseline/safe center/unsafe end를 “미지원”으로 검사하던 낡은 assertions를 교체했다. | PASS |
| 21 | New runtime test module | `align_content_baseline.rs`는 기존 RuntimeSession test tree 안에만 등록된다. | PASS |
| 22 | Runtime failure attribution | 바뀐 item shape는 `grid-auto-rows` 오류로 실패하고 alignment 오류와 혼동하지 않는다. | PASS |
| 23 | Atomicity | failed snapshot에서 `completed=None`을 단언한다. | PASS |
| 24 | Session recovery | 동일 session에 valid light mode를 다시 적용해 ready state를 복구한다. | PASS |
| 25 | `last baseline` observation | parser가 fatal `CascadeDiagnostic`을 내는 실제 동작을 projection rejection으로 잘못 서술하지 않는다. | PASS |
| 26 | Chrome precomparison fixture | 구현 전 fixture hash·viewport·independent expected JSON을 runtime fixture와 분리해 보존한다. | PASS |
| 27 | Simulator app source | positive/overflow fixture의 APK·iOS embedded HTML SHA를 원본 fixture와 비교한다. | PASS |
| 28 | Simulator target isolation | API 37 ARM64 AVD와 지정 iPhone 17 Pro만 대상으로 하고 다른 device/process를 정리하지 않는다. | PASS |
| 29 | Restoration safety | 기존 install/build/bootstrap 입력을 복사·hash 대조하고 sentinel이 있는 runner 소유 temp backup만 삭제한다. | PASS |
| 30 | Visual evidence | screenshot analyzer가 실제 첫 item top을 기준점으로 사용하고 캡처를 PixelCopy/Metal readback으로 주장하지 않는다. Roadmap Tailnet 복사본은 병합 후 별도 동기화·Agent Browser 확인 전에는 공개됐다고 말하지 않는다. | PASS |

발견한 보완점은 적용했다: 이전 테스트의 baseline 및 기존 안전 alignment rejection 기대값, screenshot analyzer의 Grid 배경 기준점, 그리고 테스트의 Clippy 단일 원소 loop lint를 고친 뒤 영향을 받은 Rust filter·Clippy·최종 hash 일치 Simulator pass를 다시 확인했다. 최종 simulator source diff hash는 pass03과 pass04 모두 `caf3c039c0cd8b1e37cc4a7f80ecab640415110dbf6228fbd2f22e8a71fd5650`이다.
