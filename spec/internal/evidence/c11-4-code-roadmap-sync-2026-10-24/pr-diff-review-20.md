# C11.4 코드·로드맵 동기화 PR 변경 검토 20개

검토 대상은 `plan/c11-4-grid-track-sizing.md`의 상태 날짜와 기존 구현 근거를 반영한 잔여 범위 문장이다. 공식 완료 상태나 API 지원 범위를 변경하지 않는다.

| # | 적대적 관점 | 확인 결과 |
| --- | --- | --- |
| 1 | 상단 날짜가 최신 기록보다 앞설 수 있는가 | 2026-10-24이며 같은 문서에 있는 #573 기록 날짜와 일치한다. |
| 2 | 활성 lane을 코드 완료 상태와 혼동하는가 | C11.4 lane 활성과 상위 미완료를 분리해 유지한다. |
| 3 | 제한 slice 병합으로 공식 완료 수를 올리는가 | `72/213`을 유지하고 상위 완료 체크는 건드리지 않는다. |
| 4 | 오래된 `74/207`, `77/213` 기록이 현재 집계처럼 보이는가 | 각 날짜 기록의 스냅샷이라고 명시한다. |
| 5 | `spanning-auto-min-flex-tracks`를 전체 flex span 알고리즘으로 확대하는가 | 단일 고정 scene이라고 제한하고 #415 실행 근거를 연결한다. |
| 6 | 두 track의 48px/48px가 근거와 다른가 | #415 oracle 및 Android 보충 실행 기록에 맞는다. |
| 7 | span item의 box와 glyph/overflow paint를 혼동하는가 | 본문은 track/span 결과만 말하며 paint 검증으로 확대하지 않는다. |
| 8 | 기존 baseline oracle을 구현 완료로 잘못 표시하는가 | `fit-content(calc())`, edge, min/max 입력은 고정 oracle로 남긴다. |
| 9 | `one-track` 경계 전체를 완료 처리하는가 | 넓은 one-track·repeat·placement 조합은 잔여라고 둔다. |
| 10 | 10,000-track cap 시험을 전체 layout 성공으로 오해하는가 | checked extent 경계와 layout 실행을 구분한다. |
| 11 | cap의 정확한 경계가 문서와 다른가 | #385 근거의 9,999 허용·10,000 허용·10,001 거부 경계를 옮긴다. |
| 12 | 10,000-track 계산에서 trace 예산 제한을 숨기는가 | 100,000-state 별도 budget의 조기 거부 가능성을 적는다. |
| 13 | reservation 실패를 모든 allocator 실패로 확대하는가 | Spinon 소유 지점으로 한정한다. |
| 14 | 실패 뒤 partial output이 남을 수 있는가 | #385의 output 부재 확인을 명시한다. |
| 15 | worker 복구가 새 worker에서만 확인된 것처럼 보이는가 | 동일 worker 재사용 결과로 적는다. |
| 16 | Taffy 내부 OOM 복구를 보장하는가 | 해당 보장은 근거 밖이라고 명시한다. |
| 17 | mixed intrinsic/flexible/fixed 조합을 모두 닫았다고 읽히는가 | 넓은 조합은 계속 남는다고 유지한다. |
| 18 | hidden, invalid input, unsupported overflow 등 잔여 범위를 누락하는가 | 각 범주를 잔여 목록에 남긴다. |
| 19 | duplicate/nested 또는 non-fixed auto-repeat을 완료 처리하는가 | 두 범주 모두 잔여 목록에 남긴다. |
| 20 | 문서 수정이 C13 판정이나 public API/contract version을 바꾸는가 | 변경 파일은 C11.4 계획과 리뷰 근거뿐이며 상태 대장·공개 API는 바뀌지 않는다. |

검토에서 차단 결함은 발견하지 않았다. `git diff --check`와 RSPress `2.0.22` 문서 빌드가 통과했고, Tailnet의 선택 route 7/7이 HTTP 200 및 SHA-256 일치로 확인됐다. Agent Browser에서 수정 plan의 #415/#385 근거와 제한 문구를 확인했다. Rust source는 변경하지 않아 Rust tests는 실행하지 않았다.
