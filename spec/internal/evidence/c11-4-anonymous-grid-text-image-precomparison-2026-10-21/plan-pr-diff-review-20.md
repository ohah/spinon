# 계획 PR #424 변경분 적대 검토 20개

최종 검토 범위는 PR #424의 실제 diff 8개 파일이다. 계획 자체의 20개 실패 관점 검토와 구분해 fixture, 고정 근거, roadmap 변경, PR 패키징과 이 검토 기록까지 다시 확인했다.

1. **PR에 구현 완료가 섞였는가?** — 최종 8개 경로는 계획·검토·fixture·Chrome evidence와 STATUS뿐이며 Android/iOS 실행을 미완료로 표기한다. 통과.
2. **공식 완료 수가 잘못 승격됐는가?** — `spec/STATUS.md`는 C11.4 checkbox와 완료 수를 유지한다. 통과.
3. **현재 실패 원인을 잘못 적었는가?** — runtime entry가 anonymous text와 image input 동시 감지를 명시적으로 거부하는 코드를 확인했고 plan에 기록한다. 통과.
4. **런타임 원본이 다른 feature profile을 쓰는가?** — 결합은 기존 `RuntimeBlockFlexGridTrackSizingV1` opt-in 경로로 제한했다. 통과.
5. **W3C 근거가 다른 단계인가?** — §6 item generation/grouping과 §11.5.1 span 분배를 각각 직접 링크했다. 통과.
6. **Chrome 값으로 expected tracks를 역산했는가?** — `XX=20`, `XXXX=40`, image width 72의 산식과 각 track 배열을 observation과 별도 기록했다. 통과.
7. **Chrome 바이너리 버전을 오인했는가?** — Agent Browser의 `navigator.userAgentData.uaFullVersion`이 `155.0.8059.85`이며 실제 process path도 고정된 Chrome for Testing 앱이었다. 통과.
8. **폰트가 fallback될 수 있는가?** — JSON의 `ahemReady=true`, Ahem SHA-256과 10px 측정을 함께 보관한다. 통과.
9. **PNG가 아직 로드되지 않았을 수 있는가?** — `imageDecodeComplete=true`, natural size 24×16, PNG SHA-256을 확인했다. 통과.
10. **이미지가 Grid item 이전의 inline 흐름처럼 다뤄졌는가?** — CSS에서 `img` display를 지정하지 않았고 Chrome observation의 Grid item computed display는 `block`이다. 통과.
11. **text node 경계가 item 경계로 오인되는가?** — comment 양쪽의 두 `XX`가 한 anonymous item으로 sizing된 장면을 포함한다. 통과.
12. **literal comment가 Spinon HostDocument에서 생성된다고 주장하는가?** — 계획은 Chrome comment와 HostDocument의 인접 Text node 표현을 구분한다. 통과.
13. **whitespace-only run이 cell을 소비하는지 빠졌는가?** — image 앞 whitespace-only run과 뒤 non-whitespace text를 별도 확인한다. 통과.
14. **`display:none` child가 run을 나눌 수 있는가?** — hidden span을 직접 text와 image 사이에 둔 다섯 번째 scene이 있다. 통과.
15. **tracks만 같으면 잘못된 provenance를 놓치는가?** — 계획의 통과 조건은 anchor/source node 목록, source order, owner, revision도 검사한다. 통과.
16. **Range 위치를 익명 box frame으로 과장하는가?** — evidence와 plan이 Range rect는 fragment 관찰이며 item frame의 대체가 아니라고 명시한다. 통과.
17. **Android 실기기 결과를 포함한다고 오해할 수 있는가?** — STATUS와 PR 본문이 Android API 37 AVD 선택과 simulator 미실행 상태를 구분한다. 통과.
18. **iOS simulator gate가 빠졌는가?** — plan과 STATUS가 iPhone 17 Pro/iOS 26.2 Simulator를 별도 gate로 명시한다. 통과.
19. **PR의 screenshot 링크가 깨질 수 있는가?** — PR #424 본문 이미지의 실제 `currentSrc`가 raw branch URL이고 Agent Browser에서 `complete=true`, `1280×577`로 표시됐다. 통과.
20. **전체 문서 빌드 실패를 이번 diff 성공으로 숨기는가?** — 683개 다른 dead-link 진단으로 build가 실패한 사실을 PR 본문에 남겼고 신규 파일 이름은 실패 로그에 없었다. 통과.

결함을 발견하지 못했다. `spec/STATUS.md`에는 PR #424와 이 diff review 링크를 반영했다. PR check rollup은 비어 있으며 required check은 보고되지 않았다. 전체 C11.4나 CSS Grid 호환 완료를 주장하지 않는다.
