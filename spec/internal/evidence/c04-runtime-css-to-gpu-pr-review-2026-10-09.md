# C04.10 PR 변경 적대 검토

검토 대상은 PR #96의 제출 커밋 `e9cd07f218c44c88439cd452b1137799311ae7fd`이다. 구현 전 계획 검토 및 구현 코드 검토와 별도로, PR 변경 전체의 통합·계약·증거 경계를 다시 대조했다.

| # | 공격 관점 | 대조 결과 |
|---:|---|---|
| 1 | 다른 작업 파일이 이번 기능 변경에 섞였는가 | 변경 파일 112개를 상태 대장 C04.10 범위와 대조했다. renderer crate, FFI, Android/iOS 호스트, fixture, 도구, 상태·계약·증거 자료로 한정되어 있다. 기존 S04 spike 구현 파일은 변경하지 않았다. |
| 2 | 출시 전 Spinon 패키지나 내부 계약 숫자 버전이 올라갔는가 | 변경된 모든 Spinon crate manifest는 `0.1.0`이다. `0031`은 명세 문서 식별 번호이며 제품 버전이 아니다. `Cargo.lock`의 형식 버전과 외부 의존성 버전은 별개다. |
| 3 | WGPU 추가가 잠금 빌드와 의존성 경계를 우회하는가 | `spinon-render-wgpu`는 별도 제품 crate이며 `wgpu 30.0.1`을 사용한다. Android/iOS 빌드와 Rust 검증은 `--locked`로 수행한다. |
| 4 | GPU 기능이나 실패 주입이 기본 FFI 제품 ABI에 무조건 노출되는가 | C04.10은 `spinon-ffi`의 opt-in feature다. 실패 주입은 별도 `c04-runtime-gpu-test-hooks` feature와 C++/Swift debug fixture 플래그로 보호된다. |
| 5 | C header와 Rust export가 함수 인자·호출 순서에서 어긋나는가 | `spinon_ffi.h`, Rust `extern "C"`, JNI 및 Objective-C++ 호출을 대조했고 Android API37/iOS Simulator 앱 링크·빌드가 통과했다. |
| 6 | 잘못된/null 포인터, 작은 출력 버퍼, 잘못된 UTF-8가 안전한 실패 대신 부분 초기화를 남기는가 | FFI 진입점은 포인터·용량을 검사한다. host 생성은 보고 복사 실패 시 소유 객체를 정리하고, Android/iOS renderer 생성은 보고 기록 실패 시 준비된 renderer/surface를 파괴한다. 이 항목은 소스·계약 검토이며 임의 포인터를 앱에서 주입하는 테스트는 포함하지 않는다. |
| 7 | JavaScript 실행 제한 시간을 취소 보장으로 오해하게 만드는가 | 헤더·Rust 문서·내부 계약은 `layout_timeout_millis`가 Stylo/Taffy 완료 대기만 제한하고 V8 평가는 동기·취소 불가라고 명시한다. |
| 8 | 문서·트리·스타일·환경 revision 중 하나가 다른 장면을 통과하는가 | `RuntimeRenderKey` 전체 다섯 축을 비교한다. Rust scene 변환 및 FFI 테스트가 축별 mismatch와 publish/invalidate 경쟁을 다룬다. |
| 9 | 중복 노드, 빠진 paint 순서, 비유한 frame이나 음수 크기가 부분 렌더로 이어지는가 | scene 구성과 geometry 변환은 전체 입력을 검증한 뒤 오류를 반환한다. 단위 테스트는 노드·순서·좌표 오류와 투명 paint를 확인한다. |
| 10 | 런타임 fixture와 Chromium 기준이 서로 다른 요소나 CSS를 비교하는가 | JS/HTML의 DOM 순서·inline style·viewport를 고정 비교한다. Chromium 버전과 입력 해시, computed style·좌표 기준, 픽셀 표본 기준을 기록했다. 각 geometry 오차는 노드별 0.5 CSS px 이내로 판정한다. |
| 11 | 투명 배경이나 sRGB 형식 fallback에서 색상이 잘못 칠해지는가 | 투명 paint는 생략하고 surface는 별도 clear color로 지운다. sRGB texture와 sRGB color-space UNORM을 구분하며 채널 readback을 비교한다. |
| 12 | 지원하지 않는 Android backend를 성공으로 보고하거나 fallback 과정에서 surface를 재사용하는가 | 기본 순서는 Vulkan 뒤 GL이며 각 실패 surface를 버리고 새 backend surface를 생성한다. 강제 backend 옵션과 초기화 보고가 있다. API37 AVD의 Vulkan 초기화 실패 및 ANGLE/SwiftShader GL 사용을 PR 본문과 근거에서 구분했다. |
| 13 | Android `ANativeWindow` 참조가 실패·성공·destroy 경로에서 누수되거나 너무 일찍 해제되는가 | JNI가 surface에서 참조를 얻고 생성 실패 때 즉시 해제한다. 성공 시 renderer destroy 뒤 release한다. renderer 수명 중 native window 참조를 보유한다. |
| 14 | surface가 바뀌는 동안 오래된 renderer 생성 완료가 새 generation에 채택되는가 | Android reconciler는 generation·surface availability·host를 재검사하고 stale 결과를 파괴한 뒤 최신 상태를 다시 조정한다. surface 변화는 presentation sequence도 먼저 무효화한다. |
| 15 | Android `surfaceDestroyed`가 GPU가 surface를 놓기 전에 반환하는가 | render lane을 닫고 barrier drain을 기다린다. 대기 중 상태 잠금을 보유하지 않고 실제 대기 시간을 기록한다. API37 lifecycle 로그와 종료 시험을 근거로 연결했다. |
| 16 | Android Activity 종료에서 runtime, renderer, native window, host의 해제 순서가 뒤바뀌거나 UI 대기가 길어지는가 | runtime lane drain 이후 render lane에서 renderer를 파괴하고 host를 해제한다. renderer 파괴 뒤 ANativeWindow를 놓는다. 현재 demo `dispose`는 Activity UI thread에서 두 executor 종료를 동기 대기한다. 고정 fixture만 실행하지만 V8에는 취소·실행 시간 상한이 없어 임의 JS 연결 시 종료가 지연될 수 있음을 계약·계획·PR 본문에 제한으로 명시했다. |
| 17 | iOS background thread가 UIView/CAMetalLayer를 만지거나 UIKit pointer가 조기 해제되는가 | UIKit surface 준비·configure는 main queue precondition을 둔다. 장치 초기화와 draw는 전용 serial render queue다. view는 shutdown drain closure가 surface 정리를 끝낼 때까지 유지된다. |
| 18 | iOS resize 중 이전 draw가 새 drawable 크기를 쓰거나 연속 크기 변화가 유실되는가 | resize 전 render queue barrier를 통과한 다음 main queue에서 surface를 구성한다. generation이 바뀌면 이전 처리를 버리고 최신 `lastDrawableSize`로 재시도한다. iOS 26.2 resize 왕복 로그·화면을 대조했다. |
| 19 | coalescing이 의미 보존이 필요한 JS/DOM 작업까지 버리거나 lane 실패 후 영구 정지시키는가 | latest-wins는 환경·표시·surface 재조정에만 적용하고 JS/DOM 명령에는 적용하지 않는다. Java/Swift lane 시험은 단일 생산자 burst, 동시 생산자, pending close, 작업 오류, executor 거부 뒤 복구를 다룬다. |
| 20 | PR 설명이 시뮬레이터 근거를 실기기·성능·제품 CSS 지원으로 과장하거나 필수 제출 자료를 빠뜨리는가 | PR 본문에 Android/iOS 시뮬레이터 캡처 4장을 붙였다. 실기기·하드웨어 성능 미검증과 미지원 CSS 범위를 명시했고 Tailnet URL은 넣지 않았다. PR 라벨은 저장소의 유형·영역 규칙에 맞다. GitHub는 이 브랜치에 자동 검사 결과를 보고하지 않아 로컬 테스트·빌드 결과만 검증 근거로 썼다. |

검토에서 차단 결함은 발견하지 못했다. 이 검토는 실제 Android 기기·실제 GPU 드라이버·성능, 전체 CSS/브라우저 적합성 또는 장시간 JS 취소를 입증하지 않는다. 병합 후 `spec/STATUS.md`의 완료 표시는 병합 상태와 맞춰 갱신한다.
