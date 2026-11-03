# PR #1104 · #1102 로드맵 반영 변경 적대 검토 30개

**대상:** `spec/public/roadmap.html`에 추가한 C11.1 PR #1102 카드
**기준:** `origin/main` `3cabd93314c604d7d766d473060e996a382430e6`
**목적:** 병합된 기능의 공개 로드맵 설명·범위·링크·집계가 원본 근거와 맞는지 서로 다른 실패 경로로 검토한다.

| # | 실패 관점 | 결과 |
| ---: | --- | --- |
| 1 | 카드가 다른 PR 번호나 제품 변경을 가리키는가 | 통과. 구현 PR #1102로 연결된다. |
| 2 | 병합 상태나 시간이 원본과 다른가 | 통과. GitHub PR은 merged이며 시각은 2026-11-03T02:59:45Z다. |
| 3 | merge SHA가 rebase 전 source SHA와 혼동됐는가 | 통과. 공개 카드에 merge SHA `7464e2ca469e5e7b7f35fc837f9adfedca6a0b3a`를 쓴다. |
| 4 | 지원한 축이나 속성이 잘못 적혔는가 | 통과. parsed Root HTML `grid-auto-rows` implicit row 경로로 한정한다. |
| 5 | 임의 `calc()` 표현식을 지원한다고 넓혔는가 | 통과. px 항과 percentage 항을 더하는 2항 사례라고 명시한다. |
| 6 | 행 목록 전체 순환이나 다른 Grid grammar를 지원한다고 암시하는가 | 통과. row 목록 일반 지원을 선언하지 않는다. |
| 7 | C11.1 또는 C11 전체를 완료했다고 표시하는가 | 통과. 두 항목 모두 미완료라고 명시한다. |
| 8 | 공식 완료 수를 임의로 올렸는가 | 통과. 공식 `72/213`을 유지한다. |
| 9 | Android AVD 결과를 실기기 결과로 부르는가 | 통과. Android API 37 ARM64 AVD라고 표기한다. |
| 10 | iOS Simulator 결과를 iOS 실기기로 부르는가 | 통과. iPhone 17 Pro/iOS 26.2 Simulator로 표기한다. |
| 11 | runtime geometry 결과와 그 범위를 넘는 pixel equality를 혼동하는가 | 통과. row 좌표와 draw/present만 기록한다. |
| 12 | 화면 캡처를 surface readback 증거로 오인할 수 있는가 | 통과. surface pixel readback을 하지 않았다고 적는다. |
| 13 | Android와 iOS에서 서로 다른 fixture 결과를 숨기는가 | 통과. 공통 8개 row 시작 좌표와 11-box 실행을 요약하고 상세 근거를 링크한다. |
| 14 | Rust·Clippy·formatter 검증을 전체 workspace 통과로 과장하는가 | 통과. focused Rust와 scoped Clippy·rustfmt로 범위를 제한한다. |
| 15 | PR check rollup이 비어 있던 사실을 CI 통과로 처리하는가 | 통과. PR check rollup이 비어 있다고 명시한다. |
| 16 | 계획 링크의 route 또는 원본 파일이 누락됐는가 | 통과. 계획 Markdown 파일과 `/spinon/plan/` route가 존재한다. |
| 17 | 실행 README route가 잘못 연결됐는가 | 통과. 기존 runtime README와 RSPress HTML route가 대응한다. |
| 18 | 구현/runtime 20관점 검토 링크가 다른 slice를 가리키는가 | 통과. PR #1102 evidence의 검토 파일을 가리킨다. |
| 19 | PR 변경 30관점 검토 링크가 다른 파일을 가리키는가 | 통과. PR #1102의 `pr-diff-review-30.md`를 가리킨다. |
| 20 | Android 화면 링크가 존재하지 않거나 다른 실행의 화면인가 | 통과. #1102 evidence의 `android/avd-screen.png`가 존재한다. |
| 21 | iOS 화면 링크가 존재하지 않거나 다른 실행의 화면인가 | 통과. #1102 evidence의 `ios/iphone17pro-screen.png`가 존재한다. |
| 22 | 외부 PR 링크의 번호와 카드 표시가 불일치하는가 | 통과. href와 표시 문구가 모두 #1102다. |
| 23 | local link가 `/spinon/` base 밖으로 빠지는가 | 통과. 신규 local link 6개 모두 base 경로를 사용한다. |
| 24 | `.html` route가 실제 Markdown 파일과 대응하지 않는가 | 통과. 계획·README·리뷰 각 route의 원본이 존재한다. |
| 25 | PNG route가 정적 자산으로 배포될 원본이 없는가 | 통과. 두 화면 PNG 원본이 모두 존재한다. |
| 26 | 신규 카드가 최신 PR #1097 아래에 놓여 시간 순서가 역전되는가 | 통과. #1102 카드를 #1097보다 먼저 둔다. |
| 27 | 기존 #1097 또는 아래 카드의 문구를 바꾸거나 제거했는가 | 통과. 기존 카드는 그대로 보존한다. |
| 28 | PR #1102 카드가 중복 삽입됐는가 | 통과. 공개 페이지에서 #1102 카드가 정확히 하나다. |
| 29 | HTML 단락 경계·속성 따옴표가 깨져 뒤 카드와 합쳐지는가 | 통과. 한 개 닫힌 `p` 단락으로 삽입했고 링크 검사가 성공했다. |
| 30 | 이 문서 변경이 API 계약이나 체크박스를 바꾸는가 | 통과. 공개 요약만 추가하며 공식 상태·명세 파일은 수정하지 않는다. |

**검증 결과:** 로컬 카드 검사에서 PR #1102 중복 0, local link 6/6 존재, 공식 집계 `72/213`, C11 미완료를 확인했다. `git diff --check`도 통과했다. 전체 사이트 dead-link 검사나 제품 동작 검증으로 확대 해석하지 않는다.
