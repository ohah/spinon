# 계획 PR diff 실패 관점 검토 20개

계획 PR의 실제 diff를 병합 전 검토했다. 계획 문서의 사전 실패 관점 검토와 별개로, 이번 문서·fixture·증거 diff가 명세와 기존 상태 기록을 잘못 바꾸지 않는지 확인했다.

| # | diff 실패 관점 | 결과 |
|---:|---|---|
| 1 | 상위 C11.4 계획이 PR #530 구현 완료 사실을 여전히 계획 예정으로 표현 | 수정했다. merge SHA와 기존 검증 범위를 명시했다. |
| 2 | PR #530 검증의 scene 수나 플랫폼 범위가 과장 | 상위 계획에 8 scene, API 37 ARM64 AVD, iPhone 17 Pro/iOS 26.2 Simulator만 기록했다. |
| 3 | `77/213` 또는 C11.4 완료 체크가 바뀜 | 상태 대장을 수정하지 않고 두 계획에 미완료로 표기했다. |
| 4 | CSS 표준 링크가 line placement 규칙이 아닌 다른 섹션을 가리킴 | 2025 CRD §8.3 `#line-placement`와 §7.2.3 auto-repeat 경계를 분리해 연결했다. |
| 5 | 표준 implicit-line 규칙을 authored name만 있는 경우로 제한 | absent/insufficient 이름과 search-direction을 분리해 썼다. |
| 6 | Taffy 구현을 규범 기준으로 격상 | vendored source를 구현 후보로만 부르고 W3C/Chrome과 분리했다. |
| 7 | 사전 관찰이 구현 통과 또는 제품 지원으로 오해될 수 있음 | README와 계획에서 Chrome-only observation이며 구현 전이라고 표시했다. |
| 8 | fixture auto-repeat count가 viewport 우연성에 의존 | CSS 고정 폭 125px, 40px track, gap 0, 3회 반복으로 명시했다. |
| 9 | positive absent-name row가 fixture에서 실제 row property를 읽지 않음 | `gridRowStart`/`gridRowEnd`로 읽도록 수정하고 브라우저를 새로 열어 기록했다. |
| 10 | positive absent-name row frame의 width/height를 CSS와 다르게 문서화 | 화면 결과에 맞춰 `(0,220) 20×10px`로 정정했다. |
| 11 | negative row fallback이 columns 결과와 동일하다고 가정 | 별도 row track list와 negative named placement를 기록했다. |
| 12 | Chrome 화면이 마지막 2 row scene을 포함하지 않음 | 전체 페이지 screenshot으로 교체하고 SHA-256을 기록했다. |
| 13 | Chrome version이 UA의 reduced version만으로 확정 | `userAgentData.fullVersionList`에서 156.0.8078.48을 기록했다. |
| 14 | raw oracle JSON이 JSON string으로 저장되어 기계 판독 불가 | 파일을 object JSON으로 다시 저장하고 `jq`로 10 scene을 읽었다. |
| 15 | computed CSSOM placement를 실제 resolved numeric line이라고 표현 | README에서 CSSOM serialization과 internal resolved integer line을 구분했다. |
| 16 | line track serialization만 비교해 placement frame 오류를 놓침 | 모든 scene에서 used tracks와 root-relative item frame을 함께 acceptance gate로 둔다. |
| 17 | same authored names가 없는 positive/negative behavior가 통합됨 | `ghost` absent와 `cell` insufficient를 별도 cases로 유지했다. |
| 18 | named span이 occurrence lookup과 같은 방향으로 처리됨 | forward/backward span을 별도 anchor와 count로 고정했다. |
| 19 | 계획이 auto-fit, areas, subgrid와 unsupported combinations까지 암묵 지원 | 해당 경계를 계획의 제외 범위에 넣었다. |
| 20 | local evidence 경로나 SHA가 잘못 연결되거나 기존 미추적 증거를 포함 | 이번 fixture directory의 정확한 5개 산출물만 링크·stage 대상으로 잡았다. |

오류가 발견된 항목 9–14는 diff 작성 중 고친 뒤 브라우저에서 다시 수집했다. 최종 입력, oracle, metadata, full-page screenshot은 [사전 비교 README](./README.md)에 연결했다.
