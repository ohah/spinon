import { h, ref } from "@vue/runtime-core";

export function createVueCounter() {
  const count = ref(0);

  return {
    render() {
      return h("main", null, [
        h(
          "button",
          {
            "aria-label": "count",
            onClick() {
              count.value += 1;
            },
          },
          `count: ${count.value}`,
        ),
      ]);
    },
  };
}
