# C11.4 generic mixed spanning PR diff 적대 검토 20개

이 검토는 구현 논리 검토와 별도로 최종 PR 변경분의 범위·계약·증거를 확인한다. 항목별 diff는 PR 브랜치 기준으로 확인하고, 실행 근거는 [README](./README.md)와 각 검토 파일에 연결했다.

| # | diff 공격 관점 | 확인 결과 |
|---:|---|---|
| 1 | 변경이 public API 또는 외부 ABI를 조용히 늘리는가 | 수정은 기존 opt-in fixture path와 내부 runtime adapter에 한정되고 공개 API/헤더 변경은 없다. |
| 2 | 원래 네 scene 외에 새 launcher case가 들어왔는가 | FFI scene table은 기존 네 ID만 노출하며 FFI focused test에서 개수와 unknown ID를 확인한다. |
| 3 | HTML scene identity를 바꾸면서 launcher identity도 깨뜨렸는가 | launcher ID는 보존하고 HTML root/item DOM ID만 새 접두사로 만들었다. |
| 4 | generic adapter를 제품 runtime 전역의 unrestricted parser로 만들었는가 | profile, root, track, item, text/image source를 제한하고 unsupported case는 fail-closed한다. |
| 5 | 기존 fixed-ID adapter 코드를 중복 구현으로 대체했는가 | 기존 fixed route와 tests를 유지하고 dispatch에 별도 generic route만 추가했다. |
| 6 | dispatch 순서가 다른 mixed fixture를 먼저 가로채는가 | 기존 orthogonal/cyclic route 뒤, fixed-ID lookup 앞에서 generic spanning shape를 식별한다. |
| 7 | 새 routing enum variant가 예상 frame/trace test에서 빠지는가 | `GenericSpanningMultiTrack`은 새 네 scene에서만 사용하고 layout assertions로 검증한다. |
| 8 | style 변경이 기존 scene의 box sizing/property 요구를 완화하는가 | 새 variant만 spanning 계약과 같은 validator를 재사용하며 다른 scene 조건은 유지된다. |
| 9 | test-only builder가 제품 module에 runtime dependency를 추가하는가 | Rust fixture builder/test 모듈은 `#[cfg(test)]` 경계에 있다. |
| 10 | text/image node source를 순서 인덱스로만 식별해 교차 장면에서 뒤섞는가 | 내용·tag·placement 검증 후 node별 intrinsic source와 frame을 테스트한다. |
| 11 | source order reverse scene이 CSS placement까지 역전시키는가 | source array만 역전되고 item별 placement 선언은 그대로다. |
| 12 | image pipeline이 임의 네트워크 접근이나 다른 fixture asset을 도입하는가 | 기존 고정 PNG URL/bytes/hash를 재사용하고 runtime decode는 로컬 PNG payload다. |
| 13 | intrinsic revision이나 image metadata를 임의 상수로 숨겼는가 | checked revision advance, decoded dimensions, layout snapshot을 검사한다. |
| 14 | app harness에 플랫폼 전용 예외 코드를 추가해 한쪽만 검증되는가 | Android/iOS app source 수정 없이 기존 fixture launcher를 재사용했다. |
| 15 | 캡처 검증기가 float 정밀도 차이를 런타임 실패로 오판하는가 | exact compare 실패를 보존하고 규정된 `1/64px` tolerance로 고쳐 두 플랫폼 전체를 다시 캡처했다. |
| 16 | screenshot을 실제 glyph/PNG pixel 결과로 과장하는가 | README는 화면을 layout marker로만 설명하고 glyph, texel, compositor 이후 확인을 명시적으로 제외한다. |
| 17 | static archive 결과를 APK/IPA 전체 exclusion으로 확대 해석하는가 | default/product/fixture-test FFI archive만 주장하고 APK/IPA 전체 검사는 미검증으로 분리한다. |
| 18 | Clippy 결과가 실패를 숨기거나 변경과 무관한 기존 warning을 누락하는가 | strict 실패의 기존 두 위치와 수정한 변경 lint, 제한 허용 재검사 결과를 모두 기록했다. |
| 19 | 변경이 계획·증거 파일 없이 구현 완료나 공개 지원을 주장하는가 | 구현 계획, plan review, runtime evidence, 구현 검토, PR diff 검토를 추가하고 C11.4/`72/213` 상태는 유지한다. |
| 20 | unrelated working tree 파일이 이 PR에 섞이거나 근거에서 빠지는가 | stage 대상은 이번 코드·계획·전용 evidence 경로로 한정한다. unrelated untracked evidence와 vendor lock은 보존하고 PR diff에 포함하지 않는다. |

**판정:** 현재 diff 검토 20/20. PR 본문에는 restricted scope, 플랫폼 결과, archive/Clippy 한계, 남은 범위를 그대로 요약한다.
