use serde::Deserialize;
use std::{
    env,
    fs::File,
    io::{self, prelude::*, BufReader},
    process,
};

#[derive(Deserialize, Debug)]
struct Oddjobs {
    oddjobs: Vec<Oddjob>,
}

#[derive(Deserialize, Debug)]
struct Oddjob {
    name: String,
    aliases: Vec<String>,
    progressions: Vec<Vec<u32>>,
    location: Option<String>,
    primary_stats: Vec<String>,
    secondary_stats: Vec<String>,
    stat_constraints: Vec<String>,
    attacks: Vec<u32>,
    notable_skills: Vec<u32>,
    notable_equips: Vec<u32>,
    notes: Vec<String>,
}

static PREAMBLE: &str = "# Odd jobs

Note that this list is inherently incomplete, because new odd jobs could
potentially be invented. Also note that the \u{201c}Stat constraints\u{201d}
reference *base* stats/abilities; that is, the stats themselves without any
bonuses from equipment or buffs.

The \u{201c}Notable equipment\u{201d} for each job intentionally excludes
equipment items that are not particularly unique to the job. For example, many
of the same weapons (those swords, axes, blunt weapons, spears, polearms, and
daggers that lack job requirements) are used by almost all melee-oriented odd
jobs (e.g. permabeginner) in common. As a result, unless these weapons are
useful for other reasons that are somewhat special to the job in question, they
are not listed under \u{201c}Notable equipment\u{201d}.

";

