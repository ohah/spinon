# PR #974 병합 후 로드맵·Tailnet 변경 적대 검토 20개

검토 대상은 C11.4 결과 문서와 공개 roadmap 변경, 선택 Rspress build·Tailnet 게시 스크립트·hash 근거·Agent Browser 캡처를 포함한 이 PR의 전체 diff다. 서로 다른 실패 경계 20개를 확인했고, 실패를 발견한 항목은 외부 반영 전에 수정했다.

| # | 실패 관점 | 대조 및 판정 |
|---:|---|---|
| 1 | 잘못된 구현 PR·merge SHA가 문서의 원천으로 쓰임 | `gh pr view 974`의 MERGED 상태·시간·merge SHA를 plan, STATUS, 계약 0059, roadmap과 대조했다. 모두 `2700a8bfad05c1b8fc474603c58ae68c2dde3fb3`. PASS. |
| 2 | PR checks가 비었는데 CI 성공을 주장 | GitHub `statusCheckRollup`은 빈 배열이다. 새 문서도 CI 통과를 주장하지 않고 check 부재를 적는다. PASS. |
| 3 | 공식 완료 수나 C11 checkbox가 조용히 변경 | STATUS checkbox 수는 base/head 모두 checked 87개이고 Agent Browser의 공식 상태 화면에서 `72/213`, C11 unchecked, C11.4 unchecked를 확인했다. PASS. |
| 4 | 제한 fixture slice를 일반 CSS 지원으로 확대 주장 | 문서는 opt-in fixture, 일반 author CSS, WPT, 실기기/hardware GPU, 전체 C11.4/C11을 명시적으로 구분한다. PASS. |
| 5 | `word-break: break-word`와 `overflow-wrap` 효과가 혼동 | plan·0059·runtime evidence가 세 computed overflow-wrap 값, computed property 보존, normal controls를 별도로 고정한다. PASS. |
| 6 | 18-scene 수와 expected values가 서로 어긋남 | 2×3×3 계획, Chrome/Ahem precomparison, Android/iOS matrix 각 18행, STATUS 기록을 대조했다. PASS. |
| 7 | 시뮬레이터 근거를 실기기로 과장 | Android API 37.1 16 KB SwiftShader AVD와 iPhone 17 Pro/iOS 26.2 Simulator를 구체적으로 적고 실기기/hardware GPU를 제외한다. PASS. |
| 8 | 화면 캡처를 glyph/pixel equality 증거로 오인 | 실행 README의 glyph-paint disabled 한계를 유지하고 공개 card는 screenshot을 pixel 근거로 제시하지 않는다. PASS. |
| 9 | Stylo parser alias adapter가 일반 CSS cascade 지원으로 읽힘 | fixture-only token substitution·cascade winner 복원과 CSS-wide 키워드 범위, author CSS/custom property 제외를 계획·0059에서 구분한다. PASS. |
| 10 | 내부 계약 숫자 버전이 초안 구현만으로 상승 | 0059가 `0.1.0`을 유지하며 숫자 버전 변경 diff가 없다. PASS. |
| 11 | 선택 root가 사용자 미추적 변경을 끌어들임 | root 생성은 `git ls-files` index 목록만 읽으며 미추적 파일 전체 복사 경로가 없다. 기존 미추적 산출물을 수정·삭제·일괄 stage하지 않았다. PASS. |
| 12 | source symlink·`..` route가 선택 root 밖에 쓰임 | 준비 script는 source 각 경로의 `lstat`, route segment 검사, 새 output 디렉터리 존재/심볼릭 링크 검사를 한다. PASS. |
| 13 | publication이 기존 산출물이나 다른 문서 output을 덮음 | 선택 root·site·publication은 별도 신규 build 경로 아래 생성하고 대상 부재를 요구한다. Rspress는 lockfile의 2.0.22로 선택 root만 빌드했다. PASS. |
| 14 | 중복 alias route가 서로 다른 바이트를 숨김 | staging은 route 충돌 때 이전/신규 SHA-256 동등을 요구하고, roadmap card의 다섯 local link를 생성 route에 대조한다. 제외된 열린 legacy alias는 manifest에 명시한다. PASS. |
| 15 | 잘못된 process나 끊어진 preview에 덮어씀 | preflight에서 Tailscale Running, 기존 예상 preview process/listener, local roadmap HTTP 200을 확인했다. server는 기존 PID 28040이며 재시작하지 않았다. PASS. |
| 16 | 목적지 symlink·열린 파일·경합 경로를 교체 | preflight는 target 경로 symlink를 거부하고 변경 route의 open handle 0개를 요구한다. copy는 preflight 뒤 target hash를 재검사하고 교체 직전 open handle을 다시 검사한다. PASS. |
| 17 | 기존 preview data 손실 또는 복구 불가 | 교체 대상마다 고유 backup 경로를 쓰고 COPYFILE_EXCL 및 이전 hash 대조를 한다. 최종 copy/verify report는 백업 수와 모든 hash를 개별 기록하며 삭제는 하지 않는다. PASS. |
| 18 | 중간 실패가 조용한 부분 복사를 성공으로 보고 | 신규 파일과 교체 파일은 같은 디렉터리 임시 파일에서 rename하고 각 local hash를 확인한다. manifest는 create/replace/same을 분리하고 verify 단계가 전체 선택 route를 재검증한다. PASS. |
| 19 | HTTPS 성공 응답이 다른 문서·stale bytes를 반환 | final local·HTTPS 검증 report에서 모든 선택 route의 HTTP 200과 SHA-256을 확인하고 source head/tree/diff를 고정했다. 전체 site dead-link 검사는 했다고 주장하지 않는다. PASS. |
| 20 | 실제 화면·모바일 표시·브라우저 수명 검증 누락 | Agent Browser에서 roadmap/STATUS/review route를 읽고 20행 모두 PASS, desktop/mobile 캡처와 390px 가로 overflow 부재를 저장했다. `close --all` 뒤 browser process가 남지 않았고 기존 preview listener만 유지됐다. PASS. |

최종 판정: 서로 다른 20개 관점에서 미해결 모순·과장·게시 경로 위험을 찾지 못했다. 전체 문서 사이트 링크 검사, C11 전체 동작, simulator 외 플랫폼, glyph pixel 판정은 이 PR의 주장 범위에 넣지 않는다.
