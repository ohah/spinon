# C11.3.3 존재하지 않는 이름의 implicit line 사전 비교

**상태:** 구현 전 비교 근거. Chrome oracle·Agent Browser 관찰이며 Spinon 구현 결과나 WPT 적합성 결과가 아니다.
**계획:** [C11.3.3 unknown named area implicit lines](../../../../plan/c11-3-3-unknown-named-area-implicit-lines.md)
**기준 JSON:** [Chrome 154 사전 비교](../../../../tests/fixtures/css/references/c11-3-3-unknown-named-area-implicit-lines-precomparison-v1.json)

## 입력·규범·WPT

- 규범: [CSS Grid Layout Module Level 1 · 2025-03-26 CRD §8.3](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#line-placement), §7.5, §8.3.1.
- WPT revision: `c999c58338ee1d223df5ad62e48be1202ced0d37`.
- WPT file: `css/css-grid/placement/grid-placement-unknown-named-area-001.html`.
- WPT SHA-256: `7fa795ad262e74399487fa817da1e99a75bbcce2e46b5f4547d55959e2e9ebd4`. Pinned raw file을 Agent Browser로 읽어 계산한 해시와 계획에 적힌 해시가 일치했다.
- WPT 네 subcase geometry를 별도 고정 HTML fixture로 옮겼다. WPT 원문이나 WPT runner를 실행한 것으로 세지 않는다.
- 추가 `unknown-template-area-with-sibling` case는 기존 C11.3.3 unknown bare area를 fixed `grid-auto-*`로 제한해 area sibling과 함께 둔 Spinon 파생 control이다.
- Fixture: [`unknown-named-area-implicit-lines-c1133.html`](../../../../tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133.html) · [inventory](../../../../tests/fixtures/css/c11/unknown-named-area-implicit-lines-c1133-inventory.json).

## Oracle 환경

- Chrome for Testing `154.0.8037.92`, CDP revision `@334b65d254ccc35df4fca82706d1753227b01039`.
- 실행 파일 SHA-256: `ce0c09ea98a7592e7799e54c1867797ccf99d4d6e821a11067c332e454732349`.
- Host: macOS `25.5.0`, arm64, Node `v24.19.0`.
- 실행: `SPINON_CHROMIUM_BIN='<pinned Chrome 154 path>' node tools/css-reference/capture-c11-3-3-unknown-named-area-precomparison.mjs`.
- Capture run: `c1133-unknown-name-chrome-20261020T141317806Z-791f700e`.
- Browser helper가 임시 process group 및 profile을 만들고 실행 종료 뒤 Chromium process group을 닫았다. 실행 후 Chrome for Testing 또는 Agent Browser 잔류 프로세스는 없었다.
- Viewport `320×240 CSS px`, DPR 1·2, LTR, `horizontal-tb`, locale `en-US`, timezone `America/Los_Angeles`, light scheme, forced colors `none`, pointer `fine`, hover `hover`.

## 결과

| Case | Chrome CSSOM columns | Chrome CSSOM rows | root-relative target rect `(x,y,w,h)` | WPT geometry |
|---|---|---|---|---|
| `unknown-both-named-row` | `30px 40px` | `10px 20px 5px 5px` | `(0,35,30,5)` | 일치 |
| `unknown-column-named-span` | `30px 40px 15px 15px 15px 15px` | `10px 20px` | `(85,0,45,10)` | 일치 |
| `unknown-row-named-span` | `30px 40px` | `10px 20px 5px 5px 5px 5px 5px 5px` | `(0,35,30,25)` | 일치 |
| `unknown-column-span-one` | `30px 40px 15px 15px` | `10px 20px` | `(85,0,15,10)` | 일치 |
| `unknown-template-area-with-sibling` | `80px 80px 15px 15px` | `60px 60px 5px 5px` | `(175,125,15,5)` | C11.3.3 파생 control |

각 geometry는 DPR 1·2에서 같았다. 전체 root·item computed properties와 rect는 reference JSON에 저장했다. capture 도구의 첫 실행은 DPR 값 자체까지 같아야 한다고 잘못 비교해 중단했다. DPR field만 제외해 CSSOM·CSS px geometry를 비교하도록 도구를 고쳤으며 첫 실패 실행은 reference나 PNG를 쓰지 않았다. 위 run은 수정한 capture 도구로 다시 실행해 성공한 결과다.

## 화면·overflow 확인

WPT 원문은 요청된 Agent Browser에서 `wpt.live`를 열어 네 item 위치와 CSSOM track 증가를 확인했다. 브라우저는 `HeadlessChrome/155.0.0.0`이었으므로 이 결과는 fixed Chrome 154 oracle과 구분한다. Agent Browser에서 본 WPT 네 geometry는 `data-expected-*`와 일치했다. Agent Browser session과 프로세스는 확인 뒤 닫았다.

파생 area control의 root는 `(0,0,160,120)`이고 unknown item은 `(175,125,15,5)`이므로 root 오른쪽·아래 경계를 넘는다. Chrome 154 baseline은 아래 두 DPR screenshot에 보존되어 있다. Agent Browser로 동일 fixture를 열어 해당 scene을 활성화했을 때 item의 red fill이 root의 dark background 바깥에서 보이는 것과 root-relative geometry를 확인했다. 화면에서 확인한 item는 `border:0` fixture라 border stroke 근거로 세지 않는다.

- DPR 1: [unknown-template-area-with-sibling.png](./c1133-unknown-name-chrome-20261020T141317806Z-791f700e/unknown-template-area-with-sibling-dpr1.png)
- DPR 2: [unknown-template-area-with-sibling DPR 2.png](./c1133-unknown-name-chrome-20261020T141317806Z-791f700e/unknown-template-area-with-sibling-dpr2.png)
- Root 내부 sample `(150,110)`은 `#101820`, root 밖 item sample `(180,127)`은 `#e5484d`, 둘 사이 `(170,127)`은 page background여야 한다. Android API 37 ARM64 AVD PixelCopy와 iPhone 17 Pro/iOS 26.2 Simulator Metal drawable에서 이 세 sample을 별도로 확인한다. 이 plan은 아직 simulator 결과를 주장하지 않는다.

이미지 10개의 SHA-256과 fixture·inventory·capture tool·helper hash는 versioned Chrome reference에 있다. Screenshot은 browser drawable 캡처이고 OS compositor 이후 화면을 뜻하지 않는다.
