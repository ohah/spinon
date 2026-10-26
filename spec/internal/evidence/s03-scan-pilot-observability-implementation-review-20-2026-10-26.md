# S03.3 pilot observability · 구현 후 독립 20관점 검토

**검토 대상:** `af6b323c` 이후 Android/iOS fixture 출력, S03 simulator runner와 iOS C9 재검증 후속 수정
**비교 모델:** [구현 전 pilot 모델](./s03-scan-pilot-observability-precomparison-2026-10-26/README.md)
**선행 계획:** [pilot observability slice](../../../plan/s03-3-pilot-scan-observability.md)
**계획 SHA-256:** `f0a86deca3ae7990a730c720c034c53f712df35d5973e10bebb6e93a6ea423da`
**검토 코드 diff SHA-256:** `ab628193529a3cdecee77fd145d61865f03b8de68c8fe1b0a67f63e0831490bd`
**검토 범위:** 실제 변경 코드·verification runner·기본 및 opt-in app artifact와 task-owned iOS clone runtime. 초기 캡처에서 앱 화면은 10개 표본을 보였지만 aggregate OSLog 이벤트에는 9개만 남았고 verifier EXIT trap이 실패 코드를 덮었다. 요약/표본별 event 분리, 정확한 순서 검사, exit-status 보존을 적용한 뒤 같은 clone에서 clean commit을 재빌드해 표본 1..10을 확인했다. Android runtime은 host disk 용량 부족으로 미검증이다.

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
| 10 | 중도 실패와 count | 앞선 sample이 실패했는데 계획된 상수 10을 실제 표본 수처럼 표시할 수 있는가? | iOS status가 실제 array count와 기존 반복 횟수의 일치도를 검사한다. verifier helper는 개수뿐 아니라 1..10 순서·중복 여부를 확인한다. self-test는 10개 positive와 9개 negative를 확인하고 cleanup trap 이후 negative exit 1을 검증했다. clean clone run에서도 10개 index 모두를 확인했다. |
| 11 | 누락/파싱 실패 | 필드 누락이 0처럼 보여 완전한 관측으로 처리되는가? | Java/Swift 기존 parse helper의 `-1` sentinel을 그대로 검사한다. owner TID는 양수, 다른 숫자는 음수 sentinel이 아니어야 metadata PASS가 된다. 추가로 manifest 장치/tool 식별 정보가 비면 runner가 실패하도록 guard를 넣었다. |
| 12 | correctness와 metadata 결합 | 자료 수집 실패가 생존집합 correctness PASS로 위장되거나 반대로 algorithm 결과를 바꾸는가? | 앱의 기존 `dom_gc` PASS 계산은 변경하지 않았다. `scan_observability`는 별도 결과이고 simulator runner가 별도로 요구한다. 회수 경로는 변경하지 않았다. |
| 13 | record delimiter/parser | sample field 내 임의 문자열이 구분자를 깨뜨리는가? | tuple에는 report에서 추출한 정수만 두고 comma/semicolon을 구분자로 사용한다. report 문자열 전체나 사용자 입력은 기록하지 않는다. |
| 14 | Android log truncation | 10 tuple이 logcat 한 줄 한도를 넘어 뒤쪽이 잘릴 수 있는가? | tuple은 정수 6개로 제한한다. simulator runtime은 host disk 문제로 아직 관찰하지 못했으며, verifier는 실제 capture에서 1..10이 모두 연속하는지 확인하도록 강화했다. |
| 15 | iOS OSLog 한 이벤트 크기 | 화면의 10개 값 중 unified log 이벤트의 끝 부분이 잘려 표본이 빠질 수 있는가? | 초기 C9 실행에서 화면은 sample 1..10을 보였지만 aggregate OSLog event는 1..9만 포함했다. 짧은 status/count 요약과 tuple별 `appendReport` event로 분리했다. clean rebuild 후 C9 unified log에 `sample_count=10`과 index 1..10 event가 각각 기록됨을 확인했다. |
| 16 | source dirty 상태 | output 생성물이 dirty source/hash로 오인되어 manifest가 재귀적으로 달라지는가? | commit/diff/untracked digest 계산을 output dir 생성보다 앞에 둔다. tracked diff와 untracked 파일 digest를 분리하고 `source_dirty`를 함께 쓴다. |
| 17 | artifact tree hashing | iOS `.app`의 상대 경로·symlink·실행 모드가 빠져 artifact fingerprint가 약해지는가? | app 디렉터리 내부 상대경로를 정렬해 file content+mode 및 symlink target을 합산한다. Android APK는 단일 파일 SHA-256, 두 플랫폼 build log도 따로 hash한다. |
| 18 | verifier exit status | 실패 gate가 EXIT cleanup에 가려 전체 verifier가 성공 코드 0을 반환할 수 있는가? | `cleanup`은 진입 시 기존 `$?`를 보존하고 종료한다. synthetic 9-record case는 helper의 실패뿐 아니라 `fail`과 EXIT trap을 거쳐 exit 1이 되는지 확인한다. |
| 19 | 빌드와 장치 경계 | build 확인 중 root worktree V8 cache를 오염시키거나 booted simulator에 설치/실행하는가? | 고정 V8 checkout/artifact를 read-only source로 지정해 새 worktree output에서 build한다. iOS runtime은 task-owned C9 clone만 대상으로 하고 다른 booted iPhone/AVD는 건드리지 않는다. |
| 20 | 근거 범위/성능 오인 | build/runtime 결과로 CPU, scheduler, 효과 기준, 또는 S03.3 완료를 주장할 수 있는가? | iOS clone runtime과 fixture metadata 검증만 통과했다. Android runtime은 AVD userdata가 요구한 7,372.80 MB에 비해 host free가 4.0 GiB여서 미검증이다. CPU/scheduler/effect threshold/N/tail/physical-device/long-loop/allocation failure/listener cycle은 미정이며 STATUS/roadmap는 수정하지 않았다. |

