import { cache, redirect } from "@solidjs/router";
import CraftEventForm from "~/components/draft-event-form";
import { isLogin } from "~/utils/fetch";

const createEvent = cache(async () => {
  "use server";
  if (!isLogin()) throw redirect("/");
  return true;
}, "create_event");

export const route = {
  preload: () => createEvent(),
};

export default function Page() {
  return (
    <div class="max-w-3xl mx-auto">
      <h1 class="text-2xl font-semibold mt-6">Create Event</h1>

      <CraftEventForm class="mt-6" />
    </div>
  );
}
