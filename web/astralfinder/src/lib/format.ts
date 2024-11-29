export function dateTime(date: Date): string {
  const formatter = new Intl.DateTimeFormat("en-US", {
    dateStyle: "medium",
    timeStyle: "short",
  });
  return formatter.format(date);
}
