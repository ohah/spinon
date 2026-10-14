# C14.2 stacked PR 적대 검토 · 2026-10-14

대상은 [PR #156](https://github.com/ohah/spinon/pull/156), head `feat/c14-2-image-ratio-sizing`, base `feat/u05-image-resource`다. 부모 [PR #155](https://github.com/ohah/spinon/pull/155)는 검토 당시 open 상태였고, PR #156도 `CLEAN` merge state지만 두 branch에 보고된 CI check가 없었다. 계획 검토와 구현 검토에서 사용한 sizing 실패 관점을 반복하지 않고, PR diff·stack 관계·계약·증거·검토 경계에 대해 별도 20개를 확인했다.

| # | 공격 관점 | 판정과 근거 |
|---:|---|---|
| 1 | PR base가 `main`으로 잘못 잡혀 U05/C14.1 부모 변경을 재포함할 수 있다 | PASS — PR #156 base는 `feat/u05-image-resource`, head는 `feat/c14-2-image-ratio-sizing`다. |
| 2 | 로컬 branch가 PR base와 다른 부모 commit에서 갈라져 diff에 관계없는 변경이 보일 수 있다 | PASS — head의 부모는 부모 branch의 `7fef2b0`이고 로컬 base/PR base가 일치한다. |
| 3 | parent PR 변경이 C14.2 commit에 포함되거나 다시 수정될 수 있다 | PASS — 부모 PR의 두 commit은 merge-base에만 있고 PR #156은 C14.2 단일 commit `663ddb3`만 가진다. |
| 4 | main까지 합친 전체 변경 수를 C14.2 변경량으로 잘못 보고할 수 있다 | PASS — `origin/feat/u05-image-resource...HEAD` 비교와 GitHub PR changed-files 목록이 같은 34개 파일이다. |
| 5 | build dependency 또는 lockfile을 바꿔 이미지 기능 검증 밖의 업데이트가 섞일 수 있다 | PASS — Cargo manifest와 lockfile 변경이 없다. |
| 6 | task/area label이 빠져 저장소 분류 계약을 어길 수 있다 | PASS — `enhancement`, `area: layout`, `area: testing`, `area: docs`가 설정되어 있다. |
| 7 | PR 본문이 변경, 명세, 근거, 미완료 경계를 빠뜨릴 수 있다 | PASS — 저장소 양식의 각 항목을 채웠고 C14.2를 구현 후보로 명시했다. |
| 8 | 상태 대장이 C14.2나 상위 C14를 구현 완료로 과장할 수 있다 | PASS — 두 항목의 체크를 해제한 상태로 유지하고 stacked PR/상위 미완료를 적었다. |
| 9 | 내부 인터페이스 버전을 구현만으로 올릴 수 있다 | PASS — 계약 0009는 `0.1.0`을 유지하고 snapshot 입력 및 오류 제한을 문서화했다. |
| 10 | 공개 CSS/API 전체 지원이나 제품 decoder/runtime가 추가된 것으로 오해할 수 있다 | PASS — API는 내부 경계로 표기했고 async decode, cache, runtime adapter, WPT, GPU/실기기를 남은 범위로 썼다. |
| 11 | 새 `LayoutIntrinsicSize`가 Taffy 타입이나 DOM resource owner를 공개 경계에 노출할 수 있다 | PASS — public snapshot은 engine-neutral optional metadata를 사용하며 Taffy adapter는 `spinon-layout` 안에 있다. |
| 12 | source/revision 검증과 malformed metadata 오류가 새 sizing path에서 건너뛰어질 수 있다 | PASS — stale source, duplicate/unknown/non-leaf node, 0/NaN/overflow ratio negative tests가 workspace 결과에 포함된다. |
| 13 | no-natural 성공 metadata와 누락/실패 resource를 같은 값으로 합칠 수 있다 | PASS — entry 유무와 nullable metadata를 분리하고 각각의 지원/거부를 계약과 테스트에서 보존한다. |
| 14 | Taffy leaf 최종 ratio 보정이 custom solver used size를 다시 덮을 수 있다 | PASS — replaced leaf 최종 크기는 border-box known size로 전달하고 ratio/min/max를 Taffy 재보정에서 비운다. |
| 15 | Flex 부모가 제공한 cross measurement가 CSS definite size로 오인될 수 있다 | PASS — 최초 실패를 재현해 solver 측정 callback을 고쳤고 Chrome 기준 8개 Flex 행을 HostDocument→Stylo→Taffy까지 비교했다. |
| 16 | hidden/fixed image에서 측정 요구가 늘거나 cache 값이 definite CSS size를 덮을 수 있다 | PASS — hidden, fixed/fixed negative controls가 layout/style-to-layout tests에 있다. |
| 17 | positioned child의 flow-only pass가 visual pass와 다른 metadata를 읽을 수 있다 | PASS — 두 tree에 snapshot을 함께 전달하고 flow/visual 크기를 같은 테스트에서 비교한다. |
| 18 | Chrome observation이 다른 DPR/browser/input을 썼거나 fixture와 관측 파일이 바뀔 수 있다 | PASS — Headless Chrome 155.0.8059.40, macOS, DPR 1, 24×16 PNG, fixture/inventory/observation SHA-256을 사전 비교 README에 기록했다. |
| 19 | AVD 결과를 실기기·앱 runtime 증거로 확대 해석하거나 다른 ADB serial을 건드릴 수 있다 | PASS — `-s emulator-5554`만 사용했고 API 37 ARM64 16 KiB AVD Rust test만 주장한다. 임시 executable 제거를 확인했으며 앱·GPU·실기기 주장은 하지 않는다. |
| 20 | PR 상태·check 유무·부모 관계를 합쳐 병합 가능으로 잘못 결론 낼 수 있다 | PASS — PR #155와 #156은 open, merge state는 `CLEAN`, branch checks는 보고되지 않았다. PR은 병합하지 않았고 상태를 검증 후보로 남겼다. |

별도 PR 적대 관점 20개를 완료했다. 해결되지 않은 코드 P1/P2는 발견하지 못했다. PR/부모 branch에 필수 CI check가 보고되지 않는 점은 외부 merge gate이며 green CI로 간주하지 않는다. 이 기록은 [구현 코드 검토 20개](c14-2-image-ratio-implementation-review-2026-10-14.md)와 별도다. 외부 reviewer 승인이나 병합을 의미하지 않는다.
