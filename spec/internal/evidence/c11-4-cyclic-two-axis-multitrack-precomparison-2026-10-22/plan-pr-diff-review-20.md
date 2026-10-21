# C11.4 동시 cyclic percentage 다중 track 계획 PR 변경 검토 20개

검토 대상: 계획 PR 변경분 전체. 계획 본문, 부모 C11.4 계획의 최근 병합 상태와 신규 slice 연결, fixed HTML 입력, Chrome 156 observation/screenshot, 독립 기대값, 별도 계획 검토 20개를 각각 대조했다. 이 검토는 구현 PR 검토로 재사용하지 않는다.

| # | 독립 실패 관점 | 판정과 근거 |
| ---: | --- | --- |
| 1 | PR #474의 이미 병합된 fit-content slice가 parent plan에 계획 단계로 남음 | parent plan을 PR #474 병합 상태로 바꾸고 동일한 공식 미완료 범위를 유지했다. |
| 2 | C11.4 공식 완료 check나 전체 합계 `72/213`을 계획 PR에서 승격 | `spec/STATUS.md`를 변경하지 않았고 계획 문서에서 `72/213` 미승격을 명시했다. |
| 3 | newly added precomparison HTML이 repo의 기존 fixture 경로와 겹침 | 고유 파일명으로 `tests/fixtures/css/c11/`에 추가했고 기존 입력을 수정하지 않았다. |
| 4 | 문서의 fixture 링크가 plan 디렉터리 기준 상대 경로에서 잘못 해석 | 계획 링크가 `../tests/...`와 `../spec/...`를 가리키는지 path 존재로 대조했다. |
| 5 | 4개 scene이 계획표·HTML id·Chrome JSON에서 서로 다름 | 네 `sceneId`와 세 node의 `id`가 각 산출물에서 일치한다. |
| 6 | input fixture에 보이지 않는 extra item/implicit track이 생김 | 각 root는 Ahem text 하나와 세 번째 explicit cell의 overflow item 하나뿐이다. |
| 7 | 두 축 track list에 mixed sizing function이 없음 | cyclic axes는 각 `50% 25% 20px`으로 percentage 두 개와 fixed length 하나를 쓴다. |
| 8 | cyclic scene의 intrinsic root basis가 CSSOM observed final size에서 역산됨 | Ahem contribution과 intrinsic pass track 구성으로 40×20px을 별도로 산출한다. |
| 9 | intrinsic 사이클 단계와 final percentage resolution이 문서에서 합쳐짐 | §11 container-size pass와 final grid-sizing pass를 분리해 기록했다. |
| 10 | inline/block percentage basis를 서로 바꾸어 적용 | 40px inline basis와 20px block basis를 분리하고 scene별 배열을 독립 계산했다. |
| 11 | columns-before-rows algorithm 단계가 빠져 row text contribution이 잘못됨 | §11.1 column-first 순서와 first-cell 20px available inline size를 설명한다. |
| 12 | overflow item이 fixed last track이나 root intrinsic basis를 추가로 키움 | fixed child 20×10px이 마지막 fixed track과 같고 root basis 합산에 반영됨을 명시한다. |
| 13 | one-axis controls가 다른 축의 cyclic percentage도 포함 | columns-only와 rows-only scene은 반대 축을 explicit fixed lengths로 고정한다. |
| 14 | definite control이 intrinsic control과 동일한 route라고 오해 | definite 40×20px root를 별도 scene으로 두고 직접 percentage resolution을 설명한다. |
| 15 | root 밖 overflow 위치가 parent box 바깥 좌표인지 증거가 없음 | scene root-local `(30,15) 20×10`과 root `40×20`에서 overflow 10×5px을 계산한다. |
| 16 | pixel sample point가 overflow box 바깥이거나 root 안쪽임 | 계획은 `(root.x+45, root.y+22)`를 overflow item 및 root 외부에 놓는다. |
| 17 | screenshot을 Spinon runtime/simulator 결과로 제시 | evidence와 plan에서 Chrome 156은 단일 precomparison oracle라고 한정한다. |
| 18 | JSON/screenshot/input hash가 실제 파일과 다른 내용에 연결 | 저장한 observation JSON을 다시 파싱했고 세 파일 hash를 staged artifact와 대조했다. |
| 19 | 브라우저 종료 증거가 없거나 타인의 프로세스를 종료 | 전용 session의 `close`가 성공했고 post-close diagnostic은 `browserLaunched:false`, `pageCount:0`이며 CLI 종료 후 process check에서 Agent Browser/HeadlessChrome을 찾지 못했다. 다른 Chrome은 건드리지 않았다. |
| 20 | 계획 PR이 공개 API, version, WPT 또는 implementation completion을 암시 | 신규 API/version은 없고 runtime, Simulator, WPT, real-device 범위를 구현 단계로 남긴다. |

**판정:** 계획 입력과 독립 기대값은 현재 CSS Grid 규범 단계 및 Chrome 156 보조 관찰과 일치한다. 구현 또는 Simulator를 이미 실행한 것처럼 주장하는 부분은 없다. 차단 결함을 찾지 못했다.
