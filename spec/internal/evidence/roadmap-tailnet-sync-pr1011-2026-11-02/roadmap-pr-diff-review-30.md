# C11.1 anonymous Grid text 병합 후 상태·로드맵 diff 검토 30개

검토 대상은 PR #1011 merge SHA `40725c44d5cfe7ea187fe7731c47fed439c3b98f` 뒤의 공식 `spec/STATUS.md`, 상위·전용 C11 계획, 공개 `spec/public/roadmap.html` 변경이다. 구현 PR 검토와 분리해 merge provenance, 지원 범위, 공식 완료 상태, 문서 링크 및 공개 표현을 대조한다. Tailnet 반영은 main 병합 이후 별도 실행하므로 이 문서는 미리 반영 완료라고 주장하지 않는다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR #1011의 다른 commit을 merge SHA로 오인함 | GitHub `mergeCommit.oid`는 `40725c44d5cfe7ea187fe7731c47fed439c3b98f`이며, 이 문서 PR의 시작 base인 `origin/main` commit과 일치한다. |
| 2 | 병합 시각을 현지 시간으로 변환해 잘못 기록함 | GitHub `mergedAt`의 UTC `2026-11-01T23:22:20Z`를 그대로 사용한다. |
| 3 | PR 상태를 계속 open으로 표기함 | GitHub는 `MERGED`; STATUS, 계획, 공개 roadmap은 Rebase and merge 완료로 정정한다. |
| 4 | 코드 병합을 GitHub CI 통과라고 확대함 | PR에는 check run이 보고되지 않았다는 기존 사실을 보존한다. |
| 5 | 상위 C11 checkbox를 완료 처리함 | `spec/STATUS.md`의 C11 상위 `[ ]`를 유지한다. |
| 6 | C11.3 완료로 확대함 | C11.3 checkbox는 `[ ]`로 유지한다. |
| 7 | C11.4 완료로 확대함 | C11.4 checkbox는 `[ ]`로 유지한다. |
| 8 | 이전 공식 완료 수에 새 slice를 더함 | 공식 `72/213`을 그대로 유지한다. |
| 9 | C11 하위 완료 checkbox 개수를 전체 진행률로 게시함 | 새 백분율이나 등가 완료 개수를 만들지 않는다. |
| 10 | PR #1011을 C11.1 전체 완료라고 표현함 | fixed-px Grid의 단일 `AB` direct-text slice로 제한한다. |
| 11 | arbitrary anonymous text도 동작한다고 암시함 | 다른 text run 조합은 fail-closed라고 기록한다. |
| 12 | mixed element/text 지원을 주장함 | mixed visible content는 지원 밖으로 남긴다. |
| 13 | C11.4 fixture gate를 기본 runtime에서 일반적으로 켠 것으로 표현함 | 전용 계획은 기존 C11.4 gate와 기본 Root HTML admission의 경계를 유지한다. |
| 14 | white-space-only sequence에 Grid item이 생긴다고 함 | 해당 control은 item을 만들지 않는다고 보존한다. |
| 15 | 계획서가 현재 Root HTML 동작을 전부 거부한다고 계속 설명함 | 고정 `AB` 지원과 그 밖 입력의 거부로 현재 상태를 좁혀 쓴다. |
| 16 | C11 상위 계획이 새 slice를 다음 작업으로 남겨둠 | stale “다음 계획” 요약을 제거하고 병합 결과를 쓴다. |
| 17 | 상위 계획의 Root HTML 현황이 최신 PR에서 멈춤 | #1000/#1002/#1006/#1011 제한 slice 병합을 반영한다. |
| 18 | Android AVD를 Android 실기기로 취급함 | API 37 ARM64 SwiftShader AVD라고 명시한다. |
| 19 | iOS Simulator를 실기기로 취급함 | iPhone 17 Pro/iOS 26.2 Simulator로 표기한다. |
| 20 | simulator 화면을 hardware GPU 증거라고 함 | hardware GPU 성능 또는 Android 실기기 증거가 아니라는 한계를 유지한다. |
| 21 | draw/present 관측을 surface readback으로 표현함 | 문구는 layout-ready/draw/present만 주장한다. |
| 22 | iOS screenshot을 compositor 이후 pixel equality로 해석함 | compositor 이후 pixel equality를 주장하지 않는다. |
| 23 | 화면의 text glyph 수를 pixel correctness로 확대함 | `runtime_text_count=1`·`glyph_batch_draw_count=1`은 runtime 지표로만 쓴다. |
| 24 | 기존 focused test 수를 잘못 바꿈 | focused 7/7, Root HTML 54/54, render 71 pass/1 ignored, layout 32/32 수치를 기존 실행 근거와 일치시킨다. |
| 25 | full style-to-layout 실패를 감춤 | 기존 C07 serialization 기대값 비교 1건으로 278/279였음을 남긴다. |
| 26 | feature Clippy 실패를 성공으로 바꿈 | 변경하지 않은 helper `too_many_arguments` 실패를 남긴다. |
| 27 | scoped Clippy 통과를 전체 Clippy로 확대함 | default scoped 통과와 feature test-helper 실패를 구분한다. |
| 28 | screenshot 또는 실행 근거 링크를 빼거나 바꿈 | Android/iOS 화면과 실행 README의 기존 경로를 유지한다. |
| 29 | RSPress `/spinon/` 상대 경로를 깨뜨림 | Rspress 2.0.22 선택 빌드가 필요한 문서 16 route를 생성했고, roadmap 카드의 6개 local link를 selected output·기존 미리보기·source에서 확인했다. 이 선택 빌드는 전체 dead-link 검사가 꺼져 있으므로 전역 링크 통과라고 주장하지 않는다. |
| 30 | Tailnet preview를 병합 전·검증 전 배포 완료로 기록함 | 문서 diff는 Tailnet 완료를 주장하지 않으며, 병합 후 HTTP/hash 및 Agent Browser 확인을 별도 증거로 남긴다. |

검토는 문서 diff의 범위·provenance·주장·링크 계약을 다룬다. 일반 CSS Grid 지원 또는 C11 완료를 증명하지 않는다.
