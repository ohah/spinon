# C11.4 depth-three nested Grid text multi-span 계획 실패 관점 검토 20개

검토 기준은 [고정 계획](./c11-4-nested-grid-multispan.md), [Chrome 사전 관측](../spec/internal/evidence/c11-4-nested-grid-multispan-precomparison-2026-10-22/chrome156-observation.json), [독립 기대값](../spec/internal/evidence/c11-4-nested-grid-multispan-precomparison-2026-10-22/spec-derived-expectations.json), 저장소의 기존 direct text multi-span·depth-three fixture와 CSS Grid 2025 CRD §11.5.1이다. 이 검토는 계획 문서에 대한 검토이며 구현/runtime 검토를 대체하지 않는다.

1. **잘못된 작업 경계 선택** — 기존 direct multi-span은 단일 Grid이고 기존 depth-three는 span 없는 leaf임을 코드에서 확인했다. 새 계획은 두 입력의 교집합인 depth-three inner Grid의 explicit multi-span만 다룬다.
2. **규범 계산과 Chrome을 혼합** — 기대값 JSON을 Chrome 관측 JSON과 분리했다. spec 분수 `[160/3,190/3,190/3,0]`과 Chrome 직렬화 값을 동등한 원값으로 쓰지 않는다.
3. **Ahem이 로드되지 않은 oracle** — Chrome 관측은 `document.fonts.check("10px Ahem") == true`를 보존한다. 준비 실패는 구현 시작 전 조사 조건이다.
4. **문자열이 다른 intrinsic width로 이어짐** — 세 텍스트와 Ahem 10px 측정값 50/70/180px을 독립 fixture 및 JSON에 고정했다.
5. **span-2 track 경계가 잘못 배치됨** — A/B의 시작선과 span 2를 각각 1·2선, 2·2선으로 exact allowlist 한다.
6. **span-3 합산 범위를 잘못 계산** — C는 첫 세 track만 span하고 180px 기여를 제공한다. fourth track은 0으로 남아야 한다.
7. **planned-increase algorithm을 단순 누적으로 오해** — `[25,35,35]`의 첫 단계 이후에만 span-3 잔여 85px을 세 track에 분배한다.
8. **DOM order가 sizing 결과를 바꾸는 결함을 놓침** — span-2 쌍과 span-2/3 쌍을 각각 forward/reversed 네 scene으로 만든다.
9. **DOM order 변경으로 placement도 이동** — 모든 item에 column line과 row line을 명시해 DOM 순서만 바뀌도록 고정한다.
10. **불필요한 row-axis 문제를 inline sizing으로 혼입** — 60px 고정 height와 세 20px row를 유지해 변화하지 않는 축을 명시한다.
11. **ancestor가 child의 min-content width를 제한** — root/middle/branch/inner 모두 width min-content와 0 edge를 사용해 expected 95/180px 전달을 보인다.
12. **세 boundary 중 일부만 검사** — 네 Grid 모두 frame/track 출력과 source-to-parent contribution trace를 acceptance 대상으로 둔다.
13. **grid container 크기와 used track 합이 불일치** — `95px` 또는 `180px` inner track 합과 동일한 각 Grid frame width를 함께 검사한다.
14. **rounded CSSOM 값을 ideal track으로 판정** — Chrome 1/64px used-value quantization과 이상적 rational track을 분리하며 비교 허용치는 구현 전에 고정한다.
15. **text item max-content width를 min-content contribution으로 착각** — oracle은 일반 inline text width가 아니라 grid sizing에 들어가는 min-content contribution을 비교한다.
16. **fixture adapter가 임의 입력을 허용** — 네 scene ID, node 수·owner/order, 텍스트·computed placement/style를 모두 고정하고 unknown/mutated input은 fail closed한다.
17. **top-level ID에서 잘못된 scene을 조용히 건너뜀** — 알려지지 않은 family scene은 layout fallback 대신 오류여야 하며 negative control을 둔다.
18. **부분 trace나 stale revision을 성공으로 취급** — trace completeness, document/style/environment revision 일치를 frame과 함께 검사한다.
19. **fixture 코드가 제품 artifact로 누출** — 기본 product archive와 opt-in fixture-test positive control을 분리해 scene ID/payload/symbol 부재와 존재를 각각 확인한다.
20. **시뮬레이터 결과를 전체 호환성으로 확대** — AVD, iOS Simulator, surface/pixel/WPT 미검증 경계를 보고하고 C11.4 checkbox·`72/213`을 유지한다.

**판정:** 발견된 차이는 fixture CSS의 `justify-self:start`를 제거해 기존 direct text multi-span baseline의 initial alignment 동작과 맞춘 것이다. 수정 후 Agent Browser Chrome 156 observation을 다시 만들고 screenshot/JSON을 교체했다. 계획은 네 고정 scene과 세 ancestor contribution을 분리해 관찰할 수 있으며 구현 전 비교 기준이 고정됐다.
