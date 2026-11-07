# C11.4 percentage fit-content 계획 PR diff 적대 검토 30개

대상: [계획 PR #1267](https://github.com/ohah/spinon/pull/1267)의 최종 GitHub diff와 PR 본문. 기능 계획의 실패 관점 20개와 분리해 PR 산출물·경로·메타데이터를 검토했다.

| # | PR 변경분 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 계획 브랜치가 오래된 다른 작업 브랜치에서 분기 | `origin/main` tree에서 단일 checkout으로 시작했다. PASS |
| 2 | PR base가 최신 main이 아님 | PR base SHA는 `77f2216e3b81b9597bc7ed7c1c0ec716e41a70d2`이며 push 시 최신이었다. PASS |
| 3 | 원격 PR head가 로컬 검토 commit과 다름 | 각 push 뒤 PR head를 재조회했고, 최종 push 이후 GitHub metadata와 로컬 branch tip을 다시 대조한다. PASS |
| 4 | PR 제목의 type/scope 또는 한글 설명 형식 오류 | `plan(c11.4):` 접두어와 한글 제목을 사용했다. PASS |
| 5 | GitHub 작업 유형/영역 라벨 누락 | 기존 `documentation`, `area: docs` 라벨 두 개가 지정됐다. PASS |
| 6 | 변경 파일 범위에 제품 소스가 섞임 | 최종 11개 경로는 계획·fixture·비교·검토 근거뿐이며 제품 소스는 없다. PASS |
| 7 | 대량의 기존 untracked evidence가 stage됨 | 명시한 경로만 추가했고 다른 5,000여 untracked 경로는 보존했다. PASS |
| 8 | 계획 파일이 C11.4 상위 계약과 연결되지 않음 | STATUS, C11.4 track-sizing 계획, 내부 계약 0059 링크가 있다. PASS |
| 9 | 기준 implementation을 특정할 수 없음 | 선행 px implicit-row PR #1265를 직접 연결했다. PASS |
| 10 | W3C 참조가 검색 결과나 2차 문서로 연결 | W3C CSS Grid Level 2 규범 절에 직접 연결한다. PASS |
| 11 | 계획의 상대 링크가 repository root 기준으로 깨짐 | 계획·fixture·evidence 상대 경로를 계산해 모두 존재함을 확인했다. PASS |
| 12 | 계획 검토 파일이 PR에 빠짐 | 독립 계획 실패 관점 20개가 변경 파일에 들어 있다. PASS |
| 13 | 계획 변경과 실제 PR 변경 검토가 혼용 | 별도 계획 PR diff 30개 파일을 두고 PR 본문에서 링크한다. PASS |
| 14 | 증거 README와 원본 fixture 연결이 깨짐 | README의 4단계 상위 경로가 fixture로 정확히 연결된다. PASS |
| 15 | Chrome observation/expected JSON 구문 오류 | 두 JSON을 Python JSON parser로 읽었다. PASS |
| 16 | 관측값과 독립 기대값이 같은 출처로 뭉개짐 | `chrome-observation.json`과 `expected-values.json`을 분리했다. PASS |
| 17 | 사전 비교 screenshot이 PR 본문에 포함되지 않음 | commit-pinned raw PNG 링크가 PR body에 포함됐다. PASS |
| 18 | screenshot metadata가 실제 PNG와 다름 | 원본 파일과 PR 본문에 기록한 1280×845 크기가 일치한다. PASS |
| 19 | screenshot이 실제 비교 장면을 보여주지 않음 | Agent Browser 전체 페이지 캡처에서 일곱 장면과 label을 눈으로 확인했다. PASS |
| 20 | screenshot URL이 현재 브랜치 mutable 경로를 가리킴 | PR 본문 URL은 최초 plan commit `3953b7550`으로 고정됐다. PASS |
| 21 | PR image가 GitHub에서 404 또는 깨진 크기로 표시 | Agent Browser에서 `complete=true`, natural size 1280×845로 로드됐다. PASS |
| 22 | PR 표시 크기를 원본 이미지 크기로 잘못 기록 | 원본 크기와 축소된 rendered size를 metadata에서 구분했다. PASS |
| 23 | PR 페이지 캡처가 Chrome oracle screenshot과 혼동 | 별도 PR 페이지 screenshot 파일과 원본 browser screenshot을 구분했다. PASS |
| 24 | PR 확인 뒤 Agent Browser가 계속 실행 | 전용 session close 후 inactive, PID null, 세션 목록 empty를 확인했다. PASS |
| 25 | PR 화면 artifact에 불필요한 개인 정보 포함 | PR 본문과 공개 이미지 확인 화면 외 credential/secret이 없다. PASS |
| 26 | SHA256 manifest가 파일 생성 뒤 stale | 최종 계획/evidence 10개 파일을 재생성한 manifest로 검증했다. PASS |
| 27 | 해시 검증 명령이 저장소 root에서 실패 | `shasum -a 256 -c`가 manifest의 모든 파일에 대해 성공했다. PASS |
| 28 | whitespace/merge-marker 문제가 PR에 포함 | 최종 `git diff --check origin/main...HEAD`가 통과했다. PASS |
| 29 | 계획 단계가 구현/테스트 완료로 오해됨 | PR 본문은 제품 구현 미시작 및 구현 PR의 후속 검증을 명시한다. PASS |
| 30 | 빈 CI 결과를 green으로 보고하거나 review 누락을 감춤 | status/workflow run/review thread가 비어 있음을 확인했고 CI green 주장을 하지 않았다. PASS |

결과: 최종 diff 30/30 관점을 통과했고 수정이 필요한 결함은 찾지 못했다. 구현 코드가 없는 계획 PR이므로 기능 테스트는 실행하지 않았다. 구현 PR에서는 이 자료를 재사용하지 않고 새 구현/runtime 20관점과 별도 PR diff 30관점 검토를 수행한다.
