# 공개 로드맵 PR 변경 검토 30개

대상은 C11.4 computed `letter-spacing` 병합 기록을 `spec/public/roadmap.html`에 추가하는 문서 PR이다. 공식 상태의 원본은 `spec/STATUS.md`이며 이 공개 카드로 C11.4 또는 C11 완료 상태를 승격하지 않는다.

| # | 실패 관점 | 판정 |
|---:|---|---|
| 1 | 새 항목이 최신 C11.4 작업 뒤에 묻히는가 | 통과. 최상단 C11.4 진행 카드로 추가했다. |
| 2 | CSS 속성 이름을 다르게 쓰는가 | 통과. `<code>letter-spacing</code>`로 표기했다. |
| 3 | 작업의 상위 범위를 잘못 연결하는가 | 통과. C11.4와 C11의 하위 후보로 표기했다. |
| 4 | direct anonymous Grid text 범위를 넓혀 쓰는가 | 통과. 기존 계획·근거 링크가 이 제한 범위를 설명한다. |
| 5 | computed-value fixture를 author CSS 지원으로 읽게 하는가 | 통과. opt-in fixture라고 명시했다. |
| 6 | 계획 PR과 구현 PR을 혼동하는가 | 통과. #920은 구현, #921은 병합 상태 문서 동기화로 구분했다. |
| 7 | 구현 PR 링크가 잘못된가 | 통과. GitHub #920 URL을 사용했다. |
| 8 | 구현 PR 병합 시각이 다른가 | 통과. 기록된 `2026-10-31T11:22:59Z`와 일치한다. |
| 9 | 구현 PR merge SHA가 다른가 | 통과. `15888d08fbbc171815fb77ce249d1fafa11cb9cd`와 일치한다. |
| 10 | 상태 동기화 PR 링크가 잘못된가 | 통과. GitHub #921 URL을 사용했다. |
| 11 | 상태 동기화 PR 병합 시각이 다른가 | 통과. 기록된 `2026-10-31T11:25:50Z`와 일치한다. |
| 12 | 상태 동기화 merge SHA가 다른가 | 통과. `0621423ca134f7ed8769b1efbf3bbb3d3c6cb090`와 일치한다. |
| 13 | fixture 장면 수가 부풀려지는가 | 통과. 고정 16-scene matrix로 적었다. |
| 14 | Android 환경에서 실기기라고 오해하게 하는가 | 통과. API 37 ARM64 AVD로 적었다. |
| 15 | iOS 환경에서 실기기라고 오해하게 하는가 | 통과. iPhone 17 Pro / iOS 26.2 Simulator로 적었다. |
| 16 | 두 플랫폼 결과가 합산되어 각자 통과한 사실을 숨기는가 | 통과. 플랫폼별 16/16으로 표기했다. |
| 17 | 검사한 runtime 값을 생략해 증거 범위를 흐리는가 | 통과. computed value, revision, intrinsic contribution, line advance, frame, tracks를 열거했다. |
| 18 | artifact 검증을 앱 전체 검증으로 확대하는가 | 통과. static archive와 fixture-test positive control만 언급한다. |
| 19 | 제품 archive의 fixture 부재 조건이 불분명한가 | 통과. 새 symbol 및 scene payload가 없다고 한정했다. |
| 20 | fixture-test positive control이 빠졌는가 | 통과. validator symbol과 16/16 payload를 적었다. |
| 21 | 앱 화면을 glyph가 보인 것처럼 설명하는가 | 통과. renderer가 검다고 명시했다. |
| 22 | screenshot을 pixel proof로 과장하는가 | 통과. glyph pixels 증거가 아니라고 명시했다. |
| 23 | 미실행 표면 검증을 감추는가 | 통과. PixelCopy, drawable readback, compositor 이후 표면을 열거했다. |
| 24 | Chrome 또는 WPT 전체 적합성을 주장하는가 | 통과. WPT 미검증을 적었다. |
| 25 | 일반 CSS 사용자의 속성 admission을 주장하는가 | 통과. 일반 CSS admission 미검증을 적었다. |
| 26 | simulator 결과를 실기기/hardware GPU 결과로 넓히는가 | 통과. 해당 범위를 미검증으로 명시했다. |
| 27 | 좁은 후보를 C11.4 완료로 승격하는가 | 통과. C11.4/C11 미완료를 적었다. |
| 28 | 공식 완료 수가 현재 상태와 달라지는가 | 통과. `72/213` 유지로 적었다. |
| 29 | PR check 부재를 CI 통과로 표현하는가 | 통과. check 미보고와 required-check 미설정을 적었다. |
| 30 | 로드맵 링크 경로·HTML 경계가 깨지는가 | 통과. plan, runtime README, artifact README, 구현 diff review, STATUS 링크를 확인했고 카드의 `<p>` 닫힘을 확인했다. |

## 결과

30개 관점에서 잘못된 병합 정보, 과장된 지원·픽셀·플랫폼 주장, 공식 완료 수 변경, 카드 링크 및 HTML 경계 문제를 찾지 못했다. 변경 범위는 공개 진행 카드 하나다.
