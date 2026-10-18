# C14.1 Tailnet fixture source link 수정 PR 실패 관점 검토 30개

**검토 대상:** C14.1 실행 근거 README의 고정 HTML fixture 링크를 GitHub source URL로 바꾼 수정과 직전 이미지 경로 보정. 기존 roadmap PR의 30개 검토와 별도로 URL·base path·evidence link 경계를 검토했다.

| # | 실패 가능성 | 확인 결과 |
|---:|---|---|
| 1 | 링크가 다른 GitHub 조직으로 간다 | `ohah` 조직 경로를 확인했다. |
| 2 | 링크가 다른 저장소를 가리킨다 | `spinon` 저장소 경로를 확인했다. |
| 3 | URL이 blob 대신 raw download 또는 search 화면으로 간다 | GitHub `blob` route를 사용한다. |
| 4 | merge 후에도 branch 이름이 남아 링크가 사라진다 | 안정된 `main` branch를 사용한다. |
| 5 | fixture 경로의 대소문자가 틀렸다 | `tests/fixtures/html/` 소문자 경로와 일치한다. |
| 6 | 파일명이 다른 C14 fixture를 연다 | `c14-1-root-html-image.html`을 지정했다. |
| 7 | Markdown renderer가 확장자를 route로 오인한다 | 링크 target이 정확한 `.html` source 파일이다. |
| 8 | 해당 파일이 main에 존재하지 않는다 | 병합된 PR #295의 fixture 경로를 기준으로 확인했다. |
| 9 | 링크가 stale fixture version으로 간다 | URL은 moving `main`이지만 SHA-256은 README에 고정돼 변경을 감지한다. |
| 10 | 링크가 기본 GitHub UI에서 열리지 않는다 | `blob/main/<path>` 표준 공개 URL 형식이다. |
| 11 | Tailnet base `/spinon/` 내부에 `/tests` route가 있어야 한다 | 외부 GitHub 절대 URL이므로 그런 local route 의존이 없다. |
| 12 | 상대경로 정규화가 site root 밖으로 빠진다 | fixture 링크에서 `../`를 제거했다. |
| 13 | browser가 URL을 현재 path 기준으로 다르게 계산한다 | scheme과 host를 포함한 절대 URL이다. |
| 14 | Rspress가 `blob`을 로컬 Markdown route로 변환한다 | URL은 `https` absolute target이라 상대 route rewrite 대상이 아니다. |
| 15 | source link 수정으로 evidence route 자체가 바뀐다 | 변경은 README의 fixture anchor만 대상으로 한다. |
| 16 | Android screenshot `./` 경로 보정이 누락된다 | README에서 `./android-avd/.../screenshot.png`를 유지한다. |
| 17 | iOS screenshot `./` 경로 보정이 누락된다 | README에서 `./ios-simulator/.../screenshot.png`를 유지한다. |
| 18 | Rspress 이미지 import가 다시 실패한다 | 선택 build에서 두 이미지가 생성되는 것을 확인한다. |
| 19 | Android screenshot이 다른 파일로 연결된다 | final opt-in report의 1080×2400 PNG를 유지한다. |
| 20 | iOS screenshot이 다른 파일로 연결된다 | final opt-in report의 1206×2622 PNG를 유지한다. |
| 21 | screenshot이 HTML 이미지 pixel paint로 과장된다 | README와 PR 본문 모두 fixture report 화면이라고 명시한다. |
| 22 | fixture 링크가 어떤 입력인지 감춘다 | URL label에 `tests/fixtures/html/c14-1-root-html-image.html`을 표시한다. |
| 23 | fixture SHA-256이 빠져 source identity를 확인 못 한다 | 기존 고정 SHA-256 표기를 유지한다. |
| 24 | 계획 URL이 잘못 바뀐다 | C14.1 plan link와 route를 수정하지 않았다. |
| 25 | evidence README가 roadmap note에서 접근 불가능해진다 | roadmap의 `/spinon/internal/evidence/.../README.html` link를 유지한다. |
| 26 | 공식 완료 count 또는 checkbox가 바뀐다 | `STATUS.md`와 roadmap counter logic을 수정하지 않았다. |
| 27 | U05.1/C14.1/C14가 완료처럼 보인다 | 완료 상태 표현을 추가하지 않았다. |
| 28 | 앱 구현·fixture payload가 함께 바뀐다 | PR diff는 README와 review Markdown만 수정한다. |
| 29 | PR #295 원본 증거와 다른 기기 결과를 연결한다 | Android API 37 AVD 및 iPhone 17 Pro/iOS 26.2 Simulator 경로를 그대로 유지한다. |
| 30 | 수정 후 최종 source가 build 검증 없이 병합된다 | 병합 전 Rspress 2.0.22 선택 build와 두 screenshot asset을 재확인한다. |

**결론:** fixture 링크는 GitHub source file을 직접 가리키고 Tailnet base path에 의존하지 않는다. Runtime·공식 완료 상태는 변경하지 않는다.
