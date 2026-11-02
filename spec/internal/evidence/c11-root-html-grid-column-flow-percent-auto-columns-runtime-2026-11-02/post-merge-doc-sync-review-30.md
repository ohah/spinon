# C11.1 percentage implicit columns post-merge documentation review (30)

검토 대상은 PR #1063 병합 뒤의 계획, 공식 상태 대장, 내부 계약 0059, public roadmap 카드와 evidence 링크 변경이다. 결과는 30개 관점 통과, blocker 0이다. C11.1/C11 완료 체크는 변경하지 않는다.

| # | 별도 실패 관점 | 결과 및 근거 |
|---:|---|---|
| 1 | status/계획에 잘못된 PR merge SHA나 시간이 기록되는가 | 통과: PR #1063의 `246babd793b27c2ca9e8edc5d81b8726c7479816`와 merge UTC 시각을 GitHub 결과와 대조했다. |
| 2 | 계획 전 후보 상태가 현재 상태처럼 남는가 | 통과: 계획 첫 상태 문단을 제한 구현 병합 상태로 바꿨다. |
| 3 | 과거 계획 수립 기록을 지워 시간 순서가 사라지는가 | 통과: status의 기존 계획 후보 이력을 보존하고 뒤에 merge record를 추가했다. |
| 4 | C11.1 checkbox를 작은 slice만으로 완료하는가 | 통과: 공식 checkbox는 diff에 없고 계획·contract·roadmap이 C11.1 미완료를 명시한다. |
| 5 | 공식 완료 집계가 72/213에서 증가하는가 | 통과: 새 status record와 public card 모두 72/213을 유지한다. |
| 6 | C11 parent 또는 전체 CSS Grid 지원을 주장하는가 | 통과: 제한 Root HTML profile, no general CSS admission과 no WPT를 명시했다. |
| 7 | plan에 구현된 지원 범위가 누락되거나 과장되는가 | 통과: finite non-negative direct px/percentage scalar·list로 제한했다. |
| 8 | implicit pattern이 explicit column 뒤 첫 track부터 반복된다는 계약이 사라지는가 | 통과: authored order와 first implicit column 규칙을 plan 및 contract에 썼다. |
| 9 | percentage basis를 viewport나 남은 공간으로 잘못 설명하는가 | 통과: definite direct CSS-pixel content width로 고정했다. |
| 10 | zero basis가 indefinite width와 혼동되는가 | 통과: definite zero width를 유효한 basis로 남겼다. |
| 11 | 100% 초과 percentage가 clamp된다고 문서화되는가 | 통과: 100% 초과 값의 unclamped 보존을 plan·contract·roadmap이 일관되게 설명한다. |
| 12 | column gap을 percentage basis에서 뺀다고 설명하는가 | 통과: basis와 gap 위치 계산이 분리되어 있다고 명시한다. |
| 13 | unsupported width·track shape 경계가 사라지는가 | 통과: indefinite/math/min/max/aspect-ratio와 intrinsic/dense/row cases를 제외한다. |
| 14 | visible item, placement, nesting 제한을 잃는가 | 통과: empty direct div, auto placement, order 0, one non-nested Grid 제약을 보존했다. |
| 15 | 오류 시 partial scene이 허용되는 것처럼 보이는가 | 통과: fail-closed 및 scene publish 전 validation을 적었다. |
| 16 | 내부 계약 버전이 feature slice 때문에 올라가는가 | 통과: contract 0059 `0.1.0` 유지가 기록됐다. |
| 17 | simulator screenshot을 pixel readback으로 부르는가 | 통과: visual-only이며 PixelCopy/Metal drawable/compositor parity가 아니라고 제한한다. |
| 18 | Android simulator를 실기기나 hardware GPU로 잘못 기술하는가 | 통과: API 37 AVD/SwiftShader 범위를 분리한다. |
| 19 | iOS simulator를 iOS physical device로 확장 해석하는가 | 통과: iPhone 17 Pro iOS 26.2 Simulator임을 고정하고 physical device를 제외한다. |
| 20 | grid/item 좌표와 outer viewport wrapper 차이를 혼합하는가 | 통과: 240×63 grid 및 item frames만 맞다고 쓰고 outer wrapper의 viewport 차이를 남긴다. |
| 21 | Chrome observation을 normative oracle로 과장하는가 | 통과: Chrome은 관측자이고 독립 expected와 CRD가 기준임을 plan에 유지한다. |
| 22 | 테스트 수가 evidence 로그와 맞지 않는가 | 통과: 15+28+9+2+1=55 focused tests를 README와 status에서 같은 기준으로 기록했다. |
| 23 | scoped Clippy/rustfmt/build 경계를 잘못 표현하는가 | 통과: affected package scope 및 archive static-library 한계를 분리했다. |
| 24 | 기본 fixture ABI/payload 분리 확인이 누락되는가 | 통과: default absence와 opt-in positive control을 contract·runtime evidence에 연결한다. |
| 25 | 새 fixture FFI를 사용자 공개 API로 게시하는가 | 통과: 내부 opt-in validation route이고 public API가 아님을 명시한다. |
| 26 | status/evidence/plan relative routes 중 하나가 오타인가 | 통과: 파일 경로를 직접 확인했고 roadmap의 evidence README route를 일치시켰다. |
| 27 | RSPress가 JSON/fixture/PNG/log 파일을 새 dead route로 처리하는가 | 통과: local non-page artifacts는 inline path 또는 GitHub file link로 바꿨다. |
| 28 | 선택된 public roadmap card가 merge 후에도 계획 후보라고 표시되는가 | 통과: `제한 구현 병합` 및 PR #1063으로 갱신했다. |
| 29 | 전체 문서 빌드 실패를 green으로 보고하는가 | 통과: `docs:build`는 기존 repo-wide dead links로 실패했고 성공이라 하지 않았다. 새 runtime/precomparison artifact 링크 오류는 조정 후 해당 경로에서 사라졌고, 기존 plan link 및 다른 C11.4/C13 evidence dead links는 남는다. |
| 30 | Pages 배포를 preview sync와 혼동하거나 자동 실행하는가 | 통과: PR에서는 Pages/gh-pages를 건드리지 않는다. Tailnet preview는 merge 뒤 선택된 route/hash 검증을 거쳐 별도로 동기화한다. |

문서 검증 당시 GitHub workflow check는 확인하지 않았고 전체 RSPress build는 실패 상태다. Tailnet publication 완료 여부는 merge 뒤 별도 실행·hash 검증 후 기록한다.
