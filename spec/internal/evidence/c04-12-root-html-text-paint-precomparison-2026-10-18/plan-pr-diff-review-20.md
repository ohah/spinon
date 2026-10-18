# C04.12 parsed root HTML direct text paint · 계획 PR 변경 검토 20개

PR #316의 실제 변경 파일, GitHub 본문·라벨, 업로드된 화면을 계획 검토와 별개인 diff 관점으로 확인했다. PR은 계획·fixture·Chrome 사전 비교만 포함하고 Rust/native 제품 구현은 포함하지 않는다.

1. **변경 파일 범위** — diff는 계획, STATUS 하위 줄, Chrome fixture/관찰/화면/hash, 계획 검토 자료뿐이다. runtime 구현이나 생성 산출물은 없다. **통과.**
2. **STATUS 체크 상태** — 새 하위 slice는 `[ ]`이고 기존 C04.12/C15 상위 항목도 미완료다. 계획만으로 완료 수가 올라가지 않는다. **통과.**
3. **상태 설명과 링크** — STATUS의 “계획만 추가” 설명과 plan/evidence 상대경로가 실제 파일에 연결된다. **통과.**
4. **계획 단계 표시** — plan 상단에 구현 전 상태, Chrome 기준, 다음 internal ID, 별도 계획 검토가 드러난다. 0080은 존재하는 계약 문서인 것처럼 링크하지 않는다. **통과.**
5. **Chrome/runtime fixture 비교** — 두 fixture의 유일한 내용 차이는 Chrome 전용 `@font-face` block이다. runtime asset 로더가 브라우저 loader와 같은 기능이라고 오해할 diff가 없다. **통과.**
6. **font 자원 상대경로** — browser oracle의 Ahem URL은 fixture 위치 기준 `tests/fixtures/resources/c15/Ahem.ttf`를 가리키며 해당 고정 font hash가 manifest에 있다. **통과.**
7. **fixture source geometry** — `html/body/main`, visible 15px div 둘, hidden div, direct `XX`/`HIDDEN` content가 계획과 일치한다. 성공 텍스트 두 곳의 markup 공백은 plan의 collapsible whitespace 계약으로 설명된다. **통과.**
8. **CSS 고정값** — viewport, 색, 폭, font size/line-height, overflow 두 경우가 CSS에 일치한다. `meta color-scheme`은 fixture input으로 남지만 그 동작을 oracle로 주장하지 않는다. **통과.**
9. **JSON/README 일치** — 요소 좌표, text Range, computed style, screenshot sample, browser/viewport 값이 관찰 JSON과 설명에서 서로 맞는다. **통과.**
10. **geometry에 대한 주장 폭** — markup whitespace에 대한 README 설명은 이 fixture의 body/main height와 block y 좌표에서 얻은 추론이라고 한정하고 임의 문서 동작으로 일반화하지 않는다. **통과.**
11. **화면 sample 경계** — visible overflow의 `(16,1)` 잉크와 hidden overflow의 `(16,11)` 배경은 15px owner의 바깥쪽을 검증한다. background control `(100,100)`도 포함한다. **통과.**
12. **screenshot 파일** — raw file은 320×240 RGB PNG이고 JSON이 기록한 SHA-256과 manifest가 일치한다. **통과.**
13. **manifest 범위** — manifest는 두 fixture, Ahem, observation JSON, PNG에 대해 검증 가능한 SHA-256을 가진다. README·review 문서를 binary oracle hash라고 잘못 취급하지 않는다. **통과.**
14. **browser 고정 수준** — Chrome `155.0.8059.85`, OS, architecture, viewport와 DPR을 기록하고 binary SHA/CDP revision 부재를 같은 근거에 썼다. 재현 binary pin이라 주장하지 않는다. **통과.**
15. **Agent Browser 사용·정리** — PR page를 Agent Browser로 열어 snapshot에서 PR 이미지 링크를 확인했고, `document.images`에서 해당 화면이 complete 및 320×240으로 로드된 것을 확인했다. browser close 뒤 session info와 process 목록에서 해당 프로세스가 inactive임을 확인했다. **통과.**
16. **계획 검토 산출물 링크** — plan의 상대 링크가 실제 review artifact를 가리킨다. 그 검토의 whitespace/revision 수정 사항은 현재 plan에도 반영돼 있다. **통과.**
17. **별도 검토 기록** — 이 artifact는 계획 자체의 실패 검토와 다른 PR diff 범위(파일 경로, content consistency, metadata, upload)를 확인한다. **통과.**
18. **PR body 상태와 약속** — PR 본문은 계획 PR임, internal contract와 simulator evidence가 아직 없음을 표시한다. 수행하지 않은 build/test/simulator 결과를 체크하지 않았다. **통과.**
19. **PR screenshot link** — 본문의 raw GitHub image URL이 HTTP 200으로 반환되고 Agent Browser 페이지에서도 이미지가 정상 완료됐다. **통과.**
20. **GitHub metadata와 repository hygiene** — PR #316은 `main`의 현재 `cd9a5076`을 base로 하며 mergeable이다. 기존 `planning`, `area: runtime`, `area: layout`, `area: renderer` 라벨을 사용했다. PR branch check는 보고되지 않았고 `git diff --check`는 통과했으며 main dirty checkout은 변경하지 않았다. **통과.**

## 남은 검증 경계

PR #316에는 workflow check가 보고되지 않았다. 이 계획 PR의 변경은 Rust/native code를 건드리지 않는다. 구현과 AVD/Simulator 검증은 이 계획 PR의 병합 뒤 별도 변경에서 수행한다.