## 검토 결과

계획의 fixture-only 표본 tuple과 run provenance 범위는 유지한다. 초기 iOS C9 캡처에서 화면 표본은 1..10이었지만 하나의 unified log event는 9개까지만 전달했고, 9개 gate failure의 exit 1이 빈 stream cleanup trap에서 0으로 덮이는 경로가 있었다. tuple별 event 출력과 ordered-index gate, exit-status 보존으로 수정했다. self-test는 10개 positive control을 받고 synthetic 9개 negative control을 cleanup trap 뒤 exit 1로 거부했다. 수정된 clean commit `6a2b4ab1c84d438a2c11a86ee9d1b68ea5581e8e`의 iOS `Debug-iphonesimulator` fixture build와 C9 clone runtime을 통과했다: manifest는 `sample_count=10`, indices `1..10`, app tree SHA-256 `b8ab9f61a5635daccbef893fa537734a82ea4b9b80343418e74e79830596ef26`, build log SHA-256 `b051a249673f24dab94d15ef849ef722443f0b46add3f3d339b6e8f304451a82`를 기록한다. raw run directory는 `/private/tmp/spinon-s03-ios-pilot-final-20261026/run-20261026T052053Z-75141`이며 manifest SHA-256 `abced9f71a1e1fd2f08e5e2da110e5c71e142fc10d16e43ebcb60a0e523d8393`, unified log SHA-256 `90340ac720d73f1c02809a84a07ce9fbc47574570b5a303db5c2139abf0b7e8e`, screen SHA-256 `0449946779921def36fa1e4f67fd270b162be8de69f34b7c504e7a77cefe6585`다. Android runtime은 AVD userdata 요구량 7,372.80 MB가 host free 4.0 GiB보다 커 실행하지 않았다. 캐시/스냅샷/AVD 정리는 하지 않았으며 성능·CPU·scheduler·S03.3 완료를 주장하지 않는다.
