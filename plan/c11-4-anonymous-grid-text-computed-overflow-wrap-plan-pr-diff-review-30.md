# C11.4 anonymous Grid text computed `overflow-wrap` 계획 PR diff 검토 30개

**검토 대상:** plan PR의 실제 변경분인 computed `overflow-wrap` 집중 계획, 계획 실패 관점 20개, C11.4/C11 색인 변경, 공식 미완료 기록, public roadmap 계획 카드, Chrome fixture/관찰/독립 기대값/screenshot이다. 이 diff 검토는 계획 내용 검토 20개를 반복하지 않고 파일 간 상태·링크·증거 경계를 살폈다.

| # | PR diff 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 새 계획이 main에 병합된 C11.4 fixture 범위 밖으로 이동할 수 있다 | `RuntimeBlockFlexGridTrackSizingV1`의 opt-in anonymous direct-text 경로로 범위를 고정했다. 통과. |
| 2 | 계획 상태가 구현 완료처럼 읽힐 수 있다 | 계획·C11.4 parent·상태 대장·public roadmap 모두 `계획`, `구현 전`이라고 적었다. 통과. |
| 3 | C11.4 checkbox 또는 공식 `72/213`이 바뀔 수 있다 | 공식 상태 기록과 부모 요약에서 checkbox/count를 유지한다. 통과. |
| 4 | 새 계획이 parent track-sizing 문서에서 검색되지 않을 수 있다 | 상위 계획 최상단에 계획 항목과 evidence 링크를 추가했다. 통과. |
| 5 | C11 상위 table의 새 항목이 완료된 slice 목록에 섞일 수 있다 | C11.4 cell에서 계획 중이라고 분리하고 구현 전으로 표시했다. 통과. |
| 6 | STATUS 항목이 새 plan route를 찾지 못할 수 있다 | 상태 기록이 집중 계획의 정확한 상대 경로를 가리킨다. 통과. |
| 7 | public roadmap이 구현 산출물이나 runtime 결과를 약속할 수 있다 | 카드가 관찰 9 scene만 연결하고 runtime 미시작을 적었다. 통과. |
| 8 | RSPress HTML route가 source Markdown 경로와 다를 수 있다 | `/spinon/plan/...html` 및 `/spinon/internal/evidence/.../README.html` 경로가 source 위치와 일치한다. 통과. |
| 9 | PR/roadmap 화면에 없는 screenshot 경로를 넣을 수 있다 | Chrome PNG가 evidence 디렉터리에 있고 HTML 카드와 PR 본문 후보 경로가 일치한다. 통과. |
| 10 | 계획 파일의 W3C 링크가 이전 섹션이나 불안정한 해시를 가리킬 수 있다 | CSS Text 3 2026-08-14 CRD §5.4 `overflow-wrap-property` anchor를 Agent Browser로 확인했다. 통과. |
| 11 | HTML fixture가 로컬 pinned font 대신 fallback으로 렌더링될 수 있다 | relative font URL이 canonical Ahem path로 해석되고 Chrome `fontLoaded=true`다. 통과. |
| 12 | README와 expected JSON이 서로 다른 font revision을 참조할 수 있다 | canonical Ahem SHA와 UPEM/advance가 README·JSON에서 같다. 통과. |
| 13 | Observation이 Chrome이 아닌 다른 browser나 terminal browser에서 나올 수 있다 | JSON UA는 HeadlessChrome 156.0.0.0이고 Agent Browser session에서 저장했다. 통과. |
| 14 | viewport/DPR이 없어 browser 숫자를 재현하지 못할 수 있다 | JSON은 1280×633 CSS px과 DPR 1을 기록한다. 통과. |
| 15 | requested CSS value와 computed value가 다른 입력을 성공으로 세울 수 있다 | 아홉 observation에 authored/computed `overflow-wrap`을 각각 기록하고 모든 값이 일치한다. 통과. |
| 16 | `normal`, `anywhere`, `break-word` 장면 수가 기대와 다를 수 있다 | 값 세 개와 각 track 세 개로 9 observation 및 9 expected scene이 맞는다. 통과. |
| 17 | Chrome 관찰 숫자를 독립 expected 값으로 재사용할 수 있다 | expected JSON의 10px/40px은 font metric과 CSS Text 의미로 계산했고 README에서 역할을 분리했다. 통과. |
| 18 | `anywhere` min-content break를 `break-word`에 적용할 수 있다 | expected·plan·roadmap이 anywhere 10px, break-word 40px의 차이를 동일하게 기록한다. 통과. |
| 19 | fixed `10px` scene에서 break-word의 actual emergency wrap 검증이 빠질 수 있다 | HTML observation은 normal 1 line과 anywhere/break-word 4 lines를 기록한다. 통과. |
| 20 | CSSOM track과 row를 text Range rectangle로 대체할 수 있다 | evidence README는 CSSOM을 track 관찰로, line tops를 wrapping control로 구분한다. 통과. |
| 21 | root box width가 120px이 아니어서 fixed track 조건이 흐트러질 수 있다 | fixture root width는 120px로 고정하고 JSON은 세 fixed/min/max track 결과를 별도로 기록한다. 통과. |
| 22 | computed `overflow-wrap`이 child owner에서 새로 선언되어 anonymous inheritance를 검증하지 못할 수 있다 | fixture는 Grid root에 property를 설정하고 direct Text node 하나만 둔다. 통과. |
| 23 | plan의 9-scene IDs와 fixture generator contract가 어긋날 수 있다 | expected JSON IDs는 HTML의 value/track loop 조합과 동일하다. 통과. |
| 24 | observation JSON의 insertion 뒤 원본 font/fixture/hash가 달라질 수 있다 | 현재 SHA 명령 출력이 README와 모두 일치한다. 통과. |
| 25 | screenshot이 계획 이전이거나 수정 후 source를 반영하지 않을 수 있다 | screenshot은 최종 9-scene HTML fixture에서 Chrome으로 다시 캡처했다. 통과. |
| 26 | plan review 20과 plan PR diff review 30이 한 표를 중복 집계할 수 있다 | 두 파일은 분리되고 각각 설계 실패와 PR 변경 일관성을 검토한다. 통과. |
| 27 | 내부 계약 0059가 계획 단계에서 불필요하게 version bump될 수 있다 | 계약 파일을 변경하지 않고 `0.1.0` 유지라고 기록한다. 통과. |
| 28 | project artifact gate가 APK/IPA 전체 제외를 뜻한다고 확장될 수 있다 | 계획은 default/fixture-test static archive만 다루고 bundle exclusion을 제외한다. 통과. |
| 29 | simulator proof가 hardware/GPU/compositor pixels로 확대될 수 있다 | 계획은 AVD software renderer와 iOS 제출 전 drawable을 구분하고 glyph pixel을 제외한다. 통과. |
| 30 | 이 plan PR만으로 required CI·implementation 또는 PR completion을 주장할 수 있다 | 아직 plan PR diff 검토만 기록하고 code/runtime/required-check 결과는 미실행으로 둔다. 통과. |

**결론:** 계획 PR 변경분의 별도 실패 관점 30/30 통과. 발견한 파일 간 상태·입력·기대값·링크의 모순은 없었다. 구현 착수는 이 계획 PR의 Rebase and merge 뒤로 제한한다.
