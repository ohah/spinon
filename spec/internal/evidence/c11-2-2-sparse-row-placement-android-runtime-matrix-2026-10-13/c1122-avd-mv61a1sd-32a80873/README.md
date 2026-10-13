# C11.2.2 모바일 runtime geometry 행렬

- run ID: `c1122-avd-mv61a1sd-32a80873`
- 대상: emulator-5554 · Android 17/API 37 · sdk_gphone16k_arm64
- 앱 SHA-256: `06e44ff415d3cef4f996d5769cb79f1acf734d58aa28cbbb0a7bb077752ad348`
- 기기 정보: [device.json](./device.json)
- Chrome 154 기준 SHA-256: `aa56820f4b80b87ff314547398ff4910e4eec93db65be85c73599c4455db32b8`
- 결과: 성공 11/11 · 기대 거부 5/5
- geometry 최대 절대 오차: 0 CSS px (허용 0.5 CSS px)
- 장면마다 새 앱 process와 HostDocument를 사용했다.
- 거부 장면은 오류 code·NodeId·CSS property, frame 부재, 양수 draw 부재를 확인했다.
- 성공 장면은 source-order NodeId·geometry·paint order·양수 WGPU draw를 확인했다.
- 화면 PNG는 실행 상태 보조 자료다. 표면 pixel 비교: matched (11/11 성공 장면).
- [세부 결과](./comparison.json): 16개 장면의 결과·오류 진단과 51개 표면 표본 RGBA를 포함한다. 장면별 화면 PNG를 보존했다. 빌드·설치 로그도 함께 둔다.
- 이 행렬은 고정 fixture 결과이며 WPT 적합성 또는 전체 CSS Grid 지원을 뜻하지 않는다.
- Pixel matrix fixture SHA-256: `3ff07215493c0e3e13dec4ae399a9e9348aa1871269075a74a9b471a2528b7f6`; 최대 RGBA 채널 오차: 0.
- 표면 표본은 OS compositor 최종 출력이 아닌 renderer surface 결과다.


- AVD 수집은 추적 가능한 임시 runner 변형으로 수행했다. [명령·적용 차이](./matrix-command.log)와 [AVD 수집기 적용 차이](./collector-adaptation.md)를 포함한다. 시나리오·비교·검증 코드는 기존 C11.2.2 수집기와 동일하다. Android AVD는 ANGLE/Vulkan/SwiftShader를 사용했으며 물리 GPU 결과가 아니다.

대표 장면: [definite-first-mixed 화면](./definite-first-mixed-android-avd.png).

보존한 console log는 판독성과 저장소 공백 검사를 위해 줄 끝 공백만 제거했다.
