export type Role = {
  id: string;
  name: string;
  jobs: string[];
  group: string;
};

export type Job = {
  id: string;
  name: string;
  role: string;
};

export const ROLE_GROUPS: Record<string, { name: string; roles: string[] }> = {
  tank: { name: "Tank", roles: ["tank"] },
  healer: { name: "Healer", roles: ["pure", "shield"] },
  dps: {
    name: "DPS",
    roles: ["melee", "ranged", "caster"],
  },
};

export const ROLES: Record<string, Role> = {
  tank: {
    id: "tank",
    name: "Tank",
    jobs: ["pld", "war", "drk", "gnb"],
    group: "tank",
  },
  pure: {
    id: "pure",
    name: "Pure Healer",
    jobs: ["whm", "sch"],
    group: "healer",
  },
  shield: {
    id: "shield",
    name: "Shield Healer",
    jobs: ["ast", "sge"],
    group: "healer",
  },
  melee: {
    id: "melee",
    name: "Melee DPS",
    jobs: ["drg", "mnk", "nin", "sam", "rpr", "vpr"],
    group: "dps",
  },
  ranged: {
    id: "ranged",
    name: "Ranged DPS",
    jobs: ["brd", "mch", "dnc"],
    group: "dps",
  },
  caster: {
    id: "caster",
    name: "Caster DPS",
    jobs: ["blm", "smn", "rdm", "pct"],
    group: "dps",
  },
};

export const JOBS: Record<string, Job> = {
  pld: {
    id: "pld",
    name: "Paladin",
    role: "tank",
  },
  war: {
    id: "war",
    name: "Warrior",
    role: "tank",
  },
  drk: {
    id: "drk",
    name: "Dark Knight",
    role: "tank",
  },
  gnb: {
    id: "gnb",
    name: "Gunbreaker",
    role: "tank",
  },

  // Pure Healers
  whm: {
    id: "whm",
    name: "White Mage",
    role: "pure",
  },
  sch: {
    id: "sch",
    name: "Scholar",
    role: "pure",
  },

  // Shield Healers
  ast: {
    id: "ast",
    name: "Astrologian",
    role: "shield",
  },
  sge: {
    id: "sge",
    name: "Sage",
    role: "shield",
  },

  // Melees
  drg: {
    id: "drg",
    name: "Dragoon",
    role: "melee",
  },
  mnk: {
    id: "mnk",
    name: "Monk",
    role: "melee",
  },
  nin: {
    id: "nin",
    name: "Ninja",
    role: "melee",
  },
  sam: {
    id: "sam",
    name: "Samurai",
    role: "melee",
  },
  rpr: {
    id: "rpr",
    name: "Reaper",
    role: "melee",
  },
  vpr: {
    id: "vpr",
    name: "Viper",
    role: "melee",
  },

  // Ranged
  brd: {
    id: "brd",
    name: "Bard",
    role: "ranged",
  },
  mch: {
    id: "mch",
    name: "Machinist",
    role: "ranged",
  },
  dnc: {
    id: "dnc",
    name: "Dancer",
    role: "ranged",
  },

  // Casters
  blm: {
    id: "blm",
    name: "Black Mage",
    role: "caster",
  },
  smn: {
    id: "smn",
    name: "Summoner",
    role: "caster",
  },
  rdm: {
    id: "rdm",
    name: "Red Mage",
    role: "caster",
  },
  pct: {
    id: "pct",
    name: "Pictomancer",
    role: "caster",
  },
};
