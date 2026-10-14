# C13.1 float formatting boundary · PR 변경 관점 30개

대상 diff는 runtime test wiring/new test와 C13 계획·내부 계약·상태 문구·증거다. 앞선 20개 implementation review와 별도로, 변경이 기존 경계나 PR 상태를 잘못 바꾸지 않는지 확인했다.

| # | 변경 관점 | 확인 결과 |
|---:|---|---|
| 1 | 새 테스트가 제품 빌드에 포함됨 | PASS — 부모 `ua_cascade::tests` 모듈은 `#[cfg(test)]`다. |
| 2 | `runtime_layout_tests.rs`에 중복 모듈 이름이 생김 | PASS — C13 float-boundary와 feature gated worker-recovery 모듈 이름·경로가 다르다. |
| 3 | fixture 전용 코드를 제품 runtime module에 넣음 | PASS — 새 파일은 테스트 트리의 `runtime_layout_tests/`에 있다. |
| 4 | `mod.rs` 또는 경로 우회로 구조 규칙을 깨뜨림 | PASS — 같은 디렉터리의 `#[path]` 모듈이며 `mod.rs`를 추가하지 않았다. |
| 5 | Cargo feature/API를 바꿔 fixture 노출이 넓어짐 | PASS — Cargo manifest와 public API 변경이 없다. |
| 6 | FFI/JNI/Android route가 테스트 때문에 바뀜 | PASS — 변경 없음. |
| 7 | C13 existing float allowlist가 늘어남 | PASS — production allowlist는 수정하지 않았다. |
| 8 | C09/C26 float 소유권을 C13으로 이동 | PASS — 계획은 실제 Spinon float geometry parity를 C26 이후로 명시한다. |
| 9 | 부모 세 가지를 한 CSS 입력으로 뭉개서 차이를 잃음 | PASS — hidden, plain clip, clip+flow-root를 독립 입력으로 계산한다. |
| 10 | 테스트 이름이 결과를 과장함 | PASS — 이름은 `fail_closed`를 주장하며 geometry parity를 주장하지 않는다. |
| 11 | 진단 문구가 어떤 경우인지 식별하지 못함 | PASS — case label을 panic/assert message에 넣는다. |
| 12 | 부모 width가 달라져 shrink-to-fit 경계가 섞임 | PASS — 각 runtime 부모 width를 120px로 고정한다. |
| 13 | 자식 float 크기가 case 사이에서 다름 | PASS — 모든 입력에서 `48×36px`다. |
| 14 | 다른 자식 수 때문에 NodeId assertion이 잘못됨 | PASS — 각 document에 한 child만 만들고 그 handle에서 ID를 가져온다. |
| 15 | 잘못된 revision key로 오래된 계산을 재사용함 | PASS — 매 snapshot의 `EnvironmentRevision::INITIAL`로 key를 만든다. |
| 16 | viewport와 environment revision이 어긋남 | PASS — request viewport는 `CssViewport::C04_FIXTURE`를 쓴다. |
| 17 | 이전 snapshot/cache가 fail-closed 확인을 우회함 | PASS — `previous_snapshot`, `previous_styles`가 없고 `force_full=true`다. |
| 18 | cascade root 누락이 float 실패로 숨겨짐 | PASS — 계산 결과에 root snapshot이 정확히 하나 있어야 한다. |
| 19 | root computed style을 다른 요소에서 읽음 | PASS — root `NodeId`로 실제 항목을 찾는다. |
| 20 | `flow-root`가 computed `block`으로 치환되어도 테스트가 통과 | PASS — computed `display`를 case별 기대값과 비교한다. |
| 21 | overflow shorthand가 축별 값에서 사라짐 | PASS — x/y 두 computed longhand를 모두 확인한다. |
| 22 | 일반 오류 문구만 확인해 다른 CSS 속성이 거부됨 | PASS — layout failure의 property가 정확히 `float`다. |
| 23 | 실패 대상 node가 float child가 아님 | PASS — child handle에서 얻은 NodeId와 error를 비교한다. |
| 24 | 예상과 달리 성공 layout이 나오면 테스트가 묵인함 | PASS — 성공 결과는 `unwrap_err()`에서 즉시 실패한다. |
| 25 | stylesheet float가 inline preflight와 다르게 허용됨 | PASS — author stylesheet negative control이 별도 계산 경로에서 실패한다. |
| 26 | 비교 증거가 source 변경 후에도 재사용될 수 있음 | PASS — JSON에 live fixture SHA-256을 기록했고 고정 fixture digest와 일치한다. |
| 27 | Chrome 버전을 UA major만으로 추정함 | PASS — full version/revision은 `chrome://version`, `userAgentData`로 확인했다. |
| 28 | 상태 대장이 C13.1 또는 C13 완료를 암시함 | PASS — C13 체크는 미완료로 유지하고 남은 iOS/WPT/negative-control/scroll 경계를 적었다. |
| 29 | 코드 변경 없는 앱 simulator 실행을 runtime 증거로 주장함 | PASS — 이 test-only diff에 앱 build를 요구하지 않았고 simulator 결과를 새로 주장하지 않는다. |
| 30 | git tree에 의도하지 않은 파일이나 whitespace error가 남음 | PASS — 마지막 status는 C13 runtime test, plan/status/internal contract와 해당 evidence/review만 포함하며 `git diff --check`와 rustfmt가 통과했다. |

Workspace test는 24 suite에서 729 passed, 0 failed, 4 ignored였고, workspace Clippy 및 focused C13 test도 통과했다. 이 PR 변경 검토는 merge approval을 뜻하지 않는다.
