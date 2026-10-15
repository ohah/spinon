# C13.3 normalized wheel delta PR 변경 검토 30개 · 2026-10-15

검토 대상은 PR #194의 최종 코드·플랫폼 adapter·문서·evidence 변경이다. 계획 검토와 구현 실패 관점 검토 표를 재사용하지 않았다. 명시한 실행 결과는 기록된 focused test와 Simulator/AVD run에 한정한다. 모듈 분리 뒤 native 앱 재빌드가 없다는 한계는 마지막에 따로 적었다.

| # | 공격 관점·불변 조건 | 확인 결과와 범위 |
| ---: | --- | --- |
| 1 | FFI output 포인터 null 또는 capacity 0 | 상태 변경 전 `ERR_ARGUMENT`로 끝난다. 코드 확인이며 capacity 경계 subprocess 실행은 없다. |
| 2 | null/임의 host handle | 상태 조회 실패를 반환하고 입력을 queue에 넣지 않는다. FFI focused test의 invalid-handle case 통과. |
| 3 | 모르는 intent/unit/source/momentum enum | 각 discriminant match의 나머지 경로를 거부한다. FFI 코드 확인. |
| 4 | magnification/zoom을 scroll로 위장 | `scroll_intent=false`를 queue 전에 거부한다. 코드 확인; 앱 실행의 zoom 입력은 없다. |
| 5 | NaN/Infinity 좌표 | finite guard 뒤에만 snapshot hit-test와 sequence 할당이 일어난다. 코드 확인. |
| 6 | NaN/Infinity delta·raw 값, 0 이하 또는 무한 scale | `valid_wheel_sample`이 거부한다. spinon-render wheel invalid-sample test 통과. |
| 7 | clip 바깥 또는 hit target 부재 | clip-aware enqueue 시 target을 얻지 못하면 queue 전에 오류다. 코드 확인; 별도 앱 miss 주입은 없다. |
| 8 | dequeue 전 scroll로 hit 위치가 바뀜 | sample이 enqueue target을 저장하고 dequeue에서 재-hit-test하지 않는다. queue target/coalescing barrier test 통과. |
| 9 | style revision만 바뀐 stale sample | sample policy epoch과 snapshot epoch mismatch에서 적용하지 않는다. 코드 확인. |
| 10 | document/tree/environment/intrinsic-size 변경 | non-scroll key 비교가 mismatch를 차단한다. 코드 확인; 앱에서 각 revision을 따로 경쟁시키진 않았다. |
| 11 | 앞선 accepted scroll 뒤 같은 batch의 wheel | 동일 non-scroll key 및 sequence/revision/generation lineage만 허용한다. 코드 확인; Android/iOS 다중 scroll batch 실행은 없다. |
| 12 | 외부 snapshot publication을 내부 scroll lineage로 오인 | 일반 snapshot publish가 lineage를 초기화한다. 코드 확인. |
| 13 | 새 surface generation 뒤 이전 sample | 새 generation 기록 시 lineage가 초기화되고 이전 generation은 reject된다. 코드 확인. |
| 14 | route 중 surface generation 변경 | route 후와 실제 publish gate 안에서 generation을 재확인한다. 코드 확인; 동시 경쟁 주입은 없다. |
| 15 | inner 최대 offset의 `auto/auto` 대각 delta | 양 축 remainder가 outer에 적용됐다. Android/iOS scenario 1에서 offset·revision과 draw 일치. |
| 16 | X=`contain`, Y=`auto`의 대각 delta | X는 멈추고 Y만 outer로 이동했다. 양 플랫폼 scenario 2에서 `(0,40)` 확인; Chrome 관찰 차이를 별도 기록. |
| 17 | `touch-action:none`에서 wheel 입력 | touch pan 제한이 wheel에 적용되지 않는다. 양 플랫폼 scenario 3에서 inner Y만 40 증가. |
| 18 | 적용할 수 없는 no-op delta | scroll revision이 바뀌지 않으면 presentation publish를 생략한다. 코드 확인; 별도 앱 no-op scenario는 없다. |
| 19 | regular queue가 포화된 wheel sample | 전체 sample을 거부하고 admitted=false를 보고한다. renderer queue pressure test 통과; 앱 포화 run은 없다. |
| 20 | coalesced float 합이 f32 범위를 넘음 | overflow 합을 버리고 공간이 있으면 원래 두 sample을 순서대로 보존한다. queue overflow test 통과. |
| 21 | source/unit/scale/phase/modifier/point/target/revision 차이를 합침 | metadata가 coalesce key barrier이며 고정 queue tests 통과. |
| 22 | coalescing이 순서를 거꾸로 하거나 sequence를 잃음 | 직전 accepted wheel과 인접한 경우에만 합치고 최신 sequence를 보존한다. queue tests와 코드 확인. |
| 23 | regular queue가 wheel로 찬 뒤 gesture terminal이 유실되거나 순서가 바뀜 | 별도 terminal slot으로 순서를 유지한다. `end_waits_behind_a_queued_move_when_regular_capacity_is_full`가 regular queue의 wheel sample 뒤 terminal 전달을 확인한다. wheel rejection과 terminal 예약의 혼합 포화 run은 없다. |
| 24 | Android fixture route 없이 wheel probe 요청 | route/정책 scenario 조합을 검증하고 준비 단계에서 거부한다. Java 경계 코드 확인. |
| 25 | iOS fixture route 없이 wheel probe 요청 | route와 scenario 1–3을 요구하며 feature-off 인자는 실행하지 않는다. Swift 경계 코드 확인. |
| 26 | 기본 artifact에 fixture ABI 누출 | feature-off Android/iOS artifact에서 wheel marker·symbol 부재, opt-in positive control에서 존재를 기록했다. |
| 27 | Android JNI 인자 순서·폭 불일치 | Java native 선언, JNI 구현, C FFI 선언을 대조했다. 최초 opt-in build 통과; 모듈 분리 후 app rebuild는 V8 checkout 부재로 막혔다. |
| 28 | iOS Swift/Objective-C++/C ABI 인자 불일치 | selector, bridge forwarding과 C declaration을 대조했다. 최초 opt-in build 통과; 모듈 분리 후 iOS rebuild는 수행하지 않았다. |
| 29 | queue admission만으로 성공 처리하거나 scrollport 밖 draw | 두 플랫폼 probe가 queue offsets 뒤 matching draw offsets/revision을 요구한다. 6개 run의 draw log와 surface-aligned screenshot 비교에서 선택 scrollport bbox 밖 changed pixel은 모두 0개. |
| 30 | roadmap·PR 화면이 fixture를 제품 지원으로 보이게 하거나 이미지 URL이 잘못됨 | STATUS/계획/공개 roadmap은 native collection·정규화와 public event 미완료를 남긴다. PR #194를 Agent Browser에서 열어 6개 inline 이미지 모두 `complete=true`와 Android 1080×2400/iOS 1206×2622 크기를 확인했다. |

## 판정과 남은 확인

이 검토에서 코드 수정이 필요한 결함은 발견하지 못했다. 앱에서 직접 실행한 것은 단일 normalized sample 세 시나리오뿐이다. clip miss, zoom, generation 경쟁, 일반 snapshot barrier, no-op와 queue pressure는 코드 또는 queue unit test 범위다. Simulator/AVD 캡처는 모듈 분리 전 successful app build에서 얻었고 모듈 분리 뒤 Rust focused test·Clippy·rustfmt는 통과했다. 고정 V8 source 부재로 Android 재빌드는 실패했으며 iOS 앱 재빌드는 다시 하지 않았다. 실제 OS wheel/trackpad 수집·정규화, 실기기, release artifact, compositor 이후 표시, public event/API와 C13.3 전체는 검증하지 않았다.
