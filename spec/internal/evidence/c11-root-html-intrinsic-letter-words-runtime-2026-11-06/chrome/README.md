# ASCII letter-word overflow sample · Chrome Agent Browser 관측

고정 [Root HTML runtime fixture](../../../../../tests/fixtures/css/c11/root-html-grid-intrinsic-letter-words-runtime.html)를 Agent Browser가 연 HeadlessChrome 157.0.0.0에서 360×800 CSS px, DPR 1로 표시했다. [화면](./runtime-360x800.png)의 SHA-256은 `e3cff334a3b8b9cb97c1ab74fcca75608915ba2557dabcdd3217b2e72f3d68cb`다.

DOM geometry는 Grid `[0,0,40,40]px`, `AAAAAAAA` item `[0,10,48.171875,10]px`다. 따라서 CSS x=42px은 40px Grid의 right edge 바깥이면서 text item 내부다. PNG의 `(42,15)`은 glyph antialias와 겹쳐 `[90,165,241,255]`였고, `(42,19)`는 text item 배경색 `[91,168,245,255]`였다. 표본은 row 중앙에서 glyph를 피할 수 없으므로 같은 row 하단 내부 `(42px,19px)`로 정했다.

이 자료는 Chrome 기준 장면의 좌표·색 관측이다. Spinon runtime 결과를 뜻하지 않는다.
