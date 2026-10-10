# C07.3 종횡비 구현 실패 경로 검토

검토 대상은 Stylo typed cascade, layout projection, Taffy adapter, V8 fixture FFI, Android JNI, iOS Objective-C++/Swift 호출, 고정 Chrome 기준, 실제 simulator 로그와 화면이다.

| # | 구현 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 속성이 author CSS profile 밖에서도 조용히 활성화되는가 | 지원 profile에서만 typed ratio를 snapshot에 싣고 기존 제한 profile 동작은 유지한다. |
| 2 | computed CSS 문자열을 다시 파싱하는가 | Stylo `GenericAspectRatio` typed value에서 auto flag와 preferred ratio를 읽는다. |
| 3 | ratio 축 방향이 뒤집히는가 | width/height 순서로 Taffy에 전달하고 16/9·1·4/3 fixed Chrome 관찰을 통과했다. |
| 4 | initial/explicit auto에 비율이 생기는가 | 두 상태는 `None`이며 Chrome reference의 빈 block frame과 일치한다. |
| 5 | zero degenerate ratio가 잘못된 양수 ratio로 바뀌는가 | zero 분자·분모 경우를 `None`으로 보존하고 fixture 기준과 맞춘다. |
| 6 | 유효한 `auto <ratio>`를 bare ratio로 오해하는가 | `auto` flag를 발견하면 node가 포함된 명시 오류를 반환한다. |
| 7 | f32 overflow/underflow 뒤 잘못된 ratio가 Taffy에 도달하는가 | 유한한 CSS 피연산자와 계산 결과 모두 검사하고 경계 negative test가 통과했다. |
| 8 | invalid declaration이 cascade의 앞선 유효 winner를 잃는가 | Chrome fallback 기준과 Stylo computed winner를 대조하는 test가 통과했다. |
| 9 | `var()` 또는 `calc()` 결과가 원문 문자열 경로를 우회하는가 | typed computed ratio만 전달하고 고정 reference의 var/calc 사례를 통과했다. |
| 10 | ratio+min/max 네 축 중 하나가 누락되는가 | min/max width/height 각각 fail-closed negative test와 property assertion이 통과했다. |
| 11 | Flex child min/max 차이가 독립 지원처럼 처리되는가 | Flex child도 adapter 오류로 거부하고 전체 계산 실패를 확인했다. |
| 12 | 제한 constraint 오류에서 partial layout을 반환하는가 | projection이 Taffy 계산 전에 실패한다. |
| 13 | 두 definite block leaf가 Taffy에서 늘어나는가 | 호출 전 크기를 resolve하고 해당 leaf 호출 복제본에서만 ratio를 꺼 `160×80`을 유지했다. |
| 14 | leaf 보정이 입력 computed style을 변형하는가 | layout 호출용 Taffy style만 복제·수정한다. |
| 15 | definite 크기 검사가 unresolved percent를 definite로 오인하는가 | 부모 basis로 Taffy dimension을 resolve한 결과에서 두 축 모두 값이 존재할 때만 보정한다. |
| 16 | content-/border-box, padding, border가 중복 가산되는가 | 30개 지원 geometry node의 각 frame field를 Chrome과 DPR 1·2로 비교했다. |
| 17 | Flex stretch, row/column 축과 정렬 차이가 사라지는가 | 30개 supported node 기준의 Rust differential과 기존 Flex 회귀가 통과했다. |
| 18 | style revision만 바뀐 ratio 값이 낡은 frame에 남는가 | ratio 2→1 업데이트가 새 StyleRevision과 기대 frame으로 전파되는 test가 통과했다. |
| 19 | 실제 플랫폼 fixture가 Rust 테스트만 실행하거나 다른 JS를 쓰는가 | Android와 iOS가 동일한 checked-in JS fixture를 FFI에서 실행했다. 양쪽 `status=0 layout=ready boxes=5` 로그를 확인했다. |
| 20 | 시각·로그를 geometry 정확도나 hardware 성능으로 과장하는가 | Android는 API 37 emulator의 ANGLE/SwiftShader, iOS는 iPhone 17 Pro iOS 26.2 Simulator Metal이다. 그림은 표시 경로 확인이며 자식별 geometry oracle은 아니다. |

## 테스트 모듈 구조 재점검

초기 통합 테스트 파일은 508줄이었다. fixture·layout 준비는 `c07_3_aspect_ratio_tests.rs`에 두고, pinned Chromium 비교·실패 계약·revision 검증은 각각 하위 모듈로 분리했다. 테스트 검색 이름과 검증 입력은 바꾸지 않았고, 분리 후 전체 Rust workspace 테스트에서 기존 411개 성공과 2개 ignored가 그대로 확인됐다. `cargo fmt --check`와 Clippy도 다시 통과했다.

## 실행 확인

- pinned Chrome 154 reference: 지원 30 node, DPR 1·2, field별 최대 허용 오차 `0.5 CSS px` 안에서 일치.
- `bun run test:css-reference`: 75/75 통과.
- `cargo test --locked --workspace --all-features --quiet`: 411 통과, 2 ignored.
- `cargo check --locked --workspace --all-features`: 통과.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: 수정 후 통과.
- Android API 37 emulator와 iPhone 17 Pro / iOS 26.2 Simulator 모두 fixture `layout=ready`, `boxes=5`, GPU surface 제출과 색 사각형을 확인했다. Android emulator는 SwiftShader software backend다.
- 화면·로그 파일과 해시는 [Taffy 차이·시뮬레이터 근거](c07-3-taffy-differential-2026-10-10.md)에 연결한다. 실기기와 hardware GPU 성능은 측정하지 않았다.
