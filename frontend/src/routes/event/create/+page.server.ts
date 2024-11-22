import { get } from "$lib/fetch";
import type { ServerLoad } from "@sveltejs/kit";

export interface Category {
  id: string;
  name: string;
  parent?: string;
}

export interface Duty {
  id: string;
  name: string;
  image: string;
  shortName?: string;
  patch: string;
}

export interface DutiesResult {
  breadcrumbs: Category[];
  categories?: Category[];
  duties?: Duty[];
}

export interface DutyResult {
  breadcrumbs: Category[];
  duty: Duty & {
    pharses: { name: string; progression: number }[];
  };
}

export const load: ServerLoad = async (event) => {
  const categoryQuery = event.url.searchParams.get("category");
  const dutyQuery = event.url.searchParams.get("duty");

  const duties =
    dutyQuery === null
      ? await get<DutiesResult>(event, {
          path: "/duties",
          query: { category: categoryQuery },
        })
      : null;

  const duty =
    dutyQuery !== null
      ? await get<DutyResult>(event, { path: `/duties/${dutyQuery}` })
      : null;

  return {
    duties,
    duty,
  };
};
