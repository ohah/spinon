# C12.4 worker stack residency 구현 검토

**대상:** Android API 37 AVD와 iOS 26.2 Simulator에서 C12.4 CSS cascade worker의 pthread 범위·mincore 표본을 수집하는 내부 진단 경로  
**판정:** 시뮬레이터 계측과 현재 범위의 회귀 검증 완료. thread별 stack commit/high-water, 실기기 수치, global allocator OOM, Spinon WPT 적합성은 증명하지 않는다.  
**실행 결과:** [요약 및 실행 ID](c12-4-worker-stack-residency-2026-10-11/README.md) · [계획](../../../plan/c12-4-worker-stack-residency.md)

## 실패 경로 검토

1. **Bionic의 guard 범위 포함:** `pthread_getattr_np` 범위를 실제 stack으로 바로 쓰면 TID VMA와 시작 주소가 어긋난다. Android는 `pthread_attr_getguardsize`를 읽고 guard를 제외한 범위를 검사한다.
2. **16 KiB page 정렬:** AVD의 page size는 16,384 bytes다. pthread 경계가 page aligned하다고 가정하지 않고 checked 산술로 sample page 범위를 계산한다.
3. **mincore 길이 단위:** stack과 겹치는 바이트 수를 page-aligned native API에 그대로 넘겨 `EINVAL`이 발생한 실행을 보존했다. mincore에는 정렬된 page-span 길이를 전달한다.
4. **끝 경계의 활성 page:** 끝을 내림 정렬하면 stack 끝의 현재 실행 page가 표본에서 빠지는 것을 확인했다. 시작은 올림, 끝은 올림하고 page-span 및 범위 밖 tail bytes를 각각 기록한다.
5. **부분 경계의 과대해석:** 마지막 resident page에는 stack 범위 밖 tail bytes가 들어간다. page 수를 stack 전용 물리 메모리로 환산하지 않고 tail 크기를 함께 보존한다.
6. **잘못된 pthread bounds:** 빈 범위, 역전, overflow, 0 page, 과도한 page 수, 현재 worker marker 바깥 범위를 거부한다.
7. **잘못된 guard 크기:** guard가 reported stack 길이 이상이면 안전한 오류로 반환한다. 조정 전·후 길이를 각기 JSON에 남긴다.
8. **분모/분자 오류:** resident page 수가 sample page 수보다 크거나 정렬 page span과 기록된 byte/page 수가 다르면 수집기에서 거부한다.
9. **mincore 오류 위장:** native mincore 실패는 진단 오류로 남기며 resident 0이나 process RSS로 바꾸지 않는다.
10. **과도한 임시 메모리:** stack 밖 `Vec`에 제한된 `try_reserve_exact`를 사용하고 실패를 record 오류로 처리한다. global allocator 복구로 일반화하지 않는다.
11. **누락 표본:** stage별 session당 1·2·3 표본과 전체 수를 확인한다. 빠진 stage/sample은 측정 실패다.
12. **중복 표본:** 같은 sample 번호가 반복되면 고정 `[1,2,3]` 검증에서 거부한다.
13. **session 잘못 귀속:** 각 단계의 session index 집합을 검증하고 하나라도 빠지면 실패한다.
14. **TID 재사용 오인:** 한 session의 세 표본에서 TID가 고정되고, 서로 다른 session 사이에서 같은 TID가 재사용되지 않는지 확인한다.
15. **bounds 흔들림:** 같은 live worker의 세 표본에서 reported/effective bounds와 page 계산이 유지되어야 한다.
16. **Android VMA 오귀속:** 앱 UID `run-as`로 `/proc/<pid>/task/<tid>/maps`와 `smaps`를 읽고, 정확한 TID 이름 및 mincore span 포함을 확인한다. login shell UID의 직접 `/proc` 실패는 worker 측정 실패로 취급하지 않는다.
17. **TLS 혼입:** Android VMA 이름은 `stack_and_tls:<tid>`다. RSS/PSS 16 KiB 표본을 stack-only RSS로 보고하지 않는다.
18. **해제 뒤 VMA 잔류:** Android 8-session 해제 후 관측한 CSS worker TID의 VMA가 사라졌는지 검사한다.
19. **iOS 경계:** iOS Simulator에서 worker별 pthread bounds와 mincore를 직접 수집한다. aggregate `footprint`/`vmmap`을 worker별 metric처럼 사용하지 않는다.
20. **검증 범위 확대:** 이 내부 feature는 기본 빌드 및 공개 JS/FFI 계약에서 제외된다. Simulator/AVD 결과를 실기기 GPU·RSS, 최대 stack use, 전체 CSS 또는 WPT 적합성으로 승격하지 않는다.

## 실행 확인

- Android API 37 AVD 및 iOS 26.2 Simulator에서 각각 session 0·1·2·4·8의 실제 worker를 관측했다. live worker마다 mincore를 세 번 표본화했고 실제 TID 수가 session 수와 일치했다.
- Android `run-as` TID별 VMA/`smaps` 교차검증과 해제 후 VMA 부재 확인을 마쳤다. iOS 8-session probe는 `freed` 이후 `finished failed=0`을 남겼다.
- `cargo test --locked --workspace`, worker probe feature tests, workspace Clippy `-D warnings`, rustfmt, JS tests, CSS reference tests 및 두 플랫폼 앱 빌드를 통과했다.
- 전용 parser negative controls는 누락·중복 표본, session 간 TID 재사용, bounds/page 불일치와 resident page 초과를 검사한다.

## 남은 한계

- `requested_stack_bytes`는 8 MiB다. pthread가 보고한 조정 범위는 Android 8,396,736 bytes, iOS 8,400,896 bytes였다. 이는 설정 요청 크기와 OS 범위가 같지 않음을 보여주는 관측이며 physical commit 크기는 아니다.
- Android의 각 worker `stack_and_tls` VMA는 RSS/PSS 16 KiB였고 mincore는 513개 16 KiB page 중 1개를 표본 순간 resident로 표시했다. 끝 boundary page가 stack 밖 tail과 TLS 범위를 일부 포함할 수 있어 이를 stack 전용 resident bytes라고 단정하지 않는다.
- iOS도 513개 16 KiB page 중 1개가 resident였지만 thread별 RSS/PSS나 stack high-water mark는 얻지 못했다.
- 실기기 stack/RSS, allocator-wide OOM 복구, 실제 하드웨어 GPU, 공식 WPT runner는 다른 검증 항목이다.
