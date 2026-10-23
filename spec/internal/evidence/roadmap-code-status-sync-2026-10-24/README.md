# 코드·로드맵 현재 상태 재대조 · 2026-10-24

## 판정

현재 공식 상태 원본은 [`spec/STATUS.md`](../../../STATUS.md)가 맞다. 원격 `main`과 이 검토의 기준 커밋은 `af23eded0853732379b2f46a1d72b74ba6bfe8f4`였고, 시작 시 checkout과 `origin/main`은 같은 커밋이었다. 열린 PR은 없었다.

- 공식 표시 작업은 상위 78개 + 세부 ID 133개 + ID 없는 하위 작업 2개로 총 213개이며, 공식 완료는 72개다. raw Markdown 체크박스 수와 공개 완료 집계는 범위가 다르다.
- C13 `overflow·scroll·클리핑` 상위 체크는 미완료다. 공개 로드맵의 진행 중 칸은 C13이 아니라 C11.4를 가리킨다. C13.1/2/3의 구현은 고정 opt-in slice로 제한된다. C13.2 ink-only `box-shadow` overflow는 fail-closed이고, C13.3 공개 DOM `scroll` event/CSSOM은 미구현이며 OS-native wheel/trackpad은 부분 검증이다. C13.4 scrollbar/gutter, C13.5 scroll snap, C13.6 logical axis도 남아 있다. C13.3 computed-policy simulator PR [#185](https://github.com/ohah/spinon/pull/185)은 이미 병합됐다.
- C11.4 상위 체크도 미완료이며 공개 로드맵의 유일한 active lane이다. PR #530, #534, #537, #539, #545, #549, #553, #561, #568은 모두 병합됐지만, 제한 fixture는 전체 CSS Grid 지원이나 C11.4 완료가 아니다. 작성 중인 zero-percent/floor 비교 자료는 미추적 계획 증거이며 구현 완료나 공식 체크 증가로 계산하지 않는다.

## 맞춘 문서

공식 상태와 [공개 로드맵](../../../public/roadmap.html)의 C13 설명을 2026-10-24 현재 기준으로 다시 명시했다. C13을 미완료 상태로 유지하면서 활성 lane과 남은 구현 경계를 구분한다. `spec/STATUS.md` 체크박스와 공식 `72/213` 집계는 바꾸지 않았다.

C11.4 live plan 문서 14개에 현재 완료 수 `72/213`을 반영했다. 이 문서들에서 현재 공식 집계처럼 쓰이던 `77/213` 표기를 고쳤다. 날짜가 붙은 parent-plan 이력과 과거 plan-review 표는 당시 기록을 보존한다. C11.4 상위 체크는 그대로 미완료다.

## 확인 결과

- Tailnet preview의 `roadmap.html?sync=20261024-pass01`을 Agent Browser로 확인했다. 호스트 주소는 이 공개 근거 파일에 복사하지 않았다. 페이지는 `213개 표시 · 공식 완료 72/213`, `상위 78개 + 세부 ID 133개 + ID 없는 하위 체크 2개`, WIP `C11.4 · ACTIVE`를 보여준다. 공개 칸반 캡처는 `roadmap-kanban-agent-browser-pass01.png`다.
- 같은 페이지는 C13을 미완료·비활성, C11.4를 유일한 active lane으로 표시한다. C13.2 `box-shadow` fail-closed, public DOM `scroll` event/CSSOM 미구현, native wheel/trackpad 부분 검증, C13.4/5/6 미완료 문구가 공식 상태 대장과 일치한다. 첫 화면 캡처는 `roadmap-agent-browser-pass01.png`다.
- 14개 C11.4 live plan 문서는 기존 파일에서 `77/213`을 `72/213`으로만 바꾼 결과와 byte-for-byte 일치한다. 남은 `77/213`은 날짜별 계획 review snapshot이나 과거 실행 기록이며 현재값으로 쓰인 14개 live plan에 남아 있지 않다. parent-plan 이력, 날짜별 PR review, 그 밖의 과거 실행 snapshot은 수정하지 않았다.
- 현재 main의 C11.4 구현·계획 기록에 연결된 PR #530/#534/#537/#539/#545/#549/#553/#561/#568/#569는 `MERGED`다. C13.3 후속 PR #185도 `MERGED`이며 merge SHA는 `71a6e28bbcf6c8181a86f3e88a79b4c4e9cbf130`이다. 이 문서는 병합 사실을 CI 성공 주장으로 확대하지 않는다.
- 변경 문서의 RSPress 선택 빌드는 통과했다. 공개 로드맵 원본과 build 산출물의 SHA-256은 모두 `79e86877052f56ab4040db18e85055ccef3a512d6160f3be9aed1281b435ae5f`다. Tailnet에 선택한 30개 경로는 모두 HTTP 200이며 build 및 local preview의 SHA-256과 일치하고 삭제 경로는 없다. 자세한 경로·hash는 `selected-build-verification-pass01.json`과 `remote-verification-pass01.json`에 있다.
- Agent Browser에서 저장한 첫 화면·칸반 화면은 각각 1280×633이다. 첫 화면 SHA-256은 `3169b3ad3f856f2582bc52b7039b860404097c29f4ae3a4832dc383bfa54468b`, 칸반 화면 SHA-256은 `04efc018f85d4d7744da8793019d7a214c68264d93a49932f53313202093c635`다. 칸반 DOM 요약은 `agent-browser-kanban-dom-pass01.json`에 보존했다. 검증 세션 `spinon-roadmap-next-20261024`를 닫은 뒤 세션 목록은 비었고, 해당 Agent Browser Chrome profile 프로세스도 없음을 확인했다. Tailnet preview server는 유지했다.
- 이 검토는 현재 코드·기록과 상태 문구의 대조다. C13/C11.4 runtime matrix를 재실행하지 않았으며, 앱 테스트도 수행하지 않았다. Tailnet preview server는 유지한다. 검증용 Agent Browser 세션 종료 결과는 연결된 `pr-diff-review-20.md`의 20번 관점에 기록했다.

## 범위와 제한

이 작업은 구현 코드를 수정하거나 C13/C11.4를 완료 처리하지 않는다. C13 각 항목의 기존 simulator/device 표본을 다시 실행한 검증도 아니다. 코드와 연결된 실행 근거를 읽어 상태 문구를 대조하고, 현재 roadmap 페이지가 같은 상태를 표시하는지 확인했다. PR용 Agent Browser screenshot은 `roadmap-agent-browser-pass01.png` 및 `roadmap-kanban-agent-browser-pass01.png`다. Tailnet 제공 경로와 문서 파일의 SHA-256은 `remote-verification-pass01.json` 및 `selected-build-verification-pass01.json`에 보존한다.
