# C11.4 Tailnet 동기화 PR 변경 적대 검토 · 30개 관점

**대상:** 이 PR의 source link 변경과 선택 렌더·동기화·브라우저 증거 전체.
**기준:** PR diff와 최종 Agent Browser 화면을 병합 전에 다시 확인했다. 앱 코드는 바뀌지 않는다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 상태 대장에서 C11.4가 실수로 완료 처리되는가 | 체크박스는 미해제다. |
| 2 | C11.4 설명이 현재 구현 범위를 과장하는가 | text intrinsic API와 Taffy callback까지만 기록하고 image·track sizing·auto-repeat·runtime을 미완료로 남겼다. |
| 3 | 상위 C11도 완료로 읽히는가 | 상태 문장은 C11.4와 C11 모두 미완료라고 유지한다. |
| 4 | 병합 뒤 로드맵 완료 수가 부풀려지는가 | Agent Browser 화면과 Tailnet 원본에서 `72/212`를 확인했다. |
| 5 | 212개 집계의 구성 설명이 서로 어긋나는가 | 화면 설명은 상위 78개 + 세부 ID 132개 + ID 없는 하위 체크 2개로 합계 212개다. |
| 6 | 계획 링크가 preview base 밖으로 이동하는가 | 상태 문서 링크를 canonical GitHub blob URL로 바꾸고 DOM href를 확인했다. |
| 7 | 계획 목적지가 잘못된 문서나 브랜치를 가리키는가 | 기존 `plan/c11-4-grid-track-sizing.md` main 경로를 가리킨다. 링크 클릭 후 GitHub 문서 렌더까지는 별도로 검증하지 않았다. |
| 8 | C11.4 사전 비교 링크가 경로 밖으로 나가는가 | 내부 route `/spinon/internal/evidence/.../README.html`로 열린다. |
| 9 | inline text 계약 링크가 경로 밖으로 나가는가 | 내부 route `/spinon/internal/0070-c15-1-inline-text-layout.html`로 열린다. |
| 10 | 선택 빌드가 고정 RSPress 버전을 바꾸는가 | 고정 버전 `2.0.22`를 사용했다. |
| 11 | 부분 렌더 결과를 전체 문서 build로 오해하게 하는가 | README와 build 결과가 13 Markdown route + 정적 roadmap의 선택 범위로 한정한다. |
| 12 | 404 문서와 정적 roadmap을 Markdown route 수에 잘못 더하는가 | 13 Markdown route, 별도 roadmap 1개, 404 및 asset 포함 42개 출력으로 구분했다. |
| 13 | 전체 dead-link 검사를 통과했다고 주장하는가 | 전체 dead-link/image 검사 성공을 주장하지 않는다. |
| 14 | GitHub Pages가 사용자 요청 없이 갱신되는가 | Pages workflow/deploy는 실행하지 않았다. |
| 15 | fixture 링크가 잘못된 상대 경로로 남는가 | precomparison fixture 링크는 canonical GitHub URL이다. |
| 16 | image fixture 링크가 잘못된 로컬 경로로 남는가 | precomparison 문서에서 GitHub fixture 경로로 바꿨고 최종 화면의 링크 목록 및 pass 3 browser 기록을 확인했다. |
| 17 | Ahem font fixture 링크가 preview root 밖으로 나가는가 | canonical GitHub URL이며 `badLocalLinks`는 빈 배열이다. |
| 18 | 계획 링크가 사전 비교 문서에서 깨지는가 | canonical GitHub URL로 열리는 href를 확인했다. |
| 19 | 사전 비교 case 수가 이전 oracle보다 줄거나 늘어나는가 | 26개 case 표기가 최종 페이지에 있다. |
| 20 | Chrome oracle 증거가 기준 버전·환경 없이 요약되는가 | 고정 Chrome 155 metadata와 observation hash를 연결한 기존 자료를 유지했다. |
| 21 | inline edge 설명이 표의 관찰값과 충돌하는가 | 최종 페이지에서 75px, 110px, 15px, 45px를 확인했다. |
| 22 | inline font readiness가 불명확한가 | `document.fonts.check("10px Ahem")`가 true인 기록을 유지했다. |
| 23 | preview로 복사하는 경로 집합이 임의로 넓어지는가 | 명시적으로 선택된 72개 파일만 allowlist에 넣었다. |
| 24 | 기존 preview 파일을 백업 전에 덮어쓰는가 | 변경 대상 15개를 먼저 백업했고 SHA-256 15/15가 일치한다. |
| 25 | copy 과정에서 무관한 파일을 지우는가 | 최종 대상 루트에서 deletion 0건이다. |
| 26 | 파일이 부분적으로 읽히는 동안 쓰이는가 | 같은 디렉터리 temp file 작성 후 atomic replace를 사용했다. |
| 27 | 복사 완료를 원본 해시만으로 판정하는가 | 로컬 target hash 72/72를 확인했다. |
| 28 | HTTP 성공 응답이 이전 또는 다른 바이트를 제공하는가 | served URL 72개 모두 HTTP 200 및 SHA-256 일치다. |
| 29 | URL 접두부와 filesystem root를 혼동하는가 | 서버 소스가 `/spinon/`을 제거해 preview root에 매핑함을 확인했다. 잘못된 nested prefix 시도에서 새로 생성한 72개 파일만 해시 검증 후 제거했고 기존 preview 파일은 덮어쓰지 않았다. [`path-correction.json`](path-correction.json)에 기록했다. |
| 30 | 브라우저 검증 흔적을 남긴 뒤 Agent Browser가 계속 실행되는가 | 종료 후 `active=false`, `pid=null`, 관련 process/listener 없음이다. |

**남은 한계:** GitHub blob 목적지의 live page fetch, 전체 문서 링크 검사, 실제 iOS/Android 앱·simulator 동작은 이 docs-only PR에서 검증 범위가 아니다.
