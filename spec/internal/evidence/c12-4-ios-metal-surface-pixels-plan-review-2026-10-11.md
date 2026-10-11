# C12.4 iOS Metal surface pixel 계획 실패 관점 검토

**대상:** [iOS Metal surface pixel 계획](../../../plan/c12-4-ios-metal-surface-pixels.md)  
**상태:** 계획 검토 완료 · 계획의 제한을 확인했으며 구현 검증은 별도 수행한다.

| # | 독립 실패 관점 | 계획의 방어 또는 수정 |
| --- | --- | --- |
| 1 | 표면 대신 headless texture를 읽고 iOS Metal surface 결과라고 부름 | `CAMetalLayer`의 WGPU surface texture만 허용하고 offscreen 대체를 금지했다. |
| 2 | `COPY_SRC` capability가 없는 장치에서 설정을 강제하거나 조용히 다른 경로로 바꿈 | WGPU capability 확인을 선행하고 미지원은 명시적 실패로 처리한다. |
| 3 | Chrome 색상 oracle이 실행 후 바뀜 | 고정 Chromium 154 fixture와 기존 reference hash를 입력 근거로 지정했다. |
| 4 | 8개 case 중 편리한 방향만 골라 결과를 일반화함 | 네 방향과 두 order 조합의 전체 8개 집합을 정확한 완료 기준으로 둔다. |
| 5 | sample이 겹침 영역 밖이라 한 색만 우연히 맞음 | 각 case sample을 WGPU 고정 overlap microfixture와 동일하게 정하고 fixture origin을 합산했다. |
| 6 | canvas origin offset 누락으로 다른 surface pixel을 읽음 | 기존 fixture의 Flex origin `(20,125)`와 local sample을 더한 CSS surface 좌표를 표에 고정했다. |
| 7 | Retina scale을 CSS 좌표와 혼동함 | 실제 drawable size와 density를 log에 남기고 `(css + 0.5) × scale` 변환을 명시했다. |
| 8 | 특정 iPhone 배율이나 해상도를 보편값으로 하드코딩함 | 실행 metadata의 scale을 사용하고 고정한 iPhone 17 Pro Simulator 외 장치에 일반화하지 않는다. |
| 9 | `Bgra8UnormSrgb` 채널을 RGBA로 잘못 읽음 | 허용 포맷을 4개 8-bit RGB(A) format으로 제한하고 BGRA swizzle을 별도로 검증한다. |
| 10 | sRGB 변환을 CPU에서 또 적용해 값이 이중 변환됨 | surface texture 저장 byte를 읽어 채널 순서만 포맷에 맞게 해석하도록 설계했다. |
| 11 | alpha나 불투명 배경을 검사하지 않고 RGB만 비교함 | 4개 channel을 비교하고 정확한 RGBA 및 zero tolerance를 완료 조건에 넣었다. |
| 12 | stale frame 또는 이전 case pixel이 현재 결과로 통과함 | case별 새 프로세스·run ID와 surface generation·presentation sequence 검증을 요구한다. |
| 13 | map 대기를 UIKit main thread에서 수행해 교착 또는 UI 정지를 만듦 | WGPU readback을 render 전용 queue에서 수행하고 main thread 대기를 금지했다. |
| 14 | GPU callback이 오지 않을 때 테스트가 무한 대기함 | 비동기 map과 5초 bounded wait를 명시하고 timeout을 실패로 남긴다. |
| 15 | texture 복사와 `present` 순서가 달라 읽지 않은 frame을 제출함 | 같은 command encoder/submission에 render와 copy를 넣어 같은 surface frame을 읽도록 지정했다. |
| 16 | copy row pitch 정렬로 sample byte offset이 밀림 | implementation 단계에서 WebGPU의 padded bytes-per-row를 기록하고 sample offset을 독립 검사한다. |
| 17 | surface resize 또는 재생성 중 old texture를 읽음 | generation 변경 시 결과를 폐기하며 run metadata에 크기·generation을 포함한다. |
| 18 | 실패 경로가 누락되어 잘못된 비교기가 전부 통과함 | missing/duplicate/stale run, channel mutation, coordinate swap, BGRA mutation negative control을 요구한다. |
| 19 | 여러 case가 같은 앱 process/state를 공유해 cascade/cache 오염 | 각 조합을 새 process와 runtime host에서 실행하도록 고정했다. |
| 20 | Simulator pixel 결과를 실제 iPhone·성능·CSS 전체 호환으로 확대함 | simulator, 8-case subset, Chrome observation 범위를 완료 조건과 경계에서 반복 명시했다. |

## 공격 후 계획 수정 사항

- surface screenshot과 offscreen WGPU texture는 의미상 다른 결과이므로 대체 경로가 성공할 수 없도록 계획에 명시했다.
- pixel 좌표 변환은 CSS viewport, canvas origin, device scale 세 축으로 나눠 기록해야 한다. 계획 표에는 canvas origin을 합산한 surface CSS 좌표를 넣었고 physical pixel은 실행 metadata에서 계산하도록 했다.
- command buffer readback은 mapping 완료 때까지 render queue를 지연시킬 수 있다. 이 비용은 진단 fixture 한 번의 한계이며 성능 결과로 쓰지 않도록 계획에 한정했다.
- wgpu Metal backend source에서 표면이 `COPY_SRC`를 capability로 제공하는 구현 근거를 확인했지만, 실제 대상 iOS Simulator의 capability는 실행 때 재검증한다. 플랫폼 소스의 capability만으로 runtime availability를 통과 처리하지 않는다.

**판정:** 계획의 범위와 비교 입력은 구현을 시작할 수 있을 만큼 구체적이다. Metal runtime capability 및 실제 copy/mapping은 아직 실행 근거가 아니며 다음 구현 검토에서 새로 공격한다.
