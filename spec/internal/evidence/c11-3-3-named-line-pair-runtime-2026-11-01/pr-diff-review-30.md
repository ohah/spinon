# PR #933 변경 diff 적대 검토 30개

검토 대상: PR #933의 실제 base→head diff, `86b63a9538af0ccc316c8fbe04d2e8aeff408a0a` 구현 commit, runtime fixtures, Android/iOS adapters, test/evidence files와 GitHub PR 본문. 구현·runtime 20개 검토와 구분해 변경 경계 및 공개 제출물을 다시 확인했다.

1. **선행 계획:** C11.3.3 계획 PR #932가 Rebase and merge되어 구현 변경의 전제 commit이 main에 있다. PASS.
2. **공식 완료 수:** `spec/STATUS.md`와 C11 상위 체크를 변경하지 않았고 PR 본문·근거 문서에 `72/213` 유지와 C11 미완료를 명시했다. PASS.
3. **독립 oracle:** Chrome 156 reference는 Spinon implementation을 실행하기 전에 기록됐으며 observation/screenshot/reference hash가 모두 고정됐다. PASS.
4. **CSS 입력 경계:** CSS adapter는 양 raw endpoint가 named line인 placement만 pair admission 후보로 삼는다. numeric, `auto`, span 문법은 기존 분기로 남는다. PASS.
5. **resolver provenance:** start/end 양쪽의 implicit-line 사용 여부를 각각 보존하고 두 쪽 모두 implicit인 경우만 새 flag를 만든다. PASS.
6. **trailing polarity:** 양 endpoint가 positive이며 explicit grid의 마지막 line 이후여야 한다. leading/negative behavior를 새 auto-sizing mode에 섞지 않았다. PASS.
7. **occurrence semantics:** bare names와 반복 occurrence는 기존 resolver에서 line으로 풀고, 2/3 occurrence Chrome case를 Rust fixture와 대조한다. PASS.
8. **equal line conflict:** `alpha / alpha`를 별도 reference와 layout case로 두고 end 제거 뒤 사용 frame/track sequence를 확인한다. PASS.
9. **reverse line conflict:** 역순 pair를 별도 case로 두어 spec line swap 뒤에도 item frame이 동일한지 확인한다. PASS.
10. **분배:** 30px item contribution이 두 `auto` implicit tracks에 15px씩 분배되는 Chrome CSSOM과 Taffy trace를 확인했다. PASS.
11. **축 방향:** Columns와 Rows runtime fixture가 각각 independent geometry 및 root 바깥 item sample을 갖는다. PASS.
12. **한 implicit 축:** adapter admission은 정확히 한 축만 implicit pair일 때 flag를 만들며 layout guard도 Rows/Columns pair 결과가 같으면 거부한다. PASS.
13. **반대 축:** opposite axis는 explicit line 범위의 definite placement로 제한한다. Runtime scenes 및 direct-layout shape guard 양쪽에서 확인했다. PASS.
14. **item 크기:** projection은 width·height가 모두 fixed일 때만 pair mode를 고르고 auto-sized item은 기존 오류 경로에 남긴다. PASS.
15. **한 child:** layout 경계가 direct layout-participating child를 다시 세어 정확히 하나인지 검사한다. 추가 visible item은 거부된다. PASS.
16. **숨김 semantics:** `display:none` subtree는 layout 참여 수에서 제외되고 `visibility:hidden`은 display를 바꾸지 않아 참여 item으로 남는다. PASS(code path).
17. **area 충돌:** template areas가 있으면 adapter pair flag가 false이고, direct layout input에서 mode를 강제해도 layout guard가 template areas를 거부한다. PASS.
18. **기존 mode 안정성:** pair flag가 없는 placement는 기존 `TemplateGridPositiveNamedLineAutoImplicitTracks`, signed, generic implicit mode에 남도록 selection 순서를 확인했다. PASS.
19. **layout 재검증:** 새 mode는 `grid_items` mapping과 normalized positive lines를 다시 검사해 projection metadata 단독으로 우회할 수 없다. PASS.
20. **placement integration:** `grid_placement` 및 dense grid normalizer에서만 새 enum mode를 받아 positive extent를 확장한다. workspace compile/Clippy가 모든 enum match를 확인했다. PASS.
21. **auto-track grammar:** `auto`와 fixed-px forms만 허용하며 지원 밖 `grid-auto-*` list는 validation error로 남긴다. PASS.
22. **resource ceiling:** pair case는 기존 10,000-track limit을 공유한다. 정확한 ceiling layout 성공과 over-limit error를 별도 Rust test에서 확인했다. PASS.
23. **실패 전달:** over-limit/shape failure는 layout `Err`로 전달되고 partial `LayoutOutput`을 반환하지 않는다. PASS.
24. **negative matrix:** endpoint 혼합, auto size, 양 축 implicit, 추가 child 입력이 success scene으로 흘러가지 않음을 C11.3.3 tests에서 확인했다. PASS.
25. **기존 결과 회귀:** named-start + auto-end profile과 fixed-px control을 계속 테스트해 새 pair mode와 기존 baseline을 분리했다. PASS.
26. **fixture allowlist:** Rust source dispatcher는 두 exact scene ID만 추가했고 injection 형태의 `;throw` ID는 거부하는 test를 둔다. PASS.
27. **oracle fixture binding:** 두 mobile JSON이 reference hash, screenshot hash, CSS frame, DPR, sample ID/RGBA를 고정하며 JS test가 실제 bytes를 다시 hashing한다. PASS.
28. **capture compatibility:** 새 schema가 별도 result schema/evidence directory를 쓰며 기존 C11.3.3 template-area assertion을 generic geometry helper로 일반화했다. 기존 static suite 17개가 통과했다. PASS.
29. **Android boundary:** Activity route와 demo route를 각각 allowlist에 등록했고 pixel sample ID·item ID·좌표·RGB를 제한했다. AVD PixelCopy 두 scene과 wrong coordinate/unknown route negative control이 통과했다. PASS(AVD only).
30. **iOS boundary:** Swift scene/pixel route를 추가해 두 scene의 Metal drawable 표본을 확인했다. PR 본문의 Android/iOS 네 이미지가 Agent Browser에서 `complete=true`, natural width/height가 0보다 큰 상태로 로드됐고, 사용한 Agent Browser 프로세스는 종료됐다. PASS(Simulator boundary).

## 검토 결과와 외부 상태

- C11.3.3 style regression 22개, `spinon-layout` 326개, CSS reference/runtime 17개, fixture-module boundary 25개, FFI route allowlist 1개 및 workspace Clippy가 통과했다.
- 두 스타일/layout package의 전체 run에서 변경되지 않은 C07 aspect-ratio computed-style precision assertion 하나가 `1.7777778 / 1` 대 `1.77778 / 1`로 실패했다. 이를 통과한 것으로 표시하지 않았으며 PR 본문과 runtime evidence에서 공개했다.
- GitHub PR #933은 `enhancement`와 `area: layout` label을 갖는다. `gh pr checks --required`는 “no checks reported”를 반환했다. 저장소 보호 설정은 approval 0개, Rebase and merge 허용, linear history 요구다.
- Rust/문서 source와 Markdown의 staged diff whitespace 검사는 통과했다. 명령·test output 원문 log는 capture된 trailing blank/whitespace를 보존하므로 raw log만 `git diff --check`에서 제외했다.

검토에서 추가 구현 결함을 찾지 못했다. 전체 package 실행의 C07 예외는 PR diff 밖 변경이어서 범위를 넓혀 고치지 않았다.
