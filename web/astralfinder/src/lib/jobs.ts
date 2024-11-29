export interface Job {
  name: string;
  role: string;
}

export const jobs: Record<string, Job> = {
  pld: {
    name: "Paladin",
    role: "tank",
  },
  war: {
    name: "Warrior",
    role: "tank",
  },
  drk: {
    name: "Dark Knight",
    role: "tank",
  },
  gnb: {
    name: "Gunbreaker",
    role: "tank",
  },
  whm: {
    name: "White Mage",
    role: "pure_healer",
  },
  sch: {
    name: "Scholar",
    role: "shield_healer",
  },
  ast: {
    name: "Astrologian",
    role: "pure_healer",
  },
  sge: {
    name: "Sage",
    role: "shield_healer",
  },
  mnk: {
    name: "Monk",
    role: "melee",
  },
  drg: {
    name: "Dragoon",
    role: "melee",
  },
  nin: {
    name: "Ninja",
    role: "melee",
  },
  sam: {
    name: "Samurai",
    role: "melee",
  },
  rpr: {
    name: "Reaper",
    role: "melee",
  },
  vpr: {
    name: "Viper",
    role: "melee",
  },
  brd: {
    name: "Bard",
    role: "ranged",
  },
  mch: {
    name: "Machinist",
    role: "ranged",
  },
  dnc: {
    name: "Dancer",
    role: "ranged",
  },

  blm: {
    name: "Black Mage",
    role: "caster",
  },
  smn: {
    name: "Summoner",
    role: "caster",
  },
  rdm: {
    name: "Red Mage",
    role: "caster",
  },
  pct: {
    name: "Pictomancer",
    role: "caster",
  },
};

export interface Combination {
  name: string;
  description: string;
  roles: Record<string, number>;
}
export const combination: Record<string, Combination> = {
  standard_light: {
    name: "Standard Light Party",
    description: "Standard Light Party with 1 tank, 1 healer and 2 dps.",
    roles: {
      tank: 1,
      healer: 1,
      dps: 2,
    },
  },
  standard_full: {
    name: "Standard Full Party",
    description:
      "Standard Full Party with 2 tanks, 1 pure healer, 1 shield healer, 2 melees, 1 ranged and 1 caster.",
    roles: {
      tank: 2,
      pure_healer: 1,
      shield_healer: 1,
      melee: 2,
      ranged: 1,
      caster: 1,
    },
  },
};
