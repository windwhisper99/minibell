export function datetimeLocalFormat(value: number): string {
  if (value === 0) return "";

  function addZero(i: number) {
    if (i < 10) return "0" + i;
    return String(i);
  }

  const d = new Date(value);
  const date = [
    d.getFullYear().toString(),
    addZero(d.getMonth() + 1),
    addZero(d.getDate()),
  ];
  const time = [addZero(d.getHours()), addZero(d.getMinutes())];

  return `${date.join("-")}T${time.join(":")}`;
}