fn main() {
    let mut input_filename = String::new();

    for (arg_i, arg) in env::args().enumerate() {
        match arg_i {
            0 => (),
            1 => input_filename = arg,
            _ => {
                eprintln!(
                    "Expected just one argument: the filename to read in",
                );

                process::exit(1)
            }
        }
    }

    if input_filename.is_empty() {
        eprintln!("Expected at least one argument: the filename to read in");

        process::exit(2)
    }

    let input_file = File::open(input_filename)
        .expect("Could not open input file for reading");
    let input_reader = BufReader::new(input_file);

    let deserialized: Oddjobs = serde_json::from_reader(input_reader)
        .expect("Error deserializing JSON input");
    let oddjobs = deserialized.oddjobs;

    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();

    stdout_handle.write_all(PREAMBLE.as_bytes()).unwrap();

    for oddjob in &oddjobs {
        writeln!(
            stdout_handle,
            "- [{}](#{})",
            oddjob.name,
            slugify(&oddjob.name),
        )
        .unwrap();
    }

    writeln!(stdout_handle).unwrap();

    for oddjob in oddjobs {
        writeln!(stdout_handle, "## {}\n", oddjob.name).unwrap();

        if let Some((last_alias, aliases)) = oddjob.aliases.split_last() {
            stdout_handle.write_all(b"Also known as: ").unwrap();

            for alias in aliases {
                write!(stdout_handle, "\u{201c}{}\u{201d}, ", alias).unwrap();
            }

            writeln!(stdout_handle, "\u{201c}{}\u{201d}\n", last_alias)
                .unwrap();
        }

        stdout_handle
            .write_all(b"Possible job progressions:\n\n")
            .unwrap();

        for prog in oddjob.progressions {
            if let Some((last_job, jobs)) = prog.split_last() {
                stdout_handle.write_all(b"- ").unwrap();

                for job in jobs {
                    write!(
                        stdout_handle,
                        "{} \u{2192} ",
                        job_name(*job).unwrap_or_else(|| {
                            eprintln!("Invalid job ID: {}", job);

                            process::exit(4)
                        }),
                    )
                    .unwrap();
                }

                writeln!(
                    stdout_handle,
                    "{}",
                    job_name(*last_job).unwrap_or_else(|| {
                        eprintln!("Invalid job ID: {}", last_job);

                        process::exit(4)
                    }),
                )
                .unwrap();
            } else {
                eprintln!("Empty progression for {}", oddjob.name);

                process::exit(3)
            }
        }

        writeln!(stdout_handle).unwrap();

        if let Some(location) = oddjob.location {
            writeln!(stdout_handle, "Location: {}\n", location).unwrap();
        }

        if let Some((last_primary_stat, primary_stats)) =
            oddjob.primary_stats.split_last()
        {
            write!(
                stdout_handle,
                "Primary stat{}: ",
                if primary_stats.is_empty() { "" } else { "s" },
            )
            .unwrap();

            for primary_stat in primary_stats {
                write!(stdout_handle, "**{}**, ", primary_stat).unwrap();
            }

            write!(stdout_handle, "**{}**", last_primary_stat).unwrap();
        }

        if let Some((last_secondary_stat, secondary_stats)) =
            oddjob.secondary_stats.split_last()
        {
            write!(
                stdout_handle,
                " | Secondary stat{}: ",
                if secondary_stats.is_empty() { "" } else { "s" },
            )
            .unwrap();

            for secondary_stat in secondary_stats {
                write!(stdout_handle, "{}, ", secondary_stat).unwrap();
            }

            write!(stdout_handle, "{}", last_secondary_stat).unwrap();
        }

        writeln!(stdout_handle, "\n").unwrap();

        if !oddjob.stat_constraints.is_empty() {
            writeln!(stdout_handle, "Stat constraints:\n").unwrap();

            for stat_constraint in oddjob.stat_constraints {
                writeln!(
                    stdout_handle,
                    "- {}",
                    stat_constraint.replace(" ", "\u{00a0}"),
                )
                .unwrap();
            }

            writeln!(stdout_handle).unwrap();
        }

        writeln!(stdout_handle, "Attacks:\n").unwrap();

        for attack in oddjob.attacks {
            writeln!(
                stdout_handle,
                "- {}",
                skill_name(attack).unwrap_or_else(|| {
                    eprintln!("Invalid skill ID: {}", attack);

                    process::exit(5)
                }),
            )
            .unwrap();
        }

        writeln!(stdout_handle).unwrap();

        if !oddjob.notable_skills.is_empty() {
            writeln!(stdout_handle, "Notable skills:\n").unwrap();

            for skill in oddjob.notable_skills {
                writeln!(
                    stdout_handle,
                    "- {}",
                    skill_name(skill).unwrap_or_else(|| {
                        eprintln!("Invalid skill ID: {}", skill);

                        process::exit(5)
                    }),
                )
                .unwrap();
            }

            writeln!(stdout_handle).unwrap();
        }

        if !oddjob.notable_equips.is_empty() {
            writeln!(stdout_handle, "Notable equipment:\n").unwrap();

            for equip in oddjob.notable_equips {
                writeln!(
                    stdout_handle,
                    "- {}",
                    item_name(equip).unwrap_or_else(|| {
                        eprintln!("Unknown item ID: {}", equip);

                        process::exit(6)
                    })
                )
                .unwrap();
            }

            writeln!(stdout_handle).unwrap();
        }

        if !oddjob.notes.is_empty() {
            writeln!(stdout_handle, "### Notes\n").unwrap();

            for note in oddjob.notes {
                writeln!(stdout_handle, "{}\n", note).unwrap();
            }
        }
    }
}

/// Kind of a hack.  I'm not really sure exactly how slugification of arbitrary
/// Unicode strings is "supposed" to work, especially considering that there is
/// no *single* such thing; everyone does it differently.
///
/// Also, uh, normalizing Latin letters to their closest ASCII equivalents
/// ("getting rid of accents") isn't handled in the Rust stdlib, so this
/// function won't even try to do that.
fn slugify(s: &str) -> String {
    let mut slug = String::with_capacity(s.len());

    for c in s.chars() {
        if c.is_whitespace() || c == '-' {
            slug.push('-');
        } else if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
        }
    }

    slug
}

