# S03.3 pilot observability · 구현 후 독립 20관점 검토

**검토 대상:** `af6b323c` 이후 Android/iOS fixture 출력 및 S03 simulator runner 변경
**비교 모델:** [구현 전 pilot 모델](./s03-scan-pilot-observability-precomparison-2026-10-26/README.md)
**선행 계획:** [pilot observability slice](../../../plan/s03-3-pilot-scan-observability.md)
**계획 SHA-256:** `f0a86deca3ae7990a730c720c034c53f712df35d5973e10bebb6e93a6ea423da`
**검토 코드 diff SHA-256:** `8fb7825478ac12d1c7cbb5dcc936ab5a9318f290aae8f5f38ff5a84efcfe5326`
**검토 범위:** 실제 변경 코드·verification runner·기본 및 opt-in app artifact. 시뮬레이터 실행은 의도적으로 하지 않아 runtime log의 실제 표본 값은 아직 관찰하지 않았다.

계획 단계 검토와 별도로 구현 diff를 다시 읽고 아래의 독립 실패 경로를 확인했다. 발견한 누락은 구현 후 검토를 닫기 전에 보완했다.

| # | 관점 | 확인한 실패 경로 | 확인 결과 또는 통제 |
|---:|---|---|---|
| 1 | Android 기본 artifact 격리 | default `BuildConfig`에서 계측 문자열이나 formatter가 제품/debug 기본 경로에 남는가? | `SPINON_S03_DOM_GC_FIXTURE=false` 빌드의 DEX에서 `scan_sample_index`와 observability marker가 없음을 확인했다. 새 출력은 생성 BuildConfig 상수의 true branch 안에 둔다. |
| 2 | Android opt-in positive control | fixture flag를 켜도 Java 컴파일러가 output block을 제거하는가? | S03=true 빌드의 `BuildConfig`가 true이고 APK DEX에 sample index/records/observability key가 있음을 확인했다. |
| 3 | iOS compile-condition 격리 | S03=false 빌드에 Swift 문자열/파싱 코드가 포함되는가? | `_0` build의 `.app` executable에서 sample marker가 없음을 확인했다. 새 Swift 블록은 `SPINON_ENABLE_S03_DOM_GC_FIXTURE_1` 조건으로 제한했다. |
| 4 | iOS opt-in positive control | S03=true에서 조건부 Swift block이 빠지거나 심볼이 연결되지 않는가? | `_1` Simulator build가 성공했고 `.app` executable에 `scan_sample_index`, `S03_SCAN_OBSERVABILITY`, `sample_records`가 있다. |
| 5 | 기존 시간 출처 | 새 출력이 별도 타이머를 더하거나 기존 duration 의미를 바꾸는가? | 두 플랫폼은 report의 기존 `document_collection_last_scan_us` 값을 그대로 사용한다. C++ timer와 report struct를 바꾸지 않았다. |
| 6 | wall interval 경계 | `scan_wall_us`가 Rust phase나 pure CPU time으로 오해될 수 있는가? | field를 전체 safe-point collector wall-duration alias로 기록하고, 계획/precomparison에서 포함 단계와 CPU/scheduler 구분을 명시한다. |
| 7 | TID 정의 | caller/callback ID를 scan owner ID로 잘못 기록할 수 있는가? | report의 `owner_tid`를 직접 읽고 callback/caller 필드로 대체하지 않는다. Android/iOS 구현은 actor의 V8 owner safe-point와 같은 thread 경로를 유지한다. OS runnable-state 관측은 주장하지 않는다. |
| 8 | monotonic clock 상관 | `steady_clock` 시작값을 OS trace timestamp 또는 wall clock으로 합칠 수 있는가? | 기존 `document_collection_last_scan_start_ns`를 동일 값으로 남기고 `scan_start_ns` 이름/문서에 clock domain 제한을 적었다. scheduler trace 미지원은 명시한다. |
| 9 | sample-to-operation join | 10회 표본이 다른 eval의 timer/TID와 섞일 수 있는가? | 같은 report에서 `seq`, `trace_cookie`, `owner_tid`, 시작 시각, duration을 읽어 sample index와 한 tuple로 기록한다. |
| 10 | 중도 실패와 count | 앞선 sample이 실패했는데 계획된 상수 10을 실제 표본 수처럼 표시할 수 있는가? | Android 실제 record count 및 iOS array count를 10과 대조한다. 반복 성공 조건은 기존 fixture가 계속 검사하고 runner는 실제 출력된 `scan_sample_index` 수를 확인한다. |
| 11 | 누락/파싱 실패 | 필드 누락이 0처럼 보여 완전한 관측으로 처리되는가? | Java/Swift 기존 parse helper의 `-1` sentinel을 그대로 검사한다. owner TID는 양수, 다른 숫자는 음수 sentinel이 아니어야 metadata PASS가 된다. 추가로 manifest 장치/tool 식별 정보가 비면 runner가 실패하도록 guard를 넣었다. |
| 12 | correctness와 metadata 결합 | 자료 수집 실패가 생존집합 correctness PASS로 위장되거나 반대로 algorithm 결과를 바꾸는가? | 앱의 기존 `dom_gc` PASS 계산은 변경하지 않았다. `scan_observability`는 별도 결과이고 simulator runner가 별도로 요구한다. 회수 경로는 변경하지 않았다. |
| 13 | record delimiter/parser | sample field 내 임의 문자열이 구분자를 깨뜨리는가? | tuple에는 report에서 추출한 정수만 두고 comma/semicolon을 구분자로 사용한다. report 문자열 전체나 사용자 입력은 기록하지 않는다. |
| 14 | Android log truncation | 10 tuple이 logcat 한 줄 한도를 넘어 뒤쪽이 잘릴 수 있는가? | tuple 길이는 정수 6개로 bounded되어 기존 10개 time 목록에 비례하는 작은 증가만 만든다. runner는 marker뿐 아니라 `scan_sample_index` 10개를 요구한다. |
| 15 | iOS OSLog 누락 | `appendReport` 화면 값이 runner가 읽는 unified log에 나오지 않는가? | `appendReport`는 기존 `SPINON_R06_IOS` logger를 사용하며 runner가 같은 predicate로 수집한다. 별도 logger/category를 만들지 않았다. simulator 실행으로 최종 캡처를 확인하지 않은 점은 남은 제한이다. |
| 16 | source dirty 상태 | output 생성물이 dirty source/hash로 오인되어 manifest가 재귀적으로 달라지는가? | commit/diff/untracked digest 계산을 output dir 생성보다 앞에 둔다. tracked diff와 untracked 파일 digest를 분리하고 `source_dirty`를 함께 쓴다. |
| 17 | artifact tree hashing | iOS `.app`의 상대 경로·symlink·실행 모드가 빠져 artifact fingerprint가 약해지는가? | app 디렉터리 내부 상대경로를 정렬해 file content+mode 및 symlink target을 합산한다. Android APK는 단일 파일 SHA-256, 두 플랫폼 build log도 따로 hash한다. |
| 18 | manifest 덮어쓰기/누락 | 반복 실행이 이전 결과를 덮거나 선택한 device/tool 식별 없이 실행되는가? | 기존 고유 output-dir 존재 거부를 유지한다. AVD model/release/API/ABI, iOS device row, Xcode/adb/mise/Bun 값이 비면 생성 전에 중단한다. |
| 19 | 빌드와 장치 경계 | build 확인 중 root worktree V8 cache를 오염시키거나 booted simulator에 설치/실행하는가? | 고정 V8 checkout/artifact를 read-only source로 지정해 새 worktree의 outputs에만 build했다. V8 checkout status는 깨끗하다. 앱 build는 generic Simulator target이며 install/launch하지 않았다. |
| 20 | 근거 범위/성능 오인 | build 성공만으로 runtime record, scheduler, CPU, 성능 개선 또는 S03.3 완료를 주장할 수 있는가? | 기본/fixture build와 string boundary만 확인했다. 실 runtime sample·CPU·scheduler·effect threshold·N·tail·physical device·long-loop·allocation failure·listener cycle은 미검증으로 남기며 STATUS/roadmap를 변경하지 않는다. |

## 검토 결과

구현 범위는 계획과 일치하고 현재 diff에서 수정이 필요한 결함은 남기지 않았다. `bash -n` 및 `git diff --check`, Android/iOS Debug 기본·S03 opt-in build, 기본 artifact marker 부재와 opt-in marker 존재를 확인했다. `tools/verify-s03-dom-lifecycle-simulators.sh`는 Android logcat clear/force-stop과 iOS install/terminate를 하므로 booted AVD·Simulator 세션을 보존하기 위해 실행하지 않았다. 이에 따라 실제 10개 tuple 생성/로그 수집은 이 PR에서 입증하지 않는다.
