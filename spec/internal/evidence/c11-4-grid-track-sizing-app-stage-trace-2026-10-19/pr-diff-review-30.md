# PR #345 변경 적대 검토 30개

**검토 날짜:** 2026-10-19
**대상:** `feat/c11-4-app-contribution-traces-20261019`의 PR #345 및 frame assertion 후 trace 보존 수정. 기존 구현 검토 20개를 재사용하지 않고 PR base·feature/package 경계·Android/iOS app flow·artifact·evidence·PR 표현을 각각 따로 확인했다.

| # | 실패 관점 | 확인 및 판정 |
|---:|---|---|
| 1 | PR base가 변경된 main을 기준으로 잘못 계산됨 | PR base SHA `9eb75d25a8c11360318f9cef1f3da0f265a180eb`와 현재 `origin/main`을 대조했다. 이 PR은 C11.4 trace 후속만 담는다. |
| 2 | dirty main 체크아웃의 C11.3 변경이 PR에 섞임 | 별도 C11.4 worktree에서만 stage·commit했다. 원래 checkout은 `feat/c11-3-template-repeat-named-lines-areas`와 340 dirty paths 그대로였다. |
| 3 | 이 후속 변경이 Grid sizing algorithm 자체를 바꿈 | diff는 runtime diagnostics 전달, app log 수집, 계획·evidence에 한정된다. Taffy/track sizing 계산 분기를 변경하지 않는다. |
| 4 | release FFI ABI에 새 C function이나 header가 필요 이상으로 추가됨 | 새 C ABI symbol을 만들지 않고 기존 C11.4 fixture route의 응답 문자열만 확장했다. 기본 header/product 경계는 그대로다. |
| 5 | Cargo default feature에서 diagnostics payload가 항상 활성화됨 | feature 그래프와 두 rlib를 확인했다. 기본 artifact marker 0개, fixture-test positive control marker 2개다. |
| 6 | FFI fixture feature가 runtime field를 활성화하지 않아 조건부 필드가 깨짐 | `spinon-ffi/c04-runtime-gpu-fixture-test`가 `spinon-runtime/c11-4-grid-track-sizing-fixture-test`를 전달한다. default와 fixture 조합 모두 compile됐다. |
| 7 | layout dependency가 제품 빌드까지 끌려옴 | `dep:spinon-layout`는 fixture-test feature edge에 추가됐다. 기본 FFI artifact에서 diagnostics payload 문자열은 빠진다. |
| 8 | 다른 C04 fixture host가 C11.4 보고서를 우연히 요구함 | 새 flag의 false 초기값과 C11.4 host constructor의 true 설정만 있다. 다른 constructor의 응답 계약은 건드리지 않았다. |
| 9 | Rust conditional field를 가진 `RuntimeLayoutCompleted` initializer가 빠져 한 feature 조합이 깨짐 | runtime와 FFI의 모든 struct initializer 및 `cfg` 조건을 검색했다. 기본 Clippy/check과 fixture Clippy가 통과했고, reporter focused tests는 이전 실행에서 2/2 통과했다. |
| 10 | fixture host에서 stage trace를 못 만들었는데 frame만 성공으로 보고됨 | completed response 생성 시 non-empty trace를 요구하고 앱 성공 summary 전에 marker 누락을 실패 처리한다. 두 앱 모두 새 버전 7개 scene에서 trace chunk를 받았다. |
| 11 | HostDocument 대신 단순 Tree revision을 앱 trace로 내보냄 | reporter는 Tree source를 거부하고 HostDocument generation/document/render-tree revision을 completed key와 비교한다. |
| 12 | style 또는 viewport environment가 다르지만 같은 document key라 통과함 | reporter는 style과 environment revision도 별도로 대조한다. 앱 JSON의 canonical revision은 두 simulator에서 같다. |
| 13 | intrinsic input revision과 공통 trace revision이 엇갈림 | measurement/item 입력 모두 같은 revision인지 확인한다. cache rekey의 동작도 함께 검토했다. |
| 14 | cache hit에서 trace가 원래 layout 결과와 분리되거나 사라짐 | `rekey_for_request`가 diagnostics snapshot을 보존하고 새 request revision을 적용한다. trace 자체에는 별도 node ID remap을 하지 않는다. |
| 15 | stage 배열을 축약해 algorithm 경계를 잃음 | JSON에 pass sequence/axis와 stage 순서를 보존한다. re-run summary는 scene별 stage 수를 별도로 계산한다. |
| 16 | renderer가 이해할 수 없는 native debug object가 JSON에 들어감 | reporter가 `serde_json::Value`로 숫자·enum name·node ID를 명시 변환하고 최종 JSON serialization 오류를 `Result`로 반환한다. |
| 17 | JSON trace가 큰데 Android Logcat 한 줄에 그대로 써 잘림 | 1,600 UTF-16 code-unit 단위로 나누고 run ID 및 `part/total`을 붙인다. 후속 AVD 로그 7/7에서 chunk sequence가 완전하다. |
| 18 | iOS log chunk 수가 현재 JSON 경계에서 과소 산정됨 | 현재 reporter 값은 enum label과 숫자 기반 ASCII JSON이라 byte count와 Character slicing이 일치했다. Unicode 문자열을 trace payload에 추가하면 해당 전제를 재검토해야 한다. |
| 19 | Android scene/run argument가 로그 식별자를 깨뜨리거나 임의 script를 실행함 | MainActivity의 7 scene allowlist와 run ID 문자/길이 검사를 확인했다. fixture source는 Rust 고정 상수에서 선택한다. |
| 20 | iOS runtime app route가 C11.4 arguments를 drop함 | AppDelegate argument allowlist와 C0410 demo parser를 확인했다. iPhone 17 Pro Simulator에서 7개 고유 run ID가 기록됐다. |
| 21 | frame assertion 실패가 diagnostic trace를 먼저 버림 | **검토 발견:** 기존 Android/iOS 코드가 frame assertion 뒤에서만 trace logger를 호출해 mismatch early-return이면 trace가 사라졌다. 두 플랫폼 모두 trace chunk logging을 frame assertion보다 먼저 실행하도록 고쳤다. |
| 22 | trace가 없어도 frame이 맞으면 앱 성공으로 빠짐 | logger 결과를 별도 보존해 frame assertion 뒤에도 trace 존재 여부를 확인한다. 누락된 경우 성공 summary를 출력하지 않는다. |
| 23 | trace는 있지만 frame mismatch인 경우 기존 frame 오류 설명이 사라짐 | trace를 먼저 기록한 뒤 기존 frame assertion/error 메시지를 그대로 실행한다. 수정된 앱 제어 흐름을 확인했다. mismatch injection은 별도 실행하지 않았다고 evidence에 명시했다. |
| 24 | 수정으로 Android 앱 설치/실행 경로가 망가짐 | Android API 37 ARM64 AVD에 수정 debug APK를 설치해 7개 scene을 재실행했고 frame assertion 및 trace chunk 검증이 7/7 통과했다. |
| 25 | 수정으로 iOS compile 또는 Swift logger 호출이 깨짐 | iPhone 17 Pro/iOS 26.2 Simulator target build가 `BUILD SUCCEEDED`였다. |
| 26 | frame 검증 경로 이동이 정상 scene에서 관찰 trace를 바꿈 | Android와 iOS 수정 버전에서 각 7개 scene의 JSON을 복원했고 canonical SHA-256이 scene별 7/7 동일했다. |
| 27 | 고정 크기 scene 결과를 text/image contribution proof로 소개함 | re-run의 `intrinsicMeasurements`·`gridItemIntrinsicInputs`는 모두 비어 있다. 해당 앱 경로는 미완료라고 남겼다. |
| 28 | overflow screenshot이 border stroke 픽셀 검증인 것처럼 PR에 보임 | PR 화면에는 auto-fill overflow simulator capture를 올렸고 README와 PR 본문에서 border 색과 surface pixel matrix는 검사하지 않았다고 제한했다. |
| 29 | evidence hash, build log 또는 Tailnet 미리보기가 PR commit과 불일치 | SHA256SUMS를 evidence directory 상대경로로 검증하고, iOS Xcode 원본은 gzip으로 보존했다. Tailnet 변경은 copy-only manifest와 pre-replacement backup을 남겼다. |
| 30 | PR 라벨·이미지·상태 문구가 구현 범위를 과장함 | PR #345는 유형·영역 라벨을 모두 갖고, 두 raw GitHub screenshot이 실제 PR 화면에서 로드됐다. C11.4 체크 상태와 text/image·WPT·실기기 한계를 미완료로 유지한다. |

**결론:** 30관점 검토에서 발견한 앱 진단 결함 1건은 두 플랫폼 모두 trace logging을 frame assertion 앞으로 옮겨 수정했다. 수정 후 두 simulator build와 성공 7-scene matrix를 다시 통과했고 canonical JSON이 7/7 일치했다. frame mismatch를 직접 주입한 실행은 하지 않았다. text/image intrinsic 앱 입력, border/surface pixel, cyclic percentage·전체 auto-repeat matrix, WPT와 실기기 근거는 계속 남아 있어 PR은 C11.4 완료로 처리하지 않는다.
