import { action } from "@solidjs/router";

export const draftEventAction = action(async (data) => {
  "use server";

  // Delay for 2 seconds
  await new Promise((resolve) => setTimeout(resolve, 2000));

  console.log(data);
}, "draft_event");
