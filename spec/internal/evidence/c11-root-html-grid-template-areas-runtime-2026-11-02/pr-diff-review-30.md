# C11.1 named-area 구현 PR diff 실패 관점 검토 30개

**범위:** 구현 source, Root HTML/cascade/projection tests, 내부 계약 0059, 계획·상태 기록, simulator evidence, review docs와 PR용 화면 링크를 순차 검토했다. PR을 만들기 전 후보 diff에 대한 사전 검토이며 GitHub PR CI 결과를 의미하지 않는다.

| # | 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 구현 후보가 상위 C11을 완료 상태로 올린다. | `spec/STATUS.md`의 C11 checkbox와 `72/213`을 유지했다. 통과. |
| 2 | candidate가 C11.1, C11.3.3, C11.4를 한 번에 완료 처리한다. | 계획·계약은 exact Root HTML slice로 한정하고 세 항목 및 C11 전체를 미완료로 남긴다. 통과. |
| 3 | 계획 PR과 구현 PR의 상태/역할이 혼동된다. | 계획 PR #1004와 구현 PR #1006을 분리해 기록했고, #1006만 구현 candidate를 병합했다. 통과. |
| 4 | C11 공식 완료 수가 하위 후보 기록에 따라 증가한다. | status 문서의 공식 집계 및 plan/contract/evidence 본문에서 `72/213`을 일치시켰다. 통과. |
| 5 | 내부 contract version이 구현 후보만으로 부당하게 변경된다. | internal/0059 version `0.1.0`을 유지하고 candidate section을 별도 표기했다. 통과. |
| 6 | 계획의 parser/profile 경계와 실제 구현이 다르다. | 새 통합은 parsed Root HTML 기본 GPU route에만 연결되고 C11.3.3 fixture route를 호출하지 않는다. 통과. |
| 7 | grid-template-areas 지원 설명이 일반 CSS Grid 호환으로 읽힌다. | 계획과 문서는 고정 tracks·area matrix·세 empty div subset 및 미지원 grammar를 명시한다. 통과. |
| 8 | `grid-area` 단독 cascade 허용이 general placement support로 확장된다. | Root HTML profile allowlist 이후 typed placement는 exact owner candidate에서만 허용한다. 단독 placement 및 다른 profile은 fail-closed다. 통과. |
| 9 | CSS-invalid fallback이 syntactically valid unsupported style을 감춘다. | parser `OtherInvalidValue` 회복과 computed profile rejection을 별도 test/control로 기록했다. 통과. |
| 10 | diagnostic matcher가 unrelated CSS error를 삼킨다. | property allowlist, exact diagnostic suffix, Root HTML profile gate와 negative controls를 확인했다. 통과. |
| 11 | standard area placement를 authored track-name 문자열 재파싱으로 구현한다. | computed areas와 placement typed values를 `ComputedCssGridLayout` 및 `ComputedCssGridItemPlacement`로 전달한다. 통과. |
| 12 | Root HTML owner를 projection 단계에서 찾지 못하거나 다른 Grid에 결합한다. | `RuntimeGridNodes`가 owner NodeId→HostNodeHandle와 item owner mapping을 저장하고 candidate가 visible container를 하나로 제한한다. 통과. |
| 13 | template area dimensions와 actual explicit tracks가 일치하지 않아 implicit lines가 생긴다. | Root profile은 2×2 dimension 및 고정 두 축 tracks를 검증하고 mismatch tests에서 거부한다. 통과. |
| 14 | 비직사각형·누락·추가 area rectangle이 허용된다. | 세 이름과 각 exact `[row-start,row-end) × [column-start,column-end)` bounds를 비교한다. 통과. |
| 15 | owner별 semantic area가 중복 또는 미지정이어도 일부 frame이 publish되거나 대량 child에서 validator collection이 불필요하게 커진다. | candidate가 직접 item 3개 초과에서 즉시 거부하고 고정 배열로 owner/area mapping을 확인한다. negative cases는 partial scene 없음까지 검사한다. 통과. |
| 16 | grid item child source order나 ID가 semantic owner가 된다. | ID·selector rename과 item reorder controls, typed area ownership을 확인한다. 통과. |
| 17 | generated `area-start`/`area-end` metadata 길이 또는 grouping이 track 수와 다르다. | Root projection은 각 축에 N+1 빈 line groups를 예약한 뒤 typed area geometry로 alias edge를 resolve한다. positive geometry assertion 통과. |
| 18 | metadata size 제한에서 Root area bytes가 상한 계산에서 빠진다. | Root profile area 경로도 combined line/area metadata validator를 통과해야 한다. 기존 budget controls 및 추가 failpoint test를 확인했다. 통과. |
| 19 | allocator failure가 `grid-template-areas` source/context 없이 abort 또는 partial success가 된다. | Root route에 line group, area vector, area-name reservation failure를 주입하고 `GridAllocationFailure` property/node를 assert한다. 통과. |
| 20 | candidate 검증의 max-width/min-size constraints가 고정 root geometry를 변형한다. | 계획의 exact size contract와 `layout_dimensions` limits 비교 및 root max-width negative case를 확인했다. 통과. |
| 21 | viewport/media query 새 revision에서 이전 completion scene이 노출된다. | environment revision 변경 후 out-of-scope placement failure를 내고 old completed snapshot 없음까지 검증한다. 통과. |
| 22 | runtime no-scene 실패를 cascade fallback 성공으로 기록한다. | failed valid computed candidate와 CSS syntax-invalid declaration fallback의 상태를 분리한다. 통과. |
| 23 | `grid-template-areas` 없는 기존 fixed Grid가 새 route에 들어가 동작을 바꾼다. | Root validator는 areas 없는 all-auto placement의 기존 허용 경계를 유지하고 existing root Grid/gap regression을 통과했다. 통과. |
| 24 | 이전 unsupported Grid 기대 코드가 cascade error로 남아 tests가 의미를 잃는다. | 추가 CSS declaration admission 후 valid unsupported candidates의 code expectation을 `unsupported_computed_value`로 바꾸고 no-scene assertion을 보존했다. 통과. |
| 25 | plan, runtime README, internal contract, status count가 서로 다른 값이다. | `7/7` Root HTML Grid focused, `50/50` Root HTML regressions, `1/1` allocation failpoint, `279: 278/1` full style-to-layout outcome을 대조한다. 통과. |
| 26 | Rust tests와 simulator APK/iOS app이 다른 HTML fixture를 사용한다. | source fixture와 both app bundles의 SHA-256 `7c49f066398d51b6ade5ccf842addfc9bf1ec857b21b9e16c7e74d39a6275f3d`를 verification logs에서 대조한다. 통과. |
| 27 | screenshot이 예상하지 않은 fixture route 또는 content에서 생성된다. | app logs는 기본 `index.html` route와 ready marker, layout ready, draw/present를 기록한다. 임시 asset 교체는 복구되어 bootstrap inputs가 unmodified다. 통과. |
| 28 | screenshots가 hardware/readback/compositor exactness를 증명한다고 PR image가 암시한다. | PNG는 visual scene evidence로만 link하고 README는 PixelCopy/Metal readback 및 post-compositor/hardware claims가 아님을 명시한다. 통과. |
| 29 | local validation이 PR CI 통과로 서술되거나 unrelated failure가 숨겨진다. | PR #1006에는 check run이 없고 main required status checks도 비활성이다. 전체 style-to-layout의 C07 aspect-ratio string serialization `1.7777778 / 1` vs `1.77778 / 1` 실패는 정확히 분리하고 CI를 통과로 주장하지 않는다. 통과. |
| 30 | review 범위/개수 또는 evidence manifest가 diff 작성과 충돌한다. | plan review 20, 구현/runtime review 20, 이 PR diff review 30을 별도 문서로 분리한다. 최종 staging은 후보 경로만 명시하고 `SHA256SUMS.txt`를 마지막에 재생성한다. 통과. |

## 판정

초기 검토에서 candidate의 unbounded temporary collection을 수정 대상으로 찾았고 고정 슬롯으로 바꾼 최종 diff에서 해당 경계를 재검토했다. Agent Browser에서 PR #1006의 56-file Files changed 화면과 본문을 열었고 Android screenshot 1080×2400 및 iOS screenshot 1206×2622가 모두 자연 크기로 로드됐다. `gh pr checks`는 check run 없음, main required status checks 조회는 비활성(HTTP 404), review decision은 비어 있고 PR은 `MERGEABLE`/`CLEAN`이었다. 이 상태에서 Rebase and merge됐으며 merge SHA는 `f4bcce22566c147277bf51ab1f3eae37f12caf4f`다. check rollup이 비어 있으므로 CI 통과로 보고하지 않는다. C11 전체와 `72/213`은 유지한다.
