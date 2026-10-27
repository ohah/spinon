# PR #693 공개 로드맵 diff 실패 관점 검토 30개

검토 대상은 `spec/public/roadmap.html`에 추가한 C11.4 PR #691 병합 요약 한 문단이다. 아래 관점은 서로 다른 링크·수치·범위·표시 실패 경로를 각각 대조했다.

| # | 실패 관점 | 확인 결과 | 판정 |
|---:|---|---|---|
| 1 | 새 기록이 기존 최신 C11.4 기록보다 앞에 삽입돼 연대가 역전되는가 | PR #688 기록 뒤, 구현 관문 개요 앞에 둔다. | 통과 |
| 2 | 날짜가 병합 일자와 다른가 | 머리말은 2026-10-27이며 병합 시간도 같은 날짜다. | 통과 |
| 3 | PR 번호가 다른 작업을 가리키는가 | 링크 대상은 `ohah/spinon/pull/691`이다. | 통과 |
| 4 | 병합 방법을 잘못 표현하는가 | 저장소와 PR 기록의 `Rebase and merge`를 적었다. | 통과 |
| 5 | 시간대 없는 시각을 현지 시각으로 오해하게 하는가 | GitHub 병합 시각 `2026-10-27T10:30:12Z`의 UTC 표기를 유지한다. | 통과 |
| 6 | SHA가 원래 PR 커밋과 최종 main SHA를 혼동하는가 | 병합 뒤 main에 반영된 `8993e3f5f28f13ab707018dda52f8f619d2b8bb4`를 쓴다. | 통과 |
| 7 | WPT test 파일 이름에 오타가 있는가 | `grid-auto-repeat-intrinsic-001.html`로 계획·실행 문서와 일치한다. | 통과 |
| 8 | 고정 fixture 결과를 WPT 전체 통과처럼 읽게 하는가 | “제한 runtime slice”와 “전체 WPT harness가 아님”을 함께 썼다. | 통과 |
| 9 | scene 수가 계획 입력과 다르게 기록되는가 | 대상은 원본 WPT 안의 8개 Grid다. | 통과 |
| 10 | Android 하드웨어 정보를 다른 AVD와 섞는가 | Android API 37 ARM64 AVD로 한정했다. | 통과 |
| 11 | iOS 기기·OS가 다른 실행과 섞이는가 | iPhone 17 Pro / iOS 26.2 Simulator로 한정했다. | 통과 |
| 12 | 플랫폼별 8/8 결과를 합계나 부분 실행으로 오독하는가 | Android와 iOS 각각의 8/8을 따로 적었다. | 통과 |
| 13 | Android software renderer를 hardware GPU로 오해하는가 | ANGLE/SwiftShader software path라고 명시했다. | 통과 |
| 14 | iOS drawable 표본을 최종 scanout으로 오해하는가 | compositor 제출 전 Metal drawable이라고 명시했다. | 통과 |
| 15 | overflow를 모든 scene에서 검사했다고 과장하는가 | 두 min-content 장면으로 범위를 한정했다. | 통과 |
| 16 | border 바깥 item 확인이 누락되는가 | overflow item 표본을 명시했다. | 통과 |
| 17 | item 바깥 대조 표본이 overflow와 섞이는가 | item 바깥 control 표본을 별도로 언급한다. | 통과 |
| 18 | Android sample 수 3/3의 대상이 모호한가 | PixelCopy의 장면별 border·overflow·control 표본임을 연결했다. | 통과 |
| 19 | iOS sample 수 3/3의 대상이 모호한가 | Metal drawable의 같은 표본 범위임을 연결했다. | 통과 |
| 20 | 계획 링크가 다른 C11.4 계획으로 연결되는가 | `c11-4-grid-auto-repeat-intrinsic-wpt.html`을 가리킨다. | 통과 |
| 21 | Chrome 기준 링크가 잘못된 precomparison으로 연결되는가 | 2026-10-27 WPT precomparison README 경로를 쓴다. | 통과 |
| 22 | 실행 README 링크가 simulator 자료와 연결되지 않는가 | 해당 runtime evidence README 경로를 쓴다. | 통과 |
| 23 | 구현 검토 링크에서 20개 관점 근거가 빠지는가 | implementation/runtime review 문서를 연결한다. | 통과 |
| 24 | 선행 PR diff 검토 경로가 다른 폴더를 가리키는가 | runtime evidence의 `pr-diff-review-30.md`를 연결한다. | 통과 |
| 25 | Android 화면 경로가 실제 run ID와 다른가 | Android 최종 run `20261027T101029040Z-44c2a0b7`의 `fill-auto-min` PNG를 쓴다. | 통과 |
| 26 | iOS 화면 경로가 실제 run ID와 다른가 | iOS 최종 run `20261027T101156827Z-602592aa`의 같은 scene PNG를 쓴다. | 통과 |
| 27 | 공식 완료 수를 PR 병합 때문에 올리는가 | 공식 `72/213` 그대로라고 적었다. | 통과 |
| 28 | 제한 slice를 C11.4 전체 완료로 승격하는가 | C11.4 미완료임을 직접 적었다. | 통과 |
| 29 | 공개 요약이 상태 대장 checkbox를 대신 갱신하는가 | 이 diff는 roadmap HTML 한 항목만 추가하며 STATUS와 체크박스는 수정하지 않는다. | 통과 |
| 30 | 빈 GitHub check rollup을 CI 통과로 표현하는가 | check rollup 공백과 required status checks 미설정을 적고 통과 주장을 하지 않는다. | 통과 |

## 결론

30개 독립 관점에서 차단 결함을 찾지 못했다. 공개 요약은 실행 README의 범위를 유지하고 공식 완료 집계나 C11.4 checkbox를 바꾸지 않는다. 선택 문서 범위 Rspress build는 생성 문서 페이지 점검용이며 이 정적 `roadmap.html` 변경을 렌더링하지 않는다. 변경 후 시도한 저장소 전체 `spec/` build는 범위 밖 기존 Markdown 언어 태그(`rust,ignore`)와 이미지 참조 오류로 실패했다. 이를 이 diff의 build 통과로 해석하지 않으며, 배포 뒤 Agent Browser에서 정적 HTML과 링크·화면을 확인한다.
