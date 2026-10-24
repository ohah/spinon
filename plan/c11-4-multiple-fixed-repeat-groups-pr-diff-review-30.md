# C11.4 multiple fixed-repeat groups 계획 PR diff 검토 · 실패 관점 30개

**대상:** 계획-only PR의 전체 변경. 코드·Spinon runtime·simulator 실행 결과는 포함하지 않는다. 계획 검토 20개와 다른 문서 diff 실패 관점이다.

| # | 독립 실패 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 의도하지 않은 source file 변경이 계획 PR에 섞일 수 있음 | 변경 목록은 신규 계획·두 검토표·Chrome fixture/evidence와 상태·roadmap 링크만 포함한다. Rust source 변경은 없다. |
| 2 | 계획 branch가 최신 merge 후 main에서 갈라질 수 있음 | branch는 `origin/main` `cac9d2d66adc5f386d2161cee1730bc4292714e5`에서 시작했다. |
| 3 | 계획-only 상태를 implementation complete로 표시할 수 있음 | 계획·상태·공개 roadmap 모두 cascade/runtime 구현 및 simulator 실행은 미시작이라고 적었다. |
| 4 | C11.4 부모 checkbox가 계획 기록만으로 체크될 수 있음 | `spec/STATUS.md`의 `C11.4 intrinsic track sizing`은 `[ ]`를 유지한다. |
| 5 | 완료 count가 계획-only PR만으로 바뀔 수 있음 | `spec/STATUS.md`와 공개 roadmap source 모두 `72/213`을 유지하고 checkbox 수식은 바꾸지 않았다. |
| 6 | roadmap이 구현 slice와 계획을 같은 상태로 묶을 수 있음 | 새 roadmap 문단 제목에 `계획-only`를 표시하고 Runtime 미시작 및 72/213 미완료를 쓴다. |
| 7 | official task ID 또는 버전이 임의 생성될 수 있음 | 새 official checkbox/ID는 추가하지 않았고 내부 계약 0059의 버전도 수정하지 않았다. |
| 8 | W3C source가 비공식 copy 또는 최신 URL alias일 수 있음 | 링크는 CSS Grid 2025 CRD §7.2.3 repeat-notation anchor에 고정했다. |
| 9 | 규범에서 multiple fixed-repeat 공존을 금지할 수 있음 | 해당 CRD text는 auto-repeat은 최대 하나로 제한하면서 fixed-repeat groups는 같은 list에 둘 수 있다고 말한다. |
| 10 | 규범의 line-name merge까지 이번 제한 slice가 구현한다고 읽힐 수 있음 | plan은 empty names만 받고 merge/collision semantics는 제외한다. |
| 11 | 넓은 standard grammar 지원을 약속할 수 있음 | exactly two fixed groups의 narrow profile이며 three groups와 other sizes는 거부한다고 적었다. |
| 12 | current code rejection 근거가 틀릴 수 있음 | `computed_auto_repeat_tracks`는 현재 integer repeat 두 번째에서 fail-closed하고 있으며 해당 source를 조사했다. |
| 13 | one-group regression이 새 grammar extension에 밀릴 수 있음 | 계획 구현 순서와 acceptance criteria가 기존 single-group controls 보존을 요구한다. |
| 14 | per-group pattern length 경계를 누락할 수 있음 | 각 pattern은 computed finite nonnegative `LengthPx` 1~2개로 고정했다. |
| 15 | prefix/suffix/split/reversed authored-order cross-product가 빠질 수 있음 | Chrome 10-scene fixture와 plan matrix에 네 columns ordering과 rows controls가 있다. |
| 16 | two-track fixed pattern이 단일 track 사례로 축소될 수 있음 | `repeat(2,20px 12px)` positive control은 네 tracks와 independent 64px breadth를 기록한다. |
| 17 | auto-fill count formula에서 gutters 또는 ordinary neighbor가 누락될 수 있음 | `126+38n`을 따로 산출하고 5회 316px, 6회 354px 경계를 적었다. |
| 18 | multitrack formula가 base formula와 혼동될 수 있음 | 7 fixed tracks의 별도 `142+38n` 계산, 4회/36.5px 값을 기록했다. |
| 19 | CSSOM fractional serialization이 exact integer로 주장될 수 있음 | Chrome 관찰 `30.7969px`를 규범 fractional 기대와 구분한다. |
| 20 | auto-fit empty와 occupied 결과가 합쳐질 수 있음 | empty 0px×5와 occupied 186px track/item offset을 별도 case로 기록했다. |
| 21 | rows count가 columns width를 이용할 수 있음 | row fixtures는 definite 320px height를 갖고 columns와 별도 axis로 추적한다. |
| 22 | negative control은 parser rejection과 typed adapter rejection을 혼동할 수 있음 | 계획은 stylesheet drop과 node/property typed error를 분리하도록 요구한다. |
| 23 | descriptor/line metadata budget 또는 allocation 실패가 사라질 수 있음 | 10,000 cap, checked arithmetic, fault injection, no partial output 및 recovery가 계획에 있다. |
| 24 | fixture가 product runtime API처럼 노출될 수 있음 | fixture를 opt-in `c04-runtime-gpu-fixture-test`로 한정하고 public CSS admission은 제외했다. |
| 25 | static archive 결과를 APK/IPA 전체 exclusion으로 과장할 수 있음 | artifact 단계는 default/positive controls로 제한하고 bundle 전체 주장은 명시적으로 금지한다. |
| 26 | Chrome output을 WPT나 Spinon support 결과로 읽을 수 있음 | README는 browser oracle/independent planning input이라고 표시하고 WPT·product proof로 쓰지 않는다. |
| 27 | scene ID와 screenshot capture 목록이 달라질 수 있음 | observation JSON은 10 unique IDs를 포함하고 full-page screenshot에서도 10장면을 시각 확인했다. |
| 28 | README hash가 현재 fixture/capture와 불일치할 수 있음 | fixture `1f7d7a99…`, JSON `25d3ff33…`, screenshot `2029d3e7…`을 SHA-256으로 재계산했다. |
| 29 | 새 plan/evidence route가 RSPress에서 생성되지 않을 수 있음 | 선택한 route build에서 plan, plan review, evidence README의 `.html` route가 생성됐다. |
| 30 | 전체 docs build 실패를 새 문서 문제로 돌리거나 감출 수 있음 | repository-wide build는 미변경 legacy dead links/image imports에서 실패했고, 새 plan/evidence만 포함한 selected build는 성공했다. 두 결과를 분리해 기록한다. |

## 결론

변경은 계획-only이며 official C11.4 completion과 `72/213`을 올리지 않는다. Chrome 156 input, observation, screenshot과 독립 계산이 계획보다 먼저 고정됐다. 전체 docs build의 기존 저장소 문제는 PR 범위로 끌어오지 않았고 selected route build는 통과했다. 계획 PR diff를 병합할 수 있다. Implementation review 20개와 implementation PR diff review 30개는 이후 코드 slice에서 새로 작성한다.
