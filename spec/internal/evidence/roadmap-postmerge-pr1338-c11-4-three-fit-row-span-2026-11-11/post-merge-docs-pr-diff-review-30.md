# PR #1338 병합 후 상태·roadmap 문서 PR diff 검토 30관점

검토 범위: `spec/STATUS.md`, 내부 계약 0059, three-fit 집중 계획, 공개 `spec/public/roadmap.html`, 이 PR의 근거 문서. 구현/runtime 20관점과 implementation PR diff 30관점을 재사용하지 않는다.

| # | 서로 다른 오류·불변 조건 | 확인 결과 |
|---:|---|---|
| 1 | C11 상위 체크박스가 우연히 완료되는 오류 | C11 unchecked 상태다. |
| 2 | C11.4 체크박스가 제한 slice 때문에 완료되는 오류 | C11.4 unchecked 상태다. |
| 3 | 공식 완료 분모·분자가 잘못 변하는 오류 | `78/214`가 그대로다. |
| 4 | 내부 숫자 계약 버전이 잘못 올라가는 오류 | `0.1.0`이 그대로다. |
| 5 | PR source head와 rebase merge SHA가 뒤바뀌는 오류 | `90daa675…`는 source head, `ebcf720d…`는 merge SHA로 기록한다. |
| 6 | merge 시간대 표기가 잘못되는 오류 | GitHub `mergedAt`의 UTC `2026-11-10T16:05:13Z`를 사용한다. |
| 7 | PR check 0건이 green으로 표현되는 오류 | workflow/status 0건과 required-check 비활성 상태를 명시한다. |
| 8 | 33-file PR diff와 docs diff 범위가 섞이는 오류 | 구현 PR과 별도 docs update scope를 구분한다. |
| 9 | three-fit 입력 multiset이 부정확하게 요약되는 오류 | 세 fit-content와 min/max/auto 각 한 개의 six-member list다. |
| 10 | fit position 조합 수가 틀리는 오류 | 여섯 행 중 세 위치 조합은 20개다. |
| 11 | intrinsic tail 순서 수가 틀리는 오류 | 나머지 세 tail의 순열은 6개다. |
| 12 | percentage cap의 basis가 숨겨지는 오류 | definite 220px basis와 direct px/percentage 제한은 계획·계약에 연결된다. |
| 13 | auto-placement 지원이 잘못 암시되는 오류 | three-fit auto-placement는 계속 거부한다고 쓴다. |
| 14 | arbitrary topology나 일반 CSS admission이 암시되는 오류 | 고정 row-span profile로 제한하고 일반 admission을 제외한다. |
| 15 | first explicit-row control의 minimum이 빠지는 오류 | explicit control은 `min-height:0`으로 유지한다. |
| 16 | mixed item minimum 지원이 암시되는 오류 | uniform mode만 포함하며 mixed mutation 거부를 기록한다. |
| 17 | authored calc gate가 direct px 허용으로 약화되는 오류 | `calc()` cap 거부 및 source AST 보존을 기존 contract/evidence에 연결한다. |
| 18 | 11,520 matrix가 Chrome oracle과 product RuntimeSession 혼동되는 오류 | RuntimeSession scene 실행과 Chrome 157 비교 결과로 범위를 적는다. |
| 19 | 기존 960-scene regression 결과가 누락되는 오류 | 기존 repeated-text row-span regression을 명시한다. |
| 20 | one/two-fit 결과가 새 three-fit admission에 흡수되는 오류 | 기존 profile controls를 분리해 기록한다. |
| 21 | broad Clippy compile failure가 사라지는 오류 | combined multi-package unresolved helper 원인을 implementation evidence에 남긴다. |
| 22 | iOS simulator screenshot을 drawable readback으로 과장하는 오류 | OS screenshot smoke와 Metal readback 부재를 구분한다. |
| 23 | Android AVD를 실기기로 표현하는 오류 | API 37.0 ARM64 AVD와 SwiftShader를 표시한다. |
| 24 | 14 boxes/draw-present가 pixel parity로 오인되는 오류 | readback·compositor parity·hardware GPU 제한을 쓴다. |
| 25 | simulator app cleanup이 shared device 전체 삭제로 보이는 오류 | task app만 제거했고 shared profile·다른 앱을 유지했다고 쓴다. |
| 26 | S03을 종료된 장치로 지칭하는 오류 | S03 이름의 등록 device가 없다고 쓴다. |
| 27 | Android rollback 상태가 빠지는 오류 | 기존 APK와 appearance 복구를 기록한다. |
| 28 | Agent Browser lifecycle이 남는 오류 | PR 이미지 확인 뒤 browser session 종료를 기록한다. |
| 29 | docs PR merge 전 Tailnet preview가 배포된 것으로 오해되는 오류 | Tailnet sync는 후속 publication step으로 분리한다. |
| 30 | GitHub Pages 수동 배포가 자동으로 실행되는 오류 | Pages deploy는 실행하지 않는다고 명시한다. |

## 검증 범위

- Source files retain C11.4/C11 unchecked states and `78/214`.
- Links target the merged implementation, its plan/evidence, and internal contract 0059.
- Agent Browser verified the edited standalone roadmap page on localhost at desktop/mobile viewports. This does not claim the full RSPress documentation build, CI, or Tailnet publication.
