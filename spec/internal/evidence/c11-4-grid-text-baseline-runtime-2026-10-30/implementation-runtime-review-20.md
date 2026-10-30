# C11.4 Grid text baseline implementation/runtime review (20 checks)

검토 대상은 현재 작업 트리의 adapter, vendored Taffy 변경, typed fixture/runtime tests와 두 simulator capture다. 범위 밖 동작은 지원 완료로 확대하지 않는다.

1. **일반 CSS의 `align-items:first baseline` admission** — PASS. ordinary projection 거부 회귀 test가 통과한다.
2. **일반 CSS의 `align-items:last baseline` admission** — PASS. 별도 거부 regression이 통과한다.
3. **Grid 전용 last-baseline marker 생성** — PASS. Grid 컨테이너/item adapter만 private Taffy sentinel을 사용한다.
4. **Flex 경로 보호** — PASS. generic alignment mapper는 기존 first-baseline 동작을 유지하며 이 slice는 Flex last-baseline을 공개하지 않는다.
5. **feature isolation** — PASS. 새 text metadata와 runtime scene route가 `c11-4-grid-track-sizing-fixture-test`에 묶여 있다. module-boundary 23/23과 기본/fixture debug archive symbol·scene 문자열 대조가 통과한다. APK/IPA 전체 제외는 검증하지 않았다.
6. **baseline participation 제외** — PASS. 기존 auto block margin 및 cyclic block-size dependency 제외 조건을 유지한다.
7. **first-baseline geometry 회귀** — PASS. 기존 one-line first group, `align-self`, source-order 역전 장면이 Rust 및 두 simulator에서 통과한다.
8. **첫 line metric 선택** — PASS. first-baseline 경로는 paragraph snapshot의 첫 line을 사용한다.
9. **마지막 line metric 선택** — PASS. last-baseline 경로가 `Baselines.last`를 선택하는 multiline layout test가 통과한다.
10. **행 공유 그룹 분리** — PASS. first와 last baseline items를 서로 다른 그룹으로 계산한다.
11. **end-edge shim** — PASS. last group의 end distance를 사용하고 shim을 block-end margin 쪽에 반영한다. 고정 fixture geometry는 규범 기대값과 일치한다.
12. **start-edge first shim** — PASS. 기존 first-baseline shim은 start 쪽에 유지되며 regression geometry가 같다.
13. **마지막 baseline 정렬 geometry** — PASS. 두 줄 작은 item과 한 줄 큰 item의 last baseline은 양쪽에서 y=18px로 맞고 row는 22px다.
14. **`align-self` override** — PASS. 부모 `start`에서 자식 두 항목의 last-baseline scene이 상속 장면과 같은 geometry를 낸다.
15. **DOM source-order 역전** — PASS. 명시된 row/column 위치를 유지하고 native validator도 node와 frame index를 함께 비교한다.
16. **first/start controls** — PASS. one-line first control은 row 20px·baseline 16px, start control은 두 item y=0으로 나온다.
17. **text metric 검증** — PASS. 첫·마지막 baseline이 유한하고 snapshot 높이 안에 있는지 검사한다. 빈 line 또는 first profile의 multi-line은 fail-closed다.
18. **nested Grid의 외부 baseline** — PASS. 내부 last-baseline 정렬 뒤 parent에게 first baseline을 export하는 regression test가 통과한다.
19. **runtime revision와 track trace** — PASS. capture가 HostDocument revision, text line metrics, Columns/Rows trace, item frames 및 draw/present를 확인한다.
20. **표면/플랫폼 경계** — PASS WITH LIMITS. Android PixelCopy 27/27과 iOS Metal drawable 27/27 표본이 exact다. glyph paint, Android hardware GPU, 실기기와 iOS compositor 이후 결과는 검증하지 않는다.

실행 근거 및 미검증 범위는 [README](./README.md)에 있다. PR 제출 직전 archive isolation/format/diff 결과는 그때 추가한다.
