# 변경 묶음 검토

| # | 검토 관점 | 확인 결과 |
|---:|---|---|
| 1 | 사용자가 요청한 실기기 Android 작업과 범위가 맞는가 | Samsung Android 16 기기에서 R08 WGPU probe를 실행했고 iOS·에뮬레이터 결과를 끌어오지 않았다. |
| 2 | 공식 R05 상태를 완료로 잘못 표시하는가 | `spec/STATUS.md`의 R05.3 체크는 미완료로 유지했다. |
| 3 | 별도 조사 계획이 저장소 계획 경로에 있는가 | 비동기 fence 조사 계획을 `plan/`에 두고 20개 실패 관점을 기록했다. |
| 4 | 상태 대장에서 계획과 실행 결과를 찾을 수 있는가 | R05.3 줄에서 보고서와 후속 계획으로 연결된다. |
| 5 | 상위 계측 계획이 실기기 결과와 이어지는가 | `r05-input-to-presentation.md`에 10월 9일 baseline 링크를 추가했다. |
| 6 | 작업용 임시 worktree 경로가 근거 문서에 남는가 | public evidence와 README에는 `/private/tmp`·사용자 home 경로를 넣지 않았다. |
| 7 | Android ADB serial이 공개되거나 로그에 포함되는가 | 실기기 serial 문자열을 전 파일에서 검색해 찾지 못했다. |
| 8 | APK 출처가 병합 이후 commit으로 잘못 소급되는가 | 이전 빌드 HEAD와 현재 source manifest 일치 commit을 각각 기록했다. |
| 9 | build를 이번 turn에 실행했다고 오해할 수 있는가 | 설치 APK를 재사용했고 새 빌드를 하지 않았다고 적었다. |
| 10 | API·기기·ABI 정보가 파일마다 충돌하는가 | environment, README, Android property 결과의 API 36.1/ARM64/4KB가 일치한다. |
| 11 | 현재 refresh와 지원 refresh를 혼동하는가 | 실행 mode 60Hz와 지원 mode 60/120Hz를 구분했다. |
| 12 | 30개 중 성공 표본만 골랐는가 | 입력 sequence 1–30 전체 로그를 유지하고 pending을 30개 모두 기록했다. |
| 13 | submit accepted를 GPU draw 완료라고 과장하는가 | submit accepted 및 fence 상태만 보고하고 GPU 완료·latency는 주장하지 않았다. |
| 14 | fence descriptor 유효를 signal 성공으로 오인하는가 | `valid=30/30`, `pending=30/30`, `usable=0/30`으로 별도 표기했다. |
| 15 | 현재 callback pending을 장기 미신호로 해석하는가 | callback 시점 관찰에 한정하고 이후 signal 여부를 미확정으로 남겼다. |
| 16 | API 36의 JankData 미지원이 누락되는가 | 원본 `api_below_37` capability와 `target_vsync_id=-1`을 설명했다. |
| 17 | input source가 실제 손가락이라고 서술되는가 | ADB 주입 synthetic MotionEvent이며 실제 touch는 미검증이라고 반복 명시했다. |
| 18 | 화면 변화 근거가 측정 run의 횟수를 바꾸는가 | 30회 block과 별도 한 번 탭 visual smoke를 다른 경로로 저장했다. |
| 19 | 기기 설정·foreground를 바꾼 뒤 복구하지 않았는가 | Chrome focus, screen-on, brightness, timeout, stay-on 값을 전후 확인했다. |
| 20 | binary/log/screenshot이 수정 뒤 손상됐는가 | run과 visual smoke의 SHA-256 manifest를 생성하고 전 항목 검증했다. |

상태나 문서에서 완료를 과장하지 않도록 경계 문구를 보강했다. 발견한 출처 혼동은 `apk_build_source_head`와 `source_manifest_matches_commit`으로 분리해 기록했다.
