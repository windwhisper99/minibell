import cn from "classnames";
import { For } from "solid-js";
import { ROLE_GROUPS, ROLES, JOBS, Role } from "~/utils/jobs";
import { Tooltip } from "./tooltip";

interface IRole {
  name: string;
  type: string;
  jobs: string[];
}

const roles: (
  | { type: "single"; role: IRole }
  | { type: "group"; roles: IRole[] }
)[] = [
  {
    type: "single",
    role: {
      name: "Tank",
      type: "tank",
      jobs: ["pld", "war", "drk", "gnb"],
    },
  },
  {
    type: "group",
    roles: [
      {
        name: "Pure Healer",
        type: "healer",
        jobs: ["whm", "ast"],
      },
      {
        name: "Shield Healer",
        type: "healer",
        jobs: ["sch", "sge"],
      },
    ],
  },
  {
    type: "group",
    roles: [
      {
        name: "Melee",
        type: "dps",
        jobs: ["drg", "mnk", "nin", "sam", "rpr", "vpr"],
      },
      {
        name: "Physical Ranged",
        type: "dps",
        jobs: ["brd", "mch", "dnc"],
      },
      {
        name: "Caster",
        type: "dps",
        jobs: ["blm", "smn", "rdm", "pct"],
      },
    ],
  },
];

function RoleSection(props: {
  role: Role;
  selected: Record<string, boolean>;
  onClick: (jobs: string | string[]) => void;
}) {
  return (
    <div>
      <button
        type="button"
        class="font-semibold text-lg bg-transparent py-2 pr-4"
        onClick={() => props.onClick(props.role.jobs)}
      >
        {props.role.name}
      </button>

      <div class="flex flex-row flex-wrap gap-3 mt-1">
        <For each={props.role.jobs}>
          {(job) => (
            <Tooltip text={JOBS[job].name}>
              {({ ref, setOpen }) => (
                <button
                  type="button"
                  class={cn("job-icon", props.role.group, {
                    active: props.selected[job],
                  })}
                  ref={ref}
                  onClick={() => props.onClick(job)}
                  onMouseEnter={() => setOpen(true)}
                  onMouseLeave={() => setOpen(false)}
                >
                  {job.toUpperCase()}
                </button>
              )}
            </Tooltip>
          )}
        </For>
      </div>
    </div>
  );
}

export function JobsPicker(props: {
  jobs: string[];
  onChanges?: (jobs: string[]) => void;
}) {
  const jobObj = () => {
    return props.jobs.reduce((acc, job) => {
      acc[job] = true;
      return acc;
    }, {} as Record<string, boolean>);
  };

  function jobButtonClick(job: string | string[]) {
    if (props.onChanges) {
      const jobs = Array.isArray(job) ? job : [job];
      const obj = jobObj();

      // If all jobs is selected, deselect all jobs. Else, select all jobs.
      const allSelected = jobs.every((job) => jobObj()[job]);
      if (allSelected) jobs.forEach((job) => (obj[job] = false));
      else jobs.forEach((job) => (obj[job] = true));

      props.onChanges(Object.keys(obj).filter((job) => obj[job] === true));
    }
  }

  return (
    <div class="flex flex-col">
      <For each={Object.keys(ROLE_GROUPS)}>
        {(role) => {
          const roles = ROLE_GROUPS[role].roles;
          if (roles.length === 1) {
            const role = ROLES[roles[0]];

            return (
              <RoleSection
                role={role}
                selected={jobObj()}
                onClick={jobButtonClick}
              />
            );
          } else {
            return (
              <div class="mt-4 flex flex-col flex-wrap md:flex-row gap-y-4">
                <For each={roles.map((r) => ROLES[r])}>
                  {(role) => (
                    <div class="md:w-1/2 even:pl-4">
                      <RoleSection
                        role={role}
                        selected={jobObj()}
                        onClick={jobButtonClick}
                      />
                    </div>
                  )}
                </For>
              </div>
            );
          }
        }}
      </For>
    </div>
  );
}
