# PR #495 공개 roadmap 카드 순서 변경 적대 검토 30개

검토 대상은 양축 cyclic minimum 후속 PR #493 카드를 선행 단일 축 PR #491 바로 뒤에 배치하는 문서 변경이다. 카드 본문과 공식 상태 값은 그대로 보존하고, HTML source 순서와 실제 브라우저 표시 순서를 대조했다.

| # | 독립 실패 관점 | 확인 및 판정 |
| --- | --- | --- |
| 1 | #493과 #491의 선후 관계가 뒤집힐 수 있다 | #491 paragraph의 닫는 `</p>` 뒤에 #493 paragraph가 온다. 통과. |
| 2 | #493이 다음으로 오래된 #480 뒤에 놓일 수 있다 | source 순서에서 #493은 #480보다 앞에 있다. 통과. |
| 3 | 앞선 #487 기록이 새 카드에 가려질 수 있다 | 순서는 #487, #491, #493, #480으로 검증했다. 통과. |
| 4 | 같은 #493 card가 중복으로 남을 수 있다 | PR #493 marker는 정확히 한 번이고 paragraph도 한 개다. 통과. |
| 5 | 이동 중 원래 card 본문 일부를 잃을 수 있다 | 이전·이후 #493 paragraph bytes와 링크·수치가 동일하다. 통과. |
| 6 | #491 card 일부가 #493 이동에 포함될 수 있다 | card 경계는 각 paragraph의 자체 `</p>`에서 분리했다. 통과. |
| 7 | #480 card의 시작 태그가 손상될 수 있다 | 이동 뒤 #480의 기존 paragraph가 독립된 한 개로 남는다. 통과. |
| 8 | 인접 카드 사이 HTML paragraph 경계가 누락될 수 있다 | #491, #493, #480 각각 열고 닫는 paragraph 태그가 하나씩 있다. 통과. |
| 9 | source 순서만 바뀌고 화면 순서는 캐시 때문에 다를 수 있다 | Tailnet 갱신 뒤 Agent Browser 화면에서 #491 다음에 #493이 보이는지 확인한다. 통과. |
| 10 | build 결과에서 raw `roadmap.html`이 이전 위치를 유지할 수 있다 | selected build가 최신 `spec/public/roadmap.html`을 raw route에 복사한다. 통과. |
| 11 | card가 다른 CSS 영역의 #493 링크와 혼동될 수 있다 | 변경 marker는 단일 양축 cyclic minimum 카드에만 일치한다. 통과. |
| 12 | #493 merge SHA를 PR head SHA로 바꿔 적을 수 있다 | 이번 diff는 paragraph 위치만 바꾸며 merge SHA `3ffd2b5a...`를 유지한다. 통과. |
| 13 | PR #493 병합 시각이 바뀔 수 있다 | `2026-10-22 02:41:32 UTC` 문구가 이동 전후 그대로다. 통과. |
| 14 | 빈 status rollup을 CI 통과로 과장할 수 있다 | 기존 “rollup은 비어 있었다” 문구를 그대로 유지한다. 통과. |
| 15 | 필수 check 미설정을 check 통과로 오인할 수 있다 | `main required status checks는 설정되어 있지 않았다`를 유지한다. 통과. |
| 16 | 공식 완료 수가 `72/213`에서 바뀔 수 있다 | card에 `72/213`을 유지하고 별도 집계 계산은 하지 않는다. 통과. |
| 17 | C11.4 checkbox가 완료로 바뀔 수 있다 | `spec/STATUS.md`는 이 PR에서 수정하지 않는다. 통과. |
| 18 | 네 장면 track 배열 중 하나가 잘릴 수 있다 | `[104]/[22]`, `[104]/[10]`, `[10]/[42]`, `[24]/[22]`가 모두 한 card에 있다. 통과. |
| 19 | Android screenshot href가 다른 실행 결과를 가리킬 수 있다 | 최종 AVD `20261022T022619123Z-93c2e1cb` 경로를 유지한다. 통과. |
| 20 | iOS screenshot href가 다른 실행 결과를 가리킬 수 있다 | 최종 Simulator `20261022T022743674Z-063697ed` 경로를 유지한다. 통과. |
| 21 | screenshot URL에 `/spinon/` base path가 빠질 수 있다 | 두 내부 이미지 URL은 기존 Tailnet 절대 경로를 그대로 보존한다. 통과. |
| 22 | Safari/Chrome 크기 확인과 scene 수가 혼동될 수 있다 | 이번 변경은 시각 정렬만 다루며 이미지 원본 해상도나 runtime scene 수를 바꾸지 않는다. 통과. |
| 23 | 일반 CSS 지원으로 범위를 확대할 수 있다 | “일반 CSS admission, WPT ... 미검증” 문구는 그대로다. 통과. |
| 24 | artifact 검사 범위를 APK/IPA 전체 제외로 과장할 수 있다 | static archive 문자열 검사 한계 문구를 수정하지 않는다. 통과. |
| 25 | PR #493의 링크가 잘못된 번호나 URL로 바뀔 수 있다 | `https://github.com/ohah/spinon/pull/493` 링크를 보존한다. 통과. |
| 26 | 상태 대장이나 계획 경로 링크가 함께 이동될 수 있다 | card paragraph만 이동하며 plan·evidence·STATUS href는 변하지 않는다. 통과. |
| 27 | roadmap source에 예기치 않은 변경이 포함될 수 있다 | Git diff를 검사해 #493 paragraph의 위치 변경만 확인한다. 통과. |
| 28 | 주변 C11.4 카드의 순서를 우연히 재배열할 수 있다 | #487·#491·#493·#480 이외의 카드 상대 순서를 비교한다. 통과. |
| 29 | Tailnet route에 이전 화면이 남을 수 있다 | 최신 main으로 다시 build/sync하고 raw `roadmap.html`의 hash와 화면을 검증한다. 통과. |
| 30 | 문서 순서 보정이 기능 지원 상태나 Pages 배포로 확장될 수 있다 | 이 PR은 static roadmap 순서만 변경하며 상태 checkbox·API 버전·GitHub Pages는 건드리지 않는다. 통과. |

근거: [Tailnet roadmap Agent Browser 화면](../c11-4-cyclic-minmax-calc-minimum-two-axis-tailnet-sync-2026-10-22/tailnet-roadmap-agent-browser-pass43.png), [기존 #493 병합 시각·SHA 기록](./README.md), [roadmap 원본](../../../public/roadmap.html). PR #495 병합 뒤 최종 순서는 새 Tailnet selected build와 Agent Browser 화면에서 다시 확인한다.
