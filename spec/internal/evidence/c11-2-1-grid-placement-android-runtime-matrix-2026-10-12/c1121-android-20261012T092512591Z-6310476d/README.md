# C11.2.1 모바일 runtime matrix

- run ID: `c1121-android-20261012T092512591Z-6310476d`
- target: emulator-5554 · qemu=1 · API 37
- app SHA-256: `0a42647465a10c7917ce3b635b845a9813e475da37c42bb67dbe000f741457a6`
- inventory SHA-256: `b2801e708e4c1ad540e119dcdba04d7c215e4029d5ec5627683e15ede8ef368b`
- HTML SHA-256: `e9b080bbc65901b17f7dead60eb8071df8ecbefee2ea62f1318776848e75f78d`
- Chrome 154 reference SHA-256: `5bbbd041e2e59fda32c684f7290f61bff41ab6dda9ab9f77ce2fb36f0e0ee034`
- 결과: 성공 6/6 · 기대 거부 4/4
- surface pixel: 6/6 scene · 20/20 sample exact · 최대 channel 오차 0
- geometry 최대 절대 오차: 0 CSS px (허용 0.5 CSS px)
- scene마다 새 앱 process와 HostDocument를 사용했다.
- 거부 장면은 정확한 code·NodeId·property를 확인하고, 비어 있지 않은 WGPU 제출과 frame을 거부했다.
- 화면 PNG는 실행 장면을 확인하는 보조 자료다. 표면 픽셀 비교는 성공 scene의 PixelCopy PNG 또는 Metal drawable readback만 사용했다.
- [세부 결과](./comparison.json). 각 scene의 process-filtered runtime log와 PNG를 함께 보존했다.
- 네 기대 거부 장면은 정확한 오류·partial frame 없음·non-empty WGPU present 없음으로 별도 확인하며 Chrome과 pixel parity를 주장하지 않는다.
- Android 실기기, Spinon WPT 적합성 및 전체 CSS Grid 지원은 별도 범위다.

