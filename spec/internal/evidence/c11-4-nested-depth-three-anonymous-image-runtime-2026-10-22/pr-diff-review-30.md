# C11.4 depth-three anonymous text + image PR diff 적대 검토 · 30개 관점

**대상:** 계획 PR #461 병합 이후 implementation diff, 플랫폼 fixture adapter, capture verifier와 관련 plan/status/evidence 문서. 각 항목은 서로 다른 경계 또는 실패 경로를 본다.

| # | 변경 관점 | 확인한 diff와 불변 조건 | 결과 |
| ---: | --- | --- | --- |
| 1 | opt-in 기능의 공개 범위 확장 | projection helper와 crate re-export는 fixture-test feature에 한정했다. | PASS |
| 2 | fixture module이 canonical `fixtures/` 경계 밖에 놓이는가 | FFI/runtime scene source는 각 fixture 하위 module에 두었다. module-boundary test 6/6. | PASS |
| 3 | scene ID가 FFI source와 runtime allowlist에서 서로 다른가 | 네 ID를 양측 exact table 및 Rust source matrix test에 맞췄다. | PASS |
| 4 | 신규 source가 기존 input adapter 선택에 앞서 등록되지 않아 누락되는가 | FFI dispatch와 runtime prepare 순서를 검사했고 최종 app run에서 네 ID가 실행됐다. | PASS |
| 5 | 미등록 C11.4 prefix가 다른 scene adapter에 빠져들어 우연히 처리되는가 | 새 adapter는 unknown depth-three ID를 Unsupported로 닫고 FFI는 고정 ID만 source를 반환한다. | PASS |
| 6 | four Grid ID 소유 관계가 scene 간 공유되거나 중복되는가 | scene ID를 포함해 root/middle/branch/inner ID를 생성한다. 각 DOM parent를 검증한다. | PASS |
| 7 | anonymous text가 box tree element인 것처럼 세어 box count가 틀리는가 | text provenance는 `gridItemIntrinsicInputs`에서 확인하고 expected boxes에는 element/grid만 포함한다. | PASS |
| 8 | source order 반전 시 text/image node 배열과 index가 뒤섞이는가 | Android, iOS 및 capture verifier에서 order별 node·kind·sourceOrder·textSources를 따로 확인한다. | PASS |
| 9 | max-content CSS source가 min-content 또는 반대로 생성되는가 | FFI scene CSS와 scene matrix test의 exact tuple이 두 constraint를 구분한다. | PASS |
| 10 | inner Grid 두 column이 outer 단일 column count로 잘못 전달되는가 | style projection에서 inner의 expected column count 2, outer count 1을 확인한다. | PASS |
| 11 | rows에 content-driven size가 전달되지 않는가 | 각 Grid의 row sizing 및 final row track을 min `[20]`, max `[16]`으로 확인한다. | PASS |
| 12 | text와 image가 다른 intrinsic revision을 섞어 사용되는가 | text registry revision, image snapshot revision 및 layout stamp revision을 모두 확인한다. | PASS |
| 13 | image natural ratio가 대체 resource 값이나 authored dimension으로 바뀌는가 | decoded PNG natural size `24×16`, auto width/height 및 aspect-ratio absence를 검증한다. | PASS |
| 14 | `align-self`/`justify-self` 변경이 item x/y를 바꾸는데 수용되는가 | image item guard가 허용되지 않는 값은 fail-closed한다. row/column placement도 Auto만 받는다. | PASS |
| 15 | capture verifier가 다른 scene의 intrinsic input matrix를 재사용하는가 | marker `nestedGridAnonymousTextImage`를 통해 text/replaced별 input 변환을 분리한다. | PASS |
| 16 | 기존 multispan 검사가 신규 input에 적용돼 타입 오류나 잘못된 비교를 하는가 | mixed-span 전용 marker와 신규 marker 경로를 구분했다. 신규 네 케이스의 source array를 정확 비교한다. | PASS |
| 17 | track verifier가 root track만 보고 inner columns 오류를 놓치는가 | 각 Grid trace에서 최종 columns/rows를 확인하고 inner columns는 순서별 배열을 검증한다. | PASS |
| 18 | item contribution trace가 자식 Grid 하나만 검사하고 image/text 중 하나를 생략하는가 | inner trace에 두 직접 item의 양 축 contribution을 각각 요구한다. 바깥 세 Grid는 child contribution을 검사한다. | PASS |
| 19 | 허용 오차가 실제 sizing drift를 가리는가 | numeric tracks/contributions는 `1/64px` tolerance로 고정하고 IDs, order 및 frame는 exact comparison이다. | PASS |
| 20 | CSS scene에 cascade 미지원 declaration이 남아 최종 scene이 실행되지 않는가 | 실 실행 중 unsupported alignment-content declaration을 발견해 제거했다. CSS와 expected geometry를 다시 확인했다. | PASS |
| 21 | Android allowlist만 갱신돼 iOS에서 scene이 차단되는가 | 두 플랫폼의 동일 네 scene ID, frame, count, inputs verifier를 대조했고 각 14/14 실행했다. | PASS |
| 22 | 플랫폼별 expected frame 수식이 reversed source order에서 틀리는가 | Java/Swift의 image x, min/max width/height 분기를 capture result 및 로그와 비교했다. | PASS |
| 23 | simulator run의 앱 바이너리와 설치물이 달라 다른 코드가 실행되는가 | Android local/installed APK hash가 일치한다. iOS app hash는 simulator manifest에 고정했다. | PASS |
| 24 | 캡처가 uncommitted/untracked source를 해시에서 빠뜨리는가 | 신규 Rust source를 포함한 commit `3346d9b3`에서 재빌드·캡처했다. 두 manifest는 동일 commit/tree, dirty path 없음이다. | PASS |
| 25 | default build에 fixture ABI/payload가 유출되는가 | 기본 static archive에서 네 ID 문자열 부재, opt-in archive positive control에서 모두 존재한다. | PASS |
| 26 | artifact boundary 검사를 app release-size 증거로 오해하는가 | README는 FFI static archive marker 검사와 debug Simulator app build를 분리하고 release 크기를 주장하지 않는다. | PASS |
| 27 | 14-scene matrix에서 기존 회귀 control을 빠뜨리는가 | 결과 JSON의 10 old + 4 new IDs를 확인했다. Android/iOS 각각 14 observations다. | PASS |
| 28 | `drawPresented`를 compositor readback 또는 pixel equality로 과장하는가 | 문서와 PR screenshot 설명은 layout surface 제출만 말하고 text/PNG pixel, iOS compositor, physical surface 한계를 표시한다. | PASS |
| 29 | 공개/공식 status를 C11.4 완료로 잘못 승격하는가 | `spec/STATUS.md`, plan, internal index, roadmap은 C11.4 checkbox와 `72/213` 미완료 상태를 유지한다. | PASS |
| 30 | PR·Tailnet 정적 preview·문서 링크가 최종 증거를 놓치는가 | PR body, 내부 evidence index, public roadmap와 Tailnet served path/hash를 별도 확인한다. PR 이미지 확인은 Agent Browser 증거로 남긴다. | PASS |

최종 구현/runtime와 Simulator 근거는 [README](./README.md), 실제 실패 경로 review는 [20-perspective report](./implementation-runtime-review-20.md)에 있다. 이 review는 PR diff가 확정된 마지막 단계에서 실제 PR 번호·merge SHA·Agent Browser 결과를 덧붙인다.
