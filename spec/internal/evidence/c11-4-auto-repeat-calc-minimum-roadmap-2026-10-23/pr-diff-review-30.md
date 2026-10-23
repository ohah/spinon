# C11.4 auto-repeat calc 로드맵 변경 PR diff 검토 30개

검토 대상은 `spec/public/roadmap.html`의 신규 PR #553 항목과 이 리뷰 파일이다. runtime 코드 변경은 PR #553에서 별도로 검토했다.

| # | 독립 실패 관점 | 대조와 판정 |
| --- | --- | --- |
| 1 | 병합되지 않은 PR을 완료 항목으로 잘못 연결한다. | PR #553은 main에 Rebase and merge됐으며 링크가 실제 PR URL을 가리킨다. |
| 2 | 페이지에 다른 SHA 또는 pre-rebase SHA를 기록한다. | entry SHA `7151605419e4777e2bd0c8ced5cb6a926d4bddab`는 PR mergeCommit 및 main SHA와 같다. |
| 3 | 설명이 순수 absolute calc 지원 결손을 확인했다고 오해시킨다. | 문장은 Stylo `LengthPx(40)` 정규화와 기존 gate 허용을 명시해 초기 거부 가설을 반박한다. |
| 4 | 런타임 slice가 제품 evaluator를 추가했다고 과장한다. | evaluator를 추가하지 않았다고 적었고 링크된 PR은 고정 fixture/runtime host 연결만 추가한다. |
| 5 | 단위 calc 전반을 지원한다고 확대한다. | 예시를 `calc(20px + 20px)`으로 제한하고 별도 계산식 지원을 주장하지 않는다. |
| 6 | `auto-fill` 결과만 검증했다고 잘못 요약한다. | columns/rows와 `auto-fill`/`auto-fit`을 모두 적는다. |
| 7 | fixed control이 실제 calc와 다르다. | entry에 `minmax(40px, 1fr)` control을 병기하고 실행 README의 동일 frame 결과와 대조했다. |
| 8 | 여섯 장면 수가 열거된 축/모드 조합과 불일치한다. | 2 axes × 2 auto-repeat modes 및 두 fixed controls로 6 scenes가 맞는다. |
| 9 | Android 기기를 실기기로 표기한다. | API 37 ARM64 AVD라고 한정한다. |
| 10 | iOS simulator를 실기기라고 부른다. | iPhone 17 Pro / iOS 26.2 Simulator라고 정확히 쓴다. |
| 11 | 플랫폼 장면 수 또는 범위를 불일치하게 적는다. | 각 플랫폼 6/6으로 실행 result JSON과 맞는다. |
| 12 | `draw/present`를 화면 픽셀 동등성으로 해석한다. | 화면은 표시 근거라고 분리하고 PixelCopy/Metal equality를 검사하지 않았다고 적는다. |
| 13 | Android PixelCopy를 완료한 것처럼 암시한다. | PixelCopy pixel equality 부재가 문장에 직접 드러난다. |
| 14 | iOS Metal 표본을 compositor 이후로 확대한다. | Metal pixel equality 부재를 명시한다. |
| 15 | FFI archive 결과를 앱 번들 제외 증거로 확장한다. | APK/IPA 전체 fixture 제외를 주장하지 않는다고 제한한다. |
| 16 | C11.4 상위 항목을 완료로 승격한다. | 전체 C11.4는 미완료라고 entry에서 명시한다. |
| 17 | 현재 완료 수를 이전 분모/분자로 되돌린다. | 신규 최신 entry는 official `74/207`을 기록한다. |
| 18 | 기존 역사 기록을 현재 수치로 덮어써 실행 시점을 잃는다. | 이전 PR 항목은 수정하지 않고 새 현황 entry를 앞에 추가했다. |
| 19 | 상태 대장의 section anchor가 틀리다. | `/spinon/STATUS.html#c11-css-grid`는 기존 roadmap 내 C11.4 링크와 같은 anchor다. |
| 20 | 계획 링크가 저장소의 다른 calc 계획을 가리킨다. | `/spinon/plan/c11-4-auto-repeat-calc-minimum.html`은 PR #553 대상 계획의 예상 RSPress route다. |
| 21 | runtime evidence 링크가 plan evidence를 가리킨다. | `/spinon/internal/evidence/c11-4-auto-repeat-calc-minimum-runtime-2026-10-23/README.html`은 실행 README의 변환 route다. |
| 22 | Android 화면 링크가 잘못된 capture run을 가리킨다. | AVD 경로의 run ID와 `columns-calc-fill.png`는 README 및 screenshot manifest에 등록돼 있다. |
| 23 | iOS 화면 링크가 Android 또는 다른 run을 가리킨다. | iPhone 17 Pro run 경로의 PNG는 README 및 screenshot manifest에 등록돼 있다. |
| 24 | PR URL을 상대 Tailnet route로 만들어 깨뜨린다. | PR 링크는 GitHub 절대 URL이다. |
| 25 | preview 내부 경로가 `/spinon/` prefix 없이 루트 바깥을 향한다. | 모든 내부 링크에 기존 Tailnet prefix `/spinon/`을 유지한다. |
| 26 | 신규 paragraph가 상위 hero heading 또는 다른 section DOM 안으로 빠진다. | 기존 첫 `kanban-note`와 같은 hero 위치에 같은 paragraph class로 앞에 추가했다. |
| 27 | `<p>` nesting 또는 tag closure가 깨진다. | 신규 entry는 paragraph, `strong`, `code`, `a` 태그가 각각 닫혀 있고 `git diff --check`가 통과한다. |
| 28 | 현재 CI 상태를 성공으로 과장한다. | entry는 merge 시 rollup이 비고 required checks가 설정되지 않았음을 기록한다. |
| 29 | 규범/일반 web support를 해당 런타임 결과에서 추론한다. | entry는 fixture slice와 FFI archive scope만 설명하며 WPT·일반 CSS 지원을 추가하지 않는다. |
| 30 | 새 텍스트가 roadmap에서 최신 C11.4 progress를 찾기 어렵게 한다. | 새 PR #553 요약을 C11.4 항목 맨 앞에 놓고, 아래 기존 계획·런타임 이력은 보존한다. |

## 판정

30개 서로 다른 diff 실패 관점에서 남은 수정은 없다. 실제 Tailnet response hash와 Agent Browser 렌더링은 병합 뒤 sync 근거에서 별도로 기록한다.
