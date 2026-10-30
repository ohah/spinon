# C11.4 direct-text aspect-ratio two-track span 계획 PR #854 diff 검토 20개

**검토 대상:** PR #854 최종 계획/evidence diff와 PR 본문. 구현 코드 변경은 없다.

| # | 적대 검토 관점 | 결과 |
|---:|---|---|
| 1 | 이 PR에 계획과 사전 비교 외 제품 변경이 섞였는가 | **통과.** Runtime/layout source, FFI, Cargo 설정은 변경하지 않았다. 파일 경계는 계획, HTML oracle, JSON/evidence와 screenshot이다. |
| 2 | STATUS checkbox 또는 공식 집계가 계획 검토만으로 올라갔는가 | **통과.** `spec/STATUS.md`는 변경하지 않았고 C11/C11.4 unchecked 및 `72/213` 유지가 계획과 parent index에 명시돼 있다. |
| 3 | C11.4 parent plan 기록을 새 후보가 덮어썼는가 | **통과.** 새 후보를 인덱스 위에 추가하고 기존 merged slice history는 그대로 뒀다. |
| 4 | 신규 계획이 계획 단계와 구현 완료를 혼동하는가 | **통과.** 계획 상태, 미실행 runtime/simulator 및 별도 구현 PR 필요를 명시한다. |
| 5 | `span`이 기존 direct-text ratio slice에 이미 포함된 범위를 반복하는가 | **통과.** 기존 finite/minmax plan은 span을 거부하며 PR #850의 한-item owner가 여러 track을 차지하는 조합을 이 계획의 공백으로 분리한다. |
| 6 | normative URL 또는 section anchor가 엉뚱한 CSS Grid 규칙을 가리키는가 | **통과.** W3C 2025 CRD의 §6.6, §11.5, §11.5.1 및 Sizing Level 4 §4 URL을 브라우저에서 확인했다. |
| 7 | CRD/Working Draft를 안정된 최종 규격이라고 부르는가 | **통과.** 문서 지위를 규범 적합성이나 WPT 결과와 구분하도록 적었다. |
| 8 | span distribution 독립 계산이 Chrome output을 거꾸로 복사했는가 | **통과.** identical max-content tracks, prior single-span contribution 없음, §11.5.1의 equal distribution을 먼저 계산하고 browser와 비교했다. |
| 9 | track list 입력과 CSSOM resolved pixel output이 섞이는가 | **통과.** 구현 acceptance는 layout 전 typed `max-content max-content`를 확인하고 oracle은 layout 후 pixel sizes와 분리한다. |
| 10 | Ahem fallback/font load 실패가 폭 40px 가정에 섞이는가 | **통과.** fixture는 저장소의 local Ahem TTF를 쓰고 browser의 `fontReady`와 computed `10px / 10px Ahem`을 검증 스크립트가 확인한다. |
| 11 | root width 100px 또는 column sizing이 scene 간 바뀌는가 | **통과.** canonical HTML에 두 max-content columns, 한 auto row, 100px width, zero gap가 고정돼 있다. |
| 12 | auto placement가 두-column span 대신 다른 위치를 선택할 수 있는가 | **통과.** item은 `grid-column:1 / span 2`; browser capture에서 computed start `1`, end `span 2`를 고정한다. |
| 13 | ratio 축/값이 잘못 serialize돼도 geometry만 우연히 통과하는가 | **통과.** expected JSON에 ratio 문자열을 두고 verifier가 computed ratio와 정확히 비교한다. |
| 14 | `min-width:auto` floor와 explicit zero가 합쳐져 max-height 결과가 모호한가 | **통과.** `[20,20]`/40×15 control과 `[15,15]`/30×15 scene을 따로 둔다. |
| 15 | fractional contribution의 rounding을 정수 oracle이 숨기는가 | **통과.** 17.75px track, 35.5×17.75 item 및 4.5px text overhang을 독립 expected JSON과 verifier가 확인한다. |
| 16 | expected item x/y가 page 좌표와 혼동돼 잘못된 비교를 하는가 | **통과.** expected frame은 root-relative이고 verifier가 observed item 좌표에서 root 좌표를 빼서 비교한다. |
| 17 | CSS text overflow를 item/track size로 오인하거나 빠뜨리는가 | **통과.** Text range와 item frame을 따로 저장하고, 양수 inline overhang은 10px/4.5px다. |
| 18 | evidence hash나 browser image가 PR 본문에서 깨지는가 | **통과.** README hash는 verifier가 확인했고 Agent Browser에서 PR 본문 image가 1280×633으로 로드되는 걸 확인했다. 이미지 alt는 Chrome oracle이며 Spinon 화면이 아니라고 밝힌다. |
| 19 | 검증 스크립트가 양성 자료만 통과하고 mutation을 놓치는가 | **통과.** `verify-precomparison.mjs`가 9개 geometry, typography/placement/constraints, 5개 SHA-256을 검사하고 item height 1px mutation을 거부한다. |
| 20 | PR 결과를 CI/implementation/simulator 완료로 과장하는가 | **통과.** PR check rollup은 비어 있고 required checks도 보고되지 않았다. 계획 PR은 simulator/WPT/runtime 검증을 하지 않았다고 기록한다. |

**최종 로컬 확인:** `git diff --cached --check` 성공, 네 Markdown 문서의 상대 경로 link targets 존재, `node verify-precomparison.mjs` 결과는 9/9 geometry·artifact hash 일치와 1px mutation 거부다. 이는 planning/evidence diff 검토다.
