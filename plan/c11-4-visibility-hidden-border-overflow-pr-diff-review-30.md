# C11.4 visibility-hidden border overflow 계획 PR diff · 실패 관점 30개

이 검토는 계획 자체의 20관점 검토와 별도로 **이 PR에 포함된 실제 변경분 전체**를 확인한다. 검토 대상은 새 고정 HTML, Chrome 관측·기대값·sample evidence, 계획, 상위 계획 보정, public roadmap 상태 메모와 두 review 기록이다. 구현 코드는 이 PR 범위에 없다.

| # | 실패 관점 | diff 검토 및 결과 |
| --- | --- | --- |
| 1 | PR에 이번 일과 무관한 미추적 파일이 포함될 수 있음 | Stage 목록은 fixture, 계획, 상위 계획, public roadmap, 전용 evidence 네 JSON/README와 screenshot 및 review 파일로 한정했다. 저장소의 기존 미추적 산출물은 stage하지 않았다. PASS |
| 2 | fixture의 상대 Ahem 경로가 다른 파일을 가리킬 수 있음 | `../../resources/c15/Ahem.ttf`를 URL 기준으로 해석했고 SHA-256은 README/expected와 일치한다. PASS |
| 3 | fixture의 상대 PNG 경로가 잘못될 수 있음 | `../../resources/c14-1-png-basic.png`가 실제 `24×16` PNG로 decode되고 저장한 SHA-256과 일치한다. PASS |
| 4 | HTML 파일이 browser-ready 상태를 보장하지 않을 수 있음 | viewport meta, `document.fonts.ready`, 모든 image decode와 두 번의 animation frame 뒤 observation을 수집한다. PASS |
| 5 | 캡처 전에 font 미로딩으로 min-content가 fallback될 수 있음 | Agent Browser observation의 `ready:true`, CSSOM frames와 PNG natural size를 확인했다. Browser-only 관찰로 한계를 기록했다. PASS |
| 6 | page layout에 의해 fixture root가 viewport 밖으로 밀릴 수 있음 | 고정 2×2 scene layout에서 네 root가 모두 1280×633 viewport 안에 있고 sample 좌표도 화면 안에 있다. PASS |
| 7 | root 크기가 content-box인지 border-box인지 모호할 수 있음 | 모든 요소에 `box-sizing:border-box`를 두고 root border 2px·width/height를 명시했다. PASS |
| 8 | border가 화면에서 투명하거나 색이 다른가 | screenshot의 네 비가림 left/top border 표본이 모두 RGB `(240,198,116)`이다. PASS |
| 9 | right/bottom border 표본이 아이템에 덮일 수 있음 | 해당 표본은 사용하지 않는다. 리뷰 중 겹침을 발견한 좌표를 left/top으로 옮기고 image sample JSON에서 재확인했다. FIXED→PASS |
| 10 | root overflow가 hidden/auto로 바뀌어 clipping을 관찰할 수 있음 | CSS root와 computed observation 모두 `overflow:visible`로 고정한다. PASS |
| 11 | first track placement가 span과 A에서 다를 수 있음 | A와 span 모두 column/row 1번 cell에 명시적으로 겹치며 네 장면 관측 frame이 같다. PASS |
| 12 | overlap sample이 실제 root 바깥이 아닐 수 있음 | target-axis root edge 100px, sample 120px로 최소 20px 밖이다. PASS |
| 13 | overlap sample이 first item frame 밖이라 빈 화면을 읽을 수 있음 | 첫 item 끝은 152px이고 표본 120px는 item frame 안이다. PASS |
| 14 | 10자/15자 text가 예상한 intrinsic base를 만들지 못할 수 있음 | Ahem 10px advance와 no-wrap 15자 문자열에서 독립 min-content 기대값 150px를 도출한다. PASS |
| 15 | row scene이 horizontal writing mode로 잘못 고정될 수 있음 | Row A·span·C에 `vertical-rl`을 지정하고 Row CSSOM/frame을 별도 관측한다. PASS |
| 16 | gap 또는 fixed track이 계산에서 빠질 수 있음 | 각 축 target content 96px, two gaps 20px, fixed track 40px를 분리해 36px을 산출한다. PASS |
| 17 | target tracks 기대가 Chrome CSSOM에서 역산될 수 있음 | `expected.json`의 계산과 browser observation을 분리하고 CRD algorithm을 출처로 표기한다. PASS |
| 18 | root frame이 viewport frame과 혼동될 수 있음 | Observation에는 root viewport rect와 item root-local rect/sample을 별도 필드로 저장했다. PASS |
| 19 | DPR/분수 좌표 rounding으로 screenshot 표본을 잘못 읽을 수 있음 | DPR 1에서 viewport CSS 좌표를 `floor`해 raster pixel을 적고 각 pixel을 Pillow로 재조회했다. PASS |
| 20 | sample list에 선언과 실제 색이 불일치할 수 있음 | 8개 전부 PNG에서 다시 읽어 sample JSON의 실제 RGB 및 기대 hex와 일치시켰다. PASS |
| 21 | screenshot이 다른 실행의 오래된 산출물일 수 있음 | 현재 pass03 Agent Browser screenshot을 evidence 폴더에 저장하고 file hash와 dimensions를 기록했다. PASS |
| 22 | README hash가 실제 screenshot과 다를 수 있음 | SHA-256 `59ac9e5783142727781df1855b932c232eb99305a294a611de27f94cdae60a79`, 1280×633으로 대조했다. PASS |
| 23 | hidden과 visible 결과 차이가 레이아웃 변화일 수 있음 | 두 control의 root, used tracks, item frames가 동일하고 visibility 값만 다르다. PASS |
| 24 | `display:none`과 `visibility:hidden`이 혼동될 수 있음 | Browser observation은 span display `block`, hidden visibility `hidden`, client rect 1을 기록했다. PASS |
| 25 | image가 sample color를 오염시킬 수 있음 | PNG는 local x/y 162px부터 시작하며 overflow sample target-axis 값은 120px이다. PASS |
| 26 | screenshot이 전체 pixel parity를 뜻한다고 문서가 확대할 수 있음 | README, expected JSON과 plan 모두 선택한 좌표만 검사하며 전체 surface equality를 주장하지 않는다. PASS |
| 27 | review 문서 자체가 implementation 검토를 대신할 수 있음 | 20관점 문서는 plan-only이고 구현/runtime 20관점은 이후 구현 PR에서 새로 수행한다고 계획에 분리했다. PASS |
| 28 | public roadmap이 구현 완료를 암시할 수 있음 | 새 항목에 Chrome oracle/계획 상태와 Simulator·runtime 검증 전임을 명시했다. PASS |
| 29 | 공식 상태 숫자나 C11.4 checkbox가 조기 승격될 수 있음 | `spec/STATUS.md`는 변경하지 않았고 public note가 C11.4 미완료, `72/213`을 명시한다. PASS |
| 30 | 기존 matrix 수를 잘못 세어 후속 범위가 빠질 수 있음 | 상위 plan의 12-scene 문장을 evidence와 맞춰 14개로 고쳤고 후속 계획은 그 14개+신규 4개=18개로 계산한다. FIXED→PASS |

최종 결과: 30개 관점 모두 PASS(사전 검토 중 발견한 border sample 가림과 상위 matrix count는 수정 뒤 재검토했다). `git diff --cached --check`와 evidence JSON syntax/pixel consistency 검사는 통과했다. 전체 `mise exec -- bun run docs:build`는 기존 문서의 dead links와 MDX loader 오류로 실패했다(두 번째 로그: 1,001 dead-link groups, 840 MDX loader failures). 링크를 수정한 뒤 생성한 로그에서 이 PR의 plan·evidence·fixture 경로는 오류 목록에 나오지 않는다. build 실패를 이 변경의 성공이나 실패로 과장하지 않는다. 이 문서의 관찰은 계획 PR diff에 한정되며 runtime 구현 검토로 재사용하지 않는다.
