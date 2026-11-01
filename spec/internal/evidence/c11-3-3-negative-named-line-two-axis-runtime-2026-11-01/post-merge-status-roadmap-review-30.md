# PR #970 병합 후 공식 상태·로드맵 대조 30개

검토 대상은 PR #970의 GitHub merge metadata와 현재 `origin/main`에 반영된 공식 상태 대장, 내부 계약, 계획, 실행 evidence 및 raw public roadmap이다. 목적은 병합 기록과 제한 slice를 맞추고 C11 전체 완료로 확대 해석되지 않도록 하는 것이다.

| # | 실패 관점 | 확인 기준과 결과 |
|---:|---|---|
| 1 | GitHub PR이 실제로 merge되지 않았는데 문서가 완료라고 쓰는가 | `gh pr view 970` 상태는 `MERGED`다. PASS. |
| 2 | 병합 시간이 잘못 기록됐는가 | GitHub `mergedAt`은 `2026-11-01T05:59:35Z`다. PASS. |
| 3 | squash/merge commit으로 잘못 분류되는가 | PR 설정 기록과 merge result는 Rebase and merge다. PASS. |
| 4 | merge SHA를 PR head SHA와 혼동하는가 | merge SHA `dd178e7...`와 PR head `6ff576e...`를 별도 필드로 기록했다. PASS. |
| 5 | runtime capture source SHA가 docs-only tip과 섞이는가 | 앱은 fix commit `0954c9c...`에서 빌드됐고 docs/evidence commit은 별도다. PASS. |
| 6 | 미보고 GitHub checks를 성공으로 기록하는가 | PR check rollup이 비어 있어 CI 통과를 주장하지 않는다. PASS. |
| 7 | C11 상위 checkbox가 limited fixture 때문에 켜지는가 | `spec/STATUS.md`에서 C11은 계속 `- [ ]`다. PASS. |
| 8 | C11.3 parent checkbox가 slice merge로 완료되는가 | C11.3은 계속 `- [ ]`다. PASS. |
| 9 | C11.3.3 template-area candidate가 전체 완료되는가 | C11.3.3 candidate는 계속 미완료로 둔다. PASS. |
| 10 | 전체 roadmap 공식 count가 잘못 증가하는가 | STATUS와 roadmap은 `72/213`을 유지한다. PASS. |
| 11 | preview 개인 체크 상태가 official count로 섞이는가 | 기록은 raw source-of-truth `spec/STATUS.md`와 count를 구분한다. PASS. |
| 12 | internal contract version이 출시 전 변경되는가 | 내부 계약 숫자 버전은 `0.1.0` 그대로다. PASS. |
| 13 | one-axis #967 mode가 양 축 mode로 대체됐다고 오해하는가 | 새 typed mode는 기존 one-axis variant와 분리돼 있다고 기록한다. PASS. |
| 14 | unrestricted CSS admission이 생겼다고 오해하는가 | 일반 author CSS admission은 명시적으로 범위 밖이다. PASS. |
| 15 | WPT 전체 적합성으로 확대 해석하는가 | Spinon WPT 적합성은 미검증으로 둔다. PASS. |
| 16 | 실기기 또는 hardware GPU로 과장하는가 | Android는 API 37 AVD/SwiftShader이고 실기기는 미검증이다. PASS. |
| 17 | iOS compositor 이후 pixels로 과장하는가 | iOS 결과는 compositor 제출 전 Metal drawable이다. PASS. |
| 18 | Android frame count 또는 surface 표본 수가 바뀌는가 | 최신 source capture는 13 element frame과 6/6 sample을 기록한다. PASS. |
| 19 | iOS frame count 또는 surface 표본 수가 바뀌는가 | 최신 source capture는 13 element frame과 6/6 sample을 기록한다. PASS. |
| 20 | fixture app hash가 다른 run에 연결되는가 | source revision manifest의 Android/iOS app SHA는 각각 latest `result.json`과 일치한다. PASS. |
| 21 | 수정 전 capture를 최신 코드 근거로 표시하는가 | root README와 contract는 auto-size fallback fix 이후 run IDs를 우선 링크한다. PASS. |
| 22 | auto-size fallback 결함이 구현 근거에서 숨겨지는가 | runtime review에 발견·수정 및 focused regression 결과를 기록한다. PASS. |
| 23 | full style suite의 known failure가 녹색으로 요약되는가 | 276 passed·1 unrelated C07.3 failure를 별도 표시한다. PASS. |
| 24 | static archive 결과를 APK/IPA 증거로 확대하는가 | macOS static archive 검사만 수행했고 mobile bundle 제외는 증명하지 않는다고 적는다. PASS. |
| 25 | 20개 구현 검토와 30개 PR diff 검토가 같은 검토로 합쳐지는가 | 별도 파일과 서로 다른 범위를 유지한다. PASS. |
| 26 | raw roadmap이 PR #970을 누락하거나 현재 PR #967을 가리는가 | 새 merged-slice card를 C11.3.3의 최신 항목으로 추가하고 #967 기록은 보존한다. PASS. |
| 27 | roadmap card가 merge metadata나 공식 count와 다른가 | card의 UTC time, merge SHA, `72/213`, 미완료 범위는 STATUS와 merge record에 맞춘다. PASS. |
| 28 | roadmap의 local links가 선택 문서 publication에서 빠지는가 | 새 card는 plan, 0059, root evidence, two reviews, merge record, Android/iOS PNG와 STATUS를 연결한다. PASS. |
| 29 | PR 본문 simulator screenshots가 실제 이미지로 표시되는가 | Agent Browser에서 두 이미지의 complete/natural dimensions를 확인하고 세션과 브라우저 프로세스를 종료했다. PASS. |
| 30 | Tailnet sync를 이미 끝난 것처럼 기록하는가 | `STATUS.md`와 roadmap evidence는 Tailnet 반영을 후속 sync로 남기며, preview 확인 전 완료라 하지 않는다. PASS. |

현재 main에서 구현 PR #970은 병합됐고 공식 C11 상태는 미완료다. 다음 Tailnet sync는 사용자가 승인한 preview 경로에 선택 파일만 반영하고 URL/hash 및 Agent Browser 화면을 별도로 검증한다.
