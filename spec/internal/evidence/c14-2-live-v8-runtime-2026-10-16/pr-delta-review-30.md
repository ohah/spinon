# C14.2 live V8 app fixture · PR 변경 적대 검토 30개

2026-10-16 최종 PR diff를 구현·Android/iOS 경계·fixture oracle·artifact 분리·문서 주장 관점에서 다시 검토했다. 계획 검토 20개와 구현 검토 20개를 재사용하지 않았다. 각 관점에서 서로 다른 실패 경로와 보존 조건을 확인했다.

| # | 실패 관점 | 확인 및 결과 |
|---:|---|---|
| 1 | 기본 Cargo feature가 C14.2 fixture를 켬 | `c14-2-app-fixture-test`는 선언만으로 기본 활성화되지 않는다. `fixtures.rs`와 FFI export도 해당 feature의 `cfg` 안에 있다. |
| 2 | 일반 FFI 소비자가 테스트 ABI를 헤더에서 봄 | C 선언은 `SPINON_ENABLE_C14_2_APP_FIXTURE_TEST`가 정의되고 1일 때만 노출된다. 기본 build define은 0이다. |
| 3 | Android C++ feature와 Rust Cargo feature가 엇갈림 | Gradle environment → `tools/build-android.sh` feature 목록과 JNI compile define이 같은 flag를 사용한다. opt-in build log가 1, default build log가 0이다. |
| 4 | iOS Simulator와 iOS native script가 서로 다른 flag를 사용 | Gradle과 함께 `build-ios-sim.sh`가 값을 검증하고 xcodebuild environment 및 native feature 선택에 전달한다. opt-in/default build log를 확인했다. |
| 5 | 잘못된 build flag가 조용히 true/false 처리 | Android·iOS shell scripts와 Gradle이 0 또는 1만 허용하고 나머지는 종료한다. 세 build script 구문 검사는 통과했다. |
| 6 | Android JNI가 크기 제한 없이 Rust output을 읽음 | JNI는 고정 65,536-byte 버퍼를 주고 UTF-8 byte array로 복사한다. null array와 JNI exception 경로를 검사했다. |
| 7 | C ABI report가 작은 output buffer에서 부분 JSON을 성공 처리 | 공용 bounded writer는 NUL 공간이 부족하면 false를 반환하고 C14.2는 `-3` 오류를 돌린다. 앱은 전체 report의 parse·case count를 다시 확인한다. |
| 8 | iOS C string 변환 오류가 빈 성공 report처럼 보임 | Objective-C++는 null UTF-8 변환 결과를 `empty report`로 바꾸며 status를 함께 반환한다. Swift는 성공 status prefix와 JSON 상태를 모두 요구한다. |
| 9 | C14.2 실행 인자가 다른 fixture를 동시에 선택 | Android는 `spinon_` extras의 활성 값과 충돌을 검사하고 iOS는 `--spinon-` arguments 충돌을 검사한다. 충돌 경로는 native 계산 전에 오류 화면으로 닫힌다. |
| 10 | 기본 앱에 route가 들어와도 fixture가 실행 | 양 플랫폼 기본 app route가 feature-off 메시지/log를 반환했다. default APK/app과 logs를 보존했다. |
| 11 | background 계산 중 화면 파괴 후 UI를 갱신 | Android main-thread 갱신은 `activityClosing`을 확인한다. iOS 결과는 메인 queue에서 화면에 반영된다. 캡처 시 앱 foreground 경로를 사용했다. |
| 12 | live V8 fixture가 synthetic document로 대체 | Block/Flex 각각 별도 `RuntimeSession::eval`과 actor snapshot을 사용하고 report origin을 `live-v8`로 고정한다. |
| 13 | snapshot의 root·자식 순서·node kind가 oracle과 다름 | `<main id=cases>` 하나와 고정 mapping count/order를 확인한다. 각 Block img 또는 Flex div→img 관계 및 HTML namespace/ID를 확인한다. |
| 14 | 다른 이미지 URL이나 detached node가 sizing 결과에 섞임 | 각 img의 연결 상태와 고정 `spinon.invalid` src를 검사한 뒤 request를 만들며 request source stamp가 snapshot과 같은지 비교한다. |
| 15 | author style이 fixture 원본에서 변경되거나 case ID와 결합되지 않음 | mapping의 inline style·parent class/style을 DOM snapshot 값과 대조한다. 고정 HTML에서 stylesheet를 추출하고 제한 runtime의 `<main>` block UA 기본값만 보충한다. |
| 16 | main 기본 display 누락이 CSS frame을 우연히 통과 | 초기 Simulator failure에서 실제 누락이 드러났다. Chrome UA 기본값을 cascade 입력에 명시한 뒤 최종 양 플랫폼 결과가 통과했다. |
| 17 | CSS cascade profile이 `aspect-ratio` 또는 Flex 규칙을 버림 | C14.2에서 사용하는 `aspect-ratio`를 가진 Flex custom-properties cascade 경로를 쓴다. 실패 당시 두 ratio case를 확인했고 최종 report에 style diagnostics가 없다. |
| 18 | PNG bytes는 바뀌었지만 natural size만 우연히 같음 | 고정 resource hash와 PNG bytes를 비교하고 decode 완료의 dimension을 24×16 CSS px oracle에 대조한다. opt-in payload scanner도 전체 97-byte PNG를 찾는다. |
| 19 | resource snapshot이 다른 document generation/revision을 참조 | 요청 source, resource snapshot, intrinsic snapshot source와 revision을 같은 live document에 대조한다. report에 document/render/style/environment/intrinsic revision을 기록한다. |
| 20 | case 사이에서 stale intrinsic revision을 재사용 | 각 Block/Flex document가 독립 resource snapshot과 intrinsic revision을 만든다. report summary에서 각 document revision 일치 여부를 확인한다. |
| 21 | 이미지 자연 크기와 CSS used size를 혼동 | report는 natural size 24×16과 CSS frame을 분리하고 자연 크기를 모든 29행에서 검사한다. |
| 22 | 브라우저나 viewport가 바뀐 oracle을 통과 처리 | oracle reader는 Chrome 155.0.8059.40, 1280×633 CSS px, DPR 1을 강제하고 고정 observation의 browser·viewport 값을 확인한다. |
| 23 | 미지원 22번째 Block row를 21-case 성공 분모에 포함 | case mapping 순서를 inventory의 실행 가능 21행과 정확히 대조한다. `aspect-ratio:auto <ratio>` row는 명시적 미지원 marker라 제외한다. |
| 24 | tolerance가 oracle 소수값을 먼저 반올림해 비교를 왜곡 | case별 inventory tolerance를 사용하며 관찰한 19.984375×13.328125 oracle 값을 그대로 report한다. NaN/무한 tolerance는 거부한다. |
| 25 | Block image의 배치 좌표 차이를 size만 비교해 숨김 | 21개 Block case 각각 image x/y/width/height를 oracle과 비교한다. |
| 26 | Flex image 위치·stretch 또는 부모 basis 차이를 size만 보고 누락 | 8개 image 전체 frame을 비교하고 Chrome oracle에 제공된 부모 width/height와 `align-items`를 비교한다. 실제 부모 frame은 report에 보존하며 oracle이 제공하지 않는 parent x/y는 독립 비교하지 않는다고 계획에 명시했다. |
| 27 | 누락·중복·잘린 report가 플랫폼에서 green 처리 | Rust는 case status/count로 summary를 만들고 Android/Swift가 29개 unique ID, 21/8 분포, finite frame, natural size, source flags를 다시 검증한다. log chunk를 재조립한 최종 report가 29행이다. |
| 28 | fixture symbol·payload가 기본 artifact에 들어감 | 기본 Android archive/native library/APK와 iOS static/app executable에서 symbol·label·ID·PNG byte가 모두 없고 opt-in 산출물에서 모두 있음을 raw-byte scanner로 대조했다. |
| 29 | 한 플랫폼 실행이나 실기기 상태를 양 플랫폼 증거로 확대 | Android API 37 ARM64 AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 각각 실행하고 독립 report·log·screenshot·hash를 보존했다. 물리 기기는 연결·조회하지 않았다. |
| 30 | sizing report 화면을 실제 image paint·overflow clip 증거라고 설명하거나 PR screenshot 링크가 깨짐 | 화면은 29/29 report text다. border/padding 포함 CSS geometry는 oracle과 비교했지만 native image paint, border 바깥 pixel overflow/clip, GPU scanout은 검사하지 않았다고 STATUS·계획·README·PR #219 본문에 기록했다. Agent Browser에서 PR을 열어 두 `raw.githubusercontent.com` 이미지가 complete=true로 로드되고 Android 1080×2400, iOS 1206×2622임을 확인했다. |

최종 판단: 해결되지 않은 P1/P2 없음. 29/29는 opt-in live V8 → HostDocument snapshot → pinned CSS cascade → Taffy sizing geometry 비교를 뜻하며 제품 image loader, native image paint/clip, 실기기 또는 전체 CSS 적합성 완료를 뜻하지 않는다.
