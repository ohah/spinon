# S03.3 · 대량 V8 wrapper registry 시뮬레이터 검증

## 실행 환경

- 실행 날짜: 2026-10-05
- Spinon 기준 commit: `747a3796b182a1f5e6bc04c5b5209727231b2a85` 및 이 PR의 미커밋 변경
- V8 revision: `7b50b62cb18f28617959e8452e2cd18195b38bcf`
- 호스트: Apple Silicon macOS arm64
- Android: Android 16 / API 36 ARM64 에뮬레이터 (`emulator-5554`)
- iOS: iPhone 17 Pro / iOS 26.2 시뮬레이터 (`ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`)
- Android의 V8 `gn` 호스트 실행 파일은 x86_64 Mach-O라 Rosetta에서 실행했다. `target_os = "android"`, `target_cpu = "arm64"`이며 Rosetta가 앱 타깃을 바꾸지는 않는다.
- Android 에뮬레이터의 이전 V8 artifact는 `autib1716`에서 `SIGILL`로 중단됐다. 재현 fixture만 실행하기 위해 생성된 `out/boson-android-mac/args.gn`에 `arm_control_flow_integrity = "none"`을 임시 추가해 V8을 다시 빌드했다. 추적 파일 `tools/v8/android-v8.args.gn`은 변경하지 않았다. 따라서 이 Android 결과는 해당 에뮬레이터용 CFI 비활성화 빌드의 동작이며 일반 배포 빌드나 실기기 결과로 확대하지 않는다.
- Android·iOS V8 생성 설정 모두 `v8_jitless = false`였다. 이는 설정값 기록이며 이 scan 관측을 엔진 성능 벤치마크로 만들지는 않는다.
- 검증 전용 `SPINON_ENABLE_S03_DOM_GC_FIXTURE=1` 빌드에서 강제 GC hook을 사용했다.

## 입력과 실행

고정 fixture가 HostRoot 아래 container 하나와 text node 16,384개를 만들고, 전체 16,385개 wrapper를 실제 V8 JavaScript에서 조회해 강한 배열과 양 끝 `WeakRef`로 보유했다. 각 text node 생성과 append는 256개 작업 상한에 맞춰 128개 노드 단위로 제출했다. 보존 후 wrapper 배열과 container 참조를 놓고 부모를 제거한 다음 GC와 owner safe-point scan을 요청했다. 양 끝 `WeakRef` 소멸, 16,385개 빈 weak Global, Rust 문서 자원 기준선 복귀를 검사했다. 이후 기존 6×32 반복 수명 회수 시나리오도 이어서 실행했다.

```sh
SPINON_ANDROID_EMULATOR_SERIAL=emulator-5554 \
SPINON_IOS_SIMULATOR_UDID=ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D \
bash tools/verify-s03-dom-lifecycle-simulators.sh
```

## 결과

| 항목 | Android | iOS |
| --- | ---: | ---: |
| wrapper 보존 후 Rust 노드 / live wrapper | `16,395 / 16,394` | `16,395 / 16,394` |
| 대량 scan과 Rust callback 시간 | `15,273 μs` | `7,067 μs` |
| wrapper root vector payload capacity | `131,072 B` | `131,072 B` |
| 회수 결과 vector payload capacity | `65,580 B` | `65,580 B` |
| 제거 뒤 비어 있던 wrapper 수 | `16,385` | `16,385` |
| 최종 Rust 노드 / UTF-16 단위 / live wrapper | `10 / 226 / 9` | `10 / 226 / 9` |
| collector 오류·poison·deferred, `scanned = live + empty` | 없음, 계수 일치 | 없음, 계수 일치 |
| 대량 경로 이후 6회 반복 기준선 복귀 | 통과 | 통과 |

scan 시간은 한 번씩 측정한 시뮬레이터 관측값이다. 반복 분포, 프레임 시간, 실기기 비용을 나타내지 않는다. buffer byte는 `std::vector` 원소 payload의 capacity만 계산하며 `std::map` node, allocator overhead, V8 heap, Rust 저장소, 전체 프로세스 RSS는 포함하지 않는다. vector capacity는 축소하지 않으므로 대량 회수 뒤에도 peak 용량이 isolate 종료까지 유지된다.

## 실패 경로 검토 결과

- 고정 resident-node/wrapper 개수만 제거했다. 한 batch 최대 256개 작업, 활성 external root 16,384개, UTF-16 문자열 budget, 양수 i32 ID 공간 검사는 별도 계약으로 유지된다.
- Rust FFI의 결과 buffer는 회수 후보 수가 아니라 현재 전체 문서 노드 수를 수용해야 한다. 부족 응답은 root가 전부 live인 경우와 root가 없는 sweep 경우에 각각 snapshot 불변 후 재시도하도록 테스트한다.
- C++ scan은 `scanned = live + empty`를 검사하며 live wrapper 조기 회수, 실제 wrapper 수와 보고 count 불일치, 최종 Rust resource baseline 미복귀는 두 플랫폼의 전체 PASS 조건을 깨뜨린다.
- `std::bad_alloc` 주입과 Rust 표준 트리 allocator의 OS 메모리 고갈 복구는 검증하지 않았다. 앱·세션 복구 보장으로 확대하지 않는다.
- Android·iOS 기본 설정(`SPINON_ENABLE_S03_DOM_GC_FIXTURE=0`)의 앱 빌드도 각각 성공했다. 이 빌드에는 검증용 강제 GC hook을 등록하지 않는다.

## 재현 자료

- Android [원본 로그](s03-dynamic-registry-android-2026-10-05.log) · [시뮬레이터 화면](s03-dynamic-registry-android-2026-10-05.png)
- iOS [원본 로그](s03-dynamic-registry-ios-2026-10-05.log) · [시뮬레이터 화면](s03-dynamic-registry-ios-2026-10-05.png)

## 판정 범위와 남은 한계

이 결과는 고정 V8 revision의 테스트 전용 앱에서 이전 16,384개 상한을 넘는 약한 wrapper registry의 생성·root scan·동적 회수 결과 buffer 확장·sweep·자원 기준선 복귀를 확인한다. allocation failure 주입, 더 큰 규모의 비용 곡선, 장기 반복, V8 heap retaining path, 실제 메모리/RSS, 실기기, DOM listener와 external root lease 수명은 검증하지 않았다. Android는 위에 적은 CFI 비활성화 V8 artifact를 사용했다. 따라서 S03.3과 J10은 미완료다.
