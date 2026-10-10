# C06·C07.1 통합 변경 적대적 검토

**대상:** C06.1~C06.6a 및 C07.1의 현재 작업 브랜치 전체 diff, Rust layout/style/FFI 연결, Android·iOS 실행 경로
**판정:** 제한된 내부 runtime profile만 확인했다. 공개 CSS 지원이나 C06·C07 전체 완료를 뜻하지 않는다.

각 검토 관점은 하위 기능 문서의 검토 표를 합산하지 않고, 기능들이 함께 바뀐 현재 diff에서 서로 다른 교차 계층 실패를 확인한다.

| # | 공격 입력·경계 | 확인 결과와 근거 |
| ---: | --- | --- |
| 1 | CSS computed 문자열을 used value 또는 layout 입력으로 재사용 | Stylo 직렬화 결과와 typed computed value가 다를 수 있다. layout은 typed 단위/AST를 소비하고 CSS 문자열 oracle과 최종 frame oracle을 분리한다. C06.1·C06.5 reference 테스트 통과. |
| 2 | percentage 크기를 CSS px로 미리 환산하거나 100% 초과를 clamp | `%` 값이 Taffy 입력까지 fraction으로 보존되고 125%를 허용한다. containing-block basis가 불명확한 입력은 성공값으로 대체하지 않는다. percentage fixture와 invalid input 테스트 통과. |
| 3 | percentage margin·padding·gap이 같은 축 규칙을 사용한다고 가정 | margin/padding basis와 row/column gap basis를 별도 projection 경로로 처리한다. fixed Chrome fixture의 computed 값과 frame 검증을 통과하고 cyclic/indefinite 경우는 실패한다. |
| 4 | 절대 단위가 플랫폼 dpi 또는 backing pixel에 따라 변함 | `in`, `cm`, `mm`, `Q`, `pt`, `pc`, `px` 입력을 CSS px 의미로 정규화하고 DPR은 layout 값에 섞지 않는다. DPR 쌍의 Chromium 비교가 고정 허용 오차 내에 있다. |
| 5 | `em`·`rem`이 항상 root 또는 항상 자기 font-size 기준으로 계산 | 자기/부모 `font-size`, 상속, 합성 HTML 문서 루트의 `rem` 특례를 별도 fixture로 비교한다. metric이 필요한 `ex`·`ch` 등은 지원처럼 낮추지 않고 거부한다. |
| 6 | `calc()`·`min()`·`max()`·`clamp()`에서 서로 다른 CSS 차원을 더하거나 함수 일부를 누락 | Stylo typed AST를 소유 layout AST로 옮긴 뒤 Taffy resolver에 전달한다. 지원 함수·인수 수·차원 오류의 제한을 reference 및 typed-math 테스트로 고정했다. |
| 7 | `var()` 대체 뒤 원본 수식의 property 또는 percentage basis가 다른 node에서 재사용 | 계산된 수식은 NodeId/property에 연결하고 CSS 변수 대체·cascade 결과를 같은 snapshot에서 layout에 전달한다. 누락·불일치 binding negative tests는 전체 계산을 실패시킨다. |
| 8 | viewport 단위가 drawable 크기, DPR 또는 잘못된 논리 축을 참조 | 기본/small/large/dynamic family와 width/height/inline/block/min/max를 CSS viewport에서 계산한다. fixture는 DPR 1·2 및 세 viewport를 따로 비교한다. |
| 9 | resize 중 예전 layout 또는 장면이 새 환경을 덮음 | environment revision 변경 시 이전 publication을 무효화하고 새 layout 완료 뒤 표시한다. Android 301×100↔341×128 왕복에서 renderer 교체가 장면을 지우던 문제를 재현해 수정했고, 양 플랫폼 V8 fixture를 다시 실행했다. |
| 10 | `min`이 `max`보다 큰 값, `auto`, `none`, 0을 하나의 숫자로 취급 | Chromium과 Rust 모두 min-over-max 결과를 비교하고 `auto`/`none`은 구별된 상태로 유지한다. 0과 물리 축별 최소·최대 테스트를 통과했다. |
| 11 | `content-box`와 `border-box`의 padding을 min/max 경계에 중복 포함하거나 누락 | 지정 크기와 바깥 frame을 따로 비교한다. padding이 제한보다 큰 border-box 하한도 Chrome frame과 일치한다. border paint 지원까지 주장하지 않는다. |
| 12 | Flex shrink/grow가 min/max 또는 `min-width:0` 의미를 무시 | 제약 있는 shrink·grow와 zero minimum을 별도 Chrome case로 확인했다. 제약 초과와 남는 공간 배치까지 비교했다. |
| 13 | 음수·NaN·무한대·범위 초과 값이 Taffy 또는 scene에 부분 적용 | DTO 검증이 layout 계산 전에 실패하고 결과를 일부 발행하지 않는다. 직접 입력·computed CSS math 실패 case 및 failure-atomic 테스트 통과. |
| 14 | `min-content`·`max-content`·`fit-content`를 임의 숫자나 0으로 처리 | 현재 profile은 intrinsic sizing 측정이 없으므로 해당 값을 fail-closed 한다. 이를 지원 범위 밖으로 남기고 C14 미완료 상태를 유지한다. |
| 15 | 새 CSS allowlist/UA 값이 기존 fixture profile의 기본 동작을 넓게 바꿈 | 새 property 투영은 제한 profile에만 연결한다. 기존 profile 회귀 테스트와 supported-element UA stylesheet 검사를 통과했다. |
| 16 | incremental cascade가 typed 값을 직렬화 cache와 다른 revision에서 재사용 | 전체 계산과 incremental 재사용 경로에서 typed style snapshot의 동일성·재계산/재사용 집계를 테스트한다. source 변경과 style 변경의 invalidation 경계를 별도 검사했다. |
| 17 | layout snapshot cache가 viewport/environment/style 변경을 놓치고 stale frame을 반환 | 입력 revision과 viewport 조건을 함께 admission 검사하고 key가 다르면 부분 snapshot 없이 거부한다. stale snapshot 및 resize tests 통과. |
| 18 | Rust 모듈 분리 뒤 C ABI 심볼·선언·오류 출력이 끊기거나 버퍼가 넘침 | surface 수명 함수와 C06/C07 fixture 함수를 별도 모듈에 두되 헤더 선언과 exported symbol 이름을 맞췄다. null/빈 출력 버퍼·status 경로를 검사했고 Android/iOS 링크 빌드가 통과했다. |
| 19 | Android demo 플래그만 처리되고 JNI→Rust 평가 또는 GPU 표시가 빠짐 | API 37 ARM64 emulator에서 새 APK를 설치·실행했다. `SPINON_C071_EVAL status=0`, `layout=ready`, `boxes=10`, renderer와 `presented` 로그, 화면 캡처를 다시 수집했다. ANGLE/SwiftShader software path라 hardware GPU 성능은 미검증이다. |
| 20 | iOS launch argument가 앱 선택·ObjC++ FFI·V8 fixture 중 한 경계에서 누락 | iPhone 17 Pro / iOS 26.2 Simulator에서 새 빌드를 설치·실행했다. `SPINON_C071_EVAL status=0`, `layout=ready`, `boxes=10`, `presented` 로그와 화면 캡처를 확인했다. 실기기와 hardware 성능은 미검증이다. |

