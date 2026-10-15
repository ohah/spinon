# C13.3 Tailnet roadmap 상태 동기화

## 목적

병합된 [PR #190](https://github.com/ohah/spinon/pull/190)의 iOS active-pan second-touch 근거와 현재 미완료 gate를 상태 대장 및 정적 로드맵 화면에 같이 연결한다. 이 변경은 문서 표시를 맞추며 runtime 동작을 수정하지 않는다.

## 반영 내용

- `spec/STATUS.md`의 이전 iOS multi-touch 미검증 문장을 PR #190 Simulator 근거로 교체했다. Android 실제 multi-touch, wheel/trackpad, public events와 S04.10은 미완료 상태로 남긴다.
- `spec/public/roadmap.html`의 C13.3 진행 note에 PR #190, iOS Simulator의 cancel/suppression/all-up recovery 요약, evidence README 및 화면 링크를 추가한다.
- 정적 화면의 두 이전 C13.3 목록에서 iOS multi-touch를 미완료 항목으로 남기지 않고, Android 실제 multi-touch를 남은 gate로 적는다.
- Tailnet server의 변경 전 화면은 agent-browser에서 확인했다. Tailnet은 저장소와 별도 정적 산출물이므로 post-merge sync의 원본 commit 및 served-file hash manifest로 실제 제공 revision을 기록한다.

## 판정 범위

PR #190에서 확인한 것은 iPhone 17 Pro/iOS 26.2 Simulator의 opt-in fixture다. 실기기 차이, Android 실제 multi-touch, 제품 입력 경로, public DOM event, wheel/trackpad, S04.10 displayed-frame targeting 및 C13.3 전체 완료를 주장하지 않는다. PR #191 자체는 문서/로드맵 변경이며 추가 runtime run을 수행하지 않는다.