fn job_name(id: u32) -> Option<&'static str> {
    Some(match id {
        0 => "Beginner",
        100 => "Warrior",
        110 => "Fighter",
        111 => "Crusader",
        112 => "Hero",
        120 => "Page",
        121 => "White Knight",
        122 => "Paladin",
        130 => "Spearman",
        131 => "Dragon Knight",
        132 => "Dark Knight",
        200 => "Magician",
        210 => "F/P Wizard",
        211 => "F/P Mage",
        212 => "F/P Archmage",
        220 => "I/L Wizard",
        221 => "I/L Mage",
        222 => "I/L Archmage",
        230 => "Cleric",
        231 => "Priest",
        232 => "Bishop",
        300 => "Archer",
        310 => "Hunter",
        311 => "Ranger",
        312 => "Bowmaster",
        320 => "Crossbowman",
        321 => "Sniper",
        322 => "Marksman",
        400 => "Rogue",
        410 => "Assassin",
        411 => "Hermit",
        412 => "Nightlord",
        420 => "Bandit",
        421 => "Chief Bandit",
        422 => "Shadower",
        500 => "Pirate",
        510 => "Brawler",
        511 => "Marauder",
        512 => "Buccaneer",
        520 => "Gunslinger",
        521 => "Outlaw",
        522 => "Corsair",
        _ => return None,
    })
}

