# PR #1137 병합 후 상태·로드맵 변경 적대 검토 30개 · 2026-11-04

대상은 `spec/STATUS.md`, `spec/public/roadmap.html`, numeric placement 계획 상태와 구현 evidence의 병합 표기다. 기준은 구현 PR [#1137](https://github.com/ohah/spinon/pull/1137), merge SHA `6af14f69512c0c73d890efc728751e0ca342f9c0`, 해당 PR의 runtime README 및 현재 `main` 보호 설정이다. 검토는 상위 C11 완료 여부가 아니라 제한 slice의 공개 기록 정확성을 확인한다.

| # | 독립 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | 잘못된 PR이 연결되는가 | #1137은 숫자형 Root HTML Grid 배치 구현 PR이며 병합 상태와 일치한다. |
| 2 | merge SHA가 원래 feature commit과 혼동되는가 | 문서에는 GitHub rebase merge가 만든 `6af14f69512c0c73d890efc728751e0ca342f9c0`만 merge SHA로 쓴다. |
| 3 | UTC 시각이 날짜 경계에서 오기되는가 | GitHub의 `2026-11-03T22:48:45Z`를 기록하고 작업일 표시는 KST 기준 11월 4일로 둔다. |
| 4 | 제한 구현이 전체 C11 완료로 읽히는가 | STATUS와 roadmap 모두 C11.1/C11 미완료, 공식 `72/213` 유지라고 명시한다. |
| 5 | 지원 route가 일반 CSS parser 전체로 확대되는가 | 기본 parsed Root HTML의 no-area row-flow Grid로 범위를 한정한다. |
| 6 | 한 Grid의 아이템만 조건을 만족해도 전체가 허용되는가 | visible item 전부가 양 축 definite numeric placement를 가져야 한다고 명시한다. |
| 7 | 한 축만 definite인 placement가 수용된다고 오해되는가 | 두 축 조건을 공개 문구에 모두 적었다. |
| 8 | string shorthand를 새 parser가 읽는다고 주장하는가 | 새 shorthand parser가 없고 기존 computed/typed 경계를 재사용한다고 적었다. |
| 9 | 두 번째 배치 algorithm이 생겼다고 주장하는가 | 기존 C11.2.1 normalizer 사용으로 서술한다. |
| 10 | implicit track 생성이 포함됐다고 오해되는가 | implicit track 지원을 추가하지 않았다고 STATUS에서 명시한다. |
| 11 | named-area 경로가 이 slice에 포함되는가 | no-area 경계가 문서에 남아 있으며 다른 route와 구별된다. |
| 12 | dense auto-placement가 numeric explicit profile로 바뀌는가 | 이 카드에는 dense 동작을 포함하지 않고 evidence가 기존 dense projection 분리 회귀를 기록한다. |
| 13 | column-flow가 함께 바뀌었다고 읽히는가 | row-flow만 scope로 명시하고 column-flow 회귀를 별도 gate로 표기한다. |
| 14 | hidden child 처리로 visible item 수가 과장되는가 | 문서 표현은 visible item 기준이고 runtime README가 hidden child control을 분리한다. |
| 15 | anonymous text가 자동 numeric placement로 취급되는가 | 구현 evidence의 rejection scope와 public bounded wording이 이를 지원 범위로 올리지 않는다. |
| 16 | mixed/partial auto rejection이 성공 경로에 섞이는가 | runtime README의 negative matrix는 장면 전체 거부와 atomic 결과를 기록한다. |
| 17 | 같은 session recovery가 누락되는가 | runtime README가 거부 후 유효한 환경 복귀 및 새 layout publish를 별도로 설명한다. |
| 18 | C11.5 self-alignment 교차 검증을 full support로 일반화하는가 | 상태 문구는 교차 장면 검증 사실만 기록하고 C11.5 전체 범위를 승격하지 않는다. |
| 19 | overflow 주장이 아무 화면 근거 없이 적히는가 | simulator PNG sample이 Grid 테두리 바깥 item 영역 red와 item 이후 white를 관찰했다고 기록하고, Markdown image 경로를 Rspress가 해석하는 `./` 상대 경로로 고쳤다. |
| 20 | 캡처를 renderer surface readback으로 오인하는가 | PixelCopy·Metal drawable readback이 아니라고 명시한다. |
| 21 | Android 검증이 실기기 실행으로 오기되는가 | API 37 ARM64 AVD와 SwiftShader라고 고정하고 실기기/hardware parity를 주장하지 않는다. |
| 22 | iOS Simulator가 실기기 결과로 확대되는가 | iPhone 17 Pro/iOS 26.2 Simulator로만 표기한다. |
| 23 | 22-box가 독립 픽셀 비교의 item 수로 혼동되는가 | 문장상 draw/present 수치이며 pixel 표본의 별도 한계를 함께 둔다. |
| 24 | focused tests 수치가 전체 package green으로 읽히는가 | focused filter 숫자로 제한하고 별도 broad alignment baseline 실패를 기록한다. |
| 25 | 변경 전 실패가 이번 변경 탓이라고 숨겨지는가 | 네 alignment filter 실패가 수정 전 baseline에서도 재현됐음을 공개한다. |
| 26 | scoped fmt/Clippy가 workspace 전체 통과로 확대되는가 | scoped gate로 명시하고 전체 suite 통과라고 쓰지 않는다. |
| 27 | GitHub check 공백을 CI green으로 오해하는가 | PR check rollup 공백과 CI 성공으로 세지 않는다는 문구를 둔다. |
| 28 | 보호 규칙에 없는 필수 check를 임의로 주장하는가 | 현재 main required status checks 미설정 사실과 일치한다. |
| 29 | 새 카드의 경로가 문서 사이트 base와 어긋나는가 | plan/evidence/status 링크는 기존 `/spinon/` 경로 구조에 맞춘다. |
| 30 | Tailnet 또는 GitHub Pages 반영이 이미 끝났다고 과장하는가 | 이 변경은 source roadmap만 기록하며 배포 성공 주장을 추가하지 않는다. |

검토 결과: 30개 관점에서 범위·merge 기록·검증 한계의 모순을 찾지 못했다. 전체 C11 상태와 완료 집계는 그대로 유지한다.
