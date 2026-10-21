# C11.4 nested Grid 양 축 intrinsic sizing 계획 실패 관점 검토 20개

계획 구현 전, 각 검토는 서로 다른 실패 경로·계약 경계를 대상으로 수행했다. 계획을 검토하는 동안 구현은 시작하지 않았다.

1. **기존 계획과 중복** — 기존 depth-two 계획은 높이 `auto`와 rows `[auto]`로 제한됨을 확인했다. 새 계획은 동시 양 축 sizing으로 구분했다.
2. **공식 상태 과장** — 신규 slice가 C11.4 전체를 완료로 만들 위험을 확인했다. checkbox와 `72/213` 유지 및 opt-in 경계를 명시했다.
3. **규범과 관측 혼동** — Chrome 수치를 규범 근거로 역산할 위험을 확인했다. §5.2/§11.1과 Ahem 입력의 독립 계산 JSON을 분리했다.
4. **잘못된 축 순서** — row 기대값이 inline wrapping 전에 계산될 위험을 확인했다. columns 선행, rows 후행과 폭 의존성을 설명했다.
5. **min-content 폭 계산** — 공백을 포함한 전체 문자열 폭을 min-content로 오인할 위험을 확인했다. 가장 긴 unbreakable word인 70px을 기준으로 고정했다.
6. **max-content 폭 계산** — 공백을 제거해 전체 문자열을 짧게 계산할 위험을 확인했다. `XX`, 공백, `XXXXXXX`의 전체 advance 100px을 적었다.
7. **block-size 계산** — min-content 폭에서 text가 한 줄로 남는다고 오인할 위험을 확인했다. 70px에서 두 개 10px line, 총 20px임을 적었다.
8. **max block-size 계산** — max-content text가 두 줄로 wrap될 위험을 확인했다. 100px 폭에서 한 줄 10px임을 명시했다.
9. **track과 box size 혼동** — frame만 같고 used row/column track이 다른 구현을 놓칠 위험을 확인했다. 두 축 track 배열과 각 Grid 크기를 별도 검사한다.
10. **중첩 전파 깊이** — depth-one만 통과하고 middle Grid 경로가 누락될 위험을 확인했다. depth-one control과 depth-two scene을 같은 조건으로 비교한다.
11. **scene ID 충돌** — 기존 inline-only scene에 새 style이 잘못 dispatch될 위험을 확인했다. 새 four scene ID를 별도로 고정한다.
12. **추가 child 경로** — 두 번째 child가 contribution을 바꾸거나 무시해 잘못 통과할 위험을 확인했다. exact child topology/순서를 allowlist에 둔다.
13. **트랙 목록 확장** — 복수 track 또는 implicit track이 우연히 허용될 위험을 확인했다. 단일 column/row만 허용하고 다른 문법은 거부한다.
14. **sizing 조합 확장** — 한 축만 intrinsic인 기존 조합이 새 경로에 들어올 위험을 확인했다. 정확한 width/height/column/row 조합만 gate한다.
15. **partial output** — projection 실패 후 일부 track/frame이 게시될 위험을 확인했다. 오류 시 partial layout과 incomplete trace를 금지한다.
16. **runtime profile 오염** — fixture 지원이 일반 CSS projection으로 번질 위험을 확인했다. 기존 opt-in fixture-test 경계만 수정한다.
17. **revision 일관성** — 부모와 자식의 다른 style/font revision이 합쳐질 위험을 확인했다. 같은 revision과 Ahem text identity를 검사한다.
18. **좌표 해석** — Chrome의 section별 y 위치를 layout 기대값으로 오인할 위험을 확인했다. root-relative offsets만 판정한다.
19. **오차와 readiness** — font 미준비 또는 subpixel 차이가 가짜 비교를 만들 위험을 확인했다. Ahem readiness exact와 1/64 CSS px tolerance를 고정했다.
20. **증거·플랫폼 과장** — Chrome 화면을 Spinon/runtime 또는 simulator 증거로 오인할 위험을 확인했다. plan oracle, 양 simulator, artifact boundary, 미검증 범위를 분리했다.

**판정:** 위 관점에서 계획의 차이·기대값·실패 경계·검증 산출물이 구체적임을 확인했다. 계획 검토 후 fixture HTML과 독립 기대값을 대조해 저장했으며 구현은 이 검토와 별도 20개 runtime 관점 검토가 작성될 때까지 시작하지 않는다.
