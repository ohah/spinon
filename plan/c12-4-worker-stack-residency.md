# C12.4 · CSS worker stack 상주 페이지 귀속 계획

**상위 계획:** [C12.4 쌓임 맥락](c12-4-stacking-context.md) · [동시 session process 메모리 측정](c12-4-concurrent-session-stack-cost.md)  
**계획 검토:** [서로 다른 20개 실패 관점](../spec/internal/evidence/c12-4-worker-stack-residency-plan-review-2026-10-11.md)  
**상태:** Android API 37 AVD·iOS 26.2 Simulator worker stack 표본 완료 · 실제 기기, stack high-water mark, per-worker commit 판정은 미측정
**목적:** process-wide PSS/RSS/footprint에서 분리해 실제 C12.4 CSS cascade worker stack의 OS 보고 범위와 현재 resident page 수를 시뮬레이터에서 관측한다.

## 측정할 값과 말하지 않을 값

- `requested_stack_bytes`: `CSS_CASCADE_WORKER_STACK_SIZE_BYTES`가 worker 생성 시 요청한 크기다.
- `pthread_reported_stack_bytes`와 `stack_guard_bytes`는 pthread API의 원본 범위와 guard 크기다. Android Bionic은 `pthread_getattr_np` 범위에 guard를 포함하므로 guard를 뺀 범위를 별도 보존한다. iOS API는 guard를 0으로 보고했다.
- `requested_stack_bytes`는 런타임이 요청한 8 MiB이고, `os_stack_range_bytes`는 guard를 반영한 pthread 범위다. 두 값이 동일하다고 가정하지 않는다. 이번 AVD 관측은 8,396,736 bytes, iOS Simulator 관측은 8,400,896 bytes였다.
- `mincore_resident_pages`는 stack 범위와 겹치는 정렬된 page 표본에서 OS가 순간 resident라고 표시한 페이지 수다. 시작의 partial page는 제외하고 끝 page는 포함한다. 끝 page에는 stack 범위 바깥의 8,256 bytes(Android) 또는 4,096 bytes(iOS)가 들어가므로 resident page를 stack 전용 물리 페이지라고 해석하지 않는다.
- Android `/proc/<pid>/task/<tid>/maps`·`smaps`는 같은 앱 UID의 `run-as`로 실제 TID 이름이 붙은 VMA를 대조했다. 이 VMA는 `stack_and_tls`라 TLS도 포함한다. 8개 worker 각각 RSS/PSS 16 KiB였지만 이를 stack 단독 비용으로 부르지 않는다.
- iOS는 앱 내부 `mincore`와 pthread stack bounds를 사용했다. `footprint`는 process 전체 stack category만 제공하므로 worker별 RSS/PSS는 분리하지 않는다. AVD/Simulator 결과를 실기기 값으로 대체하지 않는다.
- 이 값들은 worker의 committed memory, private stack RSS/PSS, 최대 stack 사용량, high-water mark 또는 안전 여유량이 아니다.

## 대상과 격리

- Android API 37 `emulator-5554` AVD와 iPhone 17 Pro / iOS 26.2 Simulator만 사용한다. 현재 사용자 지시에 따라 Android 실기기는 연결하지 않는다.
- 측정 대상은 C12.4 profile로 만든 실제 `RuntimeSession`의 CSS cascade worker다. V8 worker, UI thread, render thread를 CSS worker 값에 섞지 않는다.
- GPU surface와 renderer를 만들지 않는다. 메모리 probe 오류는 CSS 계산·session 종료 결과를 바꾸지 않는 진단 오류로 분리한다.
- 진단은 기존 앱의 내부 테스트 인수로만 활성화한다. 사용자 JS API, 공개 FFI 계약, 기본 UI/실행 경로에는 추가하지 않는다.

## 수집 방식

