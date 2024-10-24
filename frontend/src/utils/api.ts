import { action, redirect } from "@solidjs/router";
import { IDraftEventForm } from "~/components/draft-event-form";
import { get, post } from "./fetch";

export interface IEvent {
  id: string;
  status: string;

  title: string;
  description?: string;

  slots: { jobs: string[] }[];

  startAt: number;
  deadlineAt?: number;
  duration: number;
}

export const draftEventAction = action(async (input: IDraftEventForm) => {
  "use server";

  const response = await post<{ id: string; published: boolean }>(
    { path: "/api/events/draft" },
    input
  );

  if (response.published) throw redirect("/");
  else throw redirect(`/events/create?id=${response.id}`);
}, "draft_event");

export const draftEventQuery = async (id: string) => {
  "use server";
  return await get<IEvent>({ path: `/api/events/draft/${id}` });
};