fn skill_name(id: u32) -> Option<&'static str> {
    Some(match id {
        0 => "[basic attack]",
        8 => "Follow the Lead",
        1000 => "Three Snails",
        1001 => "Recovery",
        1003 => "Legendary Spirit",
        1004 => "Monster Rider",
        1002 => "Nimble Feet",
        1000000 => "Improved HP Recovery",
        1000001 => "Improved MaxHP Increase",
        1000002 => "Endure",
        1001003 => "Iron Body",
        1001004 => "Power Strike",
        1001005 => "Slash Blast",
        2000000 => "Improved MP Recovery",
        2000001 => "Improved MaxMP Increase",
        2001002 => "Magic Guard",
        2001003 => "Magic Armor",
        2001004 => "Energy Bolt",
        2001005 => "Magic Claw",
        3000000 => "The Blessing of Amazon",
        3000001 => "Critical Shot",
        3000002 => "The Eye of Amazon",
        3001003 => "Focus",
        3001004 => "Arrow Blow",
        3001005 => "Double Shot",
        4000000 => "Nimble Body",
        4000001 => "Keen Eyes",
        4001002 => "Disorder",
        4001003 => "Dark Sight",
        4001334 => "Double Stab",
        4001344 => "Lucky Seven",
        1100000 => "Sword Mastery",
        1100001 => "Axe Mastery",
        1100002 => "Final Attack: Sword",
        1100003 => "Final Attack: Axe",
        1101004 => "Sword Booster",
        1101005 => "Axe Booster",
        1101006 => "Rage",
        1101007 => "Power Guard",
        1200000 => "Sword Mastery",
        1200001 => "BW Mastery",
        1200002 => "Final Attack: Sword",
        1200003 => "Final Attack: BW",
        1201004 => "Sword Booster",
        1201005 => "BW Booster",
        1201006 => "Threaten",
        1201007 => "Power Guard",
        1300000 => "Spear Mastery",
        1300001 => "Pole Arm Mastery",
        1300002 => "Final Attack: Spear",
        1300003 => "Final Attack: Pole Arm",
        1301004 => "Spear Booster",
        1301005 => "Pole Arm Booster",
        1301006 => "Iron Will",
        1301007 => "Hyper Body",
        2100000 => "MP Eater",
        2101001 => "Meditation",
        2101002 => "Teleport",
        2101003 => "Slow",
        2101004 => "Fire Arrow",
        2101005 => "Poison Breath",
        2200000 => "MP Eater",
        2201001 => "Meditation",
        2201002 => "Teleport",
        2201003 => "Slow",
        2201004 => "Cold Beam",
        2201005 => "Thunder Bolt",
        2300000 => "MP Eater",
        2301001 => "Teleport",
        2301002 => "Heal",
        2301003 => "Invincible",
        2301004 => "Bless",
        2301005 => "Holy Arrow",
        3100000 => "Bow Mastery",
        3100001 => "Final Attack: Bow",
        3101002 => "Bow Booster",
        3101003 => "Power Knock-Back",
        3101004 => "Soul Arrow: Bow",
        3101005 => "Arrow Bomb",
        3200000 => "Crossbow Mastery",
        3200001 => "Final Attack: Crossbow",
        3201002 => "Crossbow Booster",
        3201003 => "Power Knock-Back",
        3201004 => "Soul Arrow: Crossbow",
        3201005 => "Iron Arrow",
        4100000 => "Claw Mastery",
        4100001 => "Critical Throw",
        4100002 => "Endure",
        4101003 => "Claw Booster",
        4101004 => "Haste",
        4101005 => "Drain",
        4200000 => "Dagger Mastery",
        4200001 => "Endure",
        4201002 => "Dagger Booster",
        4201003 => "Haste",
        4201004 => "Steal",
        4201005 => "Savage Blow",
        1110000 => "Improving MP Recovery",
        1110001 => "Shield Mastery",
        1111002 => "Combo Attack",
        1111003 => "Panic: Sword",
        1111004 => "Panic: Axe",
        1111005 => "Coma: Sword",
        1111006 => "Coma: Axe",
        1111007 => "Armor Crash",
        1111008 => "Shout",
        1210000 => "Improving MP Recovery",
        1210001 => "Shield Mastery",
        1211002 => "Charged Blow",
        1211003 => "Fire Charge: Sword",
        1211004 => "Flame Charge: BW",
        1211005 => "Ice Charge: Sword",
        1211006 => "Blizzard Charge: BW",
        1211007 => "Thunder Charge: Sword",
        1211008 => "Lightning Charge: BW",
        1211009 => "Magic Crash",
        1310000 => "Elemental Resistance",
        1311001 => "Spear Crusher",
        1311002 => "Pole Arm Crusher",
        1311003 => "Dragon Fury: Spear",
        1311004 => "Dragon Fury: Pole Arm",
        1311005 => "Sacrifice",
        1311006 => "Dragon Roar",
        1311007 => "Power Crash",
        1311008 => "Dragon Blood",
        2110000 => "Partial Resistance",
        2110001 => "Element Amplification",
        2111002 => "Explosion",
        2111003 => "Poison Mist",
        2111004 => "Seal",
        2111005 => "Spell Booster",
        2111006 => "Element Composition [F/P]",
        2210000 => "Partial Resistance",
        2210001 => "Element Amplification",
        2211002 => "Ice Strike",
        2211003 => "Thunder Spear",
        2211004 => "Seal",
        2211005 => "Spell Booster",
        2211006 => "Element Composition [I/L]",
        2310000 => "Elemental Resistance",
        2311001 => "Dispel",
        2311002 => "Mystic Door",
        2311003 => "Holy Symbol",
        2311004 => "Shining Ray",
        2311005 => "Doom",
        2311006 => "Summon Dragon",
        3110000 => "Thrust",
        3110001 => "Mortal Blow",
        3111002 => "Puppet",
        3111003 => "Inferno",
        3111004 => "Arrow Rain",
        3111005 => "Silver Hawk",
        3111006 => "Strafe",
        3210000 => "Thrust",
        3210001 => "Mortal Blow",
        3211002 => "Puppet",
        3211003 => "Blizzard",
        3211004 => "Arrow Eruption",
        3211005 => "Golden Eagle",
        3211006 => "Strafe",
        4110000 => "Alchemist",
        4111001 => "Meso Up",
        4111002 => "Shadow Partner",
        4111003 => "Shadow Web",
        4111004 => "Shadow Meso",
        4111005 => "Avenger",
        4111006 => "Flash Jump",
        4210000 => "Shield Mastery",
        4211001 => "Chakra",
        4211002 => "Assaulter",
        4211003 => "Pickpocket",
        4211004 => "Band of Thieves",
        4211005 => "Meso Guard",
        4211006 => "Meso Explosion",
        9001000 => "Haste (Normal)",
        9001001 => "Super Dragon Roar",
        9001002 => "Teleport",
        9101000 => "Heal + Dispel",
        9101001 => "Haste (Super)",
        9101002 => "Holy Symbol",
        9101003 => "Bless",
        9101004 => "Hide",
        9101005 => "Resurrection",
        9101006 => "Super Dragon Roar",
        9101007 => "Teleport",
        9101008 => "Hyper Body",
        1005 => "Echo of Hero",
        1121000 => "Maple Warrior",
        1121001 => "Monster Magnet",
        1121002 => "Power Stance",
        1120003 => "Advanced Combo Attack",
        1120004 => "Achilles",
        1120005 => "Guardian",
        1121006 => "Rush",
        1121008 => "Brandish",
        1121010 => "Enrage",
        1121011 => "Hero\u{2019}s Will",
        1221000 => "Maple Warrior",
        1221001 => "Monster Magnet",
        1221002 => "Power Stance",
        1221003 => "Holy Charge: Sword",
        1221004 => "Divine Charge: BW",
        1220005 => "Achilles",
        1220006 => "Guardian",
        1221007 => "Rush",
        1221009 => "Blast",
        1220010 => "Advanced Charge",
        1221011 => "Heaven\u{2019}s Hammer",
        1221012 => "Hero\u{2019}s Will",
        2121000 => "Maple Warrior",
        2121001 => "Big Bang",
        2121002 => "Mana Reflection",
        2121003 => "Fire Demon",
        2121004 => "Infinity",
        2121005 => "Ifrit",
        2121006 => "Paralyze",
        2121007 => "Meteor Shower",
        2121008 => "Hero\u{2019}s Will",
        2221000 => "Maple Warrior",
        2221001 => "Big Bang",
        2221002 => "Mana Reflection",
        2221003 => "Ice Demon",
        2221004 => "Infinity",
        2221005 => "Elquines",
        2221006 => "Chain Lightning",
        2221007 => "Blizzard",
        2221008 => "Hero\u{2019}s Will",
        2321000 => "Maple Warrior",
        2321001 => "Big Bang",
        2321002 => "Mana Reflection",
        2321003 => "Bahamut",
        2321004 => "Infinity",
        2321005 => "Holy Shield",
        2321006 => "Resurrection",
        2321007 => "Angel Ray",
        2321008 => "Genesis",
        2321009 => "Hero\u{2019}s Will",
        3121000 => "Maple Warrior",
        3121002 => "Sharp Eyes",
        3121003 => "Dragon\u{2019}s Breath",
        3121004 => "Hurricane",
        3120005 => "Bow Expert",
        3121006 => "Phoenix",
        3121007 => "Hamstring",
        3121008 => "Concentrate",
        3121009 => "Hero\u{2019}s Will",
        3221000 => "Maple Warrior",
        3221001 => "Piercing Arrow",
        3221002 => "Sharp Eyes",
        3221003 => "Dragon\u{2019}s Breath",
        3220004 => "Marksman Boost",
        3221005 => "Frostprey",
        3221006 => "Concentrate",
        3221007 => "Snipe",
        3221008 => "Hero\u{2019}s Will",
        4121000 => "Maple Warrior",
        4120002 => "Shadow Shifter",
        4121003 => "Taunt",
        4121004 => "Ninja Ambush",
        4120005 => "Venomous Star",
        4121006 => "Shadow Stars",
        4121007 => "Triple Throw",
        4121008 => "Ninja Storm",
        4121009 => "Hero\u{2019}s Will",
        4221000 => "Maple Warrior",
        4221001 => "Assassinate",
        4220002 => "Shadow Shifter",
        4221003 => "Taunt",
        4221004 => "Ninja Ambush",
        4220005 => "Venomous Stab",
        4221006 => "Smokescreen",
        4221007 => "Boomerang Step",
        4221008 => "Hero\u{2019}s Will",
        1006 => "Jump Down",
        5000000 => "Bullet Time",
        5001001 => "Flash Fist",
        5001002 => "Sommersault Kick",
        5001003 => "Double Shot",
        5001005 => "Dash",
        5100000 => "Improve MaxHP",
        5100001 => "Knuckler Mastery",
        5101002 => "Backspin Blow",
        5101003 => "Double Uppercut",
        5101004 => "Corkscrew Blow",
        5101005 => "MP Recovery",
        5101006 => "Knuckler Booster",
        5101007 => "Oak Barrel",
        5200000 => "Gun Mastery",
        5201001 => "Invisible Shot",
        5201002 => "Grenade",
        5201003 => "Gun Booster",
        5201004 => "Blank Shot",
        5201005 => "Wings",
        5201006 => "Recoil Shot",
        5110000 => "Stun Mastery",
        5110001 => "Energy Charge",
        5111002 => "Energy Blast",
        5111004 => "Energy Drain",
        5111005 => "Transformation",
        5111006 => "Shockwave",
        5210000 => "Burst Fire",
        5211001 => "Octopus",
        5211002 => "Gaviota",
        5211004 => "Flamethrower",
        5211005 => "Ice Splitter",
        5211006 => "Homing Beacon",
        5121000 => "Maple Warrior",
        5121001 => "Dragon Strike",
        5121002 => "Energy Orb",
        5121003 => "Super Transformation",
        5121004 => "Demolition",
        5121005 => "Snatch",
        5121007 => "Barrage",
        5121008 => "Pirate\u{2019}s Rage",
        5121009 => "Speed Infusion",
        5121010 => "Time Leap",
        5221000 => "Maple Warrior",
        5220001 => "Elemental Boost",
        5220002 => "Wrath of the Octopi",
        5221003 => "Aerial Strike",
        5221004 => "Rapid Fire",
        5221006 => "Battleship",
        5221007 => "Battleship Cannon",
        5221008 => "Battleship Torpedo",
        5221009 => "Hypnotize",
        5221010 => "Speed Infusion",
        5220011 => "Bullseye",
        1320005 => "Achilles",
        1320006 => "Berserk",
        1320008 => "Aura of the Beholder",
        1320009 => "Hex of the Beholder",
        1321000 => "Maple Warrior",
        1321001 => "Monster Magnet",
        1321002 => "Power Stance",
        1321003 => "Rush",
        1321007 => "Beholder",
        1321010 => "Hero\u{2019}s Will",
        _ => return None,
    })
}

