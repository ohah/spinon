# C11.4 five-fit 계획 PR #1347 실제 diff 검토 30개

**대상:** GitHub PR [#1347](https://github.com/ohah/spinon/pull/1347)의 원격 diff 및 PR 본문. 초기 원격 diff 15개 경로를 확인한 뒤, review 파일·계획 상태·로드맵 링크 변경을 포함한 최종 diff도 다시 읽었다. 각 항목은 서로 다른 실패 관점을 다루며 계획 자체의 20개 검토를 반복하지 않는다.

| # | 실제 변경에서 확인한 실패 경로 | 관찰과 판정 |
|---:|---|---|
| 1 | 계획 단계가 제품 구현 완료로 읽힘 | 계획은 Root HTML RuntimeSession admission proposal로 한정하고, 구현 전임과 C11.4/C11 미완료를 표시했다. 통과. |
| 2 | 공식 상태 수가 계획만으로 증가 | `spec/STATUS.md`는 diff에 없고 roadmap은 `78/214`를 유지한다. 통과. |
| 3 | 내부 contract version이 근거 없이 증가 | contract 0059는 diff에 없고 plan/PR body가 `0.1.0` 유지 경계를 명시한다. 통과. |
| 4 | 계획 링크가 이전 slice 또는 잘못된 filename을 가리킴 | plan, 20-point review, README, roadmap의 상대·사이트 경로가 새 five-fit artifact 이름을 사용한다. 통과. |
| 5 | 직전 구현 기준 PR과 상태가 혼동 | plan이 four-fit PR #1344를 이전 단계로 연결하고 이를 현재 five-fit 구현 완료로 부르지 않는다. 통과. |
| 6 | 규범 기준이 검색 요약이나 비공식 설명만 사용 | 계획은 W3C 2025 CSS Grid CRD §6.6, §7.2, §11.3–11.5에 직접 연결한다. 통과. |
| 7 | Chrome 구현 source가 실제 browser version과 다름 | README는 Chromium `157.0.8103.40` tag의 Grid track sizing 파일을 구현 참고로 연결하고, 직접 fixture 관측과 구분한다. 통과. |
| 8 | Headless UA와 설치된 browser patch version을 하나처럼 표기 | README는 binary `157.0.8103.40`과 `navigator.userAgent`의 `HeadlessChrome/157.0.0.0`을 따로 기록한다. 통과. |
| 9 | percentage cap 기준이 모호하거나 indefinite | fixture/계획은 definite `220px` block basis를 기록하고 indefinite percentage는 admission에서 거부한다. 통과. |
| 10 | 고정 Grid geometry가 비교 중 바뀜 | Oracle fixture와 README가 `40×220px`, `40px` explicit column, 빈 `10px` row, zero gap을 함께 고정한다. 통과. |
| 11 | 다섯 fit position 조합 일부 누락 | fixture quintuple generator와 verifier가 여섯 조합을 assert하고 report가 `6`으로 요약한다. 통과. |
| 12 | tail 종류나 authored 위치를 조용히 생략 | 관측 matrix는 `min-content`, `max-content`, `auto` 세 tail과 여섯 position을 포함한다. 통과. |
| 13 | cap 값 배정 순열 수가 잘못됨 | fixture/verifier는 고유 cap 다섯 개의 `120` authored-order permutation을 assert한다. 통과. |
| 14 | 장면 수 산식이 matrix 축과 맞지 않음 | `6 positions × 3 tails × 120 cap orders = 2,160 lists`, 이후 8 조건으로 `17,280 scenes`임을 artifact와 verifier가 일치해 기록한다. 통과. |
| 15 | baseline/candidate pairing key에서 비-minimum 축을 잃음 | verifier의 pair key는 authored list, alignment, DOM order를 사용하고 두 minimum mode가 각각 한 건인지 assert한다. 통과. |
| 16 | fit occurrence 분모가 candidate 수와 맞지 않음 | 독립 verifier가 `min-height:auto` candidate의 57,600 fit-track item occurrences를 다시 센다. 통과. |
| 17 | baseline floor 위반을 candidate 결과로 섞음 | verifier/README는 baseline `min-height:0` 21,600건과 candidate 0건을 분리한다. 통과. |
| 18 | non-fit tail floor를 fit floor와 합산 | verifier는 tail floor violation을 tail×minimum으로 따로 집계하며 결과는 0이다. 통과. |
| 19 | own-row payload overflow의 mode를 잘못 귀속 | README는 tail 종류별 baseline 7,200건과 candidate 0건을 분리해 표시한다. 통과. |
| 20 | item frame overflow와 text payload Grid-end overflow를 같은 값으로 취급 | item-frame Grid-end overflow 0과 baseline auto-tail payload Grid-end overflow 600을 별도 필드로 기록한다. 통과. |
| 21 | `align-content:stretch`의 결과 차이를 전체 tail에 일반화 | verifier와 계획은 차이 2,880 pair를 auto tail의 두 minimum mode 각 1,440으로 분해한다. 통과. |
| 22 | 8-item repeated text의 line count/source order가 바뀜 | fixture와 verifier는 `[2,2,2,3,2,2,3,1]`, forward/reverse ID 순서 및 line-top 간격을 확인한다. 통과. |
| 23 | empty explicit-row control이 matrix를 오염 | fixture에서 첫 child는 빈 row 1 control이며 반복 text와 분리해 보존한다. 통과. |
| 24 | CSSOM normalized list만 남아 authored input 증명이 사라짐 | NDJSON은 scene의 authored track list/caps와 computed list 및 used rows를 함께 저장한다. 통과. |
| 25 | collector가 오류 경로에서 task browser를 남김 | collector `finally`에서 고유 session을 닫고 `BROWSER_CLOSED`를 출력한다. 두 실행 session 모두 post-close `active:false`, `pid:null`이었다. 통과. |
| 26 | 확대 PR 이미지가 다른 layout input을 보여줌 | PR용 collector는 oracle fixture의 첫 representative scene을 사용하고 `transform`만 추가한다. 캡처를 육안 확인했다. 통과. |
| 27 | PR 이미지가 깨졌거나 raw URL이 private/branch 이동에 의존 | PR 본문 raw URL을 원격 head의 commit SHA로 고친 뒤 Agent Browser에서 `complete:true`, `1280×900` natural size로 표시되는 것을 확인했다. 통과. |
| 28 | checksums가 corpus나 재현 스크립트를 빠뜨림 | SHA256SUMS는 evidence 디렉터리의 11개 artifact를 대상으로 하며 11/11 검증이 통과했다. manifest 자체는 자기 참조하지 않는다. 통과. |
| 29 | 사용하지 않는 simulator나 Terminal Browser가 검증에 남음 | iOS booted device와 ADB attached device는 각각 0, Terminal Browser는 미사용이다. 작업 전용 Agent Browser sessions를 닫고 공유 CoreSimulator/다른 session은 보존했다. 통과. |
| 30 | 현재 작업 칸반이 계획과 구현 상태를 뒤섞음 | roadmap의 five-fit 항목은 `계획 완료`이며 description에 RuntimeSession 구현 전이라고 적고 `78/214`를 유지한다. 통과. |

## 판정

원격 PR diff와 PR 본문에서 위 30개 독립 경계를 확인했다. PR check-run/status는 0개이며 CI green으로 판정하지 않았다. Chrome oracle는 사전 비교이고 제품 구현, 전체 CSS Grid 지원, WPT, simulator 화면, pixel readback 또는 C11.4 완료를 증명하지 않는다.
