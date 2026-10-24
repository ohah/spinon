# C11.4 음수 percentage 계획 PR diff 실패 관점 검토 30개

대상 변경은 C11.4 음수 percentage auto-repeat calc neighbor 계획과 이를 뒷받침하는 Chrome 156 fixture/evidence, 공식 상태 대장 정정, Tailnet 미리보기 동기화 evidence다. 검토 범위는 실제 PR diff와 저장된 동기화 산출물이며, 앱 runtime이나 기능 구현을 검증하지 않는다. 계획 자체의 20개 검토는 별도 문서다.

| # | 실패 관점 | diff 대조와 판정 |
|---:|---|---|
| 1 | 이전 #583 상태가 미병합으로 남음 | 계획과 공식 상태에서 PR #583의 병합 시각 및 SHA를 같은 값으로 기록했다. 통과. |
| 2 | 이전 구현 계획이 현재 진행 중으로 오독됨 | 선행 계획은 #583 병합 상태와 구현 경계를 명시한다. 통과. |
| 3 | C11.4가 완료로 승격됨 | 상태 대장과 새 계획은 부모 checkbox를 그대로 두고 `72/213`을 유지한다. 통과. |
| 4 | 활성 작업 lane이 다른 항목으로 바뀜 | 공개 roadmap 원본의 active lane과 상태 대장 C11.4 기록은 일치한다. 통과. |
| 5 | 계획을 구현 지원으로 표현함 | 새 계획은 현재 projection gate가 signed percentage AST를 거부한다고 밝힌다. 통과. |
| 6 | CSS 문법 유효성을 runtime 지원과 혼합함 | `calc(20px - 5%)`의 CSS-valid 관측과 Spinon 미지원 상태를 분리했다. 통과. |
| 7 | Chrome 관측을 Spinon 결과로 표시함 | 계획, evidence README, status entry 모두 이를 Chrome 사전 비교로 한정한다. 통과. |
| 8 | 9-scene 수가 서로 다름 | fixture와 observation JSON은 같은 9개 scene ID를 가진다. 통과. |
| 9 | 열·행 percentage basis가 뒤바뀜 | fixture와 독립 계산은 column inline axis 및 row block axis를 각각 고정한다. 통과. |
| 10 | prefix·suffix 위치가 누락됨 | 긍정 final case가 양 축의 prefix/suffix를 각각 포함한다. 통과. |
| 11 | negative final clamp 장면이 빠짐 | row/column negative final 및 signed length와 percentage가 함께 음수인 control이 있다. 통과. |
| 12 | border-box outer size를 basis로 사용함 | 독립 oracle는 136px outer에서 padding/border를 제외한 116px content basis를 사용한다. 통과. |
| 13 | auto-fit을 auto-fill로 취급함 | occupied 반복 track 하나와 collapse되는 네 track을 별도 scene으로 둔다. 통과. |
| 14 | Chrome 화면과 Tailnet 페이지 화면을 혼동함 | Chrome fixture PNG와 Tailnet plan-page PNG는 각각 다른 SHA-256으로 기록되어 목적을 구분한다. 통과. |
| 15 | 브라우저 환경 재현 정보가 불충분함 | Agent Browser/Chrome 버전, macOS ARM64, viewport, DPR을 기록한다. 통과. |
| 16 | CSSOM 반올림을 정확한 실수값으로 주장함 | validator 허용치를 1/64 CSS px로 제한하고 Chrome 양자화를 적었다. 통과. |
| 17 | 기대값을 Chrome 출력에서 역산함 | validator는 fixture 입력과 고정 minimum/gap을 독립 계산한 뒤 CSSOM과 비교한다. 통과. |
| 18 | 검증 스크립트가 계획 경로와 다름 | 문서 명령의 validator 경로와 저장된 스크립트가 일치한다. 통과. |
| 19 | 예시 fixture가 사용자 문서에 공개됨 | 테스트 fixture는 `tests/fixtures`에 놓고 status에서 내부 evidence로 연결한다. 통과. |
| 20 | 새 계획 경로가 링크 별칭에 따라 깨짐 | RSPress 산출물에서 `plan/` 및 `spec/plan/` 별칭을 확인했다. 통과. |
| 21 | precomparison README 상대 asset 링크가 끊김 | `internal/evidence/`와 `spec/internal/evidence/`의 HTML 및 원본 asset 경로를 함께 선정했다. 통과. |
| 22 | build 검증에 중복 route가 들어감 | pass02 route 목록은 unique set으로 정리하고 16개 경로를 검사한다. 통과. |
| 23 | preview 전체 디렉터리를 덮어씀 | Tailnet sync는 명시된 선택 경로만 갱신하고 나머지 파일을 삭제하지 않는다. 통과. |
| 24 | 교체 전 파일이 보존되지 않음 | 사전 비교가 교체될 2개 파일을 표시했고 두 파일 모두 별도 backup hash로 확인했다. 통과. |
| 25 | preview 쓰기 전 현재 파일이 바뀜 | copy 단계는 preflight 당시 존재 여부, 크기, hash를 다시 검사한다. 통과. |
| 26 | local copy가 잘렸거나 다른 파일로 교체됨 | 선택된 16개 local target 모두 source byte count와 SHA-256이 일치한다. 통과. |
| 27 | Funnel 응답이 오래된 cache에서 옴 | 고유 `sync` query와 no-cache 요청으로 16개 원격 route를 재조회했다. 통과. |
| 28 | 원격 route가 오류 페이지를 반환함 | 선택 경로 16개가 HTTP 200이며 build source와 응답 body SHA-256이 일치한다. 통과. |
| 29 | 확인 후 Agent Browser가 남아 있음 | Agent Browser 0.31.1에서 상태/계획 DOM과 화면을 확인하고 close-all 뒤 관리 브라우저 프로세스가 없음을 확인했다. 통과. |
| 30 | 브라우저 screenshot을 앱 동작 근거로 오해함 | 저장한 이미지는 Tailnet 계획 문서 화면이며 Chrome CSS fixture와 앱 runtime evidence를 구분한다. 통과. |

## 판정

검토에서 병합을 막는 diff 결함은 발견되지 않았다. 이전 #583 merge 상태, 완료 집계, Chrome-only oracle 경계, 링크 경로, Tailnet 백업·hash 검증을 대조했다. PR은 계획과 사전 비교만 추가하며 제품 코드·지원 범위·공식 완료 수를 변경하지 않는다.
