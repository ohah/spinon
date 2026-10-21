# C11.4 anonymous text + 복수 image 계획 PR 변경분 적대 검토 20개

검토 대상은 계획 PR의 실제 파일 변경이다. 계획 실패 관점 표와 반복하지 않고, 최종 문서·fixture·Chrome 자료·roadmap 반영·PR 패키징 경계를 각각 확인했다.

1. **새 작업이 구현 완료처럼 보이는가?** — 새 계획의 상태는 Chrome 사전 비교/계획 리뷰만 완료로 두고 구현·Simulator 검증을 시작 전으로 표시한다. 통과.
2. **C11.4 전체 상태나 완료 집계가 올라갔는가?** — C11.4 checkbox와 공식 72/213 집계를 그대로 둔다. 통과.
3. **두-image scene과 세-image scene을 혼동하는가?** — fixture observation은 각 장면의 direct text/image child count가 계획 ID 및 2/3 image topology와 일치한다. 통과.
4. **fixture scene 이름이 계획 표와 다를 수 있는가?** — HTML, observation, plan의 4개 scene ID를 각각 대조해 동일함을 확인했다. 통과.
5. **anonymous item이 element로 바뀌었는가?** — 원본 Grid child list에 `XX` Text node가 남고 computed placement를 가진 element가 없어 anonymous auto-placement 입력을 보존한다. 통과.
6. **익명 item이 예상한 cell에 실제 배치되는가?** — Chrome Range fragment는 두 topology에서 local x 40/48을 보이고, preceding column track 합계와 일치한다. 이는 fragment 관찰이며 box-frame 판정으로 과장하지 않는다. 통과.
7. **이미지들이 각기 다른 source를 사용하는가?** — 모든 image src는 repository의 같은 고정 24×16 PNG를 가리키고 natural dimensions가 네 scene 전체에서 검증된다. 통과.
8. **Ahem fallback 때문에 20px 산식이 깨질 수 있는가?** — `ahemReady=true`, `XX` intrinsic width 20, Ahem asset SHA-256을 증거에 보존한다. 통과.
9. **deferred image decode 때문에 0px 결과가 저장될 수 있는가?** — 모든 PNG `decode()` 완료를 기다리고 `imageDecodeComplete=true`를 기록했다. 통과.
10. **browser minor version을 오인했는가?** — task-local Chrome for Testing binary와 UA full version 모두 `155.0.8059.85`를 기록한다. 통과.
11. **expected tracks를 DOM observation에서 복사했는가?** — 계획과 evidence는 §11.4/§11.5.1 contribution arithmetic를 별도로 쓰고 Chrome CSSOM 결과를 독립 대조 열에 보존한다. 통과.
12. **growth-limit 분배 누락이 수치를 바꾸는가?** — 20px span-1 base, 24/48px span-2 contributions, 72px span-3 contribution과 beyond-limit 4px 분배가 문서에 이어진다. 통과.
13. **reversed DOM control이 값 차이를 숨길 수 있는가?** — observation JSON에서 forward/reversed scene별 track 및 local frame이 동일하다. 통과.
14. **page y offset을 grid-local 좌표로 오인하는가?** — README frame 표는 page label inset과 case y를 제외한 root-local 좌표라고 밝히고 raw page rect는 JSON에 보존한다. 통과.
15. **이미지의 row 높이 overflow가 column 비교에 섞이는가?** — root 180px, 세 60px row와 item heights 16/32/48을 고정해 row sizing을 분리했다. 통과.
16. **evidence 경로가 깨질 수 있는가?** — 계획에서 fixture/README/plan review/diff review로 가는 상대 경로를 점검했고 모든 대상 파일이 staged diff에 포함된다. 통과.
17. **화면 캡처가 한 장면만 담는가?** — Agent Browser full-page screenshot은 PNG 1280×828이며 네 label/grid를 모두 포함한다. 통과.
18. **fixture route가 public feature로 퍼질 수 있는가?** — 구현 gate는 opt-in `RuntimeBlockFlexGridTrackSizingV1`와 default/positive-control artifact 비교를 요구한다. 통과.
19. **사용자가 정한 Android 검증 방식을 무시하는가?** — 계획은 Android API 37 AVD와 iOS 26.2 Simulator를 요구하고 Android physical device를 요구하지 않는다. 통과.
20. **Chrome 관찰을 앱 pixel 또는 WPT pass로 확대하는가?** — evidence는 구현, app, WPT, PNG texel, border 및 compositor pixels를 명시적으로 제외한다. 통과.

diff 리뷰에서 수정이 필요한 누락은 발견하지 못했다. required status checks와 PR check 결과는 PR을 연 뒤 live GitHub에서 별도 확인한다.
