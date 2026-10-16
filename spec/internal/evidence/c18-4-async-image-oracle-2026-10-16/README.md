# C18.4 비동기 image request Chrome oracle

2026-10-16 Agent Browser가 로컬 threaded HTTP fixture를 Chrome 155.0.8059.85에서 실행한 관찰이다. viewport 80×48 CSS px, DPR 1, `<img>` frame 48×32 CSS px, `object-fit: fill`을 사용했다. 네 응답과 PNG bytes 생성 방식은 `oracle/server.py`에 고정했다.

## 관찰 결과

- 500 ms 지연 A(2×2 red)를 요청한 뒤 30 ms 후 20 ms B(1×4 blue)로 `src`를 교체했다. B만 `load`되었고 decode 완료 후 및 A 응답 종료 후에도 src는 `/b-fast.png`, natural size는 1×4, 첫 표본은 `[24,64,240,255]`였다.
- B에서 500 ms 지연 C(4×2 green)로 바꾼 40 ms 뒤에는 `complete=false`, natural size `0×0`이었다. C decode 완료 뒤 natural size는 4×2이고 표본은 `[32,220,80,255]`였다.
- 손상 PNG는 `decode()`가 `EncodingError`로 reject됐고 `error` event를 발생시켰다. `load`는 없었고 완료 상태에서 natural size는 0×0이었다. Chrome UA broken-image icon을 보존한 실패 화면이다.

![Chrome 155 broken-image 화면](./oracle/chrome155-broken.png)
- 손상 응답 뒤 D(2×1 purple)는 다시 `load`되어 표본 `[200,40,220,255]`를 만들었다.
- 별도 auto-size 이미지에서 첫 A pending은 frame 0×0, A decode 뒤는 2×2였다. A에서 C source로 교체한 pending state는 natural size 0×0과 이전 2×2 frame을 기록했고, C decode 뒤 frame은 4×2로 바뀌었다. 아래 pending 화면은 C decode 뒤 A source로 교체한 상태에서 이전 C 픽셀이 사라지고 frame 4×2가 유지됨을 보인다. 이 결과는 성공한 source의 intrinsic size가 교체 decode 완료 때 재레이아웃되는 경로만 고정한다.
- auto-size 이미지에서 실패 후 UA가 선택한 frame은 0×0이 아니며 viewport 사용 가능 너비를 반영했다. 그 auto-size failure fallback은 renderer profile 범위에 넣지 않고 fixed-frame 실패 경계와 분리한다.

![기존 decoded source를 교체한 Chrome 155 pending 화면](./oracle/chrome155-auto-pending-visible.png)

## 파일과 SHA-256

| 파일 | SHA-256 |
| --- | --- |
| `oracle/server.py` | `2ea0dfb0f0d20d8e48e12e833815b59e56d7f0eb1299ace204b5049055b15acd` |
| `oracle/chrome155-server.log` | `9d878a15f1f11743f46ce593fbc06c2d73534d42f337e577962fc622b02b0b83` |
| `oracle/chrome155-observation.json` | `abd0177209e2f30a572f6530a58419adbc614bd3c680fca51ac710d02d667d81` |
| `oracle/chrome155-broken-state.json` | `5f9ba59d5644b96fcd43ce76b0e89d5339979bce7bd9ab911645c9b985fb3060` |
| `oracle/chrome155-broken.png` | `e7a8247eaca2fc5a9293a80c3ab52e3f046829e9912356115cbf4bffa6eaa188` |
| `oracle/auto_server.py` | `102396832352fe42003fe6dd340ce88c4730d66f565eb59eda234418d151ce42` |
| `oracle/chrome155-auto-observation.json` | `9d296385a369a442d1440ad54cb28924ed564defd9458bd8dc9f0416201059b7` |
| `oracle/chrome155-auto-broken-state.json` | `b36be7f80fc7eea6f411d48235de8e923eb46d23d1311fdc8138e8013c4820ff` |
| `oracle/chrome155-auto-pending-visible-state.json` | `f8fabc4f16515d2b5acb58c55a5bc5aff976f4dc484acf1f64ca66f55e9283ca` |
| `oracle/chrome155-auto-pending-visible.png` | `379a4143141e630c97a8faeec10fd4299a883b80b8f2a56e7b0c9c8712bc3aa9` |

이 자료는 HTML request lifecycle 비교다. 제품 renderer의 실패 icon 표현, URL/network loader, browser events와 공개 image 지원을 구현 또는 검증했다고 뜻하지 않는다.
