# Tailnet 공식 상태 집계 ID 정합성 사전 비교

**관찰 시각:** 2026-11-03 · **원본 HEAD:** `2bb59a35c89358eee77918f8b7e9f7b9c9fae7c1` · **페이지:** `https://macstudio.tailed42f2.ts.net/spinon/roadmap.html`

이 디렉터리는 로드맵 표시를 수정하기 전 Agent Browser 관찰과 `spec/STATUS.md` 원본 체크박스를 비교한다.

## Agent Browser 기준 화면

- 페이지 문구: `213개 표시 · 공식 완료 72/213`.
- 상태 동기화 세부 문구: `상위 78개 + 세부 ID 133개 + ID 없는 하위 체크 2개 · 완료 72/213개`.
- DOM에는 `C04.12` 작업 행이 7개 있다. 원본 대장에서는 이 ID의 7개 행 중 5개가 `[x]`, 2개가 `[ ]`인데, 페이지는 7개 모두 `[ ]`로 보여 준다.
- `C06.6` 행은 `[ ]`이고 `C06.6a`·`C06.6b` 행은 표시되지 않는다. 원본 대장에서는 `C06.6a`가 `[x]`, `C06.6b`가 `[ ]`다.
- 전체 행 상태와 counter DOM 관찰은 [before-agent-browser.json](./before-agent-browser.json), 화면은 [roadmap-before.png](./roadmap-before.png)에 보존했다. 화면 PNG SHA-256: `cdceaa948cdfd830213456ca1e6e837c12d918e79d20b55f4f3727453ad565a1`.

## 원본 대장 계산

- task identity가 있는 Markdown 체크 행은 213개이며, 완료 행은 78개다. 이 수는 ID와 제목을 함께 읽어 distinct row로 계산했다.
- `C04.12`는 동일 ID로 서로 다른 제목 7개가 있다. 현재 페이지 구현은 상태 Map key를 ID 단독으로 사용하므로 마지막 미완료 행이 앞선 5개 완료 상태를 덮어쓴다.
- 현재 정규식은 `C06.6a`와 `C06.6b`의 알파벳 접미 ID를 인식하지 않는다.
- 기존 페이지 목록의 78개 base row와 special `미구현 하위 항목` 2개도 유지해야 한다. ID 전체를 제목과 함께 파싱하면 세부 ID 135개가 더해져 예상 표시 행은 `78 + 135 + 2 = 215`, 완료 행은 78이다.

## 고정 판정

공식 원본은 항상 `spec/STATUS.md`다. 수정 화면은 모든 ID 행을 ID·정규화 제목 identity로 구분하고, 반복 ID·알파벳 접미 ID를 손실 없이 렌더링해야 한다. 상태 동기화 오류는 미완료 값이나 개인 체크 상태로 위장하지 않아야 한다. C11/C11.1의 완료 체크는 바꾸지 않는다.

## 수정 후보 Agent Browser 결과

수정된 local `spec/public/roadmap.html`을 Agent Browser에서 열었다. 페이지는 현 시점 GitHub `main`의 `spec/STATUS.md`를 다시 요청해 읽었다.

- counter와 status note는 `215개 표시 · 공식 완료 78/215`, `상위 78개 + 세부 ID 135개 + ID 없는 하위 체크 2개`로 일치했다.
- `C04.12` 7개는 source 제목대로 `[ ]`, `[x]` 5개, `[ ]` 1개, `[ ]` 1개 상태를 각각 표시했다. 이전처럼 마지막 행 상태를 전체에 복사하지 않는다.
- `C06.6a`는 checked, `C06.6b`는 unchecked 개별 행으로 표시했다.
- 215개 input의 DOM id가 모두 고유하고 status key가 빠진 행은 0개다.
- desktop viewport `1440×1000`과 mobile viewport `390×844`에서 `documentWidth == viewportWidth`; horizontal overflow는 0px이다.
- Open filter는 137개 행을 보였고 완료 수는 78/215로 유지했다. `C06.6a` 검색은 해당 하위 행 하나를 반환했다.
- 초기 후보의 Agent Browser screenshot은 최종화 과정에서 갱신했다. 반응형 줄바꿈까지 반영한 최신 desktop/mobile 화면과 SHA-256은 아래 [최종 수정 후보 확인](#최종-수정-후보-확인)을 따른다.

이는 수정 후보 page code와 현재 GitHub `main` 상태 파일의 로컬 Agent Browser 비교다. Tailnet 서버는 아직 이전 page code를 제공하므로 `72/213`을 보인다. PR 병합 뒤 기존 Tailnet preview 승인 범위에서 page를 동기화하고 URL·hash·counter·desktop/mobile overflow를 다시 확인한다. Agent Browser session은 종료했다.

## 최종 수정 후보 확인

2026-11-03에 PR #1086 병합 카드를 추가하고 제목 줄바꿈을 단어 경계에서 균형 조정한 뒤 로컬 page를 Agent Browser로 다시 열었다. GitHub `main`의 공식 `STATUS.md`를 가져온 화면은 `215개 표시 · 공식 완료 78/215`였고, 215개 checkbox 중 78개 checked, duplicate DOM id 0개였다. 반복 `C04.12` 7행과 `C06.6a`/`C06.6b` 상태는 앞선 결과와 동일했다.

최종 화면 확인은 desktop `1440×1000`과 mobile `390×844`에서 `documentWidth == viewportWidth`, `bodyWidth == viewportWidth`로 가로 넘침 0px이었다. 모바일 제목의 글자별 DOM 위치는 `스피논을` / `만드는 순서` 두 줄로 나뉘었다. 화면과 SHA-256: [desktop](./roadmap-final-desktop.png) `4f996eb9ab53a0dc09e7e68da3baf0fa0c6e8b4328febb1bfd6f9f669f711f74`, [mobile](./roadmap-final-mobile.png) `fd2afaeacedba9355c46e241b76ebdfe74ea812b3bb7ca70288bb430382dd37d`.

이번 확인이 끝난 뒤 Agent Browser를 닫았고, `ps`에서 실행 중인 Agent Browser 프로세스는 없었다. 이 검증은 local candidate page다. Tailnet은 PR 병합 후 동기화하고 별도로 다시 확인한다.

## 파서 실패 경계 Agent Browser 검사

기존 로드맵 HTML에 고정된 213개 official ID 행과 2개 특수 no-ID child를 넣은 임시 HTML을 만들고, `fetch` 응답만 synthetic `STATUS.md`로 바꿔 같은 브라우저 parser를 실행했다. 유효 control은 215행·78 checked로 동기화됐다. Duplicate `C04.12` ID/title identity, 실제 base row `R01` 누락, suffix ID `C06.6a` 문법 손상, no-ID child marker 문법 손상은 모두 동기화 실패 안내와 판정 불가 상태를 표시했다. 세부 결과는 [negative-agent-browser.json](./negative-agent-browser.json)에 저장했다.

처음에는 `C11`을 base 누락 입력으로 사용했지만, C11은 78개 static base row가 아닌 세부 ID여서 공식 원본에서 제거된 것으로 정상 반영됐다(214행). 이를 구현 결함으로 기록하지 않고 실제 static base row `R01`로 누락 검사를 다시 실행해 fail-closed를 확인했다. Agent Browser를 닫았으며 이후 `ps`에 해당 프로세스가 없었다.

## 적대 검토

- [계획 실패 관점 검토 20개](../../../../plan/roadmap-status-counter-identity-plan-review-20.md)
- [구현 실패 관점 검토 20개](./implementation-review-20.md)
- [PR diff 실패 관점 검토 30개](./pr-diff-review-30.md)
