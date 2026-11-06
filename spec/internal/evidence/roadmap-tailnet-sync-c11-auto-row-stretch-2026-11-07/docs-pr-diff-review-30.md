# Tailnet roadmap·C11.4 post-merge 문서 PR diff 검토 30개

범위: `spec/STATUS.md`, 내부 계약 0059, C11/C11.4 계획, 공개 `spec/public/roadmap.html`, 선택한 Tailnet 제공본 및 이 evidence 파일. 각 항목은 실제 변경에서 별도의 잘못된 진술·경계 누락 경로를 검토했다.

| # | 독립 실패 경로 | 결과 |
|---:|---|---|
| 1 | 구현 PR 번호를 계획 PR #1227과 혼동 | PASS — 구현은 #1228로 표기 |
| 2 | Rebase and merge 시각의 시간대를 잘못 씀 | PASS — GitHub merge time UTC 표기를 유지 |
| 3 | merge SHA 앞/뒤 문자를 빠뜨리거나 다른 SHA 복사 | PASS — 모든 새 문서가 `d7a26e93e09ba30fdfaea88ad3e8907df89f9afc` 사용 |
| 4 | 공식 상태의 C11.4 체크를 완료로 바꿈 | PASS — 체크 상태 불변 |
| 5 | 부모 C11 체크를 완료로 바꿈 | PASS — 체크 상태 불변 |
| 6 | 완료 수를 78/214에서 변경 | PASS — 공식 수 유지 |
| 7 | 제한 Root HTML 경로를 일반 author CSS 지원으로 과장 | PASS — 제한 profile 명시 |
| 8 | 40×40 Grid fixture 치수를 바꿈 | PASS — width/height 모두 40px |
| 9 | explicit/implicit row를 혼동 | PASS — 10px explicit row와 singleton implicit auto row 구분 |
| 10 | contribution과 used track size를 같은 값으로 기록 | PASS — 20px contribution과 30px stretched row 구분 |
| 11 | 기대 used row 순서가 바뀜 | PASS — `[10,30]px` 유지 |
| 12 | text item frame 좌표·크기를 잘못 기록 | PASS — `[0,10,40,30]` |
| 13 | stretch trace 변화 방향을 잘못 기록 | PASS — 20→30px |
| 14 | text minimum을 상수 주입으로 오해 | PASS — 기존 measurement/contribution에 연결 |
| 15 | legacy intrinsic mode도 auto row를 수용한다고 암시 | PASS — legacy rejection 보존 설명 |
| 16 | Android AVD를 Android 실기기 증거로 부름 | PASS — API 37 ARM64 AVD 명시 |
| 17 | iOS Simulator를 실기기/최종 compositor 증거로 부름 | PASS — iOS 26.2 Simulator 및 한계 표기 |
| 18 | OS screenshot을 surface pixel parity로 확대 | PASS — visual smoke 한계 표기 |
| 19 | WPT 실행을 주장 | PASS — WPT 미실행/미주장 |
| 20 | GitHub check가 없는데 CI green 처리 | PASS — workflow/check/status 부재 및 required check 미설정 기록 |
| 21 | focused 검증 결과를 전체 crate green으로 확대 | PASS — focused 범위와 기존의 비-green 전체검사 구분 |
| 22 | fixture marker release 분리 방향을 뒤집음 | PASS — default release 제외, test positive control 포함 |
| 23 | 새 public API/selector/FFI가 생겼다고 오해 | PASS — 내부 구현 경계와 비공개 API 유지 |
| 24 | 계약 숫자 버전을 올림 | PASS — `0.1.0` 유지 |
| 25 | 완료 판정 기준이 있는 STATUS link를 끊음 | PASS — 기존 상대 경로 보존 |
| 26 | C11 계획/track-sizing plan link를 잘못 연결 | PASS — 파일이 있는 경로로 연결 |
| 27 | 공개 카드가 최신 C11.4 PR #1224보다 앞선 이력에 삽입 | PASS — #1224 바로 뒤에 날짜/PR 순서로 추가 |
| 28 | static roadmap source와 별도 preview 제공본을 혼동 | PASS — `spec/public/roadmap.html` 및 Tailnet copy 경계를 설명 |
| 29 | 선택하지 않은 다른 preview 파일까지 동기화 | PASS — `/spinon/roadmap.html` 한 파일만 백업·교체 |
| 30 | Agent Browser 확인 뒤 browser/server process 정리 범위를 넘김 | PASS — 작업 browser session 종료, Tailscale preview server 및 별도 브라우저 보존 |

리뷰에서 발견된 문서 오정합은 계획·상태 문구와 Tailnet 카드에 수정 반영했다. 공식 완료 상태는 승격하지 않았다.