## 통합 점검에서 수정한 항목

- FFI surface 함수, Stylo incremental 계산/재사용 코드, HostDocument 계약 테스트를 책임별 파일로 분리했다. 처음 분리할 때 `LayoutJustifyContent` 재수출이 빠져 컴파일 오류가 났고 재수출을 복구했다. 변경된 Rust 파일은 프로젝트 권장 500줄 안에 둔다.
- Stylo가 `min-height: calc(10px + 2px)`를 computed string `12px`로 직렬화하는 결과를 확인했다. 계산에 쓰는 typed AST는 별도 보존하므로 기대값을 보정하고 문자열이 수식 AST를 대체하지 않게 했다.
- C07.1의 host demo 진입이 준비되지 않은 플랫폼 경로를 추가 연결했다. Android API 37과 iOS 26.2 simulator에서 실제 평가와 화면 표시까지 다시 확인했다.

## 현재 경계

- Rust workspace 392개 테스트 통과, 2개 무시. Clippy, rustfmt, FFI all-features check, C06/C07 reference 테스트 30개, Android API 37 emulator 빌드, iOS 26.2 simulator 빌드가 통과했다.
- 변경된 제품·내부 계약 숫자 버전은 없다. Spinon 패키지와 미출시 내부 계약은 `0.1.0`으로 유지하고, `0037`~`0043`은 문서 ID다.
- C06 상위는 container-relative units가 남아 미완료다. C07 상위, C08도 미완료다. 실기기, hardware GPU, text intrinsic sizing, 공개 CSS API와 전체 CSS 호환성은 이 결과로 완료 처리하지 않는다.
