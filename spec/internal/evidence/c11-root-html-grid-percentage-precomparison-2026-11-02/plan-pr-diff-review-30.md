# C11.1 Root HTML Grid percentage-track plan PR diff review · 30 checks

**대상:** staged plan-only PR diff on `plan/c11-root-html-grid-percentage-tracks` · base `06d5a5a528008ad23a0665c51c9912e67bc45659`

**판정:** 30개 서로 다른 실패 관점을 검토했다. 구현은 포함하지 않았고, 공식 C11 체크 및 `72/213`은 변경하지 않는다.

| # | PR diff 실패 관점 | 판정 |
|---:|---|---|
| 1 | C11 전체 완료로 범위를 과장 | 통과. 계획 상태·범위 밖·병합 후 절차에 C11/C11.1 미완료를 명시한다. |
| 2 | C11.1 공식 checkbox나 완료 수를 조용히 수정 | 통과. `spec/STATUS.md`를 변경하지 않고 계획상 후보만 기록한다. |
| 3 | `fr` 계획을 아직 미구현으로 두거나 다음 후보 링크를 stale 상태로 남김 | 통과. plan index가 PR #1015 병합을 기록하고 percentage 계획을 다음 후보로 가리킨다. |
| 4 | Root HTML base·gap·named-area·anonymous-text 제한을 생략 | 통과. 새 계획은 각각의 기존 계획과 validator 경계를 연결한다. |
| 5 | `LengthPx`/`Fraction` 외에 Percentage 변환기가 이미 있다는 사실을 놓침 | 통과. 코드 지점과 공용 Grid regression을 구현 전 결손으로 명시한다. |
| 6 | Root HTML route와 fixture-only `RuntimeBlockFlexGridTrackSizingV1`를 혼동 | 통과. 목표는 기존 parsed Root HTML default entry이며 opt-in constructor 변경을 제외한다. |
| 7 | 열/행 percentage basis를 서로 바꿈 | 통과. §7.2.1과 독립 expected값을 각 축에 따로 둔다. |
| 8 | percentage basis를 grid outer size나 viewport로 잘못 사용 | 통과. `inner inline/block` size를 명세하고 border/padding zero gate를 고정한다. |
| 9 | column gap을 percentage basis에서 빼버림 | 통과. full-axis basis와 origin gap을 분리한 expected geometry가 있다. |
| 10 | `% + fr`에서 gap을 두 번 빼거나 한 번도 빼지 않음 | 통과. `[60,170]`, 10px gap, x origins `[0,70]`를 고정했다. |
| 11 | 100% 초과 percentage를 clamp해 overflow를 숨김 | 통과. `[240,20]`/268px track end를 명시하고 visible overflow를 유지한다. |
| 12 | 0% track 또는 0px definite basis를 무시 | 통과. 두 boundary를 별도 positive cases로 둔다. |
| 13 | negative control의 indefinite axis를 Chrome의 auto result로 구현 | 통과. Chrome 관측을 명세 결과와 분리하고 Root HTML에서는 fail-closed한다. |
| 14 | percentage gap 지원이 track 추가에 따라 새어 들어옴 | 통과. 고정-px-only gap negative control과 범위 제한이 있다. |
| 15 | percentage + named areas가 현재 fixed named-area path를 우회 | 통과. named-area CSS는 브라우저 control로만 관찰하며 runtime은 거부한다. |
| 16 | `calc()`, `minmax()`, repeat, line names, intrinsic track이 direct percentage로 오인 | 통과. direct typed value만 허용하고 나머지 fail-closed다. |
| 17 | finite input 곱이나 누적합에서 f32 overflow 후 partial scene 공개 | 통과. resolved values·fixed aggregate·gap representability 및 pre-layout rejection을 요구한다. |
| 18 | 10,000 track 상한이 Percentage variant로 우회 | 통과. existing cap과 pre-Taffy rejection regression을 유지한다. |
| 19 | fixed-size marker content가 track sizing을 바꾸거나 intrinsic test로 오인 | 통과. simulator fixture와 matrix 범위에서 empty fixed-size item을 구분한다. |
| 20 | zero/small track의 item overflow가 clipping으로 바뀜 | 통과. visible overflow를 유지하고 scroll/clipping은 제외한다. |
| 21 | NodeId, element ID, class, source order가 CSS 의미를 대신함 | 통과. selector rewrite와 reverse source-order controls를 수락 기준으로 넣었다. |
| 22 | stale document/style/layout revision의 track과 frame을 섞음 | 통과. 단일 source revision 및 기존 identity 보존을 목표/기준에 적었다. |
| 23 | multi-case Chrome fixture의 invalid control이 simulator positive source에 섞임 | 통과. positive-only runtime fixture를 따로 고정하고 Chrome geometry를 대조했다. |
| 24 | simulator screenshot을 pixel readback이나 compositor 증거로 표현 | 통과. draw/present와 PixelCopy/Metal/compositor-after 경계를 나눠 주장한다. |
| 25 | Android 실기기 또는 hardware GPU 검증을 암시 | 통과. Android API 37 AVD 및 iOS Simulator만 plan gate로 지정한다. |
| 26 | default product artifact에 test-only scene/payload/FFI symbol 추가 | 통과. 구현 단계에서 default artifact 경계를 검사하도록 기준을 둔다. |
| 27 | 실패 진단이 `property`를 잃거나 일부 frame을 남김 | 통과. axis property 진단과 atomic fail-closed scene 조건이 있다. |
| 28 | 20개 계획 리뷰를 30개 PR diff 리뷰와 중복 취급 | 통과. 계획 실패 관점 20개와 이번 diff의 별도 30개를 연결한다. |
| 29 | 상대 문서 링크, JSON, hashes, screenshot을 실제 내용과 불일치 | 통과. JSON 파싱, SHA256SUMS, fixture/image hash를 확인했다. |
| 30 | 계획 PR의 merge를 CI 성공이나 기능 구현 완료로 표현 | 통과. required checks 재조회, Rebase and merge, 후속 구현 PR, 72/213 유지 조건을 분리한다. |

모든 항목은 현재 diff 또는 연결된 고정 evidence로 검토했다. GitHub check rollup과 merge eligibility는 PR 생성 뒤 다시 확인한다.
