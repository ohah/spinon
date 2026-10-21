# C11.4 nested Grid 행 축 구현 PR diff 실패 관점 검토 20개

구현/runtime 실패 검토와 분리해, 제출할 전체 코드·앱 harness·capture runner·명세·증거 변경분의 결합 오류를 확인했다. 변경 경로는 이 제한 slice에만 한정했고 `git diff --cached --check`도 통과했다.

| # | diff 실패 가능성 | 확인 및 판정 |
| ---: | --- | --- |
| 1 | 계획 PR #444와 구현 상태가 잘못 연결됨 | plan SHA `1359f37024ad711df4b5253745d6dbf9b96af77f` 및 고정 plan/evidence 링크가 같은 slice를 가리킨다. 통과. |
| 2 | source fixture가 Rust feature 없이 기본 제품에서 활성화 | runtime registration과 runtime test module 모두 `c11-4-grid-track-sizing-fixture-test`에 guard된다. 통과. |
| 3 | FFI scene ID가 runtime scene allowlist와 불일치 | 세 문자열과 row styles가 FFI/runtime/app/capture runner에 정확히 일치한다. 통과. |
| 4 | 새 FFI dispatch가 기존 nested Grid scene을 가로챔 | adapter는 `nested-grid-text-image-row-` prefix에만 분기하며 기존 nested scene 회귀 테스트가 통과한다. 통과. |
| 5 | 앱 실행 인자 allowlist에 신규 ID 누락 | Android와 iOS 양쪽에 동일 ID 세 개를 등록했고 성공 simulator launch가 이를 실행했다. 통과. |
| 6 | 앱 root-frame expected 문자열이 oracle와 다름 | 세 scene의 `98×40`, `98×44`, `98×60` 기대가 양 platform simulator summary와 일치한다. 통과. |
| 7 | frame validator에서 nested/text/image frame 배열 순서나 값이 drift | 양 app harness와 runner가 node 1/2/3/5, image `y=24`, text `70×20`, image `24×16`을 비교한다. 통과. |
| 8 | 신규 scene의 box count가 3 또는 5로 잘못 반영 | Android와 iOS가 네 node box를 요구하고 세 scene 3/3을 통과했다. 통과. |
| 9 | definite-flex control이 auto-height scene로 분류 | 두 platform expected root frames와 generated CSS 모두 60px definite height를 기록한다. 통과. |
| 10 | capture runner가 기존 179-scene matrix에서 중복 또는 누락 ID를 셈 | `--list-scenes`에서 182 IDs, 신규 세 ID의 정확한 목록, unique-ID assertion을 확인했다. 통과. |
| 11 | runner의 nested-axis 비교가 기존 Columns scene에 회귀 | `innerAxis`·`contributionAxis`는 기존 scene의 Columns 기본값을 보존하고 신규 세 case만 Rows를 설정한다. Node syntax와 list-scene 검사가 통과했다. 통과. |
| 12 | row contribution 검증이 다른 Grid의 동일 item을 가져옴 | verifier는 scene별 outer/nested node ID로 trace를 조회한 뒤 axis, item, kind, span, value를 묶어 검사한다. 통과. |
| 13 | 내재 row track trace의 gutter를 track count에 포함 | runner/runtime test 모두 gutter를 제외해 used rows를 oracle와 비교한다. 두 platform trace matrix 통과. |
| 14 | platform screenshots가 다른 run 또는 해상도로 연결됨 | 대표 PNG는 검증된 span-two run에서 복사했고 SHA-256, Android 1080×2400 및 iOS 1206×2622을 evidence metadata와 대조했다. 통과. |
| 15 | 실패한 초기 Android allowlist run을 성공 결과처럼 섞음 | 실패 run은 별도 timestamp directory와 `rejected=true` 로그로 남고 성공 run은 다른 ID·결과 JSON에 보관된다. README가 원인을 설명한다. 통과. |
| 16 | successful Android APK hash를 설치 artifact 검증 없이 기록 | `apkSha256`과 `installedApkSha256`가 일치하며 result JSON cross-check가 이 비교를 다시 확인한다. 통과. |
| 17 | evidence README의 클릭 대상이 실제 artifact와 다른 경로 | JSON/log/static artifacts는 Tailnet public path에 맞춘 명시 raw anchor로, screenshot embeds는 task evidence file로 연결했다. PR screenshot raw URL은 게시 후 Agent Browser에서 별도 load 확인 gate다. |
| 18 | 계획·상태 문서가 C11.4 전체 완료를 선언하거나 집계를 증가 | official checkbox와 `72/213`을 유지한다고 status, plan, roadmap, evidence에 일관되게 기록했다. 통과. |
| 19 | docs build 실패를 성공 검증으로 표현 | 전체 RSPress build의 703 dead-link 오류 및 선택 route Tailnet 절차 차이를 failure summary에 기록했고 pass로 계수하지 않았다. 통과. |
| 20 | 제한·미검증 영역이 PR evidence에서 생략 | README와 plan에 glyph/PNG texel/surface pixel, physical Android, WPT, Android hardware GPU, iOS compositor-after, 일반 nested Grid 및 전체 C11.4가 미검증임을 명시했다. 통과. |

이 diff 검토는 repository change set을 대상으로 한다. GitHub PR body의 commit-pinned PNG URL은 PR을 만든 뒤 Agent Browser에서 실제 `<img>` `complete`, `naturalWidth`, `naturalHeight`를 확인하고 브라우저를 닫은 후 merge gate를 마친다.
