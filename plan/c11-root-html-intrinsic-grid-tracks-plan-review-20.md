# C11.1 Root HTML intrinsic Grid track 계획 실패 관점 검토 20개

계획 [c11-root-html-intrinsic-grid-tracks.md](./c11-root-html-intrinsic-grid-tracks.md)와 구현 전 자료만 대상으로 검토했다. 각 항목은 서로 다른 실패 경계다. 기능 구현 뒤 review로 재사용하지 않는다.

1. **Intrinsic size와 measured output의 동일시** — `min-content`/`max-content`는 keyword가 변환됐다는 사실만으로 입증되지 않는다. 같은 HostDocument·computed style·text revision의 measured contributions와 최종 track/frame을 함께 비교한다.
2. **line break opportunity 누락** — `alpha beta`에서 `min-content`는 전체 문자열이 아니라 가장 긴 unbreakable run을 사용해야 한다. 독립 expected와 Chrome 비교를 둘 다 보존한다.
3. **font advance의 플랫폼 편차** — Host macOS font advance를 AVD/iOS에 강제하지 않는다. Chrome oracle와 각 플랫폼 geometry를 따로 비교하고 정해진 tolerance를 넘으면 차이를 설명한다.
4. **Auto track stretch 혼합** — `auto` max track은 content alignment의 영향을 받는다. `justify-content:start` 장면과 initial `normal` auto-stretch 장면을 분리해 둘 다 고정한다.
5. **minmax floor 적용 누락** — max-content가 80px minimum보다 작은 장면을 두어 resolved max가 minimum 아래로 내려가지 않는지 확인한다.
6. **MinContent·MaxContent breadth 방향 혼동** — direct track과 min/max가 반대 위치인 `minmax(min-content,max-content)`에서 contribution phase를 각각 검증한다. CSS-valid matrix와 CSS-invalid flex-minimum을 분리한다.
7. **Flexible free-space 계산 오류** — `minmax(min-content,1fr)`은 content base를 제외하고 definite available space를 나눠야 한다. 다른 축 basis, gap, aspect-ratio guards도 기존 검사에 남긴다.
8. **Repeat metadata 경계** — integer `repeat()` expansion은 intrinsic track와 함께 쓸 수 있지만 반복 수와 line metadata quota는 그대로 유지한다. 빈 두 번째 반복 track도 expected에 포함한다.
9. **Spanning contribution의 placement 선결 조건** — two-track span은 해당 Root HTML numeric placement route가 owning container와 typed tracks를 같은 frame에 전달하는지 확인한다. placement normalizer의 explicit extent와 span arithmetic guard를 약화하지 않는다.
10. **Row-axis 과장** — 기존 row oracle은 min/max keyword 간 geometry 차이가 없는 단일 line-height control이다. 이를 columns의 분기 oracle로 대체하지 않고 row-axis track/value projection 및 frame regression으로만 주장한다.
11. **Hidden subtree 오염** — `display:none` item은 intrinsic measurements와 output frames에 기여하지 않아야 한다. CSS computed state와 HostDocument source identity를 사용하고 DOM order 변경으로 건너뛰지 않는다.
12. **Grid 밖 overflow clipping** — `max-content` item은 20px Grid 밖으로 갈 수 있다. capture는 item frame/surface가 보이는지 분리하고, grid viewport clip이 생기지 않는지 확인한다.
13. **CSS-invalid fallback의 오분류** — `minmax(1fr, …)`처럼 flex minimum이 invalid인 입력을 valid-but-unsupported 진단으로 바꾸지 않는다. Stylo의 이전 유효 선언 fallback을 검증한다.
14. **Typed invalid value 방치** — 음수·NaN·무한대 breadth를 computed snapshot에 주입하는 단위 검사로, parser가 정상적으로 거부하는 문법과 숫자 경계를 각각 다룬다.
15. **Percentage gate 우회** — intrinsic breadth가 앞에 있는 track list에서도 뒤쪽 percentage/fraction track의 definite basis, zero basis, min/max override 및 aspect-ratio guard가 실행되어야 한다.
16. **Aggregate range 검증 조기 반환** — intrinsic breadth를 만난 후 함수가 성공 반환해 뒤쪽 fixed/percentage aggregate overflow를 누락하지 않도록, track list 전체를 계속 검사하고 final output finite 상태도 확인한다.
17. **부분 scene 게시** — 여러 grid 중 하나의 rejected input 때문에 다른 grid의 성공 frame만 남지 않아야 한다. 실패 matrix는 scene, layout output, draw submission 부재까지 확인한다.
18. **기본 runtime과 fixture profile 혼동** — C11.4 opt-in route의 성공을 근거로 삼지 않는다. `RuntimeSession::new_runtime_gpu_with_root_html()` 기본 profile의 parsed HTML을 end-to-end로 실행한다.
19. **기존 profile 퇴행 누락** — named areas, direct sizes, fixed repeat, implicit fixed rows, placement, gap, alignment, item auto minimum, aspect-ratio, column/dense flow를 intrinsic track 변경과 결합하지 않은 회귀 행렬로 다시 실행한다.
20. **제품 지원 과장** — 제한 Root HTML profile이 통과해도 일반 author CSS/WPT/C11.1/C11 전체를 완료로 체크하지 않는다. 상태·내부 계획·공개 roadmap은 실제 grammar, 성공 matrix, 제한과 merge SHA를 같은 PR 후속에 맞춘다.

## 판정

20개를 계획 전용 실패 관점으로 기록했다. Chrome oracle, independent expected, same-source default Root HTML runtime, 양 플랫폼 Simulator 근거, rejection/fallback, range guards, output atomicity와 공식 완료 표기의 경계를 계획에 요구했다. 구현 review에서 이 목록을 재사용하지 않는다.
