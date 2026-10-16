# C19.2 로드맵 문구 PR 변경 검토 30개 관점 · 2026-10-17

검토 대상은 `spec/public/roadmap.html`의 최근 병합 요약과 `spec/internal/README.md`의 C19.2 근거 연결이다. 구현 코드와 상태 체크박스는 바꾸지 않는다. 각 관점은 별도 실패 가능성을 확인한다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | C19.2 제한 구현을 C19 완료로 잘못 표시하는가 | 문구와 공식 대장 모두 C19.2 및 parent C19 미완료를 유지한다. 통과. |
| 2 | PR #232가 실제로 병합됐는가 | GitHub PR 상태는 MERGED이며 merge commit은 `857114f6`이다. 통과. |
| 3 | 공식 완료 수를 문구에 고정하는가 | 카운터는 그대로 GitHub `main`의 `spec/STATUS.md`에서 동적으로 읽는다. 통과. |
| 4 | 구현 근거를 전체 CSS 지원으로 확대하는가 | 표본을 제한 profile 실행으로 적고 공개 API가 없음을 유지한다. 통과. |
| 5 | Android API 37 AVD를 실기기 결과로 읽게 하는가 | AVD라고 명시하며 hardware GPU·실기기를 미검증으로 남긴다. 통과. |
| 6 | iPhone Simulator를 iPhone 실기기라고 표현하는가 | 기기명 뒤에 iOS 26.2 Simulator를 명시한다. 통과. |
| 7 | iPad Simulator를 실기기라고 표현하는가 | iPad Pro 13-inch 뒤에 iOS 26.2 Simulator를 명시한다. 통과. |
| 8 | 16/16이 세 플랫폼의 통합 합계로 오해되는가 | Android·iPhone·iPad 각각의 제한 표본이라고 문장에 연결한다. 통과. |
| 9 | Chrome 관찰 결과를 제품 표본 16개 전체와 혼동하는가 | Chrome 차이는 별도 OKLCH sRGB 단일 표본으로 한정한다. 통과. |
| 10 | Chrome과 Stylo의 실제 관찰값이 뒤집혀 기록되는가 | Chrome blue 200, Stylo Local MINDE blue 196으로 기록한다. 통과. |
| 11 | CSS Color 4 초안이 최종 권고라고 과장되는가 | `CR Draft §14.2`라고 명시한다. 통과. |
| 12 | Local MINDE를 표준의 유일한 허용 알고리즘으로 단정하는가 | Stylo가 매핑 기준에 부합한다고만 서술한다. 통과. |
| 13 | Chromium 소스의 기본 feature 설정을 실험 당시 런타임 상태로 단정하는가 | 관측한 Chrome의 Finch 설정이 보존되지 않았음을 밝힌다. 통과. |
| 14 | 해당 표본 결과를 Chrome 전체의 일반 결함으로 확대하는가 | 판정 범위가 이 입력 표본이라고 명시한다. 통과. |
| 15 | gamut mapping 차이를 C19.2 opacity compositor 결과로 잘못 귀속하는가 | 색상 알고리즘 차이로 분리해 C19.1 비교 자료를 연결한다. 통과. |
| 16 | C08 partial-alpha 호환성 수정이 누락되는가 | 자세한 수정은 C19.2 실행 근거와 구현 검토에 남아 있다. 통과. |
| 17 | 남은 이미지 opacity group 통합을 완료로 보이는가 | image opacity group이 남은 범위로 적혀 있다. 통과. |
| 18 | root element opacity 검증 여부를 혼동하는가 | root opacity 미검증을 명시한다. 통과. |
| 19 | iOS compositor 이후 픽셀 확인을 주장하는가 | compositor 이후는 남은 범위로 명시한다. 통과. |
| 20 | text/contrast 또는 hit-test 통합을 완료로 보이는가 | 둘 다 남은 범위에 있다. 통과. |
| 21 | Android software renderer 결과를 hardware GPU로 설명하는가 | hardware GPU 검증을 별도 미완료로 남긴다. 통과. |
| 22 | C13.3 상태가 이 변경으로 완료 처리되는가 | 로드맵 변경은 C13.3 공식 체크 상태를 건드리지 않는다. 통과. |
| 23 | C13.3 세부 근거가 공식 대장에서 사라지는가 | `spec/STATUS.md`가 유일한 체크 원본이고 변경 대상이 아니다. 통과. |
| 24 | C19 내부 계약 버전이 문구 업데이트로 바뀌는가 | 내부 계약은 `0.1.0`으로 유지한다. 통과. |
| 25 | 로드맵 source가 아닌 생성 파일만 고쳐 다음 동기화 때 되돌아가는가 | 저장소 원본 `spec/public/roadmap.html`을 갱신했다. 통과. |
| 26 | 정적 요약과 최신 내부 README의 PR 기록이 충돌하는가 | 내부 C19.2 행에 구현 PR #232를 추가하고 같은 미완료 경계를 유지한다. 통과. |
| 27 | 링크가 잘못된 base path를 사용해 Tailnet에서 깨지는가 | 문서 경로는 모두 `/spinon/spec/...`이고 PR 링크는 GitHub pull 요청이다. 통과. |
| 28 | Chromium 분석 링크가 Markdown 산출 route와 맞지 않는가 | source 파일명에 대응하는 `.html` route를 사용한다. 통과. |
| 29 | 변경이 roadmap의 카운터·필터·체크 동작을 바꾸는가 | 관련 JS와 상태 API URL은 변경하지 않는다. 통과. |
| 30 | HTML 요약 수정이 제품 코드나 배포 설정을 바꾸는가 | 변경은 정적 설명·문서 연결에 한정한다. 통과. |

이 검토는 로드맵 설명의 범위·근거 연결·상태 경계를 검사한다. Tailnet 제공 내용은 병합 뒤 Agent Browser와 SHA-256 manifest로 별도 확인한다.
