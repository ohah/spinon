# C13.3 touch-action simulator matrix PR 변경 관점 검토 20개

대상: PR [#187](https://github.com/ohah/spinon/pull/187), 검토 기준 head `78c55794` (구현 source commit `2891d273cfc3fdaf2cb16313ccbf42385317ff00`). 이 검토 파일은 해당 PR 변경 검토 뒤 증거로 추가했다. 작업 단위 구현 검토와는 별도로 플랫폼 간 연결, 증거의 출처·경계, 저장소 공개 문서와 PR 게시 상태를 확인했다.

1. **행렬이 선언한 교차 조합을 빠뜨리는가** — scenario 3–7 × horizontal/vertical × Android/iOS가 `matrix.json`에 20개 case로 들어 있고 20개가 모두 종료 상태다. 허용 6개와 거부 14개의 합도 20으로 맞는다.
2. **scenario ID가 앱·Rust·fixture source 사이에서 서로 다른 정책을 뜻하는가** — Kotlin/Swift argument gate, Rust `TryFrom<i32>`, JS prelude의 ID 3–7 매핑 및 host readiness report를 대조했다. 각 경로의 이름과 `touch-action` 값이 같은 scenario를 가리킨다.
3. **앞선 scenario 0–2의 고정 동작을 행렬 추가가 바꾸는가** — 0–2 mapping, overscroll policy와 boundary seed 조건을 기존 값으로 유지하며 3–7은 별도 case로 추가했다. 기본 artifact 결과도 새 fixture 경로가 기본 구성에 나타나지 않는다고 기록한다.
4. **scene 크기 변경이 Android와 iOS의 초기 draw 검사를 어긋나게 하는가** — Rust fixed scene의 frame 수 10→11, 세 owner 수, 앱의 scenario별 초기 seed/offset/revision 기대값을 같이 확인했다. 새 touch-action scene들은 seed 없는 revision 0으로 보고된다.
5. **computed action 보고값이 CSS fixture와 일치하지 않아도 앱이 ready 상태가 되는가** — host는 scenario별 target point의 computed effective action을 비교하고, Android/iOS adapter는 effective·inner·descendant report 필드를 검사한 뒤 ready를 허용한다.
6. **조상과 자식이 각각 허용하는 축을 단순 합집합으로 처리하는가** — scenario 7의 inner `pan-y`와 descendant `pan-x`는 effective `none`으로 기대된다. 양 플랫폼 4 swipe 결과가 모두 offset/revision 무변경이다.
7. **target 내부 policy와 inner owner policy를 혼동하는가** — scenario 6의 target descendant `pan-x`는 inner `auto`와 구분해 report 필드에 남는다. horizontal은 inner만 움직이고 vertical은 거부된다.
8. **허용 방향의 결과가 다른 owner까지 번지는가** — matrix 허용 6건 각각에서 root와 outer offset은 0이다. 승인된 inner offset만 변하며 case report의 revision 증가와 맞는다.
9. **거부 방향이 첫 swipe 뒤 revision만 증가하거나 offset 일부만 바꾸는가** — 거부 14건의 inner/root/outer offset과 `ScrollStateRevision`이 0이다. 독립 case log와 JSON summary의 판정이 일치한다.
10. **Android와 iOS 입력 좌표를 같은 단위로 오해해 수치를 동일 pass 기준으로 삼는가** — 두 adapter의 실제 native swipe 결과를 각각 기록하고, README는 platform별 이동량을 구분한다. 판정은 CSS policy·owner·revision이지 서로 다른 자동화 도구의 물리 거리 동등성이 아니다.
11. **시뮬레이터 결과를 실기기나 GPU 하드웨어 결과로 게시하는가** — evidence metadata는 iOS 26.2 Simulator와 API 37 ARM64 AVD/SwiftShader를 명시하고 연결된 실기기는 사용하지 않았다고 제한한다.
12. **화면 픽셀 검사가 status bar/home indicator 변화까지 scroll marker로 세는가** — pixel analysis는 system UI pixel을 판정에서 제외하고 marker RGB bbox를 inner border와 대조한다. 네 after 화면 모두 ancestor marker pixel 0이다.
13. **거부 14개 전체를 screenshot이 시각 증명한다고 과장하는가** — README는 시각 자료를 허용 방향 4개 before/after로 한정하고, 거부 방향은 각 native case log와 offset/revision 결과로 제시한다.
14. **PR 본문 screenshot URL이 이동 가능한 branch 이름이나 잘못된 파일을 가리키는가** — 네 URL 모두 immutable head commit SHA와 실제 matrix PNG를 가리킨다. Agent Browser에서 `complete=true`; iOS 1206×2622, Android 1080×2400 원본 크기로 로드됐다.
15. **PR 이미지가 정상 로드되어도 장면이 다른 실행의 screenshot인가** — alt label, 파일명, matrix/platform metadata와 screenshot dimensions를 비교했다. 화면의 C13.3 touch-action fixture와 방향별 marker가 해당 label에 맞는다.
16. **기본 제품 artifact에 앱 검증용 ABI나 fixture string이 유입되는가** — default/opt-in 정적 FFI archive positive/negative 대조가 고유 C ABI 4개와 JS marker를 확인한다. 이 결과를 최종 앱 번들 크기 검증으로 확대 해석하지 않는다.
17. **공개 roadmap이 C13.3 전체 또는 public CSS 지원 완료로 보이게 하는가** — roadmap과 `STATUS.md`는 matrix 후속만 기록하고 C13.3 전체·public event/API·WPT와 남은 입력 범위를 미완료로 유지한다.
18. **지원 계약을 내부 수치 버전 상승으로 잘못 표현하는가** — 내부 계약 0063의 UIKit zero-translation 설명만 보강했고 `0.1.0` 숫자 버전을 바꾸지 않았다.
19. **문서 빌드 실패를 PR에서 숨기거나 전체 성공으로 보고하는가** — full `docs:build` 실패를 기존 dead link/dead image 및 Shiki 언어 태그 문제로 본문에 기록했다. 변경 페이지는 별도 Rspress preview로 렌더링했으며 full-site build 성공이라고 표현하지 않았다.
20. **선형 이력 병합 뒤 PR 또는 이미지 근거를 유실하는가** — 현재 main protection은 `required_linear_history=true`, required status checks 없음, 승인 리뷰 수 0이다. 병합 전 checks와 mergeability를 다시 조회하고 Rebase and merge를 사용해야 한다. PNG 원본은 head commit SHA에 저장되어 rebase 후에도 PR 본문 이미지가 접근 가능하다.

## 판정

20개 관점에서 merge blocker를 찾지 못했다. 이 판정은 고정 fixture의 단일 pointer simulator 행렬과 게시된 evidence에 한정한다. `gh pr checks`는 검토 시점에 “no checks reported”였으며 테스트 통과로 대신 세지 않았다. full docs site 빌드는 실패 상태로 남아 있다.
