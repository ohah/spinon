# C11.4 six-fit 계획 PR diff 검토 20개

**검토 대상:** [PR #1353](https://github.com/ohah/spinon/pull/1353) 계획·사전 비교 변경분

**검토한 원격 head:** `fc22a202944c9ebd2026d9abe30d8dc5df1caf18`

**base:** `389cd0ef98f97bc125e3cd4854996977de0c2bac`

**원격 diff:** 10,413,208 bytes, SHA-256 `21463e49dc910a508887587f232f17a98316d79286a7a492efcf041927de2f6`

**로컬 대조:** `git diff origin/main...HEAD`와 `gh pr diff 1353`의 byte content 및 SHA-256 일치. 변경 파일 13개.

첫 검토에서 독립 verifier가 fixture hash와 implicit row의 실제 item y 좌표를 확인하지 않는 점을 찾았다. `fc22a20`에서 fixture SHA-256, scene identity, finite/non-negative row 값, control minimum, row→item 위치, payload 크기·위치를 검산하도록 보강했다. 전체 5,760 scene을 다시 실행한 결과 요약은 변하지 않았고, 아래 20개 관점은 수정된 원격 diff를 대상으로 다시 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR에 현재 과제 밖의 파일이 섞임 | `gh pr diff --name-only`의 13개 경로가 six-fit 계획, 사전 비교 근거, 상태·로드맵 변경으로 한정됨. 통과. |
| 2 | 문서 계획 PR이 제품 구현을 포함 | 변경 목록에 Rust, Kotlin, Swift, C/C++ 제품 파일이 없음. 통과. |
| 3 | 공식 완료 수가 바뀌거나 상위 체크가 완료됨 | `spec/STATUS.md`는 `78/214`를 유지하고 C11.4/C11 미완료를 명시함. 통과. |
| 4 | internal contract 숫자 버전을 올림 | `0.1.0`을 그대로 둠. 통과. |
| 5 | 사전 비교만으로 구현 지원을 주장 | plan/evidence/PR 본문이 Chrome CSS oracle와 Spinon RuntimeSession 증거를 구분함. 통과. |
| 6 | W3C 조건을 잘못 인용 | §11.2 fit-content minimum `auto`, §6.6 eligibility와 cap 설명이 연결됨. 통과. |
| 7 | Chrome source 링크가 관측값의 대체가 됨 | Chromium source는 버전 참고로만 설명하고 기대값은 브라우저 실행 결과에서 가져옴. 통과. |
| 8 | 다섯-fit tail track을 six-fit list에 남김 | 계획과 fixture 모두 여섯 authored track 전부 `fit-content()`임. 통과. |
| 9 | 실제 계획 admission이 관측 cap 집합보다 넓음 | exact `{0px, 10px, 20%, 30px, 40px, 50px}` multiset만 허용하고 duplicate/missing/other cap을 거부함. 통과. |
| 10 | percentage cap basis가 scene마다 달라짐 | definite 220px Grid block size를 고정; `20% = 44px`로 문서화함. 통과. |
| 11 | matrix scene 수 또는 axis 조합이 잘못 계산됨 | capture의 720×2×2×2=5,760 scenes 및 verifier의 2,880 paired key 검사를 확인함. 통과. |
| 12 | baseline/candidate가 minimum 외 입력도 다르게 가짐 | scene generator는 같은 list/alignment/DOM order pair에서 `min-height`만 바꿈. 통과. |
| 13 | explicit empty row가 candidate content minimum에 참여 | fixture와 verifier가 control의 computed `min-height:0`을 검사함. 통과. |
| 14 | 7·8번째 item의 implicit list wrapping이 누락 | 8개 item을 유지하고 used row index를 6-member cycle로 확인함. 통과. |
| 15 | item frame y가 row sequence와 달라도 검산 통과 | 수정된 verifier가 각 item top을 앞선 explicit/implicit row 합과 비교함. 통과. |
| 16 | payload line box와 outer item height가 혼동 | verifier가 item/payload 좌표·폭·높이와 line count를 별도 검사함. 통과. |
| 17 | matrix 일부만 기록되거나 fixture가 바뀌어도 기존 결과 사용 | 5,760 연속 scene ID, 720 unique track list와 fixture SHA-256을 검사함. 통과. |
| 18 | Agent Browser 세션이 실행 후 남거나 다른 작업 session을 종료 | 전용 session은 `finally`에서 닫았다. `session info --json`은 `active:false`, `pid:null`이고 unrelated HWP session은 보존됨. 통과. |
| 19 | PR screenshot이 다른 결과 또는 이동 가능한 경로를 가리킴 | screenshot 링크가 capture commit의 raw image SHA/path에 고정됨. 통과. |
| 20 | 로컬에서 본 변경분과 GitHub PR diff가 다름 | 원격 diff와 `origin/main...HEAD` bytes 및 SHA-256이 일치함. 통과. |

## 판정

수정 뒤 위 20개 관점에서 추가 수정 결함을 찾지 못했다. GitHub check-run/workflow는 0개라 CI 통과로 세지 않는다. PR은 계획·사전 비교 범위만 다루며 C11.4/C11 완료를 뜻하지 않는다.

## 문서 빌드 확인

`mise exec -- bun run docs:build`는 exit code 1로 끝났다. 전체 checkout의 RSPress link check가 1,543개 `Dead links found` 진단을 보고했으며, 다수는 기존 evidence 첨부와 기존 `STATUS.md`의 다른 상대 plan link다. 첫 빌드 로그에는 이 PR에서 추가한 두 상대 plan link도 포함되어 있어 GitHub `main` blob link로 바꾸고 재실행했다. 두 번째 전체 빌드에서도 기존 진단은 남았지만 새 six-fit plan/evidence 경로 및 수정한 STATUS link는 오류 대상에 없었다. 관련 없는 기존 링크는 이 PR 범위에서 수정하지 않았다. 따라서 문서 build 성공이나 GitHub CI 성공을 주장하지 않는다.

## 후속 diff 재검토

위 20관점 검토 뒤 변경한 raw asset 링크, STATUS plan link, `pid:null` lifecycle 확인, 문서 빌드 결과와 검토 근거 파일을 다시 대조했다. 이 갱신 직전 원격 head `40d44e7dfcf82825c6b2e8bcd2f37de3263242e7`의 14-file diff는 10,420,594 bytes이며 원격 `gh pr diff 1353`와 로컬 `origin/main...HEAD`의 byte content/SHA-256이 일치했다 (`4eaca193bfeb1f1a3086657c976293b2d10a2e72f04d8d7bc2f307c6663280be`). 새 six-fit Markdown은 빌드 오류 대상이 아니고, 첨부 파일은 commit-pinned raw link를 쓴다. 이 후속 변경으로 위 20개 판정은 달라지지 않았다.
