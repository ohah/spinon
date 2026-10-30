# C11.4 PR #861 로드맵 갱신 PR diff 적대 검토 30개

**검토 범위:** `origin/main`의 PR #861 병합 상태를 기록한 `spec/STATUS.md`·계획 문서 변경과 `spec/public/roadmap.html`의 최신 진행 카드. Tailnet 파일 복사와 브라우저 게시 확인은 이 검토 범위가 아니며 별도 receipt로 확인한다.

| # | 독립 실패 관점 | 판정과 확인 |
| ---: | --- | --- |
| 1 | PR 상태가 실제 병합과 다르게 기록됨 | PASS — `gh pr view 861`의 `state=MERGED`와 문서 기록을 대조했다. |
| 2 | 병합 시각이 잘못됨 | PASS — GitHub의 `mergedAt=2026-10-30T14:39:19Z`와 대조했다. |
| 3 | 병합 방식이 잘못됨 | PASS — PR과 계획 문서 모두 Rebase and merge로 기록했다. |
| 4 | merge SHA가 다른 커밋을 가리킴 | PASS — `9b7c3a357d94db0e47f4397b73d3376a35de4439`가 PR merge commit 및 `origin/main` HEAD와 같다. |
| 5 | PR 링크가 잘못된 항목을 가리킴 | PASS — 로드맵 링크가 `ohah/spinon/pull/861`을 가리킨다. |
| 6 | normal control track 기대값이 반대로 적힘 | PASS — normal min-content column 20px로 기록했다. |
| 7 | normal control의 줄 수/세로 크기를 누락함 | PASS — 20×20px root와 두 줄 동작을 기록했다. |
| 8 | nowrap track 기여가 normal과 혼동됨 | PASS — nowrap column 50px로 기록했다. |
| 9 | nowrap item frame이 root frame으로 오인됨 | PASS — root 20×10px와 item 50×10px를 분리했다. |
| 10 | overflow 양이 입력과 맞지 않음 | PASS — 50px item과 20px root의 차이인 30px를 기록했다. |
| 11 | 공백 줄바꿈 동작을 잘못 요약함 | PASS — normal은 두 줄, nowrap은 한 줄로 구분했다. |
| 12 | `pre-line` 지원을 과장함 | PASS — 계속 거부한다고 명시했다. |
| 13 | nonzero `order` 지원을 과장함 | PASS — 계속 거부한다고 명시했다. |
| 14 | fixture 결과를 일반 cascade 지원으로 확대함 | PASS — 제한 opt-in fixture라고 명시했다. |
| 15 | Android 실행 환경을 실기기로 확대함 | PASS — API 37 ARM64 AVD로 특정했다. |
| 16 | iOS 실행 환경을 실기기로 확대함 | PASS — iPhone 17 Pro / iOS 26.2 Simulator로 특정했다. |
| 17 | draw/present를 glyph raster 비교로 오인함 | PASS — 캡처는 표면 경로만 보여주며 glyph pixel 판정에 쓰지 않았다고 적었다. |
| 18 | Android 결과를 hardware GPU/PixelCopy로 확대함 | PASS — 카드에 해당 주장을 넣지 않았다. |
| 19 | iOS 결과를 compositor 이후 화면 검증으로 확대함 | PASS — 카드에 해당 주장을 넣지 않았다. |
| 20 | 새 동작을 공개 CSS API로 오인함 | PASS — 일반 CSS admission이 추가되지 않았다고 명시했다. |
| 21 | 두 장면으로 전체 WPT 적합성을 주장함 | PASS — 전체 WPT 미완료를 명시했다. |
| 22 | 제한 slice를 C11.4 완료로 표시함 | PASS — C11.4 미완료를 명시했다. |
| 23 | 제한 slice를 상위 C11 완료로 표시함 | PASS — C11 미완료를 명시했다. |
| 24 | 전체 완료 수를 C11 전용 수치처럼 표시함 | PASS — 공식 전체 집계 72/213으로 표기했다. |
| 25 | PR check rollup 빈 상태를 CI 성공으로 표시함 | PASS — rollup이 비어 있고 필수 check가 없어 성공으로 기록하지 않는다고 적었다. |
| 26 | 계획 링크가 없는 문서 route를 가리킴 | PASS — `/spinon/plan/c11-4-anonymous-grid-text-nowrap.html`의 source 문서를 확인했다. |
| 27 | Chrome 사전 비교 링크가 잘못됨 | PASS — `spec/internal/evidence/c11-4-anonymous-text-nowrap-precomparison-2026-10-30/README.md`가 있다. |
| 28 | 실행 근거 링크가 잘못됨 | PASS — implementation README와 PR diff review 원본 파일을 확인했다. |
| 29 | Android/iOS screenshot 링크가 없는 파일을 가리킴 | PASS — 두 `anonymous-nowrap-min-content.png` source 파일을 확인했다. |
| 30 | 카드 위치·공식 상태 링크·배포 주장이 오래된 상태임 | PASS — #861 카드를 첫 hero progress card로 두고 공식 상태 route를 연결했으며, GitHub Pages 배포 주장은 넣지 않았다. |

로드맵 source의 로컬 링크 대조와 `git diff --check`를 통과했다. 이 문서는 Tailnet 동기화가 끝났다는 증거가 아니다.
