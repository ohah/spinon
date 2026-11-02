# C11 Root HTML column-flow 계획 PR 변경분 적대 검토 30개

**기준:** `b5148f3e0e39674b9c30ac3e9ced8da3be0bb91e` 기반 `plan/c11-root-html-grid-column-flow` staged diff. 범위는 계획·상위 계획 연결·고정 fixture·독립 기대값·Agent Browser 관측/화면·해시·계획 검토 기록이다. 제품 코드 변경 PR이 아니다.

| # | 별도 실패 관점 | diff에서 확인한 통제 | 판정 |
|---:|---|---|---|
| 1 | 이미 병합된 Root HTML candidate를 다시 신규로 보고한다 | 현재 `main`의 plan 목록과 projection gate를 대조했고 column-flow dedicated plan은 없었다. | 통과 |
| 2 | 계획 PR을 구현 완료로 소개한다 | master plan과 전용 계획은 구현/runtime/platform 검증을 아직 하지 않았다고 명시한다. | 통과 |
| 3 | C11.1 또는 상위 C11을 체크한다 | 계획, master note, root integration note 모두 상위 항목을 미완료로 유지한다. | 통과 |
| 4 | 제한 slice만으로 공식 count를 올린다 | 세 문서 모두 `72/213` 유지 조건을 적었다. | 통과 |
| 5 | 새 계획을 C11 master에서 찾지 못한다 | `plan/c11-css-grid.md` 첫 부분에 새 계획과 사전 비교 링크를 연결했다. | 통과 |
| 6 | Root HTML 통합 계획과 새 계획이 충돌한다 | root integration plan은 구현 전에 이전 rejection이 유지된다고 쓰고 별도 후속 링크만 추가한다. | 통과 |
| 7 | C11 status anchor나 internal contract 링크가 깨진다 | 전용 계획의 상태 대장 및 내부 계약 상대경로가 존재하는지 확인했다. | 통과 |
| 8 | CSS Grid §7.7의 다른 개념을 column-flow 규칙으로 인용한다 | W3C CSS Grid Level 2 §7.7 Automatic Placement를 연결했다. | 통과 |
| 9 | CSS Grid §8.5 algorithm source를 누락한다 | auto-placement algorithm 링크를 포함하고 column이면 row/column 언급을 바꾸라는 규칙을 기준에 적었다. | 통과 |
| 10 | 현재 W3C draft를 최종 Recommendation으로 오인한다 | 2025-03-26 Candidate Recommendation Draft라고 evidence와 plan에 명시한다. | 통과 |
| 11 | CSSOM 관측만을 독립 정답으로 삼는다 | track과 item frame을 규칙 및 수치 산술로 먼저 구하고 Chrome은 교차 확인으로 한정했다. | 통과 |
| 12 | columns의 합이 root width와 다르다 | `30 + 70 = 100px`이며 fixture root width와 expected root frame이 100px이다. | 통과 |
| 13 | rows의 합이 root height와 다르다 | `20 + 40 = 60px`이며 fixture root height와 expected root frame이 60px이다. | 통과 |
| 14 | item fixed size가 할당 cell과 다르다 | 네 item의 CSS size와 expected size가 각 행·열 track과 일치한다. | 통과 |
| 15 | implicit initial gap을 0이 아닌 값으로 계산한다 | fixture에서 gap을 선언하지 않아 initial 0px를 사용하며 expected constraints도 0이다. | 통과 |
| 16 | body 기본 margin 또는 box edge가 frame origins를 바꾼다 | `html, body` margin 0, `box-sizing:border-box`, border/padding 미지정 및 expected edge 0을 고정했다. | 통과 |
| 17 | item 순서에 맞는 column-flow frame이 아니거나 row flow 결과를 복사한다 | A/B/C/D 각각의 row·column·frame을 구분해 열 우선 배치로 독립 기입했다. | 통과 |
| 18 | Chrome screenshot이 다른 fixture에서 생성됐다 | 관측 metadata와 fixture 경로가 같은 고정 source를 가리키고 screenshot을 직접 열어 2×2 색 배열을 확인했다. | 통과 |
| 19 | browser version이나 viewport 누락으로 관측을 재현할 수 없다 | Agent Browser 0.31.1, HeadlessChrome 156, CSS viewport 1280×633, DPR 1을 기록했다. | 통과 |
| 20 | 로컬 사용자 경로가 저장소 관측 metadata에 노출된다 | 절대 file URL 대신 `urlScheme`과 repository-relative fixture path를 보존한다. | 통과 |
| 21 | screenshot이 GPU surface evidence로 소개된다 | README는 Chrome fixture 화면으로만 분류하고 Spinon runtime·GPU pixels 실행은 안 했다고 명시한다. | 통과 |
| 22 | typed cascade와 projection이 이미 root column mode를 지원한다고 과장한다 | 코드 조사 기록은 enum mapping과 Root HTML projection/layout rejection을 구분한다. | 통과 |
| 23 | generic `DenseAutoPlacement`을 Root HTML 제품 계약으로 그대로 노출한다 | 새 계획은 별도 `RootHtmlColumnFlowV1` mode와 strict root gates를 제안한다. | 통과 |
| 24 | `column dense`를 sparse column으로 받아들인다 | 허용 값은 `Column` 하나로 제한하고 dense variants를 거부하도록 한다. | 통과 |
| 25 | authored item placement/span이 자동 배치로 무시된다 | 네 computed placement longhand 모두 auto인 조건과 authored placement reject를 명시했다. | 통과 |
| 26 | item 수가 explicit grid 용량을 넘어 implicit track을 만든다 | `visible item count ≤ columns × rows` gate와 5번째 item rejection을 명시했다. | 통과 |
| 27 | named area, repeat, named line이 제한 mode에 함께 유입된다 | 해당 조합을 새 mode에서 열지 않고 별도 rejection으로 둔다. | 통과 |
| 28 | fixed gap 조합에서 row/column gutter가 빠지거나 중복된다 | 계획에 3px row gap, 5px column gap 추가 control과 네 expected origin을 포함했다. | 통과 |
| 29 | 구현 PR의 검토 작업을 계획 검토와 혼합한다 | 계획 20, 구현/runtime 20, PR diff 30을 서로 별도 gate로 적었다. | 통과 |
| 30 | 이전 WIP/evidence가 포함되거나 plan PR에 제품 테스트 통과를 주장한다 | staged 변경은 계획/fixture/oracle 관련 11개 경로뿐이다. 전체 docs build는 저장소의 기존 dead link로 실패했으며, 새 evidence 경로 오류 수정 후 재실행 로그에는 이 후보 경로가 더 나오지 않는다. 제품/runtime 통과는 주장하지 않는다. | 통과 |

**판정:** 계획 PR diff 관점 30/30 통과. 초기 검토에서 hidden item을 fail-closed한다고 잘못 적은 범위를 수정하고 해당 계획 관점 5·11을 다시 확인했다. 전체 docs build가 새 README 링크도 보고해 fixture를 GitHub 링크로, JSON/해시 파일명을 inline code로, screenshot을 image embed로 바꿨다. 재실행은 기존 저장소 dead link 때문에 여전히 exit 1이지만 이 후보 경로는 새 로그에 없다. 이에 따라 관점 5·8·18·20·30을 재확인했다. 계획 PR 검토는 구현/runtime review 20개를 대신하지 않는다.
