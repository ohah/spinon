# C11.4 app surface clear 표본 기대값 구현 교정 검토 20관점

**검토 대상 source:** `29449e4f794e53255909b58ed8fb2fd4995dd5d2` + C11.4 전용 six-file diff<br>
**검토 diff SHA-256:** `de99a8a509db865bd3566bfe3649821f0f207343a1883939c4bca66ec589170e`<br>
**계획 blob SHA-1:** `30922642f0bd8c6763742457699222f20010723e` · [app surface 표본 계획 교정 검토](../c11-4-image-auto-max-border-overflow-plan-correction-review-2026-10-26/plan-correction-review-20.md)<br>
**이전 구현 검토:** [초기 구현 20관점 정적 검토](./implementation-review-20.md)<br>
**검토 경계:** 초기 구현에 추가된 outside-root app pixel expected correction만 확인한다. 계획 및 renderer 변경은 없다.

**후속 상태:** 이 문서는 outside-root 기대값 교정 diff만 고정한 시점별 기록이며 Android runtime 전의 정적 점검이다. 이후 Android adapter의 scene allowlist 보완과 iOS/Android simulator runtime은 [후속 구현 검토](./android-pixelcopy-scene-allowlist-review-20.md)와 [runtime evidence](../c11-4-image-auto-max-border-overflow-runtime-2026-10-26/README.md)에 기록했다.

| # | 적대적으로 확인한 실패 가능성 | 대조 및 판정 |
|---:|---|---|
| 1 | 검토 hash가 최종 source diff와 다름 | origin/main 기준 six product/source file diff SHA-256을 고정했다: `de99a8a509db865bd3566bfe3649821f0f207343a1883939c4bca66ec589170e`. PASS. |
| 2 | code correction이 root 승인된 다른 계획 revision을 사용 | app sample correction은 root 독립 PASS된 plan SHA-1 `30922642f0bd8c6763742457699222f20010723e`에 종속한다. PASS. |
| 3 | iOS outside-root 표본이 CSS body color를 계속 요구 | `C0410RuntimeGpuDemo.swift`의 이 scene tuple은 outside sample RGBA `[0,0,0,255]`를 요구한다. PASS. |
| 4 | Android PixelCopy gate가 CSS body color를 계속 요구 | `C0410RuntimeGpuDemo.java::c114GridExpectedRgb`는 같은 sample을 RGB `[0,0,0]`으로 요구한다. PASS. |
| 5 | runner가 native gate와 다른 expected color를 전송 | `capture-c11-4-auto-repeat-neighbors-simulator.mjs`도 같은 sample id/index에 RGBA `[0,0,0,255]`를 보낸다. PASS. |
| 6 | Chrome browser oracle까지 black으로 바뀜 | `spec-derived-expectations.json`의 Chrome screenshot sample은 여전히 `[16,24,32,255]`; runtime surface expectation은 따로 black이다. PASS. |
| 7 | app surface sample과 root element background가 혼동됨 | expected color는 root box 바깥의 common surface clear를 가리킨다. root style/background `[16,24,32]`는 바꾸지 않았다. PASS. |
| 8 | app fixture가 surface clear를 CSS에서 지정하는 API를 새로 받음 | fixture scene, native host API, renderer를 변경하지 않았다. Expected-value correction만 있다. PASS. |
| 9 | plan 변경으로 geometry 기대값도 무단 조정 | root/item frame과 tracks는 수정되지 않았다: root `12×40`, item `(1,1,24,16)`, columns `[24,17]`, row `[16]`. PASS. |
| 10 | four-sample gate가 표본을 생략하거나 중복 | sample ids, order, target node ids, four-sample count 및 indices `[12,8]`, `[26,8]`, `[0,30]`, `[11,30]`는 수정되지 않았다. PASS. |
| 11 | item marker가 red 대신 clear로 판정 | overflow item sample remains `[229,72,77,255]` in both native platform inputs and runner. PASS. |
| 12 | 좌측 border sample이 clear로 바뀜 | left border expected `[34,197,94,255]` remains unchanged. PASS. |
| 13 | 우측 border sample이 clear로 바뀜 | right border expected `[34,197,94,255]` remains unchanged. PASS. |
| 14 | iOS expected RGBA의 alpha가 누락되거나 달라짐 | iOS outside expected remains four-channel opaque black `[0,0,0,255]`. PASS. |
| 15 | Android RGB mapping이 PixelCopy sample order와 불일치 | Android expected switch retains the same scene/sample id dispatch and changes only the selected RGB triplet. PASS. |
| 16 | CSS pixel center mapping이 좌표를 한 pixel 이동 | runner indices remain integer indices and the `+0.5`, scale, origin mapping code is unchanged. PASS. |
| 17 | Android implementation is claimed proven without an AVD capture | The Android path remains explicitly unverified. No API 37 AVD run was started. PASS. |
| 18 | iOS corrected expected value is reported as a fresh runtime pass | Prior iOS log remains a `3/4` mismatch under its old expected value. The new gate has not been rerun, so corrected runtime status is pending. PASS. |
| 19 | correction changes product renderer or fixture admission | No renderer, layout case, FFI scene, fixture source or general product behavior changed in this correction. PASS. |
| 20 | limited slice is counted as official C11.4 completion | `spec/STATUS.md` and public roadmap are untouched; official `72/213` and C11.4 incomplete status remain unchanged. PASS. |

## Static checks

- `git diff --check`: passed.
- `python3 -m json.tool .../spec-derived-expectations.json`: passed.
- Source grep confirms iOS, Android and runner outside-root gates use opaque black; the Chrome screenshot oracle retains `[16,24,32,255]`.
- No build, simulator run, AVD run, cache cleanup, or snapshot operation was performed after this correction. The prior app run is documented in the runtime evidence README.

**Correction review result: PASS for the bounded expected-value change.** Corrected iOS runtime evidence and Android runtime evidence remain pending.
