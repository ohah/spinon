# C13.1 iOS Metal pixel-center 구현 실패 관점 검토 20개 · 2026-10-15

검토 범위는 iOS Simulator 전용 C13.1 Metal drawable 표본 readback, feature-gated FFI, Chrome 155 비교기와 Android API 37 AVD 재실행 도구다. 계획 검토의 20개 관점은 재사용하지 않았다. 제품의 전체 CSS 지원, compositor·scanout, 물리 GPU 결과로 확대하지 않는다.

| # | 실패 경로 | 확인과 결과 |
|---:|---|---|
| 1 | test-only readback 옵션이 기본 runtime에 켜진다 | LLVM 22 `llvm-nm`으로 기본 `spinon-ffi` archive에서 두 readback symbol 부재, fixture feature build에서 생성·request symbol 존재를 각각 확인했다. **통과.** |
| 2 | Rust feature를 켜도 iOS `copy_src` UIKit surface 경로가 빠진다 | `c13-1-overflow-clip-fixture-test`를 surface readback cfg에도 연결했고, iOS Simulator build와 Metal capture 3회를 통과했다. **통과.** |
| 3 | Metal 전용 launch 인수가 C0410 controller까지 도달하지 않아 잘못된 route가 기본 화면으로 빠진다 | AppDelegate의 route 선택에 Metal flag를 추가했다. 단독 인수 대조가 명시적 오류를 내고 host를 만들지 않았다. **발견 수정 후 통과.** |
| 4 | pixel route가 compact hit-identity scene 없이 실행된다 | controller가 host 생성 전에 route 조합을 검사한다. standalone negative control에서 host/capture 완료 로그가 없었다. **통과.** |
| 5 | 두 C13.1 fixture route가 함께 선택돼 한 report를 다른 oracle로 해석한다 | 다중 route negative control이 `multiple-fixture-routes`로 host 생성 전에 거부됐다. **통과.** |
| 6 | 첫 UIKit layout이 fixture eval 중 presentation sequence를 바꾸어 `status=-12`를 만든다 | 실제 Simulator에서 초기 `-12`를 재현했다. compact route runtime init 완료 전 layout callback은 presentation update를 시작하지 않고, 초기 환경 제출 뒤 renderer를 준비하도록 가드를 추가했다. 새 process 3회 모두 eval `status=0`. **발견 수정 후 통과.** |
| 7 | 첫 Metal readback이 baseline draw보다 앞서 시작되거나 시작 draw 결과를 표본으로 오해한다 | 성공 draw와 52-query 완료가 확인된 다음 별도 draw 요청을 열어 batch capture를 시작한다. 세 실행 모두 baseline marker와 469 batch를 기록했다. **통과.** |
| 8 | retina drawable의 x/y 배율을 같다고 가정하거나 density로 좌표를 환산한다 | `900×300` drawable에서 각각 `3×` x/y scale과 pixel-center `floor((css+0.5)×scale)`를 manifest와 30,000행 comparator로 대조했다. **통과.** |
| 9 | 첫 행 또는 마지막 행이 누락되어 viewport 일부를 검사하지 않는다 | row-major CSS `(0,0)`부터 `(299,99)`까지 30,000개 좌표와 469개 batch(468×64 + 48)를 전부 확인했다. **통과.** |
| 10 | 큰 readback 요청이 FFI에서 메모리·report 한도를 넘긴다 | C ABI는 1–64개만 수락한다. 0, 65, null 좌표, 곱셈 overflow, null/짧은 출력 buffer negative tests를 실행했다. **통과.** |
| 11 | batch 경계에서 좌표가 누락·중복·재정렬된다 | batch index/count 순서, 1개 outstanding 요청, JSONL index와 CSS 좌표의 연속성을 확인한다. 각 fresh run 469 batch가 연속했고 비교기 mutation test가 missing/duplicate/reorder를 거부했다. **통과.** |
| 12 | 작은 drawable에서 두 CSS center가 하나의 물리 pixel을 공유한다 | scale 1 미만을 거부하고 각 batch 내 물리 좌표 중복을 검사하며, 전체 30,000표본 uniqueness를 comparator에서도 재검증한다. **통과.** |
| 13 | surface resize 뒤 이전 generation의 RGBA가 새 표면 결과로 저장된다 | capture가 첫 generation, dimensions, format을 고정한다. generation 변경과 이전 draw report를 거부하며 runner가 generation 일치를 3회 확인했다. **통과.** |
| 14 | Metal texture format 변경 또는 보고된 texture 크기 불일치가 섞인다 | 네 가지 8-bit RGBA/BGRA format만 허용하고 `surface_texture` dimensions를 renderer report와 비교한다. 세 capture 모두 `Bgra8UnormSrgb`, `900×300`. **통과.** |
| 15 | request report의 coordinates가 예약한 좌표와 다르다 | JSON report의 정확한 `x/y` 순서와 batch 크기를 비교하고, batch 좌표는 앱 경계에서 다시 검사한다. 469 batch가 모두 정상 응답했다. **통과.** |
| 16 | 잘못된 report shape, Boolean/실수/out-of-range RGBA가 정상 자료로 인코딩된다 | sample object key를 고정하고 integer JSON number, 네 channel, 0–255를 검사한다. comparator는 row shape·RGBA 개수·범위를 독립 검증한다. **통과.** |
| 17 | batch 하나가 실패했는데 앞부분 sample 파일을 완료 자료처럼 남긴다 | capture error는 `.part`와 완성 경로를 제거하고 `complete` manifest를 쓰지 않는다. manifest는 모든 30,000행 뒤에만 생성한다. **통과.** |
| 18 | 파일 쓰기 중 종료·손상·다른 run의 표본 혼합을 감춘다 | 각 row와 manifest에 run ID/generation을 기록하고 sample SHA-256, process ID를 고정한다. manifest JSON은 atomic write, 두 `.part` 경로는 rename한다. comparator의 hash/run/generation mutation을 거부했다. **통과.** |
| 19 | draw stall이나 무한 retry가 simulator job을 무기한 점유한다 | capture는 batch 15초·전체 300초 deadline을 가지며 runner는 process 별 6분 내 완료를 요구한다. superseded batch는 세대가 다른 report로 재시도하지 않고 명시적으로 실패한다. **통과.** |
| 20 | 요소 border 바깥 paint가 전수 대조에서 빠지거나 채널 오류를 놓친다 | 30,000점 전체가 대상이고, frame 오른쪽 끝 264 px를 넘는 `positioned-escape` `(270,72)`의 Chrome RGBA `[227,84,78,255]`를 확인했다. R/G/B/A 각 1-byte mutation 4개를 모두 거부했고 iOS/AVD 실행에서도 같은 좌표를 포함했다. **통과.** |

## 실행 범위

- iPhone 17 Pro / iOS 26.2 Simulator 새 process 3회: 각 41/41 frame, 52/52 fixed query, 30,000/30,000 hit identity, 30,000/30,000 Metal CSS center, 120,000/120,000 RGBA channel exact.
- Android API 37 ARM64 AVD / ANGLE·Vulkan·SwiftShader: 41개 고유 frame, 52/52 fixed PixelCopy, 30,000/30,000 hit identity, 30,000/30,000 CSS center, 120,000/120,000 RGBA channel exact.
- Chrome 155 oracle와 범위는 `300×100` viewport의 CSS pixel centers다. 전체 physical drawable byte, compositor/scanout, 실기기·hardware GPU, DOM event·CSSOM, WPT는 확인하지 않았다.
