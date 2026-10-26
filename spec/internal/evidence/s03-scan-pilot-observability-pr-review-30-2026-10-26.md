# S03.3 pilot observability · PR diff 별도 30관점 검토

**기준 base:** `origin/main` `a1c29489`
**검토 범위:** base 대비 8개 구현/계획/review/evidence 파일(시뮬레이터 캡처 포함)과 이 30-view 기록 파일, 총 9개 PR 파일
**최종 slice 계획 SHA-256:** `cc074e3cbc93f7bf357b975da6ea1222549306bd5c1165472a3f30bfc02bd77f`
**구현 코드 diff SHA-256:** `ab628193529a3cdecee77fd145d61865f03b8de68c8fe1b0a67f63e0831490bd`
**구현 후 20관점 review:** [별도 기록](./s03-scan-pilot-observability-implementation-review-20-2026-10-26.md)

구현 전 비교 모델과 구현 후 20관점 review를 다시 사용하지 않고, PR 전체 변경에서 별도의 실패 경로와 경계 30개를 검토했다. 검토 뒤 이 파일 자체의 링크·검토 범위·결론도 다시 확인했다.

| # | 관점 | 확인한 실패 경로 | 확인 결과 또는 조치 |
|---:|---|---|---|
| 1 | 사용자 동작 범위 | fixture 관측이 일반 앱의 사용자 기능/API로 노출되는가? | 출력 코드는 S03 fixture flag 경로에 있고 공개 API나 일반 제품 동작은 추가하지 않았다. |
| 2 | 공식 완료 상태 | pilot 로그 수집을 S03.3 또는 roadmap 완료로 오인할 수 있는가? | `STATUS.md`와 public roadmap는 변경하지 않았다. PR 설명에 S03.3 완료가 아니라고 명시한다. |
| 3 | 계획 범위 | CPU·scheduler·release cost·최적화 구현까지 한 slice에 몰리는가? | 기존 wall time/TID/start tuple과 provenance만 기록한다. unresolved threshold, N, tail, device profile은 그대로 둔다. |
| 4 | 비교 oracle | 표본 기록을 위해 16,385-node 입력, 동일 Isolate 또는 safe-point 반복을 바꾸는가? | Android/iOS의 기존 S03 fixture, 10회 반복과 correctness oracle를 그대로 사용한다. |
| 5 | duration 의미 | `scan_wall_us`를 Rust phase나 pure CPU 시간으로 부를 위험이 있는가? | 기존 `document_collection_last_scan_us`를 그대로 투영하고 전체 safe-point collector wall duration이라고 적었다. |
| 6 | 계측 영향 | 새 타이머가 동작 경계를 바꾸거나 측정 오버헤드를 성능 개선처럼 보이게 하는가? | 새로운 clock/counter를 추가하지 않고 report 수치를 문자열로 읽는다. 성능 비교 주장은 없다. |
| 7 | 시작 시각 domain | `steady_clock` ns를 host wall clock 또는 OS trace timestamp로 결합하는가? | `scan_start_ns`는 process/run 내부 값으로만 둔다. cross-process/reboot correlation과 scheduler trace를 주장하지 않는다. |
| 8 | operation 상관 키 | 한 scan의 duration과 다른 operation의 TID/cookie를 합칠 수 있는가? | 매 report에서 `seq`, `trace_cookie`, `owner_tid`, start, duration을 한 tuple로 복사한다. |
| 9 | 실행 thread 정의 | callback TID를 safe-point owner TID로 잘못 기록하는가? | `owner_tid` 필드를 직접 읽고 callback TID와 혼용하지 않는다. 같은 V8 owner safe-point 경계를 유지한다. |
| 10 | 필드 parse 실패 | report에서 누락된 숫자가 0으로 해석돼 metadata PASS가 되는가? | 기존 parser sentinel `-1`을 그대로 확인하며 TID는 양수, 나머지 numeric field는 음수가 아니어야 PASS다. |
| 11 | 실제 sample 수 | loop 중간 실패를 계획값 10개로 보고할 수 있는가? | 플랫폼별 실제 record count를 기존 반복 상수와 대조하고 iOS summary에도 array count를 기록한다. |
| 12 | 인덱스 sequence | 누락, 중복, 역순 sample index가 단순 occurrence count로 통과하는가? | verifier가 정확한 index 1..10 sequence를 검사한다. positive 10건과 synthetic 9건 negative를 self-test하고 negative exit 1을 확인했다. |
| 13 | Android serialization | delimiter 또는 mutable builder 상태로 tuple 경계가 모호해지는가? | 필드는 정수만 포함하고 comma/semicolon을 사용한다. builder는 한 번의 fixture 실행 안에서만 유지된다. |
| 14 | iOS event 크기 | 10 tuples를 한 OSLog message에 붙여 마지막 record가 잘리는가? | 초기 C9에서 9/10 truncation을 확인했다. 짧은 summary와 tuple별 `appendReport` event로 나눴고 최종 실행에서 10개 event를 확인했다. |
| 15 | Android log 한도 | logcat record가 잘려도 fixture가 PASS로 보고되는가? | compact integer tuple을 기존 diagnostic output에 두고, verifier가 결과 log에서 index sequence까지 확인한다. Android runtime은 이번 환경에서 미검증으로 표기한다. |
| 16 | Android fixture opt-in | 기본 Android 앱에서도 새 formatter/로그가 활성화될 수 있는가? | `BuildConfig.SPINON_S03_DOM_GC_FIXTURE` true branch 안에서만 record를 만든다. 기존 off/on artifact 검사를 기록했다. |
| 17 | iOS fixture opt-in | 기본 iOS 앱에 sample record code가 남거나 호출되는가? | 새 Swift 수집·출력 block은 `SPINON_ENABLE_S03_DOM_GC_FIXTURE_1` compile condition으로 한정한다. 기존 off/on artifact 검사를 기록했다. |
| 18 | 기본 Android artifact | opt-in을 끈 APK DEX에 marker 문자열이 누출되는가? | `_0` DEX에서 sample marker가 없고 fixture-on DEX에는 marker가 있는 빌드 증거를 구현 review에 기록했다. |
| 19 | 기본 iOS artifact | opt-in을 끈 `.app`에 fixture marker가 남는가? | `_0` executable에는 marker가 없고 `_1` 앱에는 marker가 있는 빌드 증거를 구현 review에 기록했다. |
| 20 | 기존 correctness gate | observability status가 DOM lifecycle correctness를 대체하거나 바꾸는가? | 새 metadata PASS는 별도 marker이고 기존 `passed`, root/reclaim/baseline 판정은 그대로 둔다. |
| 21 | ABI·report contract | 관측 때문에 production report schema, Rust/C++ ABI 또는 V8 collector가 바뀌는가? | Java/Swift fixture output만 수정했다. Rust, C/C++ collector와 ABI는 diff에 없다. |
| 22 | Android 기기 선택 | `adb` 자동 감지가 물리 Android 기기나 다른 agent의 AVD를 선택하는가? | 허용 형식은 `emulator-<port>`이고, 지정 serial은 명시적으로 검증한다. 실제 실행은 Android 용량 문제로 중단했다. |
| 23 | iOS 기기 선택 | 기본 booted simulator 탐색이 잘못된 device에 install/terminate할 수 있는가? | 명시적 UDID가 booted 목록에 있는지 검증한다. 실제 실행은 새 clone `C9CD3A32-14FB-4013-A16D-4A029370DA3F`에만 했다. |
| 24 | log/app 파괴 범위 | 실행 중 기존 Android logcat clear, 앱 force-stop, iOS app terminate가 발생하는가? | 전체 runner의 대상 선택을 확인했고 task-owned 새 AVD/C9 clone만 지정했다. 기존 emulator-5554 및 booted iPhone은 건드리지 않았다. |
| 25 | 다른 작업 세션 | 검증 종료/복구가 다른 agent의 AVD·Simulator·Tailscale/tmux를 중지하는가? | C9 clone은 booted 상태로 보존하고 S03 app만 종료했다. 기존 시뮬레이터, tmux, Tailscale은 명령 대상으로 삼지 않았다. |
| 26 | manifest source identity | commit SHA만 남겨 dirty/staged/untracked 소스를 숨기는가? | source commit/branch/dirty flag, tracked diff hash와 untracked-source hash를 별도 기록한다. output dir 생성 전에 값을 산출한다. |
| 27 | manifest 환경 identity | 다른 V8·build task·OS toolchain·device로 실행한 결과를 같은 run처럼 취급하는가? | pinned V8 revision, fixture flag, platform task/config, macOS/Xcode/adb/mise/Bun, Android model/API/ABI 및 iOS UDID를 기록하고 필수 값이 비면 중단한다. |
| 28 | artifact hash | iOS bundle symlink/mode나 APK/build log가 source manifest에서 빠지는가? | Android APK SHA와 build log SHA, iOS app tree 파일/mode/symlink digest와 build log SHA를 기록한다. 이 hash를 release parity로 확대하지 않는다. |
| 29 | 실패 종료 코드 | 표본 gate 실패 뒤 EXIT trap이 전체 runner 종료 코드 1을 0으로 덮는가? | `cleanup`은 진입 때 `$?`를 저장해 반환한다. 9-record self-test는 `fail`과 trap 이후 exit 1인지 검사한다. |
| 30 | 증거·남은 범위 | iOS 단일 clone PASS를 Android/실기기/성능 또는 전체 S03.3 검증으로 확대하는가? | clean tested commit의 iOS clone에서 10/10 및 manifest를 확인했다. 첨부한 C9 화면에는 10개 중 마지막 표본 8–10의 새 metadata tuple(seq, cookie, owner TID, monotonic start, wall time)이 보인다. Android는 7,372.80 MB userdata 요구와 host 4.0 GiB free로 실행하지 않았고 S03.3·성능 완료를 주장하지 않는다. PR 본문은 iOS 화면과 Android 미검증 사유를 분리한다. |

## 결론

초기 실제 iOS 로그에서 확인한 OSLog truncation과 실패 exit-code masking은 수정 후 다시 검증했다. verifier self-test, iOS opt-in simulator build/runtime, 10개 표본 sequence와 source/build manifest는 확인했다. Android runtime 근거는 없으며 PR 본문에 그 제한을 명시한다. 첨부 PNG는 C9 화면에서 새 표본 metadata가 실제 표시되는 부분(8–10)을 보여 주며, 전체 10/10 판정은 별도 manifest/log 근거로 한정한다. 변경 범위와 리뷰 자료를 다시 대조했을 때 PR 개설을 막는 추가 결함은 남지 않았다. 병합은 하지 않는다.
