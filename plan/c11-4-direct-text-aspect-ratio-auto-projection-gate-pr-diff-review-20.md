# C11.4 direct-text `auto <ratio>` projection gate 계획 PR 변경분 검토 20개

**대상:** plan #845 후속 amendment — auto-ratio 계획의 exact `NodeId` projection gate와 상위 계획 링크

**기준:** `277fe955c054025eea6854becf99e48575d090dc` 기반 계획 보완 diff

**판정:** 20/20 PASS. 구현 코드/런타임은 이 계획 PR에서 변경하지 않는다.

| # | 변경분 실패 관점 | 결과 |
|---:|---|---|
| 1 | `C11.4`가 공유 `RuntimeBlockFlexGridTrackSizingV1` profile만으로 auto ratio를 투영 | PASS — plan은 profile과 projection authority를 구분하고 exact `NodeId`를 요구한다. |
| 2 | FFI scene ID가 직접 projection authority로 사용 | PASS — route marker는 validated HostDocument owner에서 나오고 scene label은 값 출처가 아니다. |
| 3 | 모든 `C11_4TextInput` construction이 새 marker를 기본 허용 | PASS — 계획은 `None` default와 auto-ratio route만 `Some(NodeId)`를 가진다고 명시한다. |
| 4 | Root/short item/wrong element가 content-box remapping 대상 | PASS — single direct `AAAA` owner NodeId와 collector 조건을 지정한다. |
| 5 | NodeId가 다른 generation/style snapshot에서 적용 | PASS — snapshot revision guard와 exact owner same-snapshot 전달을 함께 요구한다. |
| 6 | Projection option이 broad C11 entry point에서 무심코 활성화 | PASS — 별도 feature-test wrapper에서만 option을 채운다. |
| 7 | Base style projection에 ContentBox override를 기본 허용 | PASS — 기본 option `None` 및 marker 없는 typed ContentBox input rejection을 명시한다. |
| 8 | Wrong/missing/duplicate marker가 부분 layout을 게시 | PASS — fail-closed 및 duplicate ratio owner rejection이 acceptance에 들어 있다. |
| 9 | CSSOM serialization과 typed basis가 서로 모순 | PASS — 동일 snapshot의 `auto <ratio>`, numeric ratio, sizing-box variant를 모두 검사한다. |
| 10 | Numeric ratio가 auto ratio remapping을 받음 | PASS — numeric ratio basis는 `BoxSizing`이고 override route는 `ContentBox`만 대상으로 한다. |
| 11 | No-ratio `auto`가 auto ratio와 혼동 | PASS — typed value None이면 computed serialization 정확히 `auto`만 control로 인정한다. |
| 12 | CSS source `box-sizing` audit 값이 변경 | PASS — computed property 원문은 유지하고 Taffy item projection만 바꾼다. |
| 13 | Auto ratio mapping에 explicit size/min/max가 함께 들어옴 | PASS — 해당 fixed profile의 auto/no-minmax gate를 유지한다. |
| 14 | Taffy가 실제 border/padding edge를 두 번 더함 또는 빠뜨림 | PASS — row·column 독립 계산과 unit/runtime trace 검증을 그대로 요구한다. |
| 15 | 기존 Chrome 숫자 결과를 implementation pass로 잘못 기록 | PASS — amendment에는 계획/Chrome evidence만 있고 Simulator는 implementation gate로 남는다. |
| 16 | Contract 0059가 계획 PR 단계에서 구현됐다고 표시 | PASS — implementation PR에서 계약과 오류 경계를 갱신하도록 계획한다. |
| 17 | Official `spec/STATUS.md` 상위 state가 fixture plan으로 변경 | PASS — C11/C11.4 unchecked와 `72/213` 유지가 amendment/master plan에 있다. |
| 18 | 이전 PR #845 plan review 이력이 새로 작성된 것처럼 대체 | PASS — 기존 20-view report를 보존하고 amendment recheck를 별도 파일로 둔다. |
| 19 | Amendment plan review와 PR diff review가 같은 관점 표를 재사용 | PASS — 새 두 문서는 별도 실패 표이며 서로 다른 대상 설명을 둔다. |
| 20 | 변경분에 기존 untracked evidence나 다른 C11 작업이 섞임 | PASS — 예정 commit 범위는 plan, master plan 및 두 신규 amendment review 문서뿐이다. |

이 검토는 plan amendment 변경 경계를 본다. 실제 ratio extraction, `C11_4TextInput` 전달, style-to-layout option, Taffy geometry, static artifact 및 simulator 증거는 별도 구현 PR에서 검증한다.
