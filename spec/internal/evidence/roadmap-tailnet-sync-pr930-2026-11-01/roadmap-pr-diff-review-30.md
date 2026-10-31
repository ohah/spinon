# C11.4 `text-indent` 공개 로드맵 변경 검토 30개

검토 대상은 `spec/public/roadmap.html`에 추가한 병합 PR #930 카드와 연결 링크다. 구현 PR의 30개 review와 분리해 이 공개 문서 변경의 상태·범위·링크·표현을 점검했다.

| # | 독립 실패 관점 | 판정과 대조 |
|---:|---|---|
| 1 | candidate를 C11.4 전체 구현 완료로 표시하는가 | 통과. 제목과 본문에서 제한 opt-in fixture slice로 한정한다. |
| 2 | PR 제목/번호를 다른 C11.4 작업으로 연결하는가 | 통과. GitHub PR #930과 실제 제목이 일치한다. |
| 3 | UTC 병합 시간을 지역 날짜로 잘못 기록하는가 | 통과. GitHub `mergedAt` 원문 `2026-10-31T16:37:17Z`를 보존하고 화면 기록 날짜는 KST 기준 2026-11-01이다. |
| 4 | merge SHA 대신 PR head SHA를 표시하는가 | 통과. GitHub 결과의 merge SHA `b1b20e30cc8d36e90d87e902668eb95339698c25`를 표시한다. |
| 5 | 18 scene의 고정 값 범위를 왜곡하는가 | 통과. `0px`·`5px`·`10px` 세 값과 min-/max-content 양쪽을 명시한다. |
| 6 | 텍스트 장면을 과하게 일반화하는가 | 통과. 한·두·세 단어 Ahem fixed matrix로 설명한다. |
| 7 | Chrome 기준을 Spinon 적합성으로 표현하는가 | 통과. Chrome 156 사전 관측과 독립 Ahem 산식을 기준 비교로 분리한다. |
| 8 | Android emulator를 실기기로 오인하는가 | 통과. Android API 37 ARM64 AVD라고 명시한다. |
| 9 | iOS Simulator를 iPhone 실기기로 오인하는가 | 통과. iPhone 17 Pro/iOS 26.2 Simulator라고 명시한다. |
| 10 | 각 platform matrix의 분모를 오기하는가 | 통과. 양 플랫폼 18/18을 execution README의 matrix와 대조했다. |
| 11 | computed style·intrinsic·track 검사를 단순 screenshot 검사로 바꾸는가 | 통과. execution evidence가 별도 링크되고 screenshot 한계가 함께 쓰인다. |
| 12 | 검은 renderer 영역을 glyph paint 성공이라고 표현하는가 | 통과. glyph paint 및 PixelCopy/Metal/compositor pixel 증거가 아님을 명시한다. |
| 13 | Simulator 결과를 hardware GPU 증거로 확대하는가 | 통과. 실기기·hardware GPU는 미포함이라고 쓴다. |
| 14 | 앱 빌드/fixture archive를 전체 APK·IPA 경계로 확대하는가 | 통과. 공개 카드에는 전체 앱 artifact 제외 주장을 넣지 않는다. |
| 15 | GitHub check 부재를 CI 성공으로 쓰는가 | 통과. check이 보고되지 않았다고 적는다. |
| 16 | main required status checks가 있는 것처럼 쓰는가 | 통과. 확인 당시 비활성 상태를 적는다. |
| 17 | 일반 author CSS가 지원되는 것처럼 읽히는가 | 통과. 일반 CSS admission을 범위 밖으로 명시한다. |
| 18 | 음수 CSS가 무효인 값이라고 오해하게 하는가 | 통과. 이 profile 범위 밖 값으로만 분류한다. |
| 19 | percentage 또는 `hanging`/`each-line`이 구현됐다고 읽히는가 | 통과. 해당 broader grammar는 포함하지 않는다고 쓴다. |
| 20 | WPT 전체 적합성을 주장하는가 | 통과. WPT는 범위 밖이다. |
| 21 | C11.4 체크가 완료로 바뀐 것처럼 보이는가 | 통과. 카드에서 C11.4 미완료를 유지한다. |
| 22 | C11 상위 체크를 완료로 승격하는가 | 통과. 전체 C11 미포함이라고 쓴다. |
| 23 | 공식 완료 수가 후보 slice 때문에 증가하는가 | 통과. `72/213`을 그대로 표시한다. |
| 24 | 계획 링크가 잘못된 `/spec/plan` route를 가리키는가 | 통과. 현재 Tailnet 라우트 규칙인 `/spinon/plan/...html`을 사용한다. |
| 25 | 내부 계약 링크를 public API로 잘못 소개하는가 | 통과. 내부 계약 0059라고 표시한다. |
| 26 | 실행 README 경로가 이전 word-spacing slice를 가리키는가 | 통과. 새 text-indent runtime README의 exact route를 쓴다. |
| 27 | Android 캡처 링크가 이전 실행 폴더를 가리키는가 | 통과. 최신 Android API 37 run ID의 representative PNG를 쓴다. |
| 28 | iOS 캡처 링크가 이전 실행 폴더를 가리키는가 | 통과. 최신 iPhone 17 Pro/iOS 26.2 run ID의 representative PNG를 쓴다. |
| 29 | PR 변경 검토 또는 공식 상태 anchor가 누락되는가 | 통과. 새 30-point roadmap review와 `STATUS.html` checklist anchor를 연결한다. |
| 30 | 카드 삽입이 기존 HTML 의미 구조나 모바일 배치 규칙을 바꾸는가 | 통과. 기존 `p.kanban-note.hero-progress`와 inline link/code 패턴만 재사용하고 global style·checkbox·counter 구조는 건드리지 않는다. Tailnet browser 렌더는 publish 후 별도 확인한다. |

## 판정

30개 독립 관점에서 카드 문구·숫자·PR/commit 식별자·지원 경계·내부 링크를 확인했다. 이 review는 Tailnet에서 업데이트된 HTML과 이미지의 실제 로드를 증명하지 않으며, 해당 검증은 게시 이후 수행한다.
