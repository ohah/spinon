# C11.4 two-track span · 상태/로드맵 PR diff 검토 30개

| # | 변경분의 독립 실패 경로 또는 경계 | 판정 |
|---:|---|---|
| 1 | 계획 문서가 실제 변경 없는 경로를 가리킴 | 통과 — 연결한 상세 계획 파일이 저장소에 존재한다. |
| 2 | 계획 문서의 병합 SHA가 #756 merge commit과 다름 | 통과 — #756 merge commit SHA를 정확히 사용한다. |
| 3 | 계획 문서의 병합 시각이 PR metadata와 불일치 | 통과 — #756의 `mergedAt`과 일치한다. |
| 4 | 계획 section 제목이 아직 구현 전 후보라고 표현 | 통과 — 병합된 구현 결과로 이름을 바꿨다. |
| 5 | 테스트 대상이 Rows까지 확장된 것처럼 표시 | 통과 — 이번 five-case slice는 Columns 전용이다. |
| 6 | `visible`/`clip` 출력 track과 item frame이 서로 불일치 | 통과 — `[50,50]px`와 `100×20px`다. |
| 7 | `hidden`/`auto`/`scroll` 출력이 계획 또는 코드와 불일치 | 통과 — `[40,40]px`와 `80×20px`다. |
| 8 | maximum contribution을 minimum 결과와 혼동 | 통과 — 100px MaxContent contribution 보존을 따로 적었다. |
| 9 | 비교 허용 오차가 구현 계약을 약화 | 통과 — `assert_eq!` exact 비교를 명시했다. |
| 10 | focused 결과 1/1이 전 crate 결과처럼 표기 | 통과 — focused 1/1과 `spinon-layout` 298/298을 분리했다. |
| 11 | package suite 수가 PR 이전 값으로 남음 | 통과 — 병합한 소스의 298/298을 적었다. |
| 12 | fmt check가 formatting-only 최초 실패와 혼동 | 통과 — 최종 fmt check 통과만 기록한다. |
| 13 | Clippy 실행 범위가 workspace 전체로 과장 | 통과 — crate all-target Clippy라고 특정했다. |
| 14 | simulator 미실행 사실을 숨기거나 통과로 서술 | 통과 — pure Rust slice라 앱/Simulator는 실행하지 않았다고 적었다. |
| 15 | 미보고된 GitHub check를 CI 성공으로 바꿈 | 통과 — check rollup이 비어 있음을 적었다. |
| 16 | browser baseline을 앱 동작 증거로 전환 | 통과 — Chrome은 별도 baseline이라고 여러 문서에서 구분한다. |
| 17 | general CSS overflow admission의 완료를 암시 | 통과 — 문서는 test-only layout slice 한계 안에 있다. |
| 18 | 상세 계획의 원래 범위 밖 계약을 덮어씀 | 통과 — 기존 범위 밖 section은 보존됐다. |
| 19 | 부모 계획의 PR #752 historical result를 제거 | 통과 — 기존 단락을 복원해 새 결과 뒤에 보존했다. |
| 20 | 부모 계획의 #749/#750 기록이 새 slice와 합쳐짐 | 통과 — 기존 section이 별도로 남아 있다. |
| 21 | 최근 PR 목록에서 #755 계획 PR이 빠짐 | 통과 — 목록에 #755 링크가 추가됐다. |
| 22 | 최근 PR 목록에서 #756 구현 PR이 빠짐 | 통과 — 목록에 #756 링크가 추가됐다. |
| 23 | 순서가 아닌 병렬 구현으로 잘못 해석될 수 있음 | 통과 — 계획 #755 후 구현 #756으로 순서를 적었다. |
| 24 | `spec/STATUS.md`의 C11.4 checkbox가 승격 | 통과 — 상위 항목은 미완료 상태로 남는다. |
| 25 | 공식 완료 집계가 임의로 증가 | 통과 — 모든 수정 문서는 `72/213`을 유지한다. |
| 26 | 상태 대장 링크가 plan/evidence 루트에서 잘못 resolve | 통과 — `spec/` 기준 상대 경로를 확인했다. |
| 27 | public roadmap에 예전 “다음 계획” 문구가 남음 | 통과 — target paragraph를 PR #755/#756 결과로 교체했다. |
| 28 | public roadmap 내부 링크가 `/spinon/` prefix를 벗어남 | 통과 — plan/evidence href는 사이트 base를 유지한다. |
| 29 | 문서 외 소스/fixture/앱 파일이 변경 | 통과 — 수정 범위는 상태/계획/로드맵과 리뷰 문서다. |
| 30 | 이 검토 파일이 post-merge 상태 검토와 같은 산출물로 처리 | 통과 — PR diff 구조 검토와 post-merge 사실 동기화를 서로 다른 파일로 분리했다. |

이 검토는 PR 변경분의 구조, 링크, 이전 이력 보존과 주장 범위를 확인한다. 구현 동작을 새로 검증하거나 C11.4 완료를 선언하지 않는다.
