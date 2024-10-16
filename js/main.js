import Alpine from "alpinejs";
import "./json_enc";
import "./job_selector";
import "./components";

Alpine.directive("datetime-value", (el, { expression }, { evaluate }) => {
  if (!expression || !expression.length) {
    return;
  }

  function addZero(i) {
    if (i < 10) {
      i = "0" + i;
    }
    return String(i);
  }

  const d = new Date(Number(expression));
  const date = [
    d.getFullYear().toString(),
    addZero(d.getMonth() + 1),
    addZero(d.getDate()),
  ];
  const time = [addZero(d.getHours()), addZero(d.getMinutes())];

  const value = `${date.join("-")}T${time.join(":")}`;
  el.value = value;
});

window.Alpine = Alpine;
Alpine.start();
