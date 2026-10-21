# C11.4 nested Grid depth-two implementation PR diff 검토 30개

구현/runtime 실패 검토 20개와 별도로, 변경 파일 간 계약 전달·제품 경계·증거와 문서 일관성을 확인했다. 구현 세부를 다시 나열하지 않고 integration 및 merge 실패 경로를 따로 다룬다.

| # | Diff 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | FFI fixture Rust 파일이 기존 `runtime_gpu/fixtures/c11_4` 경계 바깥에서 중복 활성화되는가 | 신규 source 파일 하나만 기존 C11.4 fixture module graph에 등록했다. |
| 2 | FFI dispatch와 runtime layout adapter가 서로 다른 C11.4 feature를 쓸 수 있는가 | runtime module gate와 test gate가 같은 `c11-4-grid-track-sizing-fixture-test` feature를 사용한다. |
| 3 | direct runtime test가 product target으로 잘못 끌려오는가 | Rust test module 선언은 `#[cfg(feature = "c11-4-grid-track-sizing-fixture-test")]` 아래 있다. |
| 4 | `validate_intrinsic_image_inputs` rename이 기존 root Grid max-content 호출부를 바꾸는가 | 기존 root-only max-content 경로를 유지하고 새 Grid width branch만 별도로 더했다. |
| 5 | projection regression test가 profile 경계를 긍정 예제로만 검사하는가 | 새 min-content Grid width 성공 뒤 `compute_template()` other-profile rejection도 확인한다. |
| 6 | Android/iOS 앱 build flag가 fixture-test feature를 product build와 같은 조합으로 켜는가 | 최종 build 로그에서 `c04-runtime-gpu=1`, `c04-runtime-gpu-fixture-test=1` 및 비관련 fixture flags 0을 확인했다. |
| 7 | fixture 경로 이전/새 경로 추가 과정에서 orphan 또는 duplicate module이 남는가 | 새 파일 한 개를 기존 fixtures module tree에 추가했으며 같은 source를 다른 경로에 복제하지 않았다. |
| 8 | FFI fixed HTML source에 CSS parser 미지원 선언이 다시 들어가는가 | source unit test가 `writing-mode:` 부재를 검사하고 최종 app capture도 통과했다. |
| 9 | static archive exclusion이 fixtures 디렉터리 위치만 보고 추정되는가 | `c04-runtime-gpu` 제품 archive를 실제 빌드하고 strings에서 네 ID를 직접 찾지 못함을 기록했다. |
| 10 | positive control이 새 feature module을 실제로 컴파일했음을 보이는가 | 동일 staticlib를 `c04-runtime-gpu-fixture-test`로 별도 빌드해 네 scene ID를 모두 추출하고 SHA를 기록했다. |
| 11 | 이전 C11.4 fixture test 코드의 feature lint 때문에 새 fixture 검증이 통째로 막히는가 | opt-in Clippy에서 찾은 implicit-row validator의 argument lint를 expectation struct로 refactor하고 regression test와 Clippy를 다시 통과했다. |
| 12 | Android Java와 iOS Swift가 각각 다른 expected scene set을 받는가 | 두 native allowlist에 똑같은 four IDs를 추가하고 양쪽 simulator에서 같은 순서로 실행했다. |
| 13 | AVD/Simulator의 root frame expectation이 FFI scene matrix와 다른가 | 네 scene의 `70×20` 또는 `100×10` root frame expectation이 Java, Swift, JS verifier에서 일치한다. |
| 14 | 깊이에 따라 intrinsic input node가 바뀌는데 native verifier가 old depth-one identity를 재사용하는가 | depth 1/2별 Grid·leaf node IDs를 Java/Swift expected-input verifier와 runner JSON 비교에 반영했다. |
| 15 | JS expected frames의 scene ordering이 native launcher ordering과 어긋나는가 | node frame count와 각 order/node/x/y/width/height를 runner에서 전체 비교한다. |
| 16 | C11.4 app reporter가 신규 scene에서 존재하지 않는 standalone item frame을 강제하는가 | 신규 scene의 expected item frame을 빈 값으로 처리하고 root plus complete node frames를 검사한다. |
| 17 | runner selection이 scene 일부만 골라 실행하면서 성공으로 끝나는가 | requested ID 수와 unique set, 실제 선택된 scene 수를 exact compare한다. |
| 18 | 다른 fixture가 추가됐는데 runner total count와 uniqueness gate가 그대로인가 | runner count를 188로 갱신하고 ID uniqueness assertion을 유지한다. |
| 19 | fixture ID discovery evidence가 잘못된 CLI invocation에서 빈 파일로 남는가 | 최종 `--list-scenes` invocation에 Android target을 포함해 JSON의 `sceneCount=188`과 네 신규 ID를 확인했다. |
| 20 | `runKind` evidence route가 네 ID 선택 이외에도 새 검증기로 오인되는가 | 자동 inference는 선택된 scene ID 전부가 depth-two allowlist일 때만 새 kind를 쓴다. |
| 21 | Android AVD evidence가 emulator 대신 우연히 연결된 실기기에서 나올 수 있는가 | capture runner는 serial `emulator-5558`, `qemu=1`, API 37, ARM64 AVD를 기록하고 실기기 모드를 쓰지 않았다. |
| 22 | iOS runtime evidence가 지정 외 simulator/runtime에서 나올 수 있는가 | runner와 metadata는 iPhone 17 Pro UDID, iOS 26.2 booted simulator를 함께 검사한다. |
| 23 | 설치되지 않은 APK hash를 device run 결과와 연결하는가 | 최신 Android metadata에서 local APK와 installed base APK SHA-256이 일치하고 `matched=true`다. |
| 24 | iOS screenshot/evidence가 다른 app binary에서 나온 것으로 섞이는가 | 최신 iOS metadata가 app SHA-256 및 simulator UDID를 함께 기록한다. |
| 25 | evidence README의 수치가 각 raw result JSON과 다른가 | 네 scene별 root frames, trace chunk count, draw/present와 device metadata를 README가 최종 run directory에 연결한다. |
| 26 | 반복 capture 중 실패 결과가 successful final run인 것처럼 선택되는가 | 두 초기 Android 실패 원인을 README에서 설명하고 최종 성공 directory와 분리해 보존한다. |
| 27 | screenshot hash가 representative screen이 아닌 오래된 반복 실행 화면을 가리키는가 | `SHA256SUMS.txt`는 README와 PR에서 사용하는 최종 Android/iOS representative screenshot만 가리킨다. |
| 28 | plan, internal contract, status가 opt-in profile admission을 서로 다르게 설명하는가 | 세 문서는 Grid min/max-content width, intrinsic height 배제, 전체 C11.4 미완료 범위를 같은 수준으로 적는다. |
| 29 | PR 양식의 status ID, API/internal spec, 남은 범위와 labels 중 하나가 누락되는가 | PR 설명에 C11.4 slice, internal contract 0059, implementation evidence와 남은 한계를 채우고 기존 `enhancement`/`area:*` labels를 PR metadata로 설정했다. |
| 30 | PR 화면 이미지나 Tailnet 공개 미리보기를 링크만 성공한 것으로 표시하는가 | PR #450 병합 뒤 이미지 URL을 merge SHA `80837bbe03542eb55ebc983964c72fd925454d28`에 고정하고 Agent Browser에서 두 `<img>`가 complete이며 1080×2400, 1206×2622 원본 크기를 반환함을 확인했다. Tailnet preview는 병합 문서 PR 뒤 안전 동기화하며 GitHub Pages는 배포하지 않는다. |

PR [#450](https://github.com/ohah/spinon/pull/450)은 2026-10-21 14:38:40 UTC Rebase and merge됐으며 SHA는 `80837bbe03542eb55ebc983964c72fd925454d28`다. PR check rollup은 비었고 main required status checks도 설정되어 있지 않아 이를 CI 통과로 분류하지 않는다. PR 본문은 GitHub raw 이미지 URL을 merge SHA에 고정했으며 Agent Browser에서 두 이미지의 로드와 intrinsic dimensions를 확인했다.
