# C11.1 percentage named-area 병합 후 상태·로드맵 PR diff 검토 30개

검토 대상은 PR [#1035](https://github.com/ohah/spinon/pull/1035)의 병합 SHA와 C11.1 제한 구현 범위를 계획·상태 대장·내부 계약 0059·공개 `roadmap.html`에 동기화한 변경이다. 선택 Rspress build와 Agent Browser desktop/mobile 캡처도 확인했다.

| # | 독립 실패 관점 | 결과 |
| ---: | --- | --- |
| 1 | GitHub에서 조회한 구현 PR merge SHA가 문서와 다른가 | #1035 merge commit `4b921f3e8ccd86e7e156562bb7e5b4a464f082f2`가 각 문서에 일치한다. |
| 2 | 병합 시각과 문서 날짜가 혼동되는가 | UTC 시각 `2026-11-02T05:24:52Z`와 작업일을 분리해 표시한다. |
| 3 | C11 parent checkbox가 제한 구현 병합 때문에 체크되는가 | `spec/STATUS.md`의 `C11 CSS Grid`는 계속 `[ ]`다. |
| 4 | C11.1 전체를 한 조합 slice 병합으로 완료 처리하는가 | 전용 plan, contract, status, public card 모두 C11.1 전체 완료를 부인한다. |
| 5 | 공식 aggregate가 임의로 증가하는가 | 바뀐 문서에서 공식 aggregate는 `72/213`으로 유지된다. |
| 6 | public card가 지원 범위를 일반 CSS Grid로 확대하는가 | 기존 2×2 named-area map과 direct finite non-negative px/% tracks라고 한정한다. |
| 7 | named-area 없이 입력 가능한 모든 percentage tracks를 이 slice에 포함하는가 | 기존 unnamed percentage slice는 별도 plan/PR로 남는다. |
| 8 | indefinite axis에서 percentage basis가 있는 것처럼 보이는가 | definite width/height 축 경계를 각 문서에서 유지한다. |
| 9 | `%`와 `fr` 혼합을 지원한다고 오해하는가 | 혼합은 거부된다고 전용 plan과 내부 계약에 남아 있다. |
| 10 | named-area topology가 임의 map으로 일반화되는가 | 고정 `header/header`·`side/content` 2×2 map을 명시한다. |
| 11 | over-100% track이 clamp되는 것처럼 보이는가 | runtime evidence의 `[144,12]`, x=144 geometry 설명을 그대로 연결한다. |
| 12 | zero-width item frame과 draw box 수를 같은 값으로 취급하는가 | zero-track frame의 Rust 검증과 5 paint box 경계를 분리해 둔다. |
| 13 | min/max 진단 오류의 수정 여부가 문서마다 어긋나는가 | 실제 위반 property attribution 수정과 회귀 검사를 동일하게 설명한다. |
| 14 | aspect-ratio/override 거부가 사라지는가 | named-area percentage projection rejection과 네 min/max 속성을 유지한다. |
| 15 | `calc()`, `minmax()` 또는 percentage gap을 지원하는 것처럼 링크가 읽히는가 | 전용 계획과 contract에 모두 미지원 경계를 기재한다. |
| 16 | simulator 수치를 physical-device 또는 hardware GPU 결과로 확대하는가 | Android SwiftShader AVD와 iOS Simulator 한계를 분명히 남긴다. |
| 17 | 화면 캡처를 PixelCopy/Metal readback으로 오인하는가 | public card와 README가 화면 캡처를 visual evidence로만 분류한다. |
| 18 | iOS 캡처를 compositor 이후 pixel 결과로 확대하는가 | compositor 이후 pixel 비교 미실행을 표시한다. |
| 19 | 플랫폼별 scene 수가 누락되거나 서로 다른 source를 사용한 것으로 보이는가 | 동일한 네 scene을 각 플랫폼 4/4 실행한 runtime evidence로 연결한다. |
| 20 | screenshot direct links가 비어 있거나 MIME이 틀린가 | Agent Browser fetch에서 Android/iOS 둘 다 HTTP 200·image/png·양수 byte를 확인했다. |
| 21 | public card에 PR #1035가 누락되거나 중복 삽입되는가 | 선택 build에서 card marker가 정확히 한 번 나타난다. |
| 22 | public card가 잘못된 SHA나 외부 PR을 가리키는가 | 실제 PR URL과 merge SHA가 카드 텍스트에 일치한다. |
| 23 | public card의 plan/runtime/status route 중 하나가 누락되는가 | Agent Browser DOM에서 eight links를 읽고 선택 route build에서 대응 문서·이미지를 확인했다. |
| 24 | raw `roadmap.html`을 Rspress가 처리하지 않아 업데이트가 빠지는가 | raw HTML은 selected publication source route로 보존하고 빌드된 plan/status pages와 분리했다. |
| 25 | `roadmap.html`의 merged card가 viewport 밖 또는 잘린 상태로 캡처되는가 | desktop 캡처에 #1035 카드 전체와 아래 섹션 시작이 보인다. |
| 26 | mobile viewport에서 수평 overflow가 생기는가 | Agent Browser에서 390px viewport와 390px document width를 확인했다. |
| 27 | status 집계가 public roadmap과 status route에서 달라지는가 | Agent Browser page와 선택 STATUS route 모두 `72/213`을 포함한다. |
| 28 | 선택 root route 검사를 전체 문서 링크 검사라고 과장하는가 | README가 repository 전체 link validation을 보증하지 않는다고 한정한다. |
| 29 | local candidate screenshot을 Tailnet live 배포 근거로 쓰는가 | README는 local build 캡처이며 live Tailnet 증거가 아니라고 분리한다. |
| 30 | 내부 계약 버전이 문서 동기화만으로 불필요하게 변경되는가 | contract version `0.1.0`을 유지한다. |

**검토 결과:** 공개 상태 집계와 C11 완료 판정은 바꾸지 않고 병합 SHA·지원 경계·검증 제한을 같은 값으로 연결했다. 선택 Rspress build와 1440px/390px Agent Browser route 검증이 통과했다. repository 전체 Rspress build 시 unrelated 문서들의 dead link 오류가 보고되어, 이 결과를 선택 build의 통과와 혼합하지 않는다. Live Tailnet 반영은 별도 실행 후 기록한다.