fn item_name(id: u32) -> Option<&'static str> {
    Some(match id {
        1032003 => "Amethyst Earrings",
        1040013 => "Blue One-lined T-Shirt [M]",
        1041012 => "Red-Striped T-Shirt [F]",
        1062000 => "Ice Jeans",
        1072004 => "White Gomushin",
        1072169 => "Blue Snowshoes",
        1072170 => "Green Snowshoes",
        1072171 => "Black Snowshoes",
        1072338 => "Purple Snowshoes",
        1082230 => "Glitter Gloves",
        1082246 => "Flamekeeper Cordon",
        1092045 => "Maple Magician Shield",
        1092047 => "Maple Thief Shield",
        1302000 => "Sword",
        1302016 => "Yellow Umbrella",
        1302017 => "Sky Blue Umbrella",
        1302026 => "Black Umbrella",
        1302027 => "Green Umbrella",
        1302028 => "Light Purple Umbrella",
        1302029 => "Beige Umbrella",
        1302031 => "Diao Chan Sword",
        1302037 => "Trumpet",
        1302063 => "Flaming Katana",
        1302064 => "Maple Glory Sword",
        1312002 => "Scythe",
        1312004 => "Hand Axe",
        1312013 => "Green Paint Brush",
        1312032 => "Maple Steel Axe",
        1322000 => "Mace",
        1322002 => "Iron Mace",
        1322004 => "Fusion Mace",
        1322005 => "Wooden Club",
        1322007 => "Leather Purse",
        1322024 => "Purple Tube",
        1322051 => "Fruity Bamboo",
        1322054 => "Maple Havoc Hammer",
        1332006 => "Field Dagger",
        1332007 => "Fruit Knife",
        1332008 => "Coconut Knife",
        1332009 => "Cass",
        1332010 => "Iron Dagger",
        1332016 => "Sai",
        1332017 => "Serpent\u{2019}s Coil",
        1332020 => "Korean Fan",
        1332021 => "Plastic Bottle",
        1332022 => "Angelic Betrayal",
        1332025 => "Maple Wagner",
        1332026 => "Cursayer",
        1332029 => "Liu Bei Dagger",
        1332030 => "Fan",
        1332032 => "Christmas Tree",
        1332051 => "Gold Double Knife",
        1332053 => "Kebob",
        1332055 => "Maple Dark Mate",
        1332056 => "Maple Asura Dagger",
        1372002 => "Metal Wand",
        1372005 => "Wooden Wand",
        1372017 => "Streetlight",
        1372031 => "Heart Staff",
        1372033 => "Heart Wand",
        1372034 => "Maple Shine Wand",
        1372035 => "Elemental Wand 1",
        1372036 => "Elemental Wand 2",
        1372037 => "Elemental Wand 3",
        1372038 => "Elemental Wand 4",
        1372039 => "Elemental Wand 5",
        1372040 => "Elemental Wand 6",
        1372041 => "Elemental Wand 7",
        1372042 => "Elemental Wand 8",
        1382009 => "Maple Staff",
        1382012 => "Maple Lama Staff",
        1382015 => "Poison Mushroom",
        1382016 => "Pyogo Mushroom",
        1382037 => "Doomsday Staff",
        1382039 => "Maple Wisdom Staff",
        1382041 => "Nocturnal Staff",
        1382045 => "Elemental Staff 1",
        1382046 => "Elemental Staff 2",
        1382047 => "Elemental Staff 3",
        1382048 => "Elemental Staff 4",
        1402001 => "Wooden Sword",
        1402017 => "Daiwa Sword",
        1402037 => "Stonetooth Sword",
        1402039 => "Maple Soul Rohen",
        1402044 => "Pumpkin Lantern",
        1412001 => "Metal Axe",
        1412011 => "Maple Dragon Axe",
        1412027 => "Maple Demon Axe",
        1422000 => "Wooden Mallet",
        1422004 => "Monkey Wrench",
        1422011 => "Sake Bottle",
        1422014 => "Maple Doom Singer",
        1422029 => "Maple Belzet",
        1432000 => "Spear",
        1432012 => "Maple Impaler",
        1432040 => "Maple Soul Spear",
        1442000 => "Pole Arm",
        1442018 => "Frozen Tuna [level 20]",
        1442023 => "Maroon Mop",
        1442024 => "Maple Scorpio",
        1442051 => "Maple Karstan",
        1442068 => "Crimson Arcglaive",
        1452002 => "War Bow",
        1452016 => "Maple Bow",
        1452018 => "Bow of Magical Destruction",
        1452022 => "Maple Soul Searcher",
        1452045 => "Maple Kandiva Bow",
        1462001 => "Crossbow",
        1462014 => "Maple Crow",
        1462019 => "Maple Crossbow",
        1462040 => "Maple Nishada",
        1472063 => "Magical Mitten",
        1482000 => "Steel Knuckler",
        1482020 => "Maple Knuckle",
        1482021 => "Maple Storm Finger",
        1482022 => "Maple Golden Claw",
        1492000 => "Pistol",
        1492020 => "Maple Gun",
        1492021 => "Maple Storm Pistol",
        1492022 => "Maple Canon Shooter",
        1702030 => "Diao Chan Sword",
        _ => return None,
    })
}
