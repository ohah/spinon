# 계획 PR diff 실패 관점 검토 30개

검토 대상은 `c11-4-auto-repeat-percentage-calc-minimum.md`, 계획 전용 20개 검토, CSS fixture와 사전 비교 README/JSON/PNG다. 아직 Spinon runtime이나 공식 완료 체크를 바꾸지 않는다.

1. **변경 범위 고정** — 이 diff는 definite 112px axis의 percentage calc auto-repeat 후보와 계획 근거만 추가한다. 제품 CSS 경로는 바꾸지 않는다.
2. **상위 항목 연결** — 계획 제목과 링크가 C11.4 Grid track sizing 및 내부 계약 0059로 이어진다.
3. **내부 버전 불변** — 계획과 precomparison은 내부 계약 0059를 0.1.0으로 유지한다고 명시한다.
4. **표준 출처** — `<fixed-size>`, `<fixed-breadth>`, `<length-percentage>`는 CSS Grid 2025 CRD에서, mixed length/percentage math type은 CSS Values and Units §5.6·§10에서 확인한다.
5. **Column basis** — CSS track list 112px inner inline size와 percentage basis를 문서에 함께 기록했다.
6. **Row basis** — row scene의 inner block size가 112px임을 기록해 column basis로 잘못 재사용하지 않는다.
7. **Calculation** — 10% × 112px = 11.2px, 20px + 11.2px = 31.2px 계산을 README와 expected JSON에서 확인했다.
8. **Repeat count** — 112/31.2의 최대 fitting 양의 정수 3을 계획과 독립 JSON 모두에 기록했다.
9. **Control input** — fixed 31.2px control이 calc scene과 같은 input dimensions·mode·item count를 사용한다.
10. **Source HTML identity** — six `data-scene-id` 값이 plan table, Chrome observation, expected JSON의 key와 일치한다.
11. **Track-count discrimination** — calc의 percentage를 버리면 minimum이 20px로 줄어 다섯 track이 가능하므로 scene이 오류를 구분한다.
12. **Fractional track expectation** — ideal 112/3px와 Chrome의 fraction 결과를 따로 보존하고 허용오차를 1/64px로 한정한다.
13. **Chrome quantization** — observed DOMRect는 37.328125px로, serialized used track은 37.3281px로 각각 저장했다.
14. **Viewport geometry** — screenshot과 JSON은 viewport 1280×633, DPR 1과 같은 browser observation을 가리킨다.
15. **Root coordinates** — Chrome observation은 viewport 좌표, expectation은 root-relative 좌표라고 문서에서 구별한다.
16. **auto-fill case** — fill scenes의 세 item frame이 track 세 개와 대응한다.
17. **auto-fit case** — fit scenes는 pre-collapse count와 post-placement `[112,0,0]` tracks를 별도로 기록한다.
18. **Empty-track gutters** — gap 0을 의도적으로 고정했고 nonzero gap/collapse 결과를 일반화하지 않는다.
19. **No cyclic overclaim** — definite axis만 acceptance 대상이며 intrinsic/cyclic/indefinite root는 제외로 남는다.
20. **No grammar overclaim** — 한 percentage calc 식 외의 다른 연산·양수/음수 결과·arbitrary grammar는 검증하지 않는다.
21. **No CSS admission overclaim** — fixture profile만 바꿀 계획이며 공개 authored CSS admission이나 API 지원을 주장하지 않는다.
22. **No test overclaim** — Chrome 결과는 precomparison이며 Spinon runtime/WPT 결과가 아님을 README에 적었다.
23. **Simulator gate stays pending** — Android/iOS runtime은 후속 구현 관문으로 쓰며 아직 실행 결과로 기록하지 않는다.
24. **Pixel evidence scope** — future pixel sampling은 사전 고정된 mask/points만 허용하며 compositor 경계를 과장하지 않는다.
25. **Archive boundary** — static FFI archive marker 대조와 APK/IPA exclusion을 구분하는 후속 관문을 둔다.
26. **Error boundary** — unsupported/invalid value가 기존 fail-closed 동작을 유지해야 함을 구현 관문에 포함했다.
27. **Test fixture discoverability** — 파일은 저장소 CSS C11 fixture 경로에 두어 runtime fixture 코드와 섞지 않는다.
28. **Evidence reproducibility** — raw browser observation, independent expectations, screenshot과 source input을 각각 경로로 남긴다.
29. **Roadmap route mapping** — 공개 row의 plan, Chrome README, PNG 및 STATUS href가 각각 존재하는 source route로 매핑되고 문구가 plan-only 상태를 표시한다.
30. **Official status accounting** — STATUS note와 roadmap row 모두 C11.4 미완료·74/207을 유지하며 plan-only work를 완료 수에 합치지 않는다.
