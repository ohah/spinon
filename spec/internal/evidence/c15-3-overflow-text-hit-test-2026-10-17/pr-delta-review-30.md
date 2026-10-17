# C15.3 overflow text-owner hit-test PR 변경 실패 관점 30개

구현 로직의 20개 검토와 분리해 이번 PR diff의 인터페이스, 문서 상태, 실행 근거, 플랫폼 증거와 변경 범위를 검토했다. 각 관점은 다른 회귀나 오해 가능성을 대상으로 한다.

| # | PR 변경 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | fixture 변경이 공개 Rust hit-test API를 추가한다 | 새 공개 함수나 타입 없이 기존 snapshot 메서드를 opt-in fixture에서 호출한다. 통과. |
| 2 | C ABI header와 Swift/Kotlin에 새 제품 ABI가 노출된다 | C 선언·제품 ABI를 추가하지 않았다. Swift 변경은 기존 fixture report 검증만 확장한다. 통과. |
| 3 | 기본 runtime artifact에 fixture payload가 포함된다 | `#[cfg(feature = "c15-3-wgpu-glyph-paint-fixture-test")]` 모듈 선언, 기본 static archive 문자열 검사와 opt-in 47-test positive evidence를 대조했다. `nm` symbol scan은 Apple LLVM의 Rust LLVM 22 object 해석 한계 때문에 미완료이며 근거에 제한을 기록했다. 통과(범위 제한). |
| 4 | Android와 iOS가 서로 다른 feature/report 계약을 사용한다 | 두 플랫폼 모두 같은 fixture feature와 `visible_owner;hidden_inside_owner;hidden_outside_excludes_hidden_owner` report를 사용한다. 통과. |
| 5 | Swift guard의 기대 문자열과 Rust report가 갈라진다 | Rust report, iOS assertion, iOS 전용 log 값이 일치한다. 통과. |
| 6 | hit-test report 누락이 다른 로그만으로 성공 처리된다 | 앱이 exact report 문자열을 요구하고 별도 `SPINON_C153_HITTEST` 행으로 남긴다. 통과. |
| 7 | fixture sample 좌표가 기존 pixel sample와 분리된다 | 같은 fixture sample 좌표를 픽셀 표본과 hit-test에 사용하고 Rust는 CSS pixel center `+0.5`를 검사한다. 통과. |
| 8 | CSS sample을 DPR 배율의 surface 좌표로 hit-test한다 | hit-test는 CSS 좌표를 사용하며 Android PixelCopy/iOS drawable mapping은 별도 로그로 검증한다. 통과. |
| 9 | visible sample이 parent clip 안쪽으로 옮겨진다 | 계약과 fixture geometry에서 sample x=29.5는 parent의 right edge x=28보다 바깥이며 40px owner box 안이다. 통과. |
| 10 | hidden 내부 표본이 clip 경계 위에 놓인다 | 고정 정수 pixel 중심 `(13.5, 121.5)`가 hidden owner clip 내부다. 통과. |
| 11 | hidden 외부에서 `None`만 허용해 ancestor fallback을 깨뜨린다 | 계약은 hidden owner NodeId 배제만 요구하고 다른 ancestor/anonymous target은 허용한다. 통과. |
| 12 | report가 glyph별 hit-test 또는 DOM event 성공으로 과장된다 | 내부 계약은 RuntimeRenderBox owner probe로 제한하고 glyph geometry, CSSOM, DOM event를 제외한다. 통과. |
| 13 | 구현 전 Chrome DOM oracle를 제품 계약으로 오인한다 | 문서는 관찰값을 고정 oracle로 적고 runtime과 DOM의 범위 차이를 명시한다. 통과. |
| 14 | 내부 명세 숫자 버전이 실험 추가만으로 올라간다 | 새 계약은 내부 초안 `0.1.0`으로 고정했다. 통과. |
| 15 | 공개 API 지원 인덱스에 미출시 기능이 추가된다 | `spec/api` 지원 목록을 건드리지 않았다. 통과. |
| 16 | STATUS의 C15 상위 완료 체크나 완료 개수가 증가한다 | C15 checkbox는 미완료 그대로이며 제한 probe 설명만 추가했다. 통과. |
| 17 | STATUS 설명이 run evidence나 제한 범위를 잃는다 | C15.3 제한 scope, 구현 리뷰, 플랫폼 표본, 계약 0073 링크를 연결했다. 통과. |
| 18 | 계획 문서가 glyph hit geometry 완료를 암시한다 | 계획은 owner-box clip hit probe만 언급하며 glyph별 hit, DOM event, scroll을 제외한다. 통과. |
| 19 | 내부 문서 인덱스에서 0073 계약을 찾을 수 없다 | `spec/internal/README.md` 표에 계약·실행 근거·제외 범위를 연결했다. 통과. |
| 20 | 상대 Markdown 링크가 작업 브랜치의 다른 파일을 가리킨다 | contract/evidence/plan/status 링크를 파일 경로 기준으로 교차 확인했다. 통과. |
| 21 | Android 실행 로그에 이전 앱 PID의 stale 결과가 섞인다 | `android-run.txt`는 지정 AVD fixture PID 행으로 제한되어 있고 current fixture report와 3/3 sample을 담는다. 통과. |
| 22 | iOS 실행 log가 이전 run 또는 다른 simulator 앱을 가리킨다 | 재빌드 뒤 지정 iPhone 17 Pro Simulator에 설치·실행했고 PID 81811의 새 log에 report와 표본을 담았다. 통과. |
| 23 | iOS 결과가 화면 screenshot만으로 surface pixel을 주장한다 | 근거는 compositor 전 Metal drawable의 `Bgra8Unorm` pixel log와 분리된 screenshot이다. 통과. |
| 24 | simulator 로그를 실기기·하드웨어 GPU·compositor 증거로 부풀린다 | README는 Android AVD/iOS Simulator 한계와 compositor 전 표본을 명시한다. 통과. |
| 25 | iOS dSYM 공간 부족의 실패가 최종 build 상태를 모호하게 한다 | 첫 실패 로그를 별도 보존했고 최종 `ios-build.txt`는 `BUILD SUCCEEDED`, 재실행 log는 새 앱 PID를 기록한다. 통과. |
| 26 | 캡처 PNG가 없거나 텍스트 로그를 이미지 링크로 연결한다 | Android/iOS screen 및 Android PixelCopy PNG 파일과 파일 형식을 확인했다. 통과. |
| 27 | Rust 결과가 전체 workspace 또는 0 ignored 통과로 오해된다 | FFI feature test 47 passed, 2 ignored와 focused 3 passed 범위를 README에 구분했다. 통과. |
| 28 | Clippy/rustfmt 결과가 실행하지 않은 전체 CI를 뜻한다 | README는 명령을 특정 feature/targets로 기록하며 CI 전체 통과를 주장하지 않는다. 통과. |
| 29 | 다른 작업자의 shared checkout 변경이 PR에 섞인다 | 변경은 `origin/main` SHA `c41a95c5fa06470f6984968276913eaeffc95d7e`에서 만든 격리 worktree에만 있다. 통과. |
| 30 | PR 병합 방식이 프로젝트 정책과 달라지거나 main에 직접 push한다 | 변경을 작업 브랜치 PR로 제출하고 checks/merge 상태 확인 후 Rebase and merge만 사용한다. 통과. |

이 검토는 현재 diff와 함께 저장한 실행 artifact에 한정된다. 실행하지 않은 전체 CI, Android 실기기, iOS 실기기, compositor 후 pixel, DOM event와 전체 CSS conformance를 대신하지 않는다.
