# C11.4 generic mixed row-axis PR diff 적대 검토 · 30개 관점

계획 PR의 사전 검토와 구현/runtime 20관점 리뷰를 반복하지 않도록, 이번 구현 변경의 module boundary, route 통합, Android/iOS 계약, 실행기, 증거 파일 및 roadmap 상태를 서로 다른 실패 경로로 확인했다. 검토 기준은 최신 `origin/main...HEAD` diff와 이 문서에 연결된 캡처·검증 결과다.

| # | 실패 관점 | 검토 결과 |
| ---: | --- | --- |
| 1 | 고정 fixture 작업이 공개 Rust/C API, ABI 또는 내부 명세 버전을 늘린다. | 새 route는 기존 C11.4 fixture 경로에 추가됐고 public header/API나 contract version 변경은 없다. |
| 2 | 신규 runtime 모듈이 기존 product feature만으로 기본 제품 빌드에 포함된다. | Runtime 모듈은 기존 `c11-4-grid-track-sizing-fixture-test` 아래에 등록된다. 기본 FFI archive에서 신규 네 route ID가 모두 빠지고 opt-in positive control에는 모두 있다. |
| 3 | FFI scene dispatch가 다른 C11.4 route ID를 잘못 소비한다. | 네 ID는 신규 generic row module에 고유하고, static fixed row와 orthogonal routes의 dispatch는 별도로 유지된다. C11.4 FFI suite 84/84와 전체 simulator matrix가 통과했다. |
| 4 | 앱 launch catalog와 Rust-generated source가 다른 scene ID를 사용한다. | Android/iOS native allowlist 네 ID가 FFI scene catalog의 네 ID와 일치하며 플랫폼별 네 generic row scene이 실행됐다. |
| 5 | 신규 matcher가 기존 fixed-ID row scene을 선점한다. | Runtime router의 fixed scene table lookup이 generic matcher보다 먼저다. 두 플랫폼에서 fixed-row 네 scene도 새 source build로 모두 통과했다. |
| 6 | 신규 matcher가 기존 generic column route를 선점한다. | Row matcher는 row intrinsic style이 없으면 `None`을 반환하고 generic column adapter가 후속 처리한다. generic column 네 scene이 양 플랫폼에서 회귀 통과했다. |
| 7 | adapter가 HostNode ID 목록에 의존해 novel identity 지원을 가장한다. | Runtime match 조건은 mixed topology와 computed style이다. positive test는 네 개 서로 다른 `novel-generic-row-*` root/item identity로 성공한다. |
| 8 | 다른 HTML topology가 direct text/image two-child shape로 받아들여진다. | Root parent·namespace/tag·정확한 direct-child 수와 text/image tag를 검사한다. extra child, nested text, duplicate/wrong kinds는 fail-closed한다. |
| 9 | 혼합 signature가 발견된 malformed input이 text-only/image-only 결과로 내려간다. | Signature 이후 오류는 `Err`로 끝나며 partial result를 반환하지 않는다. malformed row controls는 negative tests에서 거부된다. |
| 10 | text node 문자열·font·line metrics·whitespace가 oracle와 달라도 허용된다. | Exact `XX XXXXX`, Ahem 10px/10px, wrapping·whitespace·text properties를 확인한다. changed text와 wrong font control이 거부된다. |
| 11 | image URL은 맞지만 decoded resource 크기나 intrinsic revision이 다르다. | 고정 PNG를 실제 decode해 `24×16` natural size를 확인하고 revision 1로 trace한다. wrong/missing URL controls는 실패한다. |
| 12 | generic row computed root defaults를 fixed-ID border-box 계약과 섞는다. | Generic row만 oracle의 initial `content-box`와 normal alignment defaults를 요구하고, 기존 fixed scenes는 기존 `border-box`/authored alignment 계약을 유지한다. |
| 13 | child box-sizing, edges, alignment 또는 placement가 달라도 동일 layout으로 소비된다. | Row item `content-box`, zero edges, static position, auto self-alignment, text col 1/image col 2/row 1을 확인한다. 관련 malformed controls가 거부된다. |
| 14 | 여러 row track, 잘못된 columns, named lines나 area metadata가 제한 scene을 확장한다. | Root는 정확한 `[50,24]` columns와 한 개 intrinsic row를 요구하고 auto-repeat, named lines, areas, 다른 auto-grid metadata를 거부한다. |
| 15 | stale snapshot 또는 다른 style profile의 결과가 새 layout revision으로 처리된다. | Profile·generation·document·render-tree revisions를 함께 검사하며 profile/document mismatch tests가 fail-closed한다. |
| 16 | text와 replaced-image intrinsic inputs가 서로 다른 layout revision에서 합쳐진다. | Decode된 image snapshot의 revision을 layout input에 넣고 runtime tests 및 simulator traces에서 image revision 1과 text/image provenance를 확인한다. |
| 17 | row sizing matcher가 Columns contribution을 Row contribution으로 오인한다. | Native verifier와 JS capture verifier는 Rows pass의 MinContent/MaxContent text 20px·image 16px contribution을 item·span별로 비교한다. |
| 18 | contribution은 맞지만 Columns→Rows 실행 순서 또는 최종 tracks가 틀린다. | Trace는 Columns 다음 Rows, 최종 columns `[50,24]`, rows `[20]`을 요구하며 32개 scene 결과에서 확인됐다. |
| 19 | Android launch catalog는 새 ID를 받지만 node frame expectations는 기존 column 규칙을 사용한다. | Android matcher는 row ID prefix·source order에 따라 74×20 root, forward/reversed node order 및 row frames를 비교한다. Android 16/16이 통과했다. |
| 20 | Android intrinsic trace가 scene 순서를 무시하거나 text metrics를 누락한다. | `c114ExpectedMixedIntrinsicInputs`의 row generic forward/reversed branches는 node/kind/source order와 `[50,20,80,10]` text metrics를 확인한다. 16개 runtime log가 검증됐다. |
| 21 | iOS launch allowlist가 generic row scene을 누락하거나 다른 set에 중복 등록한다. | iOS intrinsic scene set에 네 generic row ID가 한 번씩 있으며 iOS matrix 16/16이 각 ID를 실행했다. |
| 22 | iOS frame/trace helper가 column geometry를 새 row IDs에 계속 적용한다. | Helpers는 `direct-row-` prefix로 row frame, contribution axis와 `[50,24]`/`[20]` tracks를 고른다. 양 forward/reversed row logs가 기대값을 만족했다. |
| 23 | wrapper matrix가 고정·generic 또는 forward/reversed 조합 일부를 생략한다. | `run-simulator-matrix.sh`는 fixed column 4, fixed row 4, generic column 4, generic row 4를 전달한다. 각 플랫폼 manifest에 16개 unique scene이 있다. |
| 24 | 실행기가 연결된 물리 Android device나 다른 iOS simulator를 선택한다. | Android 명령은 명시적으로 `-s emulator-5554`, iOS 명령은 고정 UDID `9DCCE836-7659-4914-B6C1-13F54BFFA9DB`를 사용했다. 물리 Samsung은 선택하지 않았다. |
| 25 | 두 플랫폼이 서로 다른 tracked source 또는 도중 변경된 파일로 빌드된다. | 두 manifest는 구현 commit `0c89ffcd`와 같은 tree·source-file hashes, 빈 tracked diff를 기록하고 build 중 입력 hash 불변을 검사한다. |
| 26 | simulator가 build artifact 대신 stale install을 실행한다. | Android APK와 설치된 `base.apk` SHA-256이 같고 iOS app executable hash도 capture build provenance와 같다. |
| 27 | screenshot이 draw 전에 캡처되거나 잘못된 앱/크기 이미지가 PR에 연결된다. | 캡처는 scene-specific draw/present와 foreground/process 검증 후 수행하며 PNG signature·simulator dimensions·iOS title OCR을 검사했다. 각 플랫폼 이미지 hash는 manifest와 일치한다. 화면은 host surface/box 보조 자료이며 pixel equality를 주장하지 않는다. |
| 28 | runtime logs·manifest·archive controls가 부분 결과를 16/16 또는 default exclusion으로 과장한다. | 각 manifest 16 observations와 scene log hashes를 재검증했다. Default/fixture archive 결과는 Rust static archive에 한정하고 APK/IPA exclusion으로 확대하지 않는다. |
| 29 | validation 요약이 strict Clippy 실패를 감추거나 baseline warning을 새 수정 결과로 돌린다. | `-D warnings` 실행의 기존 `type_complexity`·`map_identity` 위치와 종료 실패가 보존됐다. 두 lint class만 허용한 후속 workspace Clippy는 통과했으며 이 차이를 README/STATUS에 적었다. Runtime 13/13, FFI 84/84, fmt 및 archive controls도 로그로 연결했다. |
| 30 | roadmap/PR 문구가 candidate를 C11.4 완료로 세거나 CI·미검증 범위를 과장한다. | 상위 checkbox와 공식 `72/213`은 유지하고, status/plan은 제한 slice·simulator 결과·Clippy caveat·남은 범위를 명시한다. GitHub checks와 PR image URL은 PR 생성 뒤 별도로 확인하며 이 diff review를 CI pass로 확대하지 않는다. |

검토 중 blocking implementation/diff 결함은 발견되지 않았다. GitHub의 check rollup, Agent Browser에서 PR 본문의 실제 image URL 렌더링, merge SHA 및 post-merge public roadmap 반영은 PR 생성/병합 뒤의 별도 단계다.
