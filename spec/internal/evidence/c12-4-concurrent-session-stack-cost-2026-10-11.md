# C12.4 동시 session worker stack 비용 관측

**계획:** [동시 session 검증 계획](../../../plan/c12-4-concurrent-session-stack-cost.md) · [계획 실패 관점 검토](./c12-4-concurrent-session-stack-cost-plan-review-2026-10-11.md) · [구현 실패 관점 검토](./c12-4-concurrent-session-stack-cost-implementation-review-2026-10-11.md)  
**판정:** iOS Simulator와 Android AVD에서 C12.4 session 0·1·2·4·8개를 생성·유지·해제했다. process memory 변화는 CSS worker stack에 귀속하지 않는다. worker별 pthread bounds/mincore 및 Android TID VMA 표본은 별도 [후속 결과](./c12-4-worker-stack-residency-2026-10-11/README.md)에 추가했다.

## 측정 방법

- 각 시뮬레이터는 앱을 새 프로세스로 시작했다. GPU surface와 renderer 없이 C12.4 `RuntimeGpuHost`만 0→1→2→4→8개 누적 생성했다.
- 각 단계는 8초 유지했고 process memory를 세 번 수집했다. 마지막에는 8개 handle을 해제한 뒤 10초 구간에서 추가 샘플을 세 번 읽었다.
- session마다 CSS cascade worker가 8 MiB stack을 요청한다. 이 설정의 합은 8개일 때 64 MiB다. 이 값은 virtual stack request/reservation 구성값이며 resident/committed memory 측정값이 아니다.
- **iOS:** iPhone 17 Pro / iOS 26.2 Simulator, Xcode 26.2. Apple `footprint` 결과의 `phys_footprint`와 process `stack` category를 보존했다.
- **Android:** `emulator-5554`, Android API 37 / Android 17, `dumpsys meminfo`의 process PSS/RSS와 `/proc/<pid>/status`의 VmSize·VmRSS·Threads를 보존했다.

## 단계별 결과

### iOS Simulator

세 샘플의 `phys_footprint`는 각 단계에서 동일하게 반올림됐다. footprint `stack` category는 process 전체 stack의 현재 dirty pages이며 CSS cascade worker만을 분리하지 않는다.

| 동시 session | CSS stack 요청 합계 | `phys_footprint` | process `stack` dirty |
| ---: | ---: | ---: | ---: |
| 0 | 0 MiB | 16 MB | 736 KiB |
| 1 | 8 MiB | 19 MB | 512 KiB |
| 2 | 16 MiB | 20 MB | 576 KiB |
| 4 | 32 MiB | 23 MB | 704 KiB |
| 8 | 64 MiB | 28 MB | 960 KiB |
| 0, 8개 해제 후 | 0 MiB 요청 | 26 MB | 448 KiB |

`footprint` 표본은 세 번 반복해도 안정됐다. macOS `vmmap -summary`는 Simulator process에 DYLD 정보를 읽지 못해 실패(exit 255, `os/kern` failure 5)했고 그 실패 출력을 보존했다. `footprint`는 동작했으므로 측정은 계속했다.

원본 실행: [iOS Simulator run](./c12-4-concurrent-session-stack-cost-2026-10-11/c124-ios-20261011T112445474Z-030fa687/). `device.txt`, `unified.log`, 단계별 `footprint`/`vmmap` 출력과 `metadata.json`이 있다.

### Android API 37 AVD

수치는 Android 도구가 출력한 KiB 단위 그대로 기록했다. 단계별 세 샘플 간 PSS/RSS 차이는 작고 안정적이었다.

| 동시 session | CSS stack 요청 합계 | PSS (KiB) | RSS (KiB) | VmSize (KiB) | process threads |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 0 MiB | 53,046 | 179,888 | 16,633,968 | 23 |
| 1 | 8 MiB | 64,832 | 192,048 | 34,035,264 | 28 |
| 2 | 16 MiB | 66,144 | 193,296 | 34,604,272 | 30 |
| 4 | 32 MiB | 68,464 | 195,616 | 35,742,288 | 34 |
| 8 | 64 MiB | 72,992 | 200,144 | 38,018,320 | 42 |
| 0, 8개 해제 후 | 0 MiB 요청 | 69,808 | 196,960 | 33,472,336 | 26 |

`VmSize`는 물리 메모리가 아니다. 첫 runtime host 생성에서 virtual size가 약 16.6 GiB 증가했고 이후 host마다 약 555.7 MiB씩 더 증가하는 관측을 했다. 8개를 해제한 뒤에도 baseline보다 약 16.1 GiB 높았다. 앱의 `/proc/<pid>/maps`와 `smaps_rollup`은 권한 거부로 읽지 못했다. 따라서 이 큰 virtual reservation의 매핑 소유자와 원인은 미확정이며 V8, stack, 기타 allocator 중 어느 것에도 귀속하지 않는다.

PSS/RSS는 0→8 session에서 각각 약 19.5 MiB, 19.8 MiB 증가했고 전부 해제한 10초 뒤 baseline 대비 약 16.4 MiB, 16.7 MiB 높았다. 이는 앱 process 전체 관측이다. V8/Java heap, runtime 및 allocator cache가 포함되며 누수 판정·CSS worker 실제 stack commit·장기 회수율로 해석할 수 없다.

원본 실행: [Android API 37 AVD run](./c12-4-concurrent-session-stack-cost-2026-10-11/c124-android-20261011T112346486Z-d2d8f9f0/). `logcat.txt`, stage별 `dumpsys-meminfo`, `/proc status`, 접근 거부 `maps`/`smaps_rollup`, 기기 정보와 `metadata.json`이 있다.

## worker별 후속 측정

초기 process-level 실행에서는 shell UID가 `/proc` 접근을 거부해 thread별 내용을 읽지 못했다. 이후 AVD package UID의 `run-as` 경로로 `/proc/<pid>/task/<tid>/maps`와 `smaps`를 수집했고, iOS worker 자신이 pthread bounds와 page 정보를 기록하도록 내부 전용 probe를 추가했다. 이번 결과에서 Android/iOS 모두 0·1·2·4·8 session별 실제 worker 수와 3회 bounds 표본을 확인했다. [모든 실행 원본과 metric 한계](./c12-4-worker-stack-residency-2026-10-11/README.md) · [구현 검토](./c12-4-worker-stack-residency-implementation-review-2026-10-11.md).

## 관측된 구현 비용과 남은 경계

- 실제 worker stack 설정은 session마다 8 MiB 요청이므로 8개 session에서는 CSS cascade worker용 요청 합계가 64 MiB다. OS별 guard page/virtual mapping 정책과 committed pages는 직접 읽지 못했다.
- 양쪽 process memory는 동시 session에 따라 증가했지만 increase per session이 일정하지 않다. 이 표본은 최대 동시 session 수를 제안하거나 성능 벤치마크를 대신하지 않는다.
- Android AVD의 PSS/RSS는 실기기 값이 아니며 ANGLE/SwiftShader GPU 성능과도 관계없다. iOS Simulator footprint 역시 iPhone physical footprint가 아니다.
- mincore 순간 page residency와 Android `stack_and_tls` VMA RSS/PSS는 별도 수집했으나, TLS와 경계 page가 포함되므로 stack-only commit/high-water를 판정하지 않는다. Android 실기기 RSS, 장기 생성·해제 반복의 회수 안정성, global allocator OOM recovery도 미측정이다.
- 이 실행으로 C12.4나 CSS 전체 지원을 완료 처리하지 않는다.

