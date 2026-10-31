# C11.2.3 Grid item `order` 사전 비교

## 판정 대상

현재 C11 Grid adapter는 직접 Grid item의 computed `order`가 0이 아니면 `UnsupportedGridOrder`로 거부한다. CSS Grid 1의 `order`는 Grid item의 auto-placement와 painting order에 적용되고, auto-placement는 order-modified document order로 진행된다. 이 비교는 row sparse auto-placement에서 order 순서가 고정 track 상의 위치를 바꾸는지 본다. 겹치는 item은 없어 painting order를 판정하지 않는다.

## 기준과 독립 기대값

- 기준: [CSS Grid Layout Module Level 1, §6.3 및 §8.5](https://www.w3.org/TR/2025/CRD-css-grid-1-20250326/#order-property). §6.3은 `order`가 auto-placement와 painting order에 적용된다고 하고, §8.5는 auto-placement가 order-modified document order로 동작한다고 정한다.
- 독립 규칙: 세 10×10px item을 order 오름차순으로 배치하되, 같은 order는 원래 문서 순서를 유지한다. 3개의 10px column, row sparse, 0 gap에서 순서가 B, C, A라면 x 좌표는 B=0, C=10, A=20이다. 네 번째 item은 두 번째 implicit row의 x=0에 놓인다.
- 입력: [chrome-order.html](./chrome-order.html), SHA-256 `50c0eddfd845a60b69a2ab500c55e5142ca2aed1f2d757f20077819b37275e9b`.

## Chrome 156 관측

Agent Browser의 HeadlessChrome 156에서 `getComputedStyle()`과 각 direct child의 `getBoundingClientRect()`를 읽었다.

| 장면 | DOM 순서와 computed order | 관측 위치 |
| --- | --- | --- |
| `mixed` | A=2, B=-1, C=0 | A=(20,0), B=(0,0), C=(10,0) |
| `ties` | A=0, B=-1, C=-1, D=0 | A=(20,20), B=(0,20), C=(10,20), D=(0,30) |

Chrome 관측은 독립 순서 계산과 일치한다. [원본 관측 JSON](./chrome-observation.json) · [페이지 캡처](./chrome-order.png).

관측 후 Agent Browser를 닫았고 process table에서 Agent Browser·HeadlessChrome·Chromium 이름의 프로세스가 0개임을 확인했다.

## 한계

이는 browser layout oracle이며 Spinon 결과가 아니다. 고정 CSS, row sparse 자동 배치, 3열, 10×10px 비겹침 item만 관측했다. painting order, row/column dense·column flow, definite placement, span interaction, intrinsic track sizing, style-to-layout admission 및 Android/iOS 화면을 검증하지 않았다.
