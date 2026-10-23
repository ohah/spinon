# C11.4 집계 정정·PR #549 문서 변경 적대 검토 30개

| # | 실패 관점 | 대조와 판정 |
|---:|---|---|
| 1 | 화면의 실제 완료 수가 아니라 상태 문구를 다시 집계할 수 있다. | Agent Browser live DOM checked state를 기준으로 72/213을 기록했다. 통과. |
| 2 | 분모가 로드맵 task ID 수와 다를 수 있다. | 표시 대상 checkbox 213개를 직접 세었고 Markdown 원시 checkbox 수를 대신 쓰지 않았다. 통과. |
| 3 | checked 수와 total 수를 혼동할 수 있다. | checked 72, total 213을 JSON과 정정 설명에 분리 기록했다. 통과. |
| 4 | 읽기 검사 중 사용자가 체크 상태를 바꿀 수 있다. | 213 checkbox가 모두 disabled였으며 브라우저 입력을 보내지 않았다. 통과. |
| 5 | 새로운 구현 PR이 C11.4 전체 완료로 오인될 수 있다. | C11.4 상위 checkbox는 그대로 미완료라고 문서화했다. 통과. |
| 6 | docs-only PR이 체크 항목을 실수로 수정할 수 있다. | status checkbox diff는 비어 있고, 변경은 서술과 근거 연결에 한정했다. 통과. |
| 7 | 이전 77/213 측정이 실제 상태 변경이었다고 오해할 수 있다. | pass58 기록에 checkbox count가 없고 main 상태 변화도 없었다는 한계를 분리해 적었다. 통과. |
| 8 | 역사적 PR 검토 문구를 지워 감사 추적을 잃을 수 있다. | 당시 구현/PR review의 77 표기는 원본 증거로 보존하고 상위 문서에서 정정 링크를 제공한다. 통과. |
| 9 | pass58의 원시 브라우저 결과를 수정해 증거가 재작성될 수 있다. | 신규 정정 기록을 만들고 원래 pass58 JSON은 바꾸지 않는다. 통과. |
| 10 | 문서에서 잘못된 merge SHA를 기록할 수 있다. | PR #549 MERGED 응답의 merge SHA fdf5a3a24f3f5f3f1e2f1bc2f5e871f29dbf7ca8와 대조했다. 통과. |
| 11 | merge 시각의 UTC와 KST가 뒤섞일 수 있다. | GitHub mergedAt 06:40:56 UTC를 15:40:56 KST로 변환했다. 통과. |
| 12 | 검사 결과가 없다는 사실을 검사 통과로 과장할 수 있다. | rollup이 비었고 required check 보고가 없었다고 표기했다. 통과. |
| 13 | 41 scene 전체가 아니라 7 신규 scene만 검증한 것처럼 읽히거나 반대로 과장될 수 있다. | 신규 layout 조건 7개와 선택 회귀 matrix 41 scene을 분리해 기록했다. 통과. |
| 14 | 41/41 scene 결과를 pixel 검사로 오해할 수 있다. | scene draw/frame/track과 area 표본 116/116을 별도 수치로 썼다. 통과. |
| 15 | 116/116 표본이 모든 41 scene에서 수집된 것으로 과장될 수 있다. | PixelCopy/Metal 수치는 29 area scene에 한정해 기록했다. 통과. |
| 16 | Android 검증을 실기기 결과로 읽을 수 있다. | Android API 37 ARM64 AVD라고 명시하고 실기기 미검증을 유지했다. 통과. |
| 17 | iOS draw 시점이 OS 합성 이후라고 오해할 수 있다. | Metal sample을 compositor 제출 전으로 표기하고 compositor 이후 화면은 미검증이다. 통과. |
| 18 | 정적 archive 검사를 최종 APK/IPA 검사로 확대할 수 있다. | final app bundle fixture exclusion은 검증하지 않았다고 명시했다. 통과. |
| 19 | 이 fixture가 일반 CSS 지원을 뜻한다고 오해할 수 있다. | RuntimeBlockFlexGridTrackSizingV1 opt-in profile 범위와 일반 CSS admission 미검증을 명시했다. 통과. |
| 20 | WPT나 전체 C11.4 적합성까지 완료한 것으로 읽힐 수 있다. | WPT와 전체 C11.4는 검증되지 않았다고 기록했다. 통과. |
| 21 | 새 문구가 제품 track sizing 구현 변경으로 오인될 수 있다. | 기존 Taffy/layout 계산을 수정하지 않았다는 설명을 유지했다. 통과. |
| 22 | 내부 계약 version이 무단으로 바뀐 것으로 읽힐 수 있다. | 0059 numeric version 0.1.0 유지 사실을 별도 계약 문서에 기록했다. 통과. |
| 23 | empty-area host fix가 layout 알고리즘 변경으로 부풀려질 수 있다. | 빈 item-rect host mapping과 회귀 guard 수정으로 범위를 한정했다. 통과. |
| 24 | public HTML에서 새 card가 section 밖에 붙을 수 있다. | 기존 roadmap section 종료 바로 앞에 단일 card를 넣었고 닫는 section tag는 유지했다. 통과. |
| 25 | PR #549 화면 링크가 잘못된 파일로 향할 수 있다. | Android와 iOS screenshot 경로가 모두 로컬에서 존재함을 확인했다. 통과. |
| 26 | 정정 screenshot이 덮어쓰기 또는 다른 크기로 바뀔 수 있다. | 1280×633 dimensions와 SHA-256 ba5d32805197374ac3f3282d08a7afcd8fd51a84733b615c1e5f7d6ac149a6d9를 고정했다. 통과. |
| 27 | 새 문서 간 상대 경로가 다른 디렉터리를 가리킬 수 있다. | plan, runtime evidence, status correction 링크의 저장소 상대 경로를 대조했다. 통과. |
| 28 | 전체 RSPress build가 성공했다고 잘못 보고될 수 있다. | 기본 전체 build는 기존 evidence의 다수 dead links로 실패했으며 성공이라고 주장하지 않는다. 통과. |
| 29 | 전역 dead-link 문제를 본 변경이 만들었다고 오인하거나 숨길 수 있다. | 실패 로그에서 기존 수백 페이지의 같은 오류와 현재 runtime README raw-log route를 분리 기록했다. 선택 root build/브라우저로 실제 게시 경로를 후속 확인한다. 통과. |
| 30 | docs-only PR이 GitHub Pages나 Tailnet을 자동 게시할 수 있다. | Pages workflow를 실행하지 않는다. Tailnet 동기화는 별도 근거와 PR에서 진행한다. 통과. |
