# C11.3.3 track-list roadmap 동기화 PR 변경 검토 30개

**검토 대상:** PR #814 병합 뒤 `spec/STATUS.md`, 내부 계약 0059, C11.3.3 계획 두 문서와 공개 `roadmap.html`의 bounded behavior 갱신. 제품 코드는 바꾸지 않는다. 공식 구현 상태와 사용자 요청에 따른 Tailnet 개발 미리보기 경계를 검토한다.

| # | 실패 관점 | 확인 결과 |
| ---: | --- | --- |
| 1 | PR #814의 기능 변경을 C11 전체 완료로 승격한다. | C11.3.3 candidate와 C11 checkbox가 계속 미완료다. 통과. |
| 2 | 세부 구현 때문에 공식 완료 수가 증가한다. | 상태 대장·계획·공개 로드맵에 `72/213`을 유지한다. 통과. |
| 3 | 기준일만 바뀌고 실제 병합 근거가 빠진다. | PR #814 URL, Rebase 병합 시각과 merge SHA `4b40ffc3…`가 함께 기록된다. 통과. |
| 4 | implementation PR의 원래 commit SHA를 merge SHA로 오기한다. | 상태 대장·track-list 상세 계획·계약·공개 roadmap의 merge SHA가 PR 병합 결과와 일치한다. 통과. |
| 5 | merge 시각을 로컬 날짜로 잘못 변환한다. | GitHub의 UTC `2026-10-29T18:48:29Z`를 그대로 기록한다. 통과. |
| 6 | area dimension에서 implicit track 생성이 아직 거부되는 것으로 남는다. | 계약 0059는 PR #807의 실제 area-dimension 확장을 설명한다. 통과. |
| 7 | track-list가 임의 template route에서 허용되는 것처럼 보인다. | `RuntimeBlockFlexGridTemplateV1`의 validated template-area route에 한정한다. 통과. |
| 8 | 목록의 빈 입력을 지원 범위에 포함한다. | 목록은 non-empty 조건을 명시하고 빈 목록은 제외한다. 통과. |
| 9 | 임의 CSS track-size 목록을 지원한다고 읽힌다. | 목록 각 항목이 fixed `px`인 경우로 제한한다. 통과. |
| 10 | 행과 열의 목록 주기를 하나로 합친다. | 각 축이 독립적으로 첫 항목부터 순환한다고 적는다. 통과. |
| 11 | authored explicit track을 implicit 목록으로 다시 크기화한다. | authored tracks 보존을 계약에 명시한다. 통과. |
| 12 | area-created track 수와 목록 phase를 혼동한다. | phase는 해당 축에서 area가 소비한 track-list 항목을 반영한다. 통과. |
| 13 | 후속 placement의 임의 숫자 선 배치를 허용한다. | phase continuation은 지원되는 positive named implicit-line route에 한정한다. 통과. |
| 14 | backward 또는 negative sizing을 회귀로 지원한다. | backward/negative sizing을 남은 범위로 명시한다. 통과. |
| 15 | numeric implicit placement의 보장이 확대된다. | numeric implicit placement는 제외 범위에 둔다. 통과. |
| 16 | non-px 목록이 자동으로 픽셀화돼 지원된다고 보인다. | non-px 목록은 명시적으로 미지원이다. 통과. |
| 17 | auto-repeat과 목록 cycling이 결합됐다고 보인다. | auto-repeat은 제외 범위로 남는다. 통과. |
| 18 | `auto`와 단일 px 기존 동작이 사라진다. | 기존 scalar 동작을 보존한다고 계약에 기록한다. 통과. |
| 19 | 구현 후보 기록을 공개 CSS 보장으로 승격한다. | public roadmap과 status가 제한 profile 및 candidate 상태를 유지한다. 통과. |
| 20 | matrix scene/node 수가 이전 31/90 결과와 혼합된다. | 신규 PR #814 행렬을 37 scene·118 node로 분리해 표기한다. 통과. |
| 21 | fallback/rejection 수가 success 수와 모순된다. | 31 success + 5 cascade fallback + 1 expected reject = 37이다. 통과. |
| 22 | visible frame 수가 전체 node 수로 오해된다. | 116 visible frame/node 비교라고 범위를 명시한다. 통과. |
| 23 | geometry 허용 오차를 pixel exact로 잘못 읽는다. | geometry와 pixel sample 결과를 별도로 기술한다. 통과. |
| 24 | Android 기기 검증으로 확대 해석한다. | Android API 37 ARM64 AVD와 SwiftShader를 명시하고 실기기 미검증을 남긴다. 통과. |
| 25 | iOS sample을 compositor 이후 화면으로 확대한다. | Metal drawable 표본은 compositor 제출 전이라고 적는다. 통과. |
| 26 | 28 pixel sample을 28개 scene으로 오기한다. | 6 scene / 28 samples 및 28/28 exact를 구분한다. 통과. |
| 27 | static archive 근거를 APK/IPA 제외 근거로 확대한다. | archive 확인만 증명하며 release bundle은 증명하지 않는다고 제한한다. 통과. |
| 28 | GitHub required check 미설정을 CI 통과로 바꿔 쓴다. | PR에 required check와 check run이 보고되지 않았다고 정확히 쓴다. 통과. |
| 29 | 공개 roadmap의 링크가 다른 C11.3.3 slice를 가리킨다. | 계획, 계약 0059, PR #814 runtime evidence 경로를 새 slice에 맞춘다. 통과. |
| 30 | Tailnet 개발 미리보기를 공식 GitHub Pages 배포로 혼동한다. | Tailnet은 별도 개발 미리보기로 갱신하고 Pages workflow는 실행하지 않는다. 통과. |

## 판정

30개 독립 실패 관점에서 공식 상태, 계약 범위, 시뮬레이터 증거와 배포 경계를 대조했다. 차단 결함은 발견되지 않았다. Tailnet 페이지와 이미지 로드는 Agent Browser로 별도 확인하고 브라우저 세션을 종료한다.