1. CSS worker 자신이 시작할 때 Android Bionic `pthread_getattr_np`/`pthread_attr_getstack`/`pthread_attr_getguardsize` 또는 Apple `pthread_get_stackaddr_np`/`pthread_get_stacksize_np`로 TID와 bounds를 얻는다. Android guard를 제외하고 exclusive `[start, end)` 범위를 만든다. page alignment를 pthread API 계약으로 가정하지 않는다.
2. Rust coordinator는 worker가 게시한 bounds를 진단 호출에서 읽는다. `mincore` 표본은 같은 앱 프로세스의 coordinator 경로에서 수행한다. page size를 런타임 조회하고 page 시작·끝을 checked 산술로 정렬한다. stack 끝 경계 page와 stack 바깥에 포함되는 tail byte를 명시한다.
3. resident vector는 stack 범위 밖의 제한된 heap buffer에 둔다. 요청 크기, pthread 원본 범위, guard, 조정된 범위, 시스템 page size, mincore page span, resident bit count, `mincore` errno를 기록한다. `mincore` 실패를 0 resident로 바꾸거나 process PSS/RSS로 대체하지 않는다.
4. Android 수집기는 AVD package UID로 `/proc/<pid>/task/<tid>/maps`와 해당 주소 범위의 `smaps`를 읽는다. CSS worker TID 및 mincore page span이 같은 `stack_and_tls:<tid>` VMA에 포함되는지 확인한다. 이 VMA의 RSS/PSS에는 TLS가 포함됨을 유지한다.
5. 각 플랫폼 앱을 fresh process로 실행하고 session 0·1·2·4·8 단계를 만든다. 모든 live worker의 stack bounds를 수집한 뒤 단계당 세 번 `mincore` sample을 취한다. 8개 해제 후 probe log의 종료·thread join과 Android VMA 부재를 대조한다. iOS에서는 해제·종료 marker를 확인하되 per-TID VMA 제거를 외부에서 확인했다고 주장하지 않는다.
6. 각 record에 run ID, app PID, session ID, TID, thread role, OS/API build, page size, requested size, OS range, sample timestamp, resident page count, 오류 상태와 Android 매핑 cross-check를 남긴다. 원시 주소는 로컬 검증 증거에만 보존한다. GitHub 저장소와 Tailscale Funnel은 인터넷에서 접근할 수 있으므로 주소 원본을 올리지 않고 요약값만 공개한다.

## 실패 검증과 완료 기준

- 비교기 negative control은 누락·중복 session/TID, 오래된 run ID, 다른 PID/platform/page size, `start >= end`, 범위 overflow, resident pages 초과, 잘못된 metric 이름, 오류를 0으로 위장한 결과를 거부한다.
- `mincore` 권한/주소 실패와 pthread bounds 실패를 앱 crash나 성공 값으로 바꾸지 않는다. 실패 표본은 platform별 measurement gate 실패로 남기고 일반 C12.4 기능의 pass/fail과 분리한다.
- [x] Android 및 iOS Simulator 0·1·2·4·8 단계에서 실제 worker 수, session별 고유 TID, 3회 안정된 bounds를 확인했다.
- [x] Android `run-as`로 동일 TID의 `maps`·`smaps`를 읽어 mincore sample span과 VMA 범위를 대조하고, 해제 뒤 worker VMA 부재를 확인했다.
- [x] iOS Simulator에서 실제 pthread bounds와 mincore 결과를 세 번 수집하고 8개 해제 뒤 `freed`·`finished failed=0`를 확인했다.
- [x] `cargo test --locked --workspace`, worker probe feature test, workspace Clippy, rustfmt, JS 및 CSS reference 테스트와 Android/iOS Simulator app build/run을 실행했다.
- [x] Android page alignment, Bionic guard 범위, end boundary page, mincore page-span 길이 계산의 실패를 수집 로그에서 재현해 수정했다.
- [ ] Spinon 공식 WPT 적합성, Android 실기기 RSS/GPU, iOS per-worker RSS/commit, global allocator OOM 복구는 이 계측 결과의 완료 범위가 아니다.
- 결과는 순간 resident page 표본과 OS stack range attribution에 한정한다. worker의 전체 CPU stack high-water mark, OS commitment, 실기기 RSS, 누수, 최대 session 수, hardware GPU 또는 전체 CSS 지원으로 일반화하지 않는다.
- 이 표본은 `spec/internal/evidence/c12-4-worker-stack-residency-2026-10-11/README.md`에 기록했다. 20개 구현 실패 관점 검토는 `spec/internal/evidence/c12-4-worker-stack-residency-implementation-review-2026-10-11.md`에 기록한다.
- 완료해도 C12.4 전체를 체크하지 않는다. 실제 Android 기기·Spinon 공식 WPT·global allocator OOM 복구 등 다른 게이트를 별도로 유지한다.

## API 근거

- [Android Bionic `pthread_getattr_np`와 stack 속성 구현](https://android.googlesource.com/platform/bionic/+/2a1bb4e64677b9abbc17173c79768ed494565047/libc/bionic/pthread_attr.cpp)
- [Apple libpthread stack 주소·크기 API 선언](https://github.com/apple-oss-distributions/libpthread/blob/main/include/pthread/pthread.h)
- [Apple `mincore(2)` page residency 설명](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/mincore.2.html)
- [Linux `mincore(2)` page residency 설명](https://man7.org/linux/man-pages/man2/mincore.2.html)
