# C13.1 WPT 분류 사전 기준 · 2026-10-14

## 고정 입력

- 원본 저장소: web-platform-tests/wpt
- 고정 commit: [a73e14cfbaeba97744ff40cbc34d1ca96e5a92c7](https://github.com/web-platform-tests/wpt/tree/a73e14cfbaeba97744ff40cbc34d1ca96e5a92c7)
- commit 시각: 2026-10-14T10:07:55Z
- directory: `css/css-overflow`
- inventory: 파일명이 `clip-`, `overflow-clip`, `single-axis-overflow-clip`으로 시작하는 `.html` 테스트 원본만 수집하고 `-ref` 파일은 제외
- 수집 방법: agent-browser로 GitHub API directory listing과 고정 commit의 원본 파일을 확인했다. terminal-browser는 사용하지 않았다.

## 사전 통과 기준

1. inventory는 정확히 58개이며 경로가 서로 달라야 한다.
2. 각 경로는 `spinon_fixture_candidate`, `chrome_oracle_only`, `unsupported` 중 하나에만 배정한다.
3. 각 행에는 원본 근거를 남기고, 고정 commit SHA·정렬 경로 목록 SHA-256·경로/범주/근거 튜플 SHA-256을 validator에 고정한다.
4. fixture 후보를 WPT 실행 또는 지원 PASS로 해석하지 않는다.
5. 음수 길이의 oracle 차이, paint effect, 동적 style, scroll/CSSOM, scrollbar, 논리 축, radius, transform, SVG, 다단 레이아웃의 소유 범위를 분리한다.

## 사후 관찰

고정 source inventory는 58개다. Spinon fixture candidate 13개, Chrome oracle-only 17개, unsupported 28개로 분류했다. validator는 목록·분류 digest를 고정하고 중복/누락 행, 허용되지 않은 범주, 개수를 보존한 category 교환, inventory 교체, WPT 통과 허위 표기에 대한 negative control을 둔다. WPT 테스트는 실행하지 않았다.
