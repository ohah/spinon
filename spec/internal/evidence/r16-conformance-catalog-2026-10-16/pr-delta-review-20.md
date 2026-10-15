# R16 적합성 카탈로그 PR 변경 적대 검토 · 2026-10-16

검토 대상은 이 PR의 실제 diff, 특히 새 `0009` 문서와 `0001`·인덱스·상태 대장 연결이다. 시나리오 실행이나 앱 동작 검증은 포함하지 않는다.

| # | PR 변경 실패 관점 | 실제 diff 확인 | 결과 |
|---:|---|---|---|
| 1 | R16을 제품 구현 완료로 표시 | STATUS 문구가 문서 산출물 범위와 미실행/API 0개를 명시 | 통과 |
| 2 | draft version을 출시 버전으로 승격 | 0009와 표의 계약 버전이 `0.1.0-draft` | 통과 |
| 3 | 공개 명세 인덱스에 0009가 빠짐 | `spec/README.md`에 0009 목적과 링크 추가 | 통과 |
| 4 | conformance 방법이 0001과 충돌 | 0001은 0009를 연결하되 지원 표기/판정 원본 유지 | 통과 |
| 5 | 문서 ID 0001–0008 중 항목 누락 | 0009 표를 실제 문서 파일/metadata와 대조 | 통과 |
| 6 | 내부 ABI를 공개 API로 표기 | 공개 계약과 internal README를 분리 | 통과 |
| 7 | 지원 API index를 잘못 갱신 | `spec/api/index.md`를 바꾸지 않고 API 수 0 유지 | 통과 |
| 8 | scenario ID 충돌/중복 | 일곱 `SC-` ID를 catalog와 evidence에서 대조 | 통과 |
| 9 | scenario status가 실행 PASS를 암시 | all-platform 미실행과 기존 evidence 비합산을 명시 | 통과 |
| 10 | SC-UI child order가 fixture 입력과 다름 | text/span/text/em 순서 및 `textContent` 기대를 재검산 | 통과 |
| 11 | SC-CSS geometry 평균이 필드 오류를 숨김 | Chrome C08 reference의 per-field `0.5 CSS px` 판정을 명시 | 통과 |
| 12 | SC-CSS paint sample이 hidden subtree를 통과시킴 | sample 좌표/opaque color와 hidden scene exclusion 조건을 확인 | 통과 |
| 13 | accepted tap/cancel path가 합쳐짐 | SC-INPUT tap 1회와 canceled pan 0회를 분리 | 통과 |
| 14 | handler 완료를 frame presentation으로 간주 | SC-ASYNC에서 handler/microtask/frame marker 분리 | 통과 |
| 15 | unsupported CSS가 기본값으로 조용히 대체 | SC-DIAG source location과 positive control을 기록 | 통과 |
| 16 | surface recreation 뒤 stale frame 제출 누락 | SC-LIFE에 old generation rejection과 revision 분리 포함 | 통과 |
| 17 | accessibility action을 click과 동일시 | SC-A11Y role/name/action을 따로 관찰하고 미정 경계 보존 | 통과 |
| 18 | 미해결 제품 결정을 R16에서 임의 확정 | unresolved register에 owner·영향·금지 주장 포함 | 통과 |
| 19 | 문서 빌드의 기존 실패를 신규 링크 성공으로 오해 | 기준과 최종 모두 dead-link 433페이지·Shiki 2건이며, 최종 로그에는 신규 R16 dead link가 없음 | 통과 |
| 20 | PR 이미지/preview 상태가 문서 산출물보다 앞서감 | 앱 동작 screenshot 없음, main merge 뒤 Tailnet 확인 예정 | 통과 |

검토 결론: 문서 diff는 R16 산출물과 근거에 한정되고, 일곱 scenario는 모두 미실행이다. 문서 빌드는 기준과 같은 기존 오류로 종료됐고 새 R16 링크 오류는 없다. 이 review는 실행·지원 상태를 변경하지 않는다.
