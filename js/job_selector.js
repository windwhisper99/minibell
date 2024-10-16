import Alpine from "alpinejs";

function normalizeJobs(jobs) {
  const list = Array.isArray(jobs) ? jobs : jobs.split(",");
  return list.reduce((acc, key) => {
    acc[key] = true;
    return acc;
  }, {});
}

function listJobs(selected) {
  return Object.keys(selected).filter((key) => selected[key]);
}

Alpine.data("jobSelector", (initJobs) => ({
  selected: {},
  update(jobs) {
    if (jobs) this.selected = normalizeJobs(jobs);
    else this.selected = {};
  },
  init() {
    if (initJobs) this.selected = normalizeJobs(initJobs);
  },

  jobButton(job) {
    return {
      ["@click"](evt) {
        this.selected[job] = !this.selected[job];

        evt.target.dispatchEvent(
          new CustomEvent("jobschange", {
            detail: listJobs(this.selected),
            bubbles: true,
          })
        );
      },
      [":class"]() {
        return this.selected[job] ? "active" : "";
      },
    };
  },

  roleButton(roleJobs) {
    const jobs = Array.isArray(roleJobs) ? roleJobs : roleJobs.split(",");

    return {
      ["@click"](evt) {
        // If all jobs are selected, deselect all role jobs
        const allSelected = jobs.every((key) => this.selected[key]);

        if (allSelected) {
          jobs.forEach((key) => {
            this.selected[key] = false;
          });
        } else {
          jobs.forEach((key) => {
            this.selected[key] = true;
          });
        }

        evt.target.dispatchEvent(
          new CustomEvent("jobschange", {
            detail: listJobs(this.selected),
            bubbles: true,
          })
        );
      },
    };
  },
}));
