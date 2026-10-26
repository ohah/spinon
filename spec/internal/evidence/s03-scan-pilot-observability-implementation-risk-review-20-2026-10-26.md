# S03.3 pilot observability · 구현 전 위험 검토 20관점

**검토 대상:** [pilot observability slice 계획](../../../../plan/s03-3-pilot-scan-observability.md)
**검토 시점 source:** `origin/main` `8247a12b`
**구현 전 계획 SHA-256:** `f0a86deca3ae7990a730c720c034c53f712df35d5973e10bebb6e93a6ea423da`
**실행 경계 갱신 SHA-256:** `cc074e3cbc93f7bf357b975da6ea1222549306bd5c1165472a3f30bfc02bd77f` (PR 전 iOS clone 실행은 output 검증만, formal baseline은 병합 후)
**범위:** 계획 문서만 검토했다. 이 시점까지 Android/iOS source, verification script, Rust/C++ 제품 코드는 수정하지 않았다.

각 관점은 독립 실패 경로 하나를 다룬다. 이 기록은 계획 단계 리뷰이며 구현 후 review나 PR diff review로 재사용하지 않는다.

| # | 관점 | 질문과 실패 경로 | 계획의 통제 또는 남은 제한 |
|---:|---|---|---|
| 1 | fixture opt-in | S03 버튼 또는 새 로그가 기본 앱에서 동작/노출될 수 있는가? | Android는 `BuildConfig.SPINON_S03_DOM_GC_FIXTURE`가 켜진 화면 안에서만 수행하고 iOS는 S03 fixture 경로에 한정한다. 기본/fixture-on artifact 경계를 build 확인한다. |
| 2 | 비교 대상 고정 | 새 fixture가 16,385 node 입력이나 동일 Isolate 반복 경계를 바꾸는가? | 기존 script와 expected sample count 10을 유지한다. 입력 변경은 별도 slice로 둔다. |
| 3 | wall interval 의미 | ns 시작값과 µs duration이 일부 Rust 단계만 측정하는 것으로 오해될 수 있는가? | 기존 outer C++ timer의 시작/끝 정의를 그대로 기록하고 전체 callback·검증·wrapper cleanup 포함을 명시한다. |
| 4 | clock domain | `steady_clock` nanoseconds가 OS trace monotonic clock과 같다고 잘못 연결될 수 있는가? | field 이름과 해석 한계를 기록한다. clock correlation이나 scheduler claim은 후속 검증 전까지 금지한다. |
| 5 | TID 출처 | `owner_tid`가 scan thread가 아닌 caller/callback thread일 수 있는가? | operation report의 owner identity와 V8 safe-point owner thread 경로를 source에서 확인한다. scheduler-state identity 검증과 구분한다. |
| 6 | process/thread 재사용 | 같은 TID가 process 재시작 또는 thread 종료 뒤 재사용되어 표본을 합칠 수 있는가? | sample은 run output와 operation sequence/cookie, TID 안에서만 사용한다. process/thread lifetime 상관은 이 slice 밖이다. |
| 7 | 필드 누락 | report에서 field가 빠져 `-1` 또는 빈 문자열이 되어도 기존 fixture가 통과할 수 있는가? | sample metadata completeness를 별도 gate로 출력하고 harness가 통과 표본 개수를 확인한다. |
| 8 | 숫자 overflow | 64-bit ns·µs가 Android Java `long`/Swift `Int` parsing에서 잘리거나 음수가 될 수 있는가? | 두 플랫폼의 signed 64-bit 수치 범위를 확인하고 report key parsing 실패 시 data completeness 실패로 처리한다. |
| 9 | 표본과 operation 연결 | UI나 순서가 다른 eval과 sample record를 연결할 수 있는가? | fixture 반복 순서대로 1..10 index와 기존 `seq`, `trace_cookie`를 같은 tuple에 둔다. |
| 10 | 동일 Isolate 반복 | 각 report가 직전 scan이 아닌 해당 eval의 마지막 scan을 나타내는가? | 기존 runtime은 eval 후 safe-point 회수 한 번을 하므로 호출별 report를 사용한다. callback 중복/다중 safe-point를 새 주장으로 만들지 않는다. |
| 11 | root correctness 영향 | 출력 수집이 wrapper root, callback root, GC 시점을 바꿀 수 있는가? | 새 기록은 report 숫자를 읽어 문자열을 만들 뿐 GC 요청/배열 강한 참조/회수 순서를 건드리지 않는다. |
| 12 | log truncation | 10개 tuple이 platform log/UI 제한을 넘어 잘릴 수 있는가? | 기존 결과는 10개 µs 배열을 이미 출력한다. tuple 크기를 bounded하게 유지하고 결과에서 10개 모두 존재하는지 검사한다. |
| 13 | Android Logcat side effect | harness가 기존 user log를 지우거나 앱을 force-stop할 수 있는가? | 현재 harness 동작을 계획에 적었다. active AVD를 보존해야 할 때 실행을 금지하고 build-only 확인을 쓴다. |
| 14 | iOS Simulator side effect | verification harness의 install/terminate가 booted app/session을 덮을 수 있는가? | booted device를 확인하고 active session을 보존할 수 없으면 simulator runner를 실행하지 않는다. |
| 15 | output 덮어쓰기 | 기존 evidence가 같은 run directory에 덮일 수 있는가? | 기존 고유 output dir 및 덮어쓰기 거부 계약을 유지하고 결과를 build ignored path에 둔다. |
| 16 | source provenance | clean commit SHA만 기록해 uncommitted source를 놓칠 수 있는가? | tracked diff와 untracked source digest를 manifest에 함께 둔다. commit SHA 단독으로 재현을 주장하지 않는다. |
| 17 | artifact identity | build log나 APK만 고정하고 iOS 앱/구성 차이를 잃을 수 있는가? | Android APK SHA, iOS app bundle tree digest, platform별 build task/config와 build log hash를 각각 기록한다. 이는 release parity 주장이 아니다. |
| 18 | fixture/product leakage | extra observable field가 production API, ABI, 로그에 새로 추가될 수 있는가? | Rust/C++ ABI와 production report를 변경하지 않고 기존 report 값을 fixture 출력에만 투영한다. 기본 artifact에서 opt-in 없는 sample label을 확인한다. |
| 19 | 성능 판정 오염 | 표본 10개 또는 정밀 field 추가를 충분한 통계·개선 결과라고 부를 수 있는가? | 10은 기존 정확성 반복 수라고 명시한다. N, effect threshold, tail bound 미정이며 이 PR에 성능 pass/fail을 두지 않는다. |
| 20 | 범위 확장/완료 오인 | pilot instrumentation이 allocation failure·long-loop·device 검증이나 S03.3을 완료한 것으로 오해될 수 있는가? | STATUS/roadmap를 건드리지 않고 PR 설명에 미완료 gate를 유지한다. OOM, scheduler, long-loop, physical-device, listener cycle 작업을 다음 단계로 남긴다. |

## 결정

계획에 명시한 platform fixture 출력과 run manifest만 구현한다. 필드 출처, 같은 run 상관 키, manifest digest 계산 및 simulator session 보존 검토가 구현에서 불가능하면 해당 metric을 임의 대체하지 않고 그 이유를 기록해 중단한다. 코드가 작성된 뒤에는 별도의 새 20관점 구현 검토를 수행한다.
