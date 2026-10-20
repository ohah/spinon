# C11.4 multi-span iOS fixture와 Simulator 실행 검토 20개

검토 대상은 [iOS scene allowlist와 기대 frame](../../../../platforms/ios/Sources/C0410RuntimeGpuDemo.swift), [142-scene capture runner](../../../../tools/css-reference/capture-c11-4-auto-repeat-neighbors-simulator.mjs), iPhone 17 Pro / iOS 26.2 Simulator의 네 장면 log와 screenshot이다. Android 실기기와 Rust layout 검토는 기존 [구현 검토](implementation-review-20.md)에 분리했다.

1. **새 scene ID가 앱에서 거부되는가?** — 네 ID를 C11.4 exact allowlist와 app fixture selection에 연결했고 네 장면 모두 `status=0`을 냈다. 통과.
2. **다른 Grid fixture route와 인자가 충돌하는가?** — 기존 C11.3 repeat/named-line/template-area 및 C11.2 placement route를 제외하는 guard를 유지했다. C11.4 scene과 run ID만 전달했다. 통과.
3. **root intrinsic size가 잘못 기대되는가?** — 네 장면 모두 220×40 CSS px root로 app frame assertion과 runner 독립 frame 비교를 통과했다. 통과.
4. **reversed source order에서 item box count가 틀리는가?** — span-2 두 장면은 root+item 2개인 3 boxes, span-2/3 두 장면은 4 boxes로 각각 검사했다. 통과.
5. **첫 item frame만 확인해 뒤 item 순서 오류를 놓치는가?** — 전체 preorder node frame을 node ID와 x/y/width/height로 비교했다. 네 frame 배열이 Chrome 사전 관찰의 1/64 CSS px 범위에 들었다. 통과.
6. **Chromium 양자화와 runtime f32 표현 차이가 정상 case를 실패시키는가?** — 비교는 문자열 동일성이 아니라 각 numeric field 1/64 CSS px tolerance로 수행한다. 최대 track 차이는 0.010415, 최대 frame 차이는 0.005207 CSS px였다. 통과.
7. **span-3 source item이 먼저여도 짧은 span이 먼저 계산되는가?** — 두 mixed case의 전체 runtime trace에서 `IntrinsicItemBatch` 순서 `[2,3]`을 확인했다. 통과.
8. **같은 span-2 경쟁의 DOM 순서가 used tracks를 바꾸는가?** — forward/reversed 두 scene의 최종 Columns tracks 모두 `40,60,60,0`이다. 통과.
9. **span-3 contribution에서 네 번째 열까지 증가하는가?** — mixed case의 최종 네 track은 Chrome quantization tolerance 안에서 `53.328125,73.328125,73.34375,0`과 일치한다. 통과.
10. **collapse 상태가 빈 track을 감춘 채 오해되는가?** — 네 fixture의 최종 track 네 개 모두 collapsed=false임을 runner가 확인했다. 통과.
11. **trace가 일부 chunk만 도착해도 성공하는가?** — scene별 declared total과 수집 chunk 수를 비교했다. span-2 scene은 12개, mixed scene은 14개 trace chunk가 모두 복원됐다. 통과.
12. **stage parser가 문자열의 다른 숫자를 span으로 오인하는가?** — `IntrinsicItemBatch { span: N, ... }` prefix만 추출하고 중복 제거 없이 기대 sequence와 배열로 대조한다. 통과.
13. **잘못된 run의 오래된 OS 로그를 이번 실행으로 읽는가?** — iOS process PID와 고유 run ID를 함께 고정하고 summary, trace, draw/present가 일치할 때만 종료한다. 통과.
14. **frame assertion 실패를 성공으로 기록하는가?** — app은 exact expected frame과 box 수가 다르면 assertion error/status failure를 내며 runner는 그 marker를 통과 결과로 받아들이지 않는다. 네 scene에 assertion error가 없다. 통과.
15. **화면 캡처가 layout 완료 전에 찍히는가?** — runner는 target process의 C11.4 summary, trace, 예상 box count의 draw/present를 확인한 뒤 screenshot을 저장한다. 통과.
16. **다른 부팅된 simulator가 테스트 대상이 되는가?** — UDID `ACA7BF91-E2D5-4CF7-909A-08D1AD95FF3D`를 명시했고 runner가 iPhone 17 Pro 및 iOS 26.2 Booted 상태를 확인했다. 다른 simulator는 종료하거나 변경하지 않았다. 통과.
17. **fixture payload가 제품 기본 빌드에 섞이는가?** — iOS 앱 빌드는 `SPINON_ENABLE_C04_RUNTIME_GPU_FIXTURE_TEST=1`을 명시했다. 제품 기본 feature를 변경하거나 release 지원을 선언하지 않았다. 통과.
18. **capture runner가 새 ID를 세지 않거나 ID 충돌을 놓치는가?** — 기존 138 scene 뒤에 네 ID를 추가하고 전체 count 142와 unique-ID 수를 assertion했다. 통과.
19. **증거가 iOS 실기기 또는 compositor 완료를 주장하는가?** — metadata는 iPhone 17 Pro/iOS 26.2 Simulator임을 기록하고 README는 물리 iPhone 및 compositor 이후 확인이 아니라고 한정했다. 통과.
20. **고정 fixture 검증이 C11.4 전체 지원으로 승격되는가?** — 계획과 STATUS의 C11.4 checkbox 및 완료 수를 유지하고 nested/hidden, 다른 track grammar, WPT와 전체 C11.4를 미완료로 남겼다. 통과.

## 검증 결과

- iOS fixture-test build가 `BUILD SUCCEEDED`로 끝났다.
- 새 stage-order assertion을 포함한 iOS capture runner가 네 장면에서 frame, track, collapse, `[2]`/`[2,3]` span 단계, 완전한 trace, draw/present를 통과했다.
- simulator screenshot을 열어 root와 네 item frame이 출력된 앱 화면을 확인했다.
- 이 기록은 iPhone 17 Pro Simulator 실행만 다루며 iOS physical device 및 Android evidence를 대체하지 않는다.
