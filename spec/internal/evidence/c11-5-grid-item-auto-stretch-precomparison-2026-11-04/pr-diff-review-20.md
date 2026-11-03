# C11.5 auto-size/stretch 계획 PR 변경분 실패 관점 검토 20개

검토 대상은 이 계획 PR의 전체 변경분이다. 계획 자체의 20관점 검토와 별도로 경로·근거·로드맵 상태·외부 게시 범위를 확인한다.

| # | 실패 관점 | 결과 및 방어 |
|---:|---|---|
| 1 | 새 계획을 C11.5/C11 구현 완료로 표현 | **통과.** 계획 상태로 표시했고 runtime 구현은 없다고 명시한다. |
| 2 | 공식 체크박스 또는 완료 수 증가 | **통과.** C11 checkbox를 건드리지 않고 공식 `72/213`을 유지한다. |
| 3 | 계획이 C11.5 단계 색인에서 발견되지 않음 | **통과.** 상위 C11 계획과 기존 C11.5 self-alignment 계획에서 새 계획으로 연결한다. |
| 4 | 기존 fixed-size slice의 범위를 소급 변경 | **통과.** 기존 PR #1128 계획은 유지하고 auto/stretch를 별도 후속으로 연결한다. |
| 5 | 상태 대장에 계획이 빠지거나 완료처럼 들어감 | **통과.** `STATUS.md`에 checkbox 없는 계획 단계 기록을 추가했다. |
| 6 | 공개 roadmap과 공식 상태 원본이 다르게 완료를 표현 | **통과.** 두 문서 모두 계획 단계·미구현·72/213 유지로 쓴다. |
| 7 | implementation/runtime 결과를 아직 수집한 것처럼 작성 | **통과.** 구현 및 simulator 실행이 계획의 성공 조건으로만 표기된다. |
| 8 | fixture source가 증거 hash와 달라짐 | **통과.** fixture SHA-256 `d034f5584f4475270e11e2eec3947671571f98496e3b31b145c53e237cfcc40e`가 `input.json`, Chrome observation 및 README에서 일치한다. |
| 9 | screenshot의 파일 hash나 화면 크기 불일치 | **통과.** Agent Browser 캡처는 `1280×633`이며 README와 input manifest hash가 일치한다. |
| 10 | Chrome oracle 버전/commit을 재현할 수 없음 | **통과.** HeadlessChrome `156.0.8078.80`, revision, OS, viewport, DPR을 기록했다. |
| 11 | computed alignment value와 resolved dimension을 혼동 | **통과.** alignment keyword는 computed value, dimension은 `cssomResolvedWidth/Height`, geometry는 `frame`으로 분리한다. |
| 12 | 독립 기대값을 Chrome output에서 복사해 oracle circularity 유발 | **통과.** `expected.json`과 Chrome observation을 별도 파일로 두고 규범 규칙을 기록했다. |
| 13 | 외부 근거가 2차 출처거나 버전 없는 요약 | **통과.** CSS Box Alignment와 CSS Grid W3C 규범 문서에 직접 연결한다. |
| 14 | fixed axes가 stretch로 변하는 regression | **통과.** both-fixed, width-fixed/height-auto, width-auto/height-fixed scene이 있다. |
| 15 | 한 축 stretch의 효과가 다른 축으로 새어 감 | **통과.** align-only와 justify-only scene, 축별 fixed controls를 둔다. |
| 16 | auto inheritance와 explicit normal/stretch가 혼동 | **통과.** auto+parent normal/start, self normal, self stretch를 별도 scene으로 보존한다. |
| 17 | cell 경계를 넘어가는 frame 회귀를 검증하지 않음 | **통과.** 계획 성공 조건은 모든 item frame의 cell containment를 요구하고 fixture에는 fixed-size control이 있다. |
| 18 | negative input이 partial scene을 남기거나 원래 범위를 넓힘 | **통과.** invalid dimensions, edges, ratio, replaced/content child, track/placement inputs의 pre-publication 거부를 계획한다. |
| 19 | simulator·GPU·compositor 확인 범위를 과장 | **통과.** Android software renderer 및 iOS drawable/compositor 경계를 별도로 기록하도록 한다. |
| 20 | Tailnet 게시를 완료했다고 잘못 보고 | **통과.** 변경은 저장소 roadmap 원본까지이며 Tailnet preview 갱신은 merge 후 별도로 hash와 화면 확인이 필요하다. |

## 결론

이 PR diff는 계획, 규범 사전 비교, 상태 안내만 추가한다. 런타임 동작·공식 완료 수·Tailnet 게시 성공은 주장하지 않는다.
