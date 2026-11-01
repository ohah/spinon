# C11.4 Unicode 계획 로드맵 동기화 PR diff 검토 30개

검토 대상은 기준 `2cfc2f09aa8a03b50aaef9db9d1895ef44f10d34` 대비 공개 로드맵 문구, 계획 사전 비교 문서의 링크 수정, 그리고 Agent Browser 캡처·이 근거 문서다. 코드는 수정하지 않았다.

| # | 독립 실패 관점 | 결과 |
|---:|---|---|
| 1 | 계획이 실제로 병합되지 않았는데 완료 PR처럼 소개한다 | PR #989의 Rebase and merge 시각과 merge SHA를 병합 기록과 대조했다. |
| 2 | 계획 카드가 구현 카드처럼 읽힌다 | 제목을 계획 slice로 한정했다. |
| 3 | C11.4 상위 항목이 완료된 것으로 오해된다 | 카드에서 C11.4 미완료를 명시했다. |
| 4 | C11 전체 Grid 지원이 끝난 것으로 오해된다 | C11 미완료와 공개 CSS 미지원 경계를 유지했다. |
| 5 | 공식 완료 수가 바뀐 것으로 오해된다 | `72/213`을 명시하고 `spec/STATUS.md`는 변경하지 않았다. |
| 6 | 구현 또는 runtime 검증이 이미 실행된 것으로 오해된다 | 구현/runtime/Simulator 검증이 시작되지 않았다고 썼다. |
| 7 | 기존 ASCII `capitalize` 기준이 누락된다 | 계획 카드가 PR #986 후속임을 원 계획에서 확인했다. |
| 8 | 계획의 scene 수가 잘못 요약된다 | 이전 12개와 Unicode 11개, 합계 23개가 원 계획과 일치한다. |
| 9 | Unicode 규범 기준이 모호하다 | CSS Text Level 3 CRD 날짜와 Unicode 17 기준을 카드에 표시했다. |
| 10 | 단어 분절 기준의 버전이 누락된다 | UAX #29 revision 47을 원 계획과 대조했다. |
| 11 | 지원 범위가 임의 locale로 확대된다 | 직접 `lang=en`인 고정 profile이라고 제한했다. |
| 12 | Chrome 동작이 규범 기대값으로 승격된다 | `ßeta`의 Chrome 관측과 규범 `Sseta` 차이를 명시했다. |
| 13 | Chrome oracle screenshot이 Spinon 앱 화면으로 오해된다 | 로드맵 캡처를 문서 화면으로 구분하고 앱 동작 증거로 쓰지 않았다. |
| 14 | 병합 시각 또는 SHA가 잘못되었다 | 카드 값과 `main`의 #989 merge commit을 대조했다. |
| 15 | 카드가 잘못된 역사적 위치에 들어간다 | 같은 C11.4 anonymous text-transform 기록 인접 위치에 배치했다. |
| 16 | 계획 링크 route가 잘못되었다 | 로컬 `plan/` 원본 및 기존 `/spinon/plan/*.html` route 규칙과 대조했다. |
| 17 | 사전 비교 README 링크가 잘못되었다 | 해당 evidence 원본과 정적 route를 확인했다. |
| 18 | 계획 실패 검토 20개 링크가 잘못되었다 | 원본 파일과 route를 확인했다. |
| 19 | 계획 PR diff 검토 30개 링크가 잘못되었다 | 원본 evidence 파일과 route를 확인했다. |
| 20 | 공식 상태 링크가 빠진다 | 로드맵 카드에 STATUS anchor를 유지했다. |
| 21 | 상대 JSON 링크가 RSPress에서 가짜 `.json.html` route로 변환된다 | JSON은 명시적 HTML anchor로 연결했다. |
| 22 | fixture 링크가 Markdown resolver에서 잘못 해석된다 | fixture 경로는 저장소 경로 텍스트로 표시했다. |
| 23 | 넓은 화면에서 신규 카드가 누락된다 | Agent Browser 넓은 화면 캡처에서 Unicode 카드가 표시되는 것을 확인했다. |
| 24 | 390px 화면에 가로 overflow가 생긴다 | 모바일 viewport와 document/body scroll width가 모두 390이었다. |
| 25 | 좁은 화면에서 PR 번호나 완료 수가 누락된다 | 모바일 rendered text에 PR #989와 `72/213`이 있었다. |
| 26 | 캡처 파일이 저장소에 없거나 이름이 불일치한다 | 넓은 화면·모바일 캡처의 경로와 실제 파일을 확인했다. |
| 27 | 캡처가 이전 Agent Browser 화면을 잘못 재사용한다 | 현재 변경된 로컬 roadmap을 해당 세션에서 열어 캡처했다. |
| 28 | 브라우저 프로세스가 세션 종료 뒤 남는다 | 종료 후 `agent-browser session list`에서 활성 세션이 없었다. |
| 29 | 전체 docs 빌드 실패를 성공으로 표현한다 | 저장소 다른 문서의 dead-link 때문에 exit 1임을 README에 기록했다. |
| 30 | PR 문서 작업으로 runtime 또는 Tailnet 배포까지 끝났다고 오해된다 | Tailnet은 병합 뒤 별도 갱신, 구현·Simulator는 후속으로 명시했다. |

## 판정

검토한 변경은 계획 상태와 공식 완료 수를 그대로 보존하는 문서 동기화다. 전체 RSPress 빌드는 기존 dead-link 진단으로 실패했으며 성공으로 간주하지 않는다. Unicode 구현, Simulator 검증, Tailnet 미리보기 갱신은 이 diff의 완료 범위가 아니다.
