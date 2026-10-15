# C13.3 WheelDelta 구현 실패 관점 20개 · 2026-10-15

대상은 fixed opt-in `WheelDelta` queue→snapshot router→draw 경로다. 계획 검토 20개를 재사용하지 않았다. 코드와 기존 queue tests, Android API 37 ARM64 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 실행 근거를 각각 대조했다. “통과”는 이 fixture 경계에서 해당 실패를 막거나 드러냈다는 뜻이다.

| # | 적대적 입력·불변 조건 | 결과와 근거 |
| ---: | --- | --- |
| 1 | FFI output 포인터 null 또는 capacity 0 | 조기 `ERR_ARGUMENT` 반환. ABI 경계 코드에서 확인; 이 조합을 subprocess로 실행한 테스트는 없음. |
| 2 | 해제되었거나 임의의 host handle | 유효 state 조회 실패를 반환한다. `c13_3` FFI focused test의 invalid-handle case가 통과했다. |
| 3 | 모르는 unit/source/momentum/intent enum 정수 | 허용된 discriminant만 match하고 나머지는 FFI 경계에서 거부한다. `ffi/fixtures/c13_3.rs` 코드 확인. |
| 4 | magnification/zoom intent를 scroll로 가장 | `scroll_intent=false`면 queue 접근 전 `Err`로 종료한다. 공개 FFI는 intent 1을 이 guard로 연결한다. |
| 5 | NaN/Infinity hit-test 좌표 | 좌표의 finite guard가 hit-test와 sequence 증가 전에 거부한다. |
| 6 | NaN/Infinity normalized/raw delta 또는 0·음수·무한 scale | queue의 `valid_wheel_sample`이 reject한다. `spinon-render` wheel tests의 invalid sample case가 확인한다. |
| 7 | clip 밖이거나 hit target이 없는 지점 | clip-aware hit test가 target 부재를 반환하며 queue에 enqueue하지 않는다. `state/wheel.rs` 코드 확인. |
| 8 | 오래된 surface generation이 새 surface 뒤 도착 | 최신 generation만 허용하고 surface update gate로 publish 시점도 다시 검사한다. 실제 2-generation 역전 주입은 하지 않았으므로 코드 경로 검토에 한정. |
| 9 | 한 번의 scroll publish 뒤 같은 비-scroll key의 다음 sample | scroll revision 차이만 lineage로 허용하며 sequence/revision/generation 조건을 검사한다. 코드 검토; 두 event를 한 batch에 넣는 Simulator 재현은 없음. |
| 10 | style/document/tree/environment/intrinsic-size key 변경 뒤 오래된 sample | non-scroll key mismatch 또는 policy epoch mismatch에서 discard한다. `same_non_scroll_key` 단위 검사와 `process_c13_3_wheel` 가드를 확인. |
| 11 | enqueue 시 고른 target과 dequeue 사이 target이 바뀌도록 다른 위치 sample을 혼합 | 각 sample의 target을 queue에 저장하고 coalescing은 위치·target·revision 경계에서 차단한다. queue target barrier tests 통과. |
| 12 | inner가 경계에 있고 `auto/auto`인 대각선 delta | 남은 두 축이 outer에서 소비됨. Android/iOS scenario 1에서 outer `(40,40)`, inner `(86,106)`, revision 2와 최종 draw 일치. |
| 13 | X=`contain`, Y=`auto`인 동시 delta | 축별 router가 X ancestor chain을 막고 Y만 넘김. Android/iOS scenario 2에서 outer `(0,40)`으로 일치. Chrome 관찰과의 차이는 plan/evidence에 남김. |
| 14 | `touch-action:none`에서 wheel을 touch pan처럼 거부 | wheel 경로는 touch-action gate를 쓰지 않는다. scenario 3에서 inner Y 40, ancestor 0, revision 1로 양 플랫폼 완료. |
| 15 | bounded queue 용량을 초과한 wheel event | sample 전체를 거부하고 진단을 올린다. enqueue report의 `admitted=false`를 fixed probe가 성공으로 오인하지 않게 guard한다. queue pressure test 통과. |
| 16 | 인접 sample coalescing 중 delta 합이 overflow | 합성 결과를 저장하지 않고 공간이 있으면 두 event 순서로 보존, 그렇지 않으면 다음 sample 전체를 거부한다. overflow/coalescing tests 통과. |
| 17 | sample마다 단위·scale·source·phase·modifier가 달라짐 | metadata barrier를 coalescing에 적용하고 원 단위·정규화 값·metadata를 trace에 기록한다. `wheel_coalescing_keeps_raw_units_and_rejects_metadata_barriers` 통과. |
| 18 | 경계에서 delta가 소비되지 않아 offset/revision이 변하지 않음 | router 결과에서 revision 변경 없으면 snapshot publish를 생략한다. 코드 확인; 완전 소비되지 않는 별도 AVD scenario는 실행하지 않음. |
| 19 | queue accepted만 되고 renderer가 새 snapshot을 그리지 않거나 scrollport 밖으로 그려짐 | probe는 동일 revision/offset draw 완료까지 기다린다. 양 플랫폼 여섯 log에서 matching `C0410_DRAW`를 확인했고, aligned before/after screenshot 비교에서 해당 outer/inner border bbox 바깥 changed pixel은 모두 0개다([pixel 결과](pixel-boundary-analysis.json)). |
| 20 | fixture route가 기본 앱으로 유출되거나 합성 입력을 OS 입력으로 잘못 표기 | default APK/iOS app에서 symbol/marker 제외, opt-in positive control 포함을 확인했다. 화면과 로그는 synthetic normalized fixture로 표기한다. |

## 결론과 열린 경계

이 검토에서 구현 결함 수정은 필요하지 않았다. 직접 관찰은 세 단일 sample scenario와 build boundary에 한정된다. host lineage의 동일 batch 다중 wheel, queue가 포화된 앱 경로, generation 경쟁, no-consumption revision 보존은 단위/코드 검사에 의존하며 새 앱 scenario로 검증하지 않았다. raw OS event collection과 단위 정규화, public `WheelEvent`, real device, release artifact, S04.10 displayed-frame hit target 및 C13.3 완료는 다루지 않는다.
