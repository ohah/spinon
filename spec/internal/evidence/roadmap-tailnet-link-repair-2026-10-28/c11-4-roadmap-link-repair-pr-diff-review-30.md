# C11.4 로드맵 링크 복구 PR 변경분 적대적 검토 30관점

대상 변경은 `spec/public/roadmap.html`의 #738 카드 링크와 `plan/c11-4-grid-track-sizing.md`의 요약 상태다. Tailnet 정적 미리보기에 로드맵 파일만 복사되어 새 상대 경로가 404였고, 계획 요약은 병합 후에도 계획 단계로 남아 있었다. 아래 검토는 서로 다른 경로·상태·증거 경계를 확인한다.

| # | 실패 가설 / 검증 관점 | 판정 |
|---:|---|---|
| 1 | 집중 계획 URL이 저장소의 실제 Markdown 파일을 가리키지 않는다. | 통과 — 실제 Markdown 경로와 Agent Browser 페이지 제목 확인 |
| 2 | 실행 근거 README 링크에서 증거 디렉터리 이름이 달라진다. | 통과 — README 경로·파일명 대조 및 Agent Browser 열림 확인 |
| 3 | 검증 JSON 링크가 이전 visibility matrix를 가리킨다. | 통과 — 날짜 포함 검증 JSON 경로 대조 및 Agent Browser 열림 확인 |
| 4 | archive 경계 링크가 HTML 변환본에 남아 있다. | 통과 — artifact-boundary.md 경로 대조 및 Agent Browser 열림 확인 |
| 5 | 구현/runtime 검토 링크가 다른 20관점 문서로 바뀐다. | 통과 — runtime review 파일명 대조 및 Agent Browser 열림 확인 |
| 6 | PR diff 검토 링크가 계획 검토 문서로 오인된다. | 통과 — 구현 PR diff review 파일명 대조 및 Agent Browser 열림 확인 |
| 7 | 병합 후 문서 검토 링크가 로컬 Tailnet 경로에 의존한다. | 통과 — 문서 동기화 review 파일명 대조 및 Agent Browser 열림 확인 |
| 8 | Android 화면 링크가 PR head의 rebase 전 SHA에 의존한다. | 통과 — raw 이미지에 #738 merge SHA 고정 |
| 9 | iOS 화면 링크가 Android 이미지를 가리킨다. | 통과 — iOS 이미지 파일과 run 경로가 Android와 분리됨 |
| 10 | 화면 URL은 열리지만 이미지 바이트가 다른 run 산출물이다. | 통과 — 두 raw PNG 모두 브라우저 로드 완료 |
| 11 | PNG를 `blob` URL로 연결해 브라우저에서 이미지 대신 코드 뷰가 열린다. | 통과 — Android 1080×2400, iOS 1206×2622 이미지 표시 확인 |
| 12 | Markdown/JSON 문서를 Tailnet 미리보기에 다시 복사해야만 열 수 있다. | 통과 — 문서·JSON 링크가 로컬 미리보기 경로를 사용하지 않음 |
| 13 | GitHub 문서 주소에 저장소 소유자나 저장소 이름 오타가 있다. | 통과 — 모든 저장소 URL의 owner/repository가 ohah/spinon |
| 14 | `main` 브랜치 링크가 이번 문서 PR 병합 후 최신 내용을 가리키지 않는다. | 통과 — GitHub main 경로 문서들이 실제 열림 |
| 15 | immutable PNG 주소가 문서 브랜치 SHA로 바뀌어 산출물을 잃는다. | 통과 — PNG 두 개 모두 immutable merge SHA 사용 |
| 16 | 공식 상태 링크가 Tailnet의 누락된 `STATUS.html`을 계속 참조한다. | 통과 — 공식 상태 spec/STATUS.md 페이지 열림 확인 |
| 17 | GitHub Markdown 페이지에서 확인되지 않은 fragment에 의존한다. | 통과 — 확인되지 않은 fragment를 제거해 문서 루트로 연결 |
| 18 | 같은 페이지 카드에서 일부 링크만 바뀌어 상대 경로 404가 남는다. | 통과 — 계획·근거·JSON·리뷰·이미지·상태 링크 전체를 점검 |
| 19 | 다른 카드의 기존 링크까지 무관하게 변경된다. | 통과 — roadmap 변경은 #738 카드 한 줄에 한정 |
| 20 | #738의 PR 번호·merge SHA·시각이 문서 수정 중 바뀐다. | 통과 — PR 번호·merge SHA·병합 시각 유지 |
| 21 | Android/iOS 각각 14/14 결과가 다르게 서술된다. | 통과 — Android/iOS 각 14/14 문구 유지 |
| 22 | 표면 표본 설명을 전체 GPU·compositor 검증으로 확대한다. | 통과 — 제한된 PixelCopy·Metal 표본 표현 유지 |
| 23 | 비어 있는 GitHub check rollup을 CI 통과로 오표기한다. | 통과 — 비어 있는 PR check rollup을 그대로 표시 |
| 24 | 임의 CSS·전체 WPT·실기기 등 미검증 범위가 사라진다. | 통과 — 기존 미검증 범위 문장 유지 |
| 25 | 상위 C11.4가 이 slice로 완료 처리된다. | 통과 — C11.4 상위 항목 미완료 유지 |
| 26 | 공식 완료 수가 72/213에서 증가한다. | 통과 — 공식 완료 수 72/213 유지 |
| 27 | 계획 인덱스가 #738을 아직 미병합으로 표시한다. | 통과 — 상단 요약의 PR #738 미병합 표현 제거 |
| 28 | 계획 인덱스의 Android/iOS 검증 수가 최종 14/14와 다르다. | 통과 — 상단 요약에 각 플랫폼 14/14 기록 |
| 29 | 사전 비교와 계획 단계 기록이 사라져 구현 판정의 출처가 끊긴다. | 통과 — #737 계획 PR과 precomparison 링크 유지 |
| 30 | Tailnet에서 HTML만 있어야 로드맵 내부 근거 링크가 열린다. | 통과 — Tailnet 배포물에 의존하지 않는 GitHub 직접 링크 사용 |

Agent Browser에서 GitHub 문서 링크 8개와 PR 페이지를 열었고 두 raw PNG의 complete 상태와 크기(Android 1080×2400, iOS 1206×2622)를 확인했다. Tailnet 화면은 PR 병합 뒤 roadmap.html 단일 파일 동기화 후 별도로 기록한다. 이 문서는 로드맵 링크 수정만 검토하며 runtime 동작을 새로 검증했다고 주장하지 않는다.
