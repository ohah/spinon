# C04.12 기본 앱 route 상태 문서 PR 적대 검토 30개

검토 대상은 PR #326 병합 뒤 `spec/STATUS.md`, `plan/c04-12-default-app-launch.md`, 실행 근거 README의 상태 문구와 이 검토 기록이다. 각 항목은 서로 다른 실패 경로 또는 문서 경계를 확인했다.

| # | 실패 관점 | 확인 결과 |
|---:|---|---|
| 1 | PR 상태가 아직 열림/대기라고 남을 수 있음 | GitHub API에서 `MERGED` 상태를 확인했고 대기 문구를 제거했다. |
| 2 | 병합 시각이 현지 시각과 UTC 사이에서 어긋날 수 있음 | GitHub의 `2026-10-18T18:22:23Z`를 UTC로 적었다. |
| 3 | PR branch head와 rebase merge commit을 혼동할 수 있음 | 병합 커밋 `b99594d1b188e3addc0bd02802d12a53db1b8d95`를 main의 값과 대조했다. |
| 4 | PR 번호 링크가 다른 작업을 가리킬 수 있음 | #326 제목과 기본 root HTML route 변경을 확인했다. |
| 5 | #324의 fixture 경로가 #326 기본 앱 연결과 합쳐질 수 있음 | #324 구현 설명을 그대로 두고 #326을 별도 후속으로 기록했다. |
| 6 | 상위 C04.12가 완료로 승격될 수 있음 | 상태 대장 상위 checkbox는 미완료로 유지된다. |
| 7 | C15/C16/C19도 함께 완료로 오인될 수 있음 | 계획과 실행 근거에서 세 상위 항목이 미완료임을 명시했다. |
| 8 | 단일 route 연결을 전체 root HTML 지원으로 확대할 수 있음 | 제한 후속 slice라는 범위를 문서에 적었다. |
| 9 | 제한 구현의 성공을 공개 CSS 지원으로 해석할 수 있음 | 공개 지원 완료를 주장하는 문구를 추가하지 않았다. |
| 10 | DOM-only fixture route와 기본 GPU route가 혼동될 수 있음 | 계획은 실제 기본 앱 route 후속으로 명명했고 #324와 구별했다. |
| 11 | 앱 기본 route와 실험 인수 route가 혼동될 수 있음 | 기존 실행 설명을 유지하고 기본 route 범위를 좁게 표시했다. |
| 12 | root HTML과 root script가 서로 다른 session이라는 인상을 줄 수 있음 | 구현 계약과 실행 근거 내용은 수정하지 않고 병합 상태만 갱신했다. |
| 13 | false extra 회귀 수정이 증거 없이 기록될 수 있음 | 해당 수정을 뒷받침하는 기존 PR #326 근거 파일과 화면이 main에 존재함을 확인했다. |
| 14 | false/null 인수 회귀 수정이 일반 시스템 extra 결과와 섞일 수 있음 | 기존 실행 근거에서 일반 extra와 false extra 사례가 분리되어 있다. |
| 15 | Android AVD를 실기기로 확대 해석할 수 있음 | 실행 근거의 AVD 한계를 유지했다. |
| 16 | iPhone Simulator 결과를 iOS 실기기 결과로 확대할 수 있음 | Simulator 기기명을 유지하고 실기기 검증을 주장하지 않았다. |
| 17 | iPad Simulator 결과를 일반 태블릿 지원으로 확대할 수 있음 | 해당 기기 결과는 특정 Simulator 실행으로 남아 있다. |
| 18 | Android screenshot을 PixelCopy 근거로 오인할 수 있음 | PixelCopy를 검사하지 않았다는 기존 제한을 유지했다. |
| 19 | iOS drawable 표본을 compositor 이후 픽셀로 오인할 수 있음 | compositor 이후 readback 미검증 문구를 유지했다. |
| 20 | Simulator 결과를 hardware GPU 결과로 오인할 수 있음 | hardware GPU 미검증 문구를 유지했다. |
| 21 | fixture flags 확인을 모바일 최종 artifact 완전성으로 확대할 수 있음 | final mobile linker artifact 한계를 유지했다. |
| 22 | 정적 라이브러리 경계 검사를 앱 번들 검사로 확대할 수 있음 | 기존 macOS arm64 archive 한계를 유지했다. |
| 23 | PR CI 부재를 CI 성공으로 오인할 수 있음 | status 문구에 새 CI 통과 주장을 추가하지 않았다. |
| 24 | 빈 check rollup을 필수 검사 통과로 해석할 수 있음 | PR #326의 빈 rollup은 현재 문서의 성공 주장에 사용하지 않았다. |
| 25 | 20개 구현 검토와 30개 PR 검토를 같은 검토로 셀 수 있음 | 두 검토 기록 링크와 완료 상태를 각각 유지했다. |
| 26 | 계획·실행 근거·검토 기록의 상대 링크가 깨질 수 있음 | 계획·README 링크의 기존 경로와 README에서 새 30개 검토 기록으로 가는 상대 경로 target을 확인했다. |
| 27 | 실행 근거 문서의 PR 상태가 상태 대장과 모순될 수 있음 | 세 문서 모두 같은 merged timestamp와 merge commit을 사용한다. |
| 28 | 병합 날짜가 local timezone 날짜로 바뀔 수 있음 | 세 문서 모두 UTC 표기를 사용한다. |
| 29 | merge SHA를 #326의 원래 head SHA로 오인할 수 있음 | 기록된 값은 `main` merge commit으로 대조했다. |
| 30 | 상태 정정이 문서 사이트 배포나 roadmap 완료로 해석될 수 있음 | 변경은 상태 계획·근거·검토 문서에 한정하고 자동 배포·상위 완료를 주장하지 않는다. |
