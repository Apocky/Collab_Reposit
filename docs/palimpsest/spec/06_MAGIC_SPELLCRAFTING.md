# 06 — Magic and Spellcrafting
As of 2026-10-11. Status: BINDING. Conforms to canon-1.5.

This chapter owns aether, casting, the glyph grammar, the glyph catalog, spell cost and potency, magic schools, glyph learning, the Grimoire, texts, wands, enchanting, summons, polymorph, dispel and anti-magic, prayer and Devotion, and the 16 patrons. Every number here is final for implementation. Where this chapter needs something the canon or another chapter has not yet defined, the need is stated under Interface assumptions or Canon gaps, together with the value this chapter uses.

## Imports

From `spec/00_CANON.md` (canon-1.5):

| Canon section | What this chapter consumes |
|---|---|
| §1.1, §1.10, §1.11 | content-ID grammar, `id32`, text keys and genre skin keys, ascending-ID iteration, BTree-only collections |
| §1.2 to §1.6 | `Milli`, `Bp`, `Ticks`, `Bam`, `mm` content encoding, display-unit rule, `floor_div`, `ceil_div`, `mul_bp`, left-to-right Bp chains |
| §2.2, §2.3 | tick schedule (status pulse 15, AI think 6), "N s = 60N ticks", active-interval semantics, refresh rule, rate accumulator, `in_combat`, `light_at` |
| §3.3, §3.4 | keyed draws; `rng.magic` (owner 06), `rng.combat` (Contradiction failure, crits), `rng.status` (`st.confused`); fixed draw counts |
| §4.1, §4.2 | humanoid capsule, walk 4300 mm/s, dodge, jump, final move speed rule |
| §4.3 | fixed terrain, the `destructible` block tag, Errata placeables (spells break only `destructible` blocks, §13) |
| §5.2, §5.3 | genre table with `aether_axiom` and Aether skins; component dependency defaults; `Req[a] = max(1, origin_axiom − 2)` |
| §5.4 to §5.8 | tolerance, Contradiction table and overrides, Anachronism Edge, AETHER, GLYPH and MYTHOS ambient rows, local genre, seam zones |
| §5.10, §5.11 | World Laws (`law_bound`, `law_grants`); `st.unmoored` after three Contradiction failures in 600 ticks |
| §6.4 rows 3, 7, 8, 13, 17, 24, 25, 26, 27, 29, 30, 41 | healing received, max aether, aether regeneration, crit, use-item speed, mind and arcane thresholds, complexity budget, Grimoire slots, spell potency, cast interruption resistance, buildup multiplier |
| §6.5 | NPCs never use rows 26 to 29 |
| §7, §7.1 | magic skills, Devotion, Inscription, Handling, Conjuration scope; skill XP rules |
| §9, §9.1 | body plans, part pools, severing and reattachment by prayer |
| §10.1 to §10.9 | damage types and `ward_mult`, step 0 source magnitude, damage pipeline (step 2.1 spell potency), other entry points, lethal resolution, healing |
| §11.1 to §11.8 | status list (rows owned by 06), buildup, DoT, direct application, player-controlled bearers |
| §12, §12.1, §12.2 | `res.aether`, `res.ward`, `res.favor`, `res.composure`; surfaces and reactions; sight concealment |
| §13.1, §13.3, §13.5 to §13.9 | `cat.wand`, `cat.text`, `cat.book`, `cat.offhand`; rarity power; quality; knowledge; enhancement; `P`, `T`, `S` with the depth cap `min(lvl, 300)` and `i64` table multipliers; source level (`u32`, at least 1); item instance contract (`charges`) |
| §15, §15.1 | factions, stance bands, minion contract and caps |
| §16.4, §16.5 | `afflict.beast` polymorph; patrons, favor, prayer timers, sacrifice |
| §17.1, §17.2, §17.3, §17.5, §17.6, §17.9 | Colophon aether refresh, party tables, rank control multipliers and immunities, friendly fire, ghosts, invaders |
| §18, §19 | core events; namespace ownership (`glyph.`, `spell.`, `school.`, `prayer.`, `ench.`, `summon.`, `patron.` belong to 06) |

Limited building and opt-in invasions, decided in `DECISIONS.md` items 16 and 15, are canon since canon-1.4 (§4.3, §17.9) and are consumed through the canon rows above.

From other chapters, checked against every file in `spec/` on 2026-10-11 (docs 07, 09, 17 and 18 included); the exact shapes this chapter calls are listed under Interface assumptions.

| Doc | What this chapter consumes |
|---|---|
| 03 | system order: `sys.actions` (system 7) runs aether regeneration and casts, `sys.damage` (17) resolves the hit queue, `sys.env_sample` (10) re-evaluates Contradiction; the per-tick action `seq` reset in `sys.tick_begin`; `comp.pools.aether`; payloads of `ev.spell_cast` (with `fizzled`), `ev.healed`, `ev.status_expired` (reason 1 cleansed) and `ev.minion_released` (reasons 0 dismissed, 1 died, 2 folio exit); `raycast_blocks` for line of effect |
| 04 | mastery perk IDs `skill.<name>.m25` to `.m100` and their effects (§5.3), whose exact hooks §7.3 fixes (doc 04's magic perk texts mirror §7.3); the XP event table (§5.2), whose magic rows mirror §7.4; doc 04 citing the §6.2 cost factor and the glyph tier gates; Spellwright, Oracle and Netrunner talents; `aug.blood_battery`; Origin traits `favored` and `lucid` |
| 05 | `arch.offhand.focus`; the combat phase and hit queue; `HitRecord`; keyed-draw allocation (seq 0 primary action, `16 + i` hits); stance base speeds and their minimum rule (§1.6); projectiles and sweeps; friendly-fire classes |
| 08 | `liq.mana`, `alch.eff.restore_aether` and aether potions; `liq.holy_water`; liquid and gas placement |
| 10 | `cat.wand` classification and type key (`wand_pool` spells, at most 16); the `appr.eff.*` effects of the divination essences (§5.10); text `use_id` and `observable`; blessed and cursed consumables (§7.5); the wand curses `curse.misfire`, `curse.backfire`, `curse.leaking`, `curse.bloodprice` |
| 11 | `arch.ranged.staff`, `arch.ranged.wand` and `arch.offhand.focus` loot fields and tendency C; implement affixes, perks and parts (`affix.spell_dmg`, `affix.aether_cost`, `affix.kill_aether`, `perk.*_conduit`, `perk.deep_reservoir`, `perk.quick_siphon`, `perk.stable_reservoir`, `perk.arcane_surge`, `perk.overflow`, `perk.leg.spellweaver`, `part.focus.*`); `item.text.*`; `item.book.primer_*`; `item.trinket.holy_symbol` |
| 12 | World Laws, among them `law.mythic_antiquity.favor` and `law.mythic_antiquity.hubris`; `room.shrine` and `secret.room.shrine` altars; the Scriptorium; campfires that refill aether |
| 13 | NPC template fields `aether_max_mae`, `aether_regen_mae_s` (default 2,000 per second) and `spells`; NPC interrupting statuses (§4); the minion contract (§6): profiles, leash, owner states; `enemy.editors.invader` |
| 14 | Calling kits (`item.book.primer_evocation` teaches flame and frost, the alternate kit storm and chain; `item.book.primer_abjuration` teaches aura); `unlock.loot.glyphs_2`; Echo traits that touch spells; the Colophon Hearth (aether set to max) |
| 15 | Script unlocks `unlock.glyph.<family>` for `ember`, `hollow`, `mirror`, `storm`, `thorn`, `tide` |
| 16 | `st.downed` forbids casting and praying; Restoration 100 completes revival at once; cast prediction with keyed draws |

## 1. Principles

1. **One grammar, sixteen voices.** A spell is a sentence of glyphs. Glyphs are genre-neutral archetypes. The genre in which a glyph was learned is its skin and its dependency: a bolt learned in Cyberpunk is a `Packet` that runs on TECHNE, the same glyph learned in Wuxia is a `Flying Needle` that runs on VERVE (Canon §5.3 spell-glyph row).
2. **Aether is genre-native.** Regeneration follows the local genre's `aether_axiom` (Canon §5.7 AETHER). Spell potency follows each glyph's dependency axis (Canon §5.7 GLYPH). A Cyberpunk netrunner casts well in Cyberpunk and Space Opera, poorly in Fairy Tale.
3. **No hidden casting roll.** A spell never fails because of skill. It fails only through Contradiction (§11), a cursed wand's misfire (§8.4) or interruption (§3.7).
4. **Determinism.** Every random draw in this chapter names its stream and draw count (§19.2). Compilation, cost, potency, naming, enchanting and favor arithmetic are deterministic and draw nothing.
5. **Same pipeline.** Spell damage enters Canon §10.5 at step 0 like any item source. Status power enters Canon §11.2. Healing obeys Canon §10.9.

## 2. Aether

### 2.1 Pool and regeneration

- `res.aether` is `Milli`. Max aether is Canon §6.4 row 7: `20 + FOC + L`, stored × 1000. Max changes follow the Canon §12 max-change rule.
- Regeneration is Canon §6.4 row 8 times the AETHER term of Canon §5.7, applied every tick through the Canon §2.3 rate accumulator with `k = 60`. It runs during every cast state, including Channeling. It never waits for a delay.
- `st.unraveled` sets regeneration to 0 and resets the accumulator (Canon §11.1, §2.3).
- Fairy Tale Bargain Debt (§10.11) is repaid before the pool rises.

```rust
// palimpsest_sim::magic::aether — runs first in sys.actions (doc 03 system 7) for each entity, ascending EntityId,
// before that entity's cast machine advances. NPCs regenerate their template rate instead (§6.5).
fn aether_tick(e: &mut Caster, local: &Genre) {
    if e.has(ST_UNRAVELED) { e.aether_acc = 0; return; }
    let aa = local.aether_axiom;                                   // Canon §5.2
    let term = if aa == Axis::None { 10000 } else { 10000 + (local.axiom(aa) - 5) * 1000 };
    let rate = mul_bp(500 + e.atn() as i64 * 20, term);            // m-aether per second, Canon §6.4 row 8
    let total = rate + e.aether_acc;
    let mut q = floor_div(total, 60);
    e.aether_acc = total - 60 * q;
    let repay = q.min(e.bargain_debt);                             // §10.11
    e.bargain_debt -= repay; q -= repay;
    e.aether = (e.aether + q).min(e.max_aether());
}
```

Example: ATN 40 in `genre.cyberpunk` (aether axiom TECHNE, TEC 9): `rate = mul_bp(1300, 14000) = 1820` m-aether per second (Canon acceptance test 35). Over 60 ticks the accumulator adds exactly 1,820.

### 2.2 Genre-native casting table

This table is the complete genre skin of aether and of every glyph. The glyph dependency of a glyph is fixed by the genre in which it was learned, generated or last Rewritten (its **glyph origin**): `dep = {aether_axiom}`, or `{arcana}` when the aether axiom is `none` (Canon §5.3). `Req = max(1, origin_axiom[dep] − 2)`. The **register** is the genre voice a spell speaks in: the glyph origin of the root stage's Form glyph (§4.2). Each register carries one rider (§10).

| Genre | Aether skin | aether_axiom | Glyph dep | Glyph Req | GLYPH term at home (Bp) | AETHER term at home (Bp) | Regen at ATN 40 (m-aether/s) | Register (spell noun) | Rider |
|---|---|---|---|---|---|---|---|---|---|
| `genre.mythic_fantasy` | Mana | arcana | arcana | 6 | 10600 | 13000 | 1690 | Spell | Ley-Bound |
| `genre.gothic_horror` | Sanguine | mythos | mythos | 7 | 10800 | 14000 | 1820 | Sanguine Rite | Blood Price |
| `genre.cosmic_horror` | Gnosis | mythos | mythos | 8 | 11000 | 15000 | 1950 | Gnostic Rite | Composure Toll |
| `genre.space_opera` | Psi | arcana | arcana | 4 | 10200 | 11000 | 1430 | Discipline | Focused Channel |
| `genre.cyberpunk` | Bandwidth | techne | techne | 7 | 10800 | 14000 | 1820 | Program | Silent Execution |
| `genre.post_apocalypse` | Static | techne | techne | 4 | 10200 | 11000 | 1430 | Rig | Jury-Rigged |
| `genre.weird_west` | Hex | mythos | mythos | 4 | 10200 | 11000 | 1430 | Hex | Quick-Draw |
| `genre.hardboiled_noir` | Nerve | verve | verve | 1 | 9400 | 7000 | 910 | Gambit | Desperation |
| `genre.wuxia` | Qi | verve | verve | 8 | 11000 | 15000 | 1950 | Technique | Flowing Body |
| `genre.steampunk` | Pressure | techne | techne | 5 | 10400 | 12000 | 1560 | Device | Pressure Vessel |
| `genre.fairy_tale` | Glamour | arcana | arcana | 8 | 11000 | 15000 | 1950 | Bargain | Credit |
| `genre.mythic_antiquity` | Ichor | mythos | mythos | 8 | 11000 | 15000 | 1950 | Invocation | Heard Above |
| `genre.oneiric` | Lucidity | arcana | arcana | 7 | 10800 | 14000 | 1820 | Lucid Dream | Dream Logic |
| `genre.redacted` | Clearance | techne | techne | 6 | 10600 | 13000 | 1690 | Procedure | Redundant Copy |
| `genre.unwritten` | Potential | none | arcana | 1 | 9000 | 10000 | 1300 | none: no glyph is ever generated or Rewritten here (ARCANA 0, Canon §5.3) | none |
| `genre.ur_text` | Logos | none | arcana | 3 | 10000 | 10000 | 1300 | Word | none |

Worked check: a glyph generated in Cyberpunk has `dep = {techne}`, `Req = max(1, 9 − 2) = 7`; one generated in Mythic Fantasy has `dep = {arcana}`, `Req = 6` (Canon acceptance test 36).

### 2.3 Paying aether

- A cast debits `res.aether` at the moment §3.3 names. A cast never starts unless the caster can pay, except through `aug.blood_battery` (doc 04), Blood Price (§10.2) or Credit (§10.11). `can_pay` checks them in that order: an Erratum with `aug.blood_battery` pays any shortfall at 1 mHP per m-aether for every register, never below 1,000 mHP; otherwise a Sanguine Rite uses Blood Price; otherwise a Bargain uses Credit; otherwise the cast fails with `Aether`.
- Aether never goes below 0 and never above max.
- Aether sources besides regeneration: aether potions and `liq.mana` (doc 08), `glyph.mod.siphon` (§6.6), `ench.siphoning` (§9.4), item affixes and perks that restore aether on kill (doc 11), the Netrunner talent `feedback` (doc 04), prayer results (§16.3, §16.4), campfires (doc 12) and the Colophon Hearth (Canon §17.1, doc 14). The surge entry Aether Burn and `st.unraveled` remove aether.

## 3. Casting

### 3.1 Casting sources

| Source | Content | Pays with | Potency source | `src_lvl` (Canon §13.8.1) |
|---|---|---|---|---|
| Grimoire spell | a compiled spell in a Grimoire slot (§8.2) | aether | caster row 29 | active implement `ilvl`, else `max(1, d_current − 3)` |
| Tome spell | a spell inscribed in a held tome implement (§8.5) | aether | caster row 29 | tome `ilvl` |
| Text (`cat.text`) | one inscribed or authored spell, or a doc 10 effect (§8.3); consumed | nothing | reader row 29 | text `ilvl` (Canon gap G16) |
| Wand (`cat.wand`) | one authored `wand_pool` spell (§8.4); charges | one charge | wielder row 29 | wand `ilvl` |
| On-use enchantment | `ench.spellstore` and other on-use entries (§9.4) | one use per Folio | wielder row 29 | item `ilvl` (Canon gap G16) |
| Prayer (`prayer.`) | supplication or petition (§16) | favor and prayer timer | row 29 with school skill = Devotion | `ilvl` of an equipped `item.trinket.holy_symbol` (doc 11), else the active implement's `ilvl`, else `max(1, d_current − 3)` (Canon gap G16) |

Step 0 of every source uses that source item's rarity power, quality, `eff_enh` and primary-material `dmg_bp` (Canon §10.4): the implement for Grimoire, tome and prayer casts (the holy symbol for prayers when one is equipped), the wand for zaps, the enchanted item for on-use enchantments, and the text for texts with material factor 10000. With no implement every factor is 10000.

### 3.2 Inputs: Grimoire and quick-cast wheel

- The **Grimoire** has `2 + floor_div(MEM, 20)` slots (Canon §6.4 row 27). Each slot holds one compiled spell. Slot contents persist for the run.
- The **active spell** is the Grimoire slot or wheel entry most recently selected. `input.cast` (doc 17) casts it: press for instant and cast-time spells, hold for channel and charge spells.
- The **quick-cast wheel** has 8 entries. Each entry references one of: a Grimoire slot, a tome spell, a carried text, a carried wand, an on-use enchantment, or a prayer. Selecting an entry sets the active spell and casts it on release of the wheel button in the same input command. Opening the wheel never pauses or slows the simulation, in single player or co-op.
- The replay command (Canon §2.4) carries the cast press in `buttons` (doc 17: `input.cast` bit 9, `input.cast_cancel` as `ACTION` value 5 in bits 26 to 28, a wheel selection as bits 19 and 20 set together with the entry index 0..=7 in `WHEEL_ENTRY` bits 29 to 31). The cast's RNG key is `(caster, input tick, seq 0)` (§3.9, §19.2). Seq 0 is the doc 05 §1.4 primary-action slot; a cast start and a weapon action start never share a tick (§3.5).

### 3.3 Cast types

`ct` is the final cast ticks after §3.4.

| Cast type | Forms | Behavior | Aether debit |
|---|---|---|---|
| instant | touch, self, any form reduced to 0 cast ticks | resolves on the tick its input is applied | at resolution |
| cast_time | bolt, orb, cone, aura, field, wall, rune, nova, chain, manifest | Winding for `ct` ticks; resolves on the last Winding tick; a single press starts it | at Winding start |
| channel | beam | Winding for `ct` ticks (windup, no debit), then Channeling while held, up to the form's duration (300 ticks); a pulse fires on the first Channeling tick and every `pulse` ticks after | per tick during Channeling: the cast cost (§6.2) is a per-second rate through a `k = 60` accumulator |
| charge | any cast_time form with `glyph.mod.charged` | Charging while held: `held` counts ticks from the press; a release at `held < ct` cancels; a release at `held >= ct` resolves; at `held = ct + 120` it auto-releases | at release: `mul_bp(cast_cost, charge_bp(c))` |

```
c            = min(held − ct, 60) once held >= ct; it stops rising as described below   // charge ticks past the normal cast time
charge_bp(c) = 10000 + floor_div(5000 × c, 60)                                           // 10000 at c = 0, 12500 at 30, 15000 at 60
// with skill.evocation.m50 (§7.3): c runs 0..=90, charge_bp(c) = 10000 + floor_div(7500 × c, 90), auto-release at held = ct + 150
```

Charging starts only if `aether >= cast_cost`. While Charging, `c` stops rising on any tick where `mul_bp(cast_cost, charge_bp(c + 1)) > aether`. At release the debit is `mul_bp(cast_cost, charge_bp(c))`. If aether has fallen below that debit since `c` stopped rising (for example through `st.unraveled`), `c` drops to the largest value whose debit is payable; if even `cast_cost` is unpayable, the release is a cancel. Magnitude is multiplied by the same `charge_bp` at resolution (§6.4). A release at `held < ct` is a voluntary cancel: nothing is debited and the 30-tick lockout applies.

Balance note: the curve starts at the normal cast time, so charging never beats the plain cast's damage rate. A charged bolt released at `c = 60` deals magnitude 15000 per `24 + 60 + 20 = 104` ticks (144 per tick) against 10000 per 44 ticks (227 per tick) uncharged, at the same aether per point of magnitude: it trades sustained damage for one larger hit. The earlier curve (5000 at `c = 0` with no Winding) let a charged bolt fire at half magnitude every 21 ticks, 5% above the uncharged rate and with no cast time, which made `glyph.mod.charged` a strict upgrade.

A Channeling tick whose drain `q` exceeds current aether ends the channel. Releasing the input ends it. Duration modifiers extend the 300-tick limit.

### 3.4 Timing, recovery and cooldowns

```
ct_raw   = form.cast_ticks + Σ essence.cast_add + Σ modifier.cast_add      // root stage only
ct_q     = if quicken { mul_bp(ct_raw, 5000) } else { ct_raw };  if quicken && ct_q <= 6 { ct_q = 0 }
ct_m     = ct_q → × 5000 if Quick-Draw is active (§10.7) → × each [T] cast-tick factor in ascending ID byte order
speed_bp = 10000 → × implement.cast_speed_bp → × ench.celerity → × 13000 if elite.surging (NPC) → × 11500 if st.hasted
           // one mul_bp each: non-status factors first, then statuses in ascending ID order (Canon §11.7)
ct       = if ct_m == 0 { 0 } else { max(1, ceil_div(ct_m × 10000, speed_bp)) }
R        = if ct == 0 { INSTANT_RECOVERY_TICKS } else { CAST_RECOVERY_TICKS }   // 45 and 20
recovery = ceil_div(R × 10000, speed_bp)
cooldown = form.cooldown + Σ essence.cd_add + Σ modifier.cd_add                 // root stage; never scaled
```

`[T]` cast-tick factors: doc 11 `perk.swift_conduit`, `perk.stable_reservoir` and `part.focus.conduit` on the active implement; doc 04's Spellwright talent `quicken` (8500). `[T]` marks, here and in §6, a slot where doc 04 talents and perks, doc 11 item modifiers and doc 14 Echo traits enter, one `mul_bp` each, in ascending ID byte order.

- A form with cast ticks 0 is instant. A cast-time form reduced to 0 by quicken is instant.
- **Recovery** follows every resolution and every channel end. No new cast starts during recovery.
- **Cooldown keys.** A spell's cooldown keys are its root form when `form.cooldown > 0` and each root essence or modifier whose `cd_add > 0`. Each caster stores `BTreeMap<u32 glyph id32, Tick>`. A cast starts only when every key of the spell is ready. On the resolution tick (the first Channeling tick for channels) every key of the spell is set to `max(current, t + cooldown)`. Keying by glyph instead of by `spell_hash` closes the bypass of copying a spell with one extra modifier to get a second cooldown. Texts, wands, on-use enchantments and payload stages ignore cooldowns. `st.chilled` does not change cast speed (Canon §11.1 limits it to move and attack speed).
- **Instant recovery.** An instant cast (`ct = 0`) recovers for 45 ticks instead of 20. With 20 ticks, `touch flame` in Mythic Fantasy at Folio 1 (skill 20, ATN 40, magnitude 13000, no cast time) dealt 37,159 mHP every 21 ticks, 106 HP per second, 3.5 times `REF_PLAYER_DPS_D1` (Canon §17.4). With 45 ticks it deals 48 HP per second, a 1.24 × premium for melee range over the 39 HP per second of `bolt flame`, and both are limited by aether.
- **Interrupt lockout** after an interruption or cancel: 30 ticks, replacing recovery.
- Example: an orb (36 ticks) with quicken: `ct_q = 18`; with `st.hasted` and a focus of cast speed 10000: `speed_bp = 11500`, `ct = ceil_div(180000, 11500) = 16`, `recovery = ceil_div(200000, 11500) = 18`.

### 3.5 Cast state machine

States: `Idle`, `Winding`, `Channeling`, `Charging`, `Recovering`. "Casting states" are `Winding`, `Channeling` and `Charging`.

| # | From | Event | Guard | To | Side effects |
|---|---|---|---|---|---|
| 1 | Idle | cast input on tick `t` | `try_start_cast` fails | Idle | nothing; the UI shows the `CastError` |
| 2 | Idle | cast input on tick `t` | `try_start_cast` succeeds; `ct = 0` (instant) | Recovering for `recovery` ticks | debit `cast_cost`; resolve on tick `t` (§3.9); set cooldown keys |
| 3 | Idle | cast input on tick `t` | succeeds; cast_time or channel, `ct >= 1` | Winding | cast_time: debit `cast_cost`; channel: no debit; record key `(caster, t, 0)` |
| 4 | Idle | cast input held on tick `t` | succeeds; charge | Charging | `held = 0`; record key; no debit |
| 5 | Winding | Winding timer reaches `ct` | cast_time | Recovering | resolve (§3.9); set cooldown keys |
| 6 | Winding | Winding timer reaches `ct` | channel | Channeling | set cooldown keys; Contradiction roll (§3.9 steps 1 to 4); on success, first pulse on this tick |
| 7 | Channeling | tick with `(t − start) % pulse == 0` | the tick's drain is paid | Channeling | pulse |
| 8 | Channeling | release, duration end, or a drain the caster cannot pay | — | Recovering | `ev.spell_cast` with `channel_ticks` |
| 9 | Charging | tick with input held | `held + 1 < auto` | Charging | `held += 1`; `c` rises per §3.3 |
| 10 | Charging | release with `held >= ct`, or `held + 1 = auto` | — | Recovering | debit at `charge_bp(c)`; resolve; set cooldown keys |
| 11 | Charging | release with `held < ct` | — | Recovering with 30-tick lockout | nothing debited |
| 12 | any casting state | hard or failed soft interruption (§3.7), or a voluntary cancel: `input.cast_cancel`, sprint, jump, dodge (Techniques exempt sprint, jump and dodge, §10.9), or any weapon, item, interact or mount input | — | Recovering with 30-tick lockout | Winding refunds `floor_div(debited, 2)`; Charging debits nothing; Channeling keeps what it drained |
| 13 | any casting state, Recovering | `st.downed`, death or `st.ghost` | — | Idle | no refund; no resolution |
| 14 | Recovering | recovery or lockout elapsed | — | Idle | none |
| 15 | Recovering | cast input | — | Recovering | ignored and not buffered |

`auto` is `ct + 120`, or `ct + 150` with `skill.evocation.m50` (§3.3).

A caster holds at most one cast at a time. A cast and a doc 05 weapon action never start on the same tick: when both inputs arrive, the cast wins and the weapon input is ignored; `try_start_cast` fails with `Busy` while the weapon state machine is outside its idle state; and while the cast machine is outside `Idle`, the weapon state machine accepts no new action. Rows 12 and 13 cover every way out of a casting state, so no cast is left without a resolution, a refund rule or a cancel. A core glyph that becomes Disconnected during Winding or Charging does not interrupt; the resolution then fizzles without a roll (§3.9). Wands, texts, on-use enchantments and prayers use this machine with their own timing (§8.3, §8.4, §16).

### 3.6 Movement while casting

| State | Base move speed (mm/s) | Sprint | Jump, dodge |
|---|---|---|---|
| Winding, Charging | 2150 (walk × 5000) | pressing sprint cancels the cast | cancel the cast |
| Channeling | 1720 (walk × 4000) | cancels | cancel |
| Recovering, instant casts | normal | normal | normal |

These are base-speed stances in the sense of doc 05 §1.6: when several stances apply, the lowest base speed wins, crouch walk (2000 mm/s) included, and the Canon §4.2 final-speed rule applies on top (row 11, then statuses). The Wuxia Technique rider (§10.9) removes every restriction in this table for Techniques. Canon gap G6 records the two new base speeds.

### 3.7 Interruption

Interruption is checked for a caster in `Winding`, `Channeling` or `Charging`.

**Hard interrupts** end the cast with no roll: gaining `st.staggered`, `st.stunned`, `st.frozen`, `st.knocked_down`, `st.sleeping`, `st.polymorphed`, `st.berserk` (Canon §11.8 for players, doc 13 §4 for NPCs), `st.feared` (player casters only while facing within 8192 Bam of the source, Canon §11.8; NPC casters always), `st.silenced` (except Programs, §10.5), and for NPC casters also `st.charmed` and `st.hacked` (doc 13 §4); counterspell (§14.3); entering a null field (§14.4). Gaining `st.downed`, dying or becoming a ghost ends the cast through §3.5 row 13.

**Soft interrupts**: each hit on the caster that reduces HP by at least 1 mHP after ward (Canon §10.5 step 7) and is not a DoT pulse or a self-cost (§10.2) rolls Canon §6.4 row 30 once:

```
keep_bp = clamp(2500 + (FOC − 40) × 75, 0, 9500)                  // Canon §6.4 row 30
rng     = keyed_rng("rng.magic", folio, caster, hit_tick, 0x2000 | (k & 0x0FFF))   // k = 0-based index of this hit on this caster this tick
keep    = rng.roll_bp(keep_bp)                                      // exactly 1 draw
```

FOC 40 keeps 25.0%; FOC 60 keeps 40.0%; FOC 150 keeps 95.0%. Steampunk Devices skip soft interrupts (§10.10). A ward that absorbs the whole hit prevents the roll. `ev.cast_interrupted` (Canon gap G3) is emitted with the cause.

### 3.8 Implements (foci)

The **active implement** is the staff in the active weapon slot if any; otherwise the item in `slot.offhand` if it is a focus or a tome; otherwise none. A wand is the implement only for its own spell.

| Implement | Archetype | `cost_bp` | `cast_speed_bp` | Other |
|---|---|---|---|---|
| Focus: orb, crystal, cyberdeck, jade disc, gauge-box | `arch.offhand.focus` (doc 05 exports it) | 9500 | 10000 | — |
| Tome | `arch.offhand.tome` (requested from doc 05, Canon gap G5) | 10000 | 9000 | holds 2 inscribed spells (§8.5) |
| Staff | `arch.ranged.staff`, two-handed (doc 11 §3.2 lists it; requested from doc 05, Canon gap G5) | 9000 | 10500 | also a weapon (doc 05) |
| None | — | 11000 | 10000 | `src_lvl = max(1, d_current − 3)`; rarity, quality, enhancement factors 10000 |

- The implement supplies step 0 of spell damage: `P(ilvl)`, rarity power, quality, `10000 + 600 × eff_enh` and its primary material's `dmg_bp` (§3.1, §6.4).
- Legendary and Exotic implements carry one **bound glyph**: a modifier ID applied to every compatible spell cast through the implement at 0 complexity. Its positive cost add is waived; a negative one (`glyph.mod.thrift`) applies, so a bound thrift is never a pure penalty. It is never written into the spell, never counts toward the complexity budget, adds no GLYPH term to `spell_pot` (it is a component of the implement, with the implement's dependency and potency), and never forms a cooldown key. An incompatible bound glyph is ignored for that spell.
- Doc 11 implement modifiers: `affix.spell_dmg`, `perk.focused_conduit`, `part.focus.crystal` and `perk.leg.spellweaver` enter Canon §10.5 step 1; `affix.aether_cost`, `perk.stable_reservoir`, `perk.deep_reservoir` (staff) and manufacturer tendency C multiply `cost_bp` (tendencies are folded in at generation); `perk.swift_conduit`, `perk.stable_reservoir` and `part.focus.conduit` are `[T]` cast-tick factors (§3.4); `perk.wide_conduit` multiplies area radii after widen (§6.4); `perk.arcane_surge` and `item.palimpsestic.authors_quill` make the casts they name free (§6.2).
- Contradiction of the implement's own base function: Disconnected makes it count as None. Otherwise its `cost_bp` and `cast_speed_bp` deviations from 10000 scale by its potency: `eff = 10000 + mul_bp(field − 10000, potency)`.

### 3.9 Cast resolution

```rust
fn try_start_cast(c: &mut Caster, src: CastSource, t: Tick) -> Result<(), CastError> {
    let sp = src.compiled_spell();                                     // §4.5, cached; recompiled on stat change
    if c.state != Idle || !c.weapon_idle() { return Err(Busy); }       // §3.5 mutual exclusion with doc 05 actions
    if c.cannot_cast() { return Err(Disabled); }                       // st.downed, st.ghost, st.polymorphed, every §3.7 hard-interrupt
                                                                       // status, player st.feared while facing the source
    if src.uses_cooldown() && sp.cooldown_keys().any(|k| c.cooldown_until(k) > t) { return Err(Cooldown); } // §3.4
    if c.has(ST_SILENCED) && src.needs_voice() && sp.register != CYBERPUNK { return Err(Silenced); }   // §14.2
    if sp.register == CYBERPUNK && (c.has(ST_SHOCKED) || c.has(ST_HACKED)) { return Err(Jammed); } // §10.5
    if in_null_field(c.pos) { return Err(Nullified); }                 // §14.4
    if any_core_glyph_disconnected(sp, c) { return Err(Disconnected); } // §11
    if sp.root_form() == MANIFEST && folio_minions() >= FOLIO_MINION_CAP && !c.has_spell_summon() {
        return Err(MinionCap);                                          // §12.1
    }
    let cost = cast_cost(sp, c, src);                                  // §6.2; 0 for texts, wands, on-use enchantments, prayers, free casts
    if !can_pay(c, cost, sp.register) { return Err(Aether); }          // §2.3, §10.2, §10.11
    c.cast = CastState { spell: sp, key: (c.id, t, 0), cost, started: t, .. };   // seq 0: §19.2
    Ok(())                                                             // transition per §3.5 rows 2 to 4
}

fn resolve_root(c: &mut Caster, cs: &CastState) {
    // 1. Contradiction at the resolution tick = max C over the root stage's core glyphs (form + essences), §11.1
    let (mut cc, e) = spell_contradiction(cs.spell.root(), c);
    if c.has(ST_UNMOORED) && (cc == 1 || cc == 2) { cc = 3; }          // Canon §5.11: Strained uses the C = 3 row
    // 2. rng.magic: exactly 4 draws, always, in this order
    let mut rm = keyed_rng("rng.magic", folio(), cs.key.0, cs.key.1, cs.key.2);
    let v0 = rm.range(10000); let v1 = rm.range(10000); let v2 = rm.range(65536); let v3 = rm.range(10000);
    if cc >= 6 { fizzle(c, cs, FizzleCause::Disconnected); return; } // went Disconnected mid-cast: no roll, no Unmoored count
    // 3. rng.combat: 1 draw only when the failure chance is above 0 (Canon §5.5); a wand adds its curse.misfire m (doc 10)
    let fail_bp = [0, 0, 0, 500, 1000, 1500][cc as usize] + cs.source.misfire_bp();
    let failed = fail_bp > 0 && keyed_rng("rng.combat", folio(), cs.key.0, cs.key.1, cs.key.2).roll_bp(fail_bp);
    // 4. outcome
    if failed && cc >= 3 { c.note_contradiction_failure(now()); }      // Canon §5.11: 3 within 600 ticks → st.unmoored 300 ticks
    if failed && (cc <= 3 || c.is_npc()) { fizzle(c, cs, FizzleCause::Rolled); return; } // §11.2; NPCs never surge (doc 13 §5)
    if failed { wild_surge(c, cs, cc, v1, v2); return; }               // §11.3, C ∈ {4, 5}
    let wild_growth = cs.spell.register == ONEIRIC && v3 < 500;        // §10.13
    resolve_stage(c, cs.spell.root(), Origin::caster(c), Ctx { depth: 0, v0, wild_growth, c_spell: cc, e_spell: e });
    emit(EV_SPELL_CAST, ..);                                            // Canon §18, fizzled = false
}
```

A misfire caused only by a wand curse (`cc <= 2`) is a fizzle but not a Contradiction failure, so it never counts toward `st.unmoored`.

Payload stages (§4.2) resolve through `resolve_stage` on the tick after their trigger event, with no failure roll, no cooldown, no cost, and one draw `v0 = keyed_rng("rng.magic", folio, caster, event_tick, 0x4000 | (stage_index << 8) | fire_index).range(10000)`. `fire_index` counts that stage's fires within one cast, across split projectiles and twin repeats, from 0 in order of event tick, then ascending host `EntityId` (at most 255). Their potency and Contradiction are computed fresh at the caster's position on that tick; a payload whose core glyph is Disconnected there resolves nothing. A Twinned stage reuses the root's `v0`.

## 4. Spell grammar

### 4.1 Glyph classes

| Class | Prefix | Count | Role |
|---|---|---|---|
| Form | `glyph.form.` | 13 | shape, delivery and cast type; exactly one per stage, first |
| Essence | `glyph.ess.` | 33 | what the spell does; 1 or 2 per stage |
| Modifier | `glyph.mod.` | 22 | adjusts numbers and delivery; 0 to 3 per stage |
| Trigger | `glyph.trig.` | 8 | opens a payload stage fired by an event of the host stage; 0 or 1 per stage |
| Condition | `glyph.cond.` | 8 | gates a trigger; 0 or 1, only directly after a trigger |

Every glyph reference in a spell is a pair `(glyph ID, glyph origin genre)`. Two references to the same glyph ID with different origins are different Lexicon entries and carry different dependencies.

### 4.2 BNF

```
<spell>          ::= <stage>                                        ; the root stage, depth 0
<stage>          ::= <form> <essences> <modifiers> <trigger-clause>
<essences>       ::= <essence> | <essence> <essence>                ; first = primary, second = secondary
<modifiers>      ::= ε | <modifier> | <modifier> <modifier> | <modifier> <modifier> <modifier>
                   | <modifier> <modifier> <modifier> <modifier>      ; only with skill.spellcraft.m100 (§7.3)
<trigger-clause> ::= ε | <trigger> <condition-opt> "(" <stage> ")"  ; the inner stage is the payload, depth + 1
<condition-opt>  ::= ε | <condition>
<form>           ::= <ref "glyph.form.">
<essence>        ::= <ref "glyph.ess.">
<modifier>       ::= <ref "glyph.mod.">
<trigger>        ::= <ref "glyph.trig.">
<condition>      ::= <ref "glyph.cond.">
<ref P>          ::= P <name> "@" <genre-suffix>                     ; e.g. glyph.form.bolt@mythic_fantasy
```

- **Maximum trigger depth is 2.** A depth-2 stage has an empty trigger clause. A spell therefore has at most 3 stages and at most 24 glyphs (8 per stage), or 27 (9 per stage) with Spellcraft 100.
- **Canonical text.** Glyph references are written in grammar order, separated by one space, with the payload in parentheses, for example `glyph.form.bolt@mythic_fantasy glyph.ess.flame@mythic_fantasy glyph.trig.on_impact@mythic_fantasy ( glyph.form.nova@mythic_fantasy glyph.ess.frost@mythic_fantasy )`.
- `spell_hash = fnv1a64(canonical text)`. It keys the compiled-spell cache, name variants and doc 04's "new self-designed spell" XP check; cooldowns are keyed by glyph (§3.4). It hashes one field, the canonical text, whose single spaces and parentheses make it self-delimiting, so canon-1.5's length-prefix rule for multi-field hashes (Canon §2.5) does not apply.
- **Register** = the glyph origin of the root Form. **School** of a stage = the school of its primary essence. The spell's school is the root stage's school.

### 4.3 Composition rules

Validation runs these checks in this order and reports the first failure. Authored `spell.` content runs checks 1, 2 and 4 to 11 at content load and must pass all of them.

| # | Error code | Rule |
|---|---|---|
| 1 | `E_GRAMMAR` | the glyph sequence does not parse under §4.2 (no form, form not first, 0 or 3 essences, more than 3 modifiers or more than 4 with `skill.spellcraft.m100`, condition without trigger, trigger without payload) |
| 2 | `E_DEPTH` | a trigger clause appears in a depth-2 stage |
| 3 | `E_UNKNOWN` | a reference `(glyph, origin)` is not in the caster's Lexicon (§8.1) |
| 4 | `E_DUPLICATE` | the same glyph ID appears twice in one stage |
| 5 | `E_FORM_ESSENCE` | an essence is not allowed with the stage's form (§4.3.1); both essences are checked, except the secondary of `glyph.ess.summon`, which check 8 covers |
| 6 | `E_OPPOSED` | the stage holds an opposed pair: flame–frost, radiance–shadow, swiftness–bind, mend–venom, ward–unmaking, veil–lumen |
| 7 | `E_POLARITY` | the stage mixes a helpful essence with a harmful one (§5.3 polarity column; neutral mixes with either) |
| 8 | `E_SUMMON_PAIR` | `glyph.ess.summon` is secondary, or its secondary is not listed in §12.2 |
| 9 | `E_MODIFIER` | a modifier is not allowed on the form (§5.4 "Applies to"); quicken with charged; quicken or charged in a payload stage |
| 10 | `E_TRIGGER_HOST` | the trigger's host forms (§5.5) exclude the stage form, or the stage lacks what the trigger needs: on_kill needs a damage essence, on_status needs a status-applying essence, on_struck needs a helpful essence |
| 11 | `E_PAYLOAD_FORM` | a payload stage uses beam or aura, or a manifest payload sits under `glyph.trig.pulse` |
| 12 | `E_TIER` | the caster's skill is below a glyph's tier gate: tier 1, 2, 3, 4 need 0, 25, 50, 75 in the glyph's school; triggers and conditions gate on Spellcraft |
| 13 | `E_COMPLEXITY` | spell complexity (§4.4) exceeds the budget: Canon §6.4 row 26, `4 + floor_div(LOG, 10)`, plus doc 04 `RowFlat(26, ·)` grants (talents `complex_lattice` and `grand_design`) and Echo traits (`echo.wren_long_index` −1) |

#### 4.3.1 Form–essence compatibility

| Essences | Allowed forms |
|---|---|
| the 12 damage essences | bolt, orb, beam, cone, touch, aura, field, wall, rune, nova, chain |
| mend, ward | self, touch, bolt, orb, cone, aura, field, nova, chain |
| swiftness, levitate, veil | self, touch, bolt, orb, aura, nova, chain |
| bind | bolt, orb, cone, touch, field, wall, rune, nova, chain |
| dread, dominion, slumber, hush | bolt, orb, cone, touch, aura, field, rune, nova, chain |
| polymorph | bolt, touch |
| unmaking | self, touch, bolt, orb, field, nova, chain |
| summon | manifest; manifest accepts only summon as its primary essence |
| shape | bolt, touch, wall, nova |
| tide | bolt, orb, beam, cone, touch, field, nova |
| gale | bolt, beam, cone, aura, nova |
| lumen | self, touch, bolt, orb, aura, field |
| sight | self, bolt, orb, field, nova |
| appraise | self, touch |
| omen | self, nova |
| psychometry | self |

### 4.4 Complexity

```
complexity(stage, depth) = form.cx + Σ essence.cx + Σ modifier.cx + trigger.cx + condition.cx   // condition.cx = 0 with Divination 75
                         + (if depth >= 1 { max(0, 2 × depth − spellcraft_hook) } else { 0 })    // spellcraft_hook = 1 with Spellcraft 50, else 0
                         + complexity(payload, depth + 1)
```

Examples: `bolt flame` = 1 + 1 = 2. `orb flame on_impact foe ( nova frost )` = 2 + 1 + 2 + 1 + (2 + 1 + 2) = 11, which fails `E_COMPLEXITY` for LOG 50 (budget 9). `bolt flame on_impact ( nova frost )` = 1 + 1 + 2 + (2 + 1 + 2) = 9, which fits budget 9 exactly.

### 4.5 Compile algorithm

```rust
pub struct CompiledSpell {
    pub hash: u64, pub canonical: String, pub register: GenreId, pub school: SchoolId,
    pub stages: Vec<CompiledStage>,          // index 0 = root, then depth-first payloads
    pub complexity: i32, pub base_cost: i64, // m-aether; depends on the caster only through 3 perks (§6.1, §7.3)
    pub cast_ticks_raw: u32, pub cooldown: u32, pub cast_type: CastType,
    pub required: BTreeMap<SkillId, i32>,    // tier gates
    pub name_key: NameKey,                   // §17
}
pub struct CompiledStage {
    pub depth: u8, pub form: GlyphRef, pub essences: Vec<GlyphRef>, pub modifiers: Vec<GlyphRef>,
    pub trigger: Option<GlyphRef>, pub condition: Option<GlyphRef>, pub payload: Option<usize>,
    pub stage_cost: i64,                     // §6.1, before depth and fire factors
    pub dmg_base_mhp: i64, pub shares: Vec<(DmgType, i32)>, pub status_powers: Vec<(StatusId, i32, DmgType)>,
    pub heal_bp: i32, pub ward_bp: i32, pub duration_bp: i32, pub mag_bp: i32,
    pub range_mm: i32, pub speed_mm_s: i32, pub radius_mm: i32, pub half_angle: u16, pub polarity: Polarity,
}

pub fn compile(refs: &[GlyphRef], caster: Option<&CasterStats>) -> Result<CompiledSpell, CompileError> {
    let tree = parse(refs)?;                                  // checks 1, 2
    if let Some(c) = caster { check_lexicon(&tree, c)?; }     // check 3
    validate_stages(&tree)?;                                  // checks 4 to 11, stage by stage, depth-first
    if let Some(c) = caster { check_tiers(&tree, c)?; }       // check 12
    let cx = complexity(&tree.root, 0, caster.map_or(0, |c| c.spellcraft_hook()));
    if let Some(c) = caster { if cx > c.row26_budget() { return Err(E_COMPLEXITY); } }   // check 13
    let stages = flatten_depth_first(&tree).map(compute_stage_stats);   // §6.1, §6.4
    let base_cost = tree_cost(&tree);                         // §6.1
    let canonical = canonical_text(&tree);
    Ok(CompiledSpell { hash: fnv1a64(&canonical), name_key: name_key(&tree), .. })
}
```

Compilation is pure: the same glyph references and the same caster stats always produce the same `CompiledSpell`. A Grimoire slot stores only the glyph references. The compiled form is a cache that is rebuilt when LOG, a school skill, Spellcraft or the Lexicon changes; if the rebuilt spell fails a check, the slot stays filled but greyed until it passes again.

### 4.6 Data schemas

Glyph record (one per glyph, `data/magic/glyphs.json`, validated by JSON Schema; integers only):

```json
{ "id": "glyph.form.orb", "class": "form", "school": "school.evocation", "tier": 1, "cx": 2, "pool": "base",
  "cost_maero": 2000, "cast_type": "cast_time", "cast_ticks": 36, "mag_bp": 8000,
  "range_mm": 30000, "speed_mm_s": 15000, "radius_mm": 2000, "duration_ticks": 0,
  "pulse_ticks": 0, "cooldown_ticks": 0, "noise_db": 60 }
```

```json
{ "id": "glyph.ess.flame", "class": "essence", "kind": "damage", "polarity": "harm",
  "school": "school.evocation", "tier": 1, "cx": 1, "cost_maero": 4000, "cast_add": 0, "cd_add": 0,
  "dmg_type": "dmg.fire", "base_mhp": 24000, "status": "st.burning", "status_power": 40,
  "forms": ["glyph.form.bolt","glyph.form.orb","glyph.form.beam","glyph.form.cone","glyph.form.touch","glyph.form.aura",
            "glyph.form.field","glyph.form.wall","glyph.form.rune","glyph.form.nova","glyph.form.chain"],
  "opposed": "glyph.ess.frost", "block_power": 0 }
```

```json
{ "id": "glyph.mod.empower", "class": "modifier", "school": "school.spellcraft", "tier": 1, "cx": 1,
  "cost_add_bp": 5000, "mag_bp": 13000, "cast_add": 0, "cd_add": 0, "forms": "all" }
```

```json
{ "id": "glyph.trig.on_impact", "class": "trigger", "school": "school.spellcraft", "tier": 1, "cx": 2,
  "cost_maero": 1000, "max_fires": 1, "fires_bp": 10000,
  "hosts": ["glyph.form.bolt","glyph.form.orb","glyph.form.chain","glyph.form.rune"] }
```

```json
{ "id": "glyph.cond.foe", "class": "condition", "school": "school.spellcraft", "tier": 1, "cx": 1, "cost_maero": 500 }
```

Authored spell record (`spell.` content; used by wands, texts, NPCs and Echo kits):

```json
{ "id": "spell.cinder_bolt", "origin_genre": "genre.mythic_fantasy",
  "root": { "form": "glyph.form.bolt", "essences": ["glyph.ess.flame"], "modifiers": [],
            "trigger": "glyph.trig.on_impact", "condition": null,
            "payload": { "form": "glyph.form.nova", "essences": ["glyph.ess.frost"], "modifiers": [] } },
  "ai": { "tags": ["damage_single","damage_area"], "min_range_mm": 3000, "max_range_mm": 30000,
          "min_targets": 1, "avoid_allies_mm": 5000 } }
```

An authored spell's glyphs all take the record's `origin_genre` unless a glyph entry is written as `{ "id": ..., "origin": "genre.x" }`. When a wand, text or NPC is generated or Rewritten in another genre, every glyph origin of its spell becomes that genre. `pool` is `base`, `glyphs_2` or `family.<name>` (§8.1). Authored spells skip checks 3, 12 and 13 at cast time, so wands, texts and NPCs cast them whatever the caster's Lexicon, skills and LOG.

A wand-pool spell carries its tag and draw weight (doc 10 §4.2, doc 11):

```json
{ "id": "spell.wand_striking", "origin_genre": "genre.mythic_fantasy", "tags": ["wand_pool"], "wand_weight": 800,
  "root": { "form": "glyph.form.bolt", "essences": ["glyph.ess.force"], "modifiers": [],
            "trigger": null, "condition": null, "payload": null },
  "ai": { "tags": ["damage_single"], "min_range_mm": 2000, "max_range_mm": 40000, "min_targets": 1, "avoid_allies_mm": 0 } }
```

Summon template (`data/magic/summons.json`; values at `P = 10000` and `pot = 10000`, §12.2):

```json
{ "id": "summon.elemental", "body": "body.amorphous", "hp_mhp": 60000, "ar": 0,
  "attack": { "base_mhp": 8000, "type_from": "secondary_essence", "interval_ticks": 45, "reach_mm": 8000,
              "kind": "projectile", "status_power": 30 },
  "move_mm_s": 4000, "flies": false, "profile": "artillery", "duration_ticks": 3600, "light": 0,
  "secondary": ["glyph.ess.flame", "glyph.ess.frost", "glyph.ess.storm", "glyph.ess.acid", "glyph.ess.thunder", "glyph.ess.arcane"],
  "immune_own_type": true, "tags": [] }
```

Polymorph template (§12.3):

```json
{ "id": "summon.poly.toad", "registers": ["genre.fairy_tale", "genre.wuxia"], "body": "body.quadruped",
  "capsule_r_mm": 200, "capsule_h_mm": 400, "move_mm_s": 2400, "jump_mm": 1500, "swim_mm_s": 3000,
  "attack": { "reach_mm": 1500, "ref_bp": 1000, "interval_ticks": 60 }, "resist": {} }
```

Runtime Grimoire slot (part of the saved simulation state):

```json
{ "slot": 0, "refs": [ { "glyph": "0xfbdaf888", "origin": "0x99b54427" }, { "glyph": "0xf8199aa5", "origin": "0x99b54427" } ] }
```

`glyph` and `origin` are `id32` values written as hex strings: `fnv1a32("glyph.form.bolt") = 0xfbdaf888`, `fnv1a32("glyph.ess.storm") = 0xf8199aa5`, `fnv1a32("genre.cyberpunk") = 0x99b54427`. The slot above is a Cyberpunk `Packet` of `Surge`. The canonical text `glyph.form.bolt@mythic_fantasy glyph.ess.flame@mythic_fantasy` hashes to `spell_hash = 0x9d32104fc5e81a6e`.

## 5. Glyph catalog

### 5.1 Dependency and skin of every glyph

Every glyph in this catalog has exactly one dependency axis, set by its glyph origin through the §2.2 table, and that table is the full per-genre dependency list for all 84 glyphs. Its display name in a genre is `loc.glyph.<class>.<name>.skin.<genre_suffix>.name` (Canon §1.10). §5.7 gives the canonical English skin of every glyph in all 15 genres that have a register, the Form rows of ten of them taken unchanged from doc 18 §7.5; doc 18 owns the localization tables and translates these strings. `genre.unwritten` has no glyph skins: there the Canon §1.10 resolution falls through to the glyph-origin skin, so a glyph whose origin is `genre.ur_text` (an Unwritten Origin's run-start glyphs, §8.1) shows its Ur-Text name, and the archetype name (the Mythic Fantasy column) appears only where both lookups miss.

Glyph tiers gate compilation (§4.3 check 12). Cost columns are in m-aether. Cx is complexity points. Magnitude is the stage multiplier of §6.4. Pulse forms state magnitude per pulse.

### 5.2 Forms

| ID | Cast type | Cast ticks | Cost (m-aether) | Cx | Tier | School | Magnitude (Bp) | Range (mm) | Speed (mm/s) | Radius (mm) | Duration (ticks) | Pulse (ticks) | Cooldown (ticks) | Cast noise (dB) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `glyph.form.bolt` | cast_time | 24 | 1000 | 1 | 1 | evocation | 10000 | 40000 | 30000 | 0 | 0 | 0 | 0 | 55 |
| `glyph.form.orb` | cast_time | 36 | 2000 | 2 | 1 | evocation | 8000 | 30000 | 15000 | 2000 | 0 | 0 | 0 | 60 |
| `glyph.form.beam` | channel | 12 | 2000 | 2 | 2 | evocation | 3000 | 12000 | 0 | 0 | 300 | 15 | 0 | 50 |
| `glyph.form.cone` | cast_time | 12 | 1500 | 1 | 1 | evocation | 7000 | 6000 | 0 | 0 | 0 | 0 | 60 | 60 |
| `glyph.form.touch` | instant | 0 | 0 | 1 | 1 | alteration | 13000 | 2500 | 0 | 0 | 0 | 0 | 0 | 40 |
| `glyph.form.self` | instant | 0 | 0 | 0 | 1 | alteration | 10000 | 0 | 0 | 0 | 0 | 0 | 0 | 40 |
| `glyph.form.aura` | cast_time | 30 | 3000 | 2 | 2 | abjuration | 600 | 0 | 0 | 4000 | 600 | 30 | 900 | 50 |
| `glyph.form.field` | cast_time | 30 | 2500 | 2 | 2 | alteration | 800 | 25000 | 0 | 3000 | 480 | 30 | 300 | 55 |
| `glyph.form.wall` | cast_time | 30 | 2000 | 2 | 2 | alteration | 800 | 20000 | 0 | 0 | 600 | 30 | 300 | 55 |
| `glyph.form.rune` | cast_time | 48 | 1500 | 2 | 2 | abjuration | 12000 | 4000 | 0 | 2500 | 3600 | 0 | 0 | 30 |
| `glyph.form.nova` | cast_time | 24 | 3000 | 2 | 2 | evocation | 7000 | 0 | 0 | 5000 | 0 | 0 | 180 | 75 |
| `glyph.form.chain` | cast_time | 30 | 2500 | 2 | 3 | evocation | 8000 | 25000 | 0 | 6000 | 0 | 0 | 0 | 60 |
| `glyph.form.manifest` | cast_time | 60 | 3000 | 2 | 1 | conjuration | 10000 | 15000 | 0 | 0 | 3600 | 0 | 600 | 50 |

Form behavior. **Line of effect** means the segment between two points crosses no block with collision (voxel DDA, doc 03). Area membership uses an entity's center (origin + half its capsule height) and requires line of effect from the area center. Every area hit carries the `area` flag (Canon §10.5 step 2.10).

| Form | Behavior |
|---|---|
| bolt | Projectile, capsule radius 150 mm, launched from the eye along the aim, no gravity, counts toward `PROJECTILE_CAP`. Ends on the first entity or collision block (impact) or at max range (despawn, no effect). Helpful bolts pass through non-allies. Hit tests are doc 05 swept tests. |
| orb | Projectile, radius 300 mm, no gravity. Bursts at impact or at max range: sphere of the form radius around the burst point. |
| beam | Hitscan from the eye each pulse, to range. Hits the first entity or block. |
| cone | Resolves once: every entity within range whose center lies within the half-angle of the aim vector (angle test by dot product against `cos_bam(half_angle)` in Q2.30). Half-angle 6144 Bam. |
| touch | The first entity whose hitbox the aim ray meets within 2500 mm, or the block it meets for world essences. A helpful touch with no ally under the ray targets the caster. |
| self | The caster. |
| aura | Sphere of radius 4000 mm around the caster center, following the caster; pulses every 30 ticks for 600 ticks. One active aura per caster; a new one ends the old one. |
| field | Placed on top of the first collision block the aim ray meets within 25000 mm: a cylinder of radius 3000 mm and height 3000 mm. Pulses every 30 ticks for 480 ticks. Two active fields per caster; a third ends the oldest. |
| wall | A plane of 5 × 3 cells centered on the aimed ground cell, perpendicular to the caster yaw rounded to the nearest multiple of 16384 Bam. Occupies air and liquid cells only. Damage walls pulse on entities inside their cells. One active wall per caster. |
| rune | Placed on the block face the aim ray meets within 4000 mm. Arms 60 ticks after placement. Bursts when a hostile origin is within 1500 mm (checked every tick), as a sphere of radius 2500 mm at the rune magnitude. Lasts 3600 ticks. Three armed runes per caster; a fourth ends the oldest. Hostiles see runes only through doc 09 trap detection. |
| nova | Sphere of radius 5000 mm around the caster center (or the payload origin). A harmful nova excludes the caster; a helpful or neutral nova includes it. |
| chain | Hitscan to the first hostile on the aim ray within 25000 mm, then up to 3 jumps, each to the hostile not yet hit with minimum `dist2` from the previous target within 6000 mm (ties to lower EntityId), each with line of effect. First hit magnitude 8000; each jump × 7500 of the previous hit. A helpful chain starts on an ally and jumps among allies. |
| manifest | Spawns the summon (§12) at the aimed standable cell within 15000 mm with the template's headroom. |

### 5.3 Essences

| ID | Kind | Polarity | School | Tier | Cost (m-aether) | Cx | Cast add (ticks) | Cooldown add (ticks) | Payload |
|---|---|---|---|---|---|---|---|---|---|
| `glyph.ess.flame` | damage | harm | evocation | 1 | 4000 | 1 | 0 | 0 | `dmg.fire` base 24000 mHP; `st.burning` power 40 |
| `glyph.ess.frost` | damage | harm | evocation | 1 | 4000 | 1 | 0 | 0 | `dmg.frost` base 20000 mHP; `st.frozen` power 35 |
| `glyph.ess.storm` | damage | harm | evocation | 1 | 4500 | 1 | 0 | 0 | `dmg.shock` base 22000 mHP; `st.shocked` power 35 |
| `glyph.ess.acid` | damage | harm | alteration | 1 | 4000 | 1 | 0 | 0 | `dmg.acid` base 20000 mHP; `st.corroded` power 45 |
| `glyph.ess.force` | damage | harm | alteration | 1 | 3500 | 1 | 0 | 0 | `dmg.blunt` base 22000 mHP; `st.stunned` power 25 |
| `glyph.ess.edge` | damage | harm | conjuration | 1 | 3500 | 1 | 0 | 0 | `dmg.slash` base 22000 mHP; `st.bleeding` power 35 |
| `glyph.ess.radiance` | damage | harm | restoration | 2 | 5000 | 2 | 0 | 0 | `dmg.radiant` base 22000 mHP; `st.blinded` power 35 |
| `glyph.ess.shadow` | damage | harm | conjuration | 2 | 5000 | 2 | 0 | 0 | `dmg.umbral` base 22000 mHP; `st.withering` power 40 |
| `glyph.ess.mind` | damage | harm | illusion | 2 | 5000 | 2 | 0 | 0 | `dmg.psychic` base 20000 mHP; `st.dread` power 45 |
| `glyph.ess.arcane` | damage | harm | evocation | 1 | 4500 | 1 | 0 | 0 | `dmg.arcane` base 21000 mHP; `st.unraveled` power 35 |
| `glyph.ess.venom` | damage | harm | alteration | 1 | 4000 | 1 | 0 | 0 | `dmg.toxin` base 18000 mHP; `st.poisoned` power 50 |
| `glyph.ess.thunder` | damage | harm | evocation | 2 | 4500 | 1 | 0 | 0 | `dmg.sonic` base 20000 mHP; `st.deafened` power 40 |
| `glyph.ess.mend` | heal | help | restoration | 1 | 5000 | 1 | 0 | 0 | heal 2000 Bp of recipient Max HP |
| `glyph.ess.ward` | heal | help | abjuration | 1 | 5000 | 1 | 0 | 0 | `st.shielded` pool 2500 Bp of recipient Max HP, 600 ticks |
| `glyph.ess.swiftness` | buff | help | alteration | 2 | 5000 | 2 | 0 | 0 | direct `st.hasted`, base 480 ticks |
| `glyph.ess.levitate` | buff | help | alteration | 2 | 4000 | 1 | 0 | 0 | direct `st.levitating`, base 600 ticks |
| `glyph.ess.veil` | buff | help | illusion | 2 | 6000 | 2 | 0 | 600 | direct `st.invisible`, base 600 ticks |
| `glyph.ess.bind` | control | harm | alteration | 1 | 4000 | 1 | 0 | 0 | direct `st.rooted` (single-target forms) or `st.slowed` (area forms) |
| `glyph.ess.dread` | control | harm | illusion | 1 | 4000 | 1 | 0 | 0 | `st.feared` power 60, carrier `dmg.psychic` |
| `glyph.ess.dominion` | control | harm | illusion | 3 | 7000 | 2 | 12 | 300 | `st.charmed` power 40, carrier `dmg.psychic` |
| `glyph.ess.slumber` | control | harm | illusion | 2 | 5000 | 1 | 0 | 0 | `st.sleeping` power 50, carrier `dmg.psychic` |
| `glyph.ess.hush` | control | harm | abjuration | 2 | 5000 | 1 | 0 | 0 | `st.silenced` power 50, carrier `dmg.arcane` |
| `glyph.ess.polymorph` | poly | harm | alteration | 3 | 9000 | 3 | 30 | 900 | direct `st.polymorphed`, template by register (§12.3) |
| `glyph.ess.unmaking` | dispel | neutral | abjuration | 2 | 6000 | 2 | 0 | 0 | dispel count 2 + floor_div(abjuration, 25) (§14.1) |
| `glyph.ess.summon` | summon | neutral | conjuration | 1 | 8000 | 2 | 0 | 0 | one minion, template by second essence (§12.2) |
| `glyph.ess.shape` | world | neutral | alteration | 2 | 4000 | 2 | 0 | 0 | break or place blocks, shape power 3 + floor_div(alteration, 25) (§13.2) |
| `glyph.ess.tide` | world | neutral | conjuration | 1 | 3000 | 1 | 0 | 0 | creates `liq.water`, 2000 ml per bolt impact (§13.1) |
| `glyph.ess.gale` | world | neutral | alteration | 1 | 3000 | 1 | 0 | 0 | impulse 8000 mm/s; moves gas cells (§13.1) |
| `glyph.ess.lumen` | world | neutral | divination | 1 | 2000 | 1 | 0 | 0 | light source level 14 (§13.1) |
| `glyph.ess.sight` | world | neutral | divination | 1 | 4000 | 1 | 0 | 0 | direct `st.marked` 480 ticks; reveals hidden objects (§13.1); applies doc 10's `appr.eff.aura` to items within the omen radius (doc 10 §5.10) |
| `glyph.ess.appraise` | lore | neutral | divination | 2 | 8000 | 2 | 60 | 3600 | doc 10 `appr.eff.divine_identify` on one chosen item (doc 10 §5.10) |
| `glyph.ess.omen` | lore | neutral | divination | 1 | 4000 | 1 | 0 | 600 | doc 10 `appr.eff.divine_omen` within `4000 + 2000 × div_tier` mm |
| `glyph.ess.psychometry` | lore | neutral | divination | 2 | 5000 | 2 | 30 | 1800 | doc 10 `appr.eff.psychometry`: one unknown type-hidden item in the caster's inventory; 1 keyed `rng.ident` draw |

Essence rules:

- **Damage essences** produce one hit per target per resolution (per pulse for pulse forms) through Canon §10.5. `glyph.ess.frost` hits also apply `st.chilled` (Canon §10.1). `glyph.ess.force` adds a knockback impulse of 5000 mm/s horizontally along the hit direction plus 2000 mm/s upward to targets with body mass at most 400,000 g that are not of rank boss, editor or hunter. `glyph.ess.thunder` hits emit noise by Canon §10.1. Block effects are in §13.
- **Status power on aura, field and wall pulses** uses `max(form magnitude, 2500)` Bp instead of the form magnitude, so each pulse carries 2500 Bp of the essence's status power. Beam pulses use the beam magnitude (3000).
- **Lore essences** (appraise, omen, psychometry) produce no hit. Their item target is fixed at cast start: for `self`, the inventory item uid carried in the cast command (a doc 17 wheel sub-selection; when doc 10 rejects the target the cast does not start); for `touch`, the ground item nearest the aim ray within 2500 mm. Contradiction potency multiplies their radii and chances (doc 10 §5.10). Their 3600, 1800 and 600-tick cooldown keys (§3.4) keep spells from replacing doc 10's identification economy.
- **Carrier.** A control essence's hit carries 0 damage and the listed carrier type for the Canon §11.2 resistance factor. Immunity to the carrier type blocks the status.
- **Direct statuses** use Canon §11.4 with base `mul_bp(status base, duration_bp)` (§6.4). Area forms of `glyph.ess.bind` apply `st.slowed` (base 240 ticks); single-target forms apply `st.rooted` (base 120 ticks). On pulse forms every pulse re-applies with refresh semantics.
- **Polarity** decides targets. A stage is harmful if any essence is harmful, helpful if any essence is helpful (check 7 forbids both), and neutral otherwise. Harmful stages affect hostiles, and allies only through friendly fire (Canon §17.5); helpful stages affect the caster and `fac.errata` members only; neutral stages affect everything in the area, and their world effects ignore faction. A harmful root resolution ends the caster's `st.invisible` (§15).

### 5.4 Modifiers

| ID | Cx | Tier | School | Cost add (Bp) | Applies to | Effect |
|---|---|---|---|---|---|---|
| `glyph.mod.empower` | 1 | 1 | spellcraft | +5000 | all forms | magnitude × 13000 |
| `glyph.mod.extend` | 1 | 1 | alteration | +4000 | stages with a duration, a direct status or a summon | every duration of the stage × 20000 |
| `glyph.mod.widen` | 1 | 2 | evocation | +4000 | orb, cone, aura, field, rune, nova, chain | radius, cone half-angle and chain jump range × 15000 (half-angle capped at 16384 Bam); magnitude × 9000 |
| `glyph.mod.reach` | 1 | 1 | spellcraft | +2000 | bolt, orb, beam, cone, chain, field, wall, rune, manifest, touch | range × 20000; projectile speed × 15000 |
| `glyph.mod.quicken` | 1 | 2 | spellcraft | +3000 | cast_time forms | cast ticks × 5000; a result of 6 or less becomes instant |
| `glyph.mod.charged` | 1 | 1 | evocation | +0 | bolt, orb, cone, nova, chain | cast type becomes charge (§3.3); magnitude and cost × charge factor |
| `glyph.mod.split` | 2 | 2 | evocation | +6000 | bolt, orb | 3 projectiles at yaw offsets −1820, 0, +1820 Bam; magnitude × 4500 each |
| `glyph.mod.homing` | 1 | 2 | divination | +3000 | bolt, orb | steers toward a locked hostile at 136 Bam per tick |
| `glyph.mod.pierce` | 1 | 2 | evocation | +3000 | bolt, beam | passes through 2 more entities; each later hit × 8000 of the previous |
| `glyph.mod.ricochet` | 1 | 2 | spellcraft | +2000 | bolt, orb | bounces off up to 3 solid blocks; magnitude × 9000 per bounce |
| `glyph.mod.linger` | 2 | 2 | conjuration | +5000 | bolt, orb, nova, rune | impact leaves a 2000 mm field of the stage essences for 300 ticks at pulse magnitude 800 per 30 ticks, plus surfaces (§13.1) |
| `glyph.mod.delay` | 1 | 2 | spellcraft | +2000 | bolt, orb, nova | resolution waits 60 ticks (rules below); magnitude × 12000 |
| `glyph.mod.hushed` | 1 | 1 | illusion | +3000 | all forms | every noise of the stage −30 dB, floor 0 |
| `glyph.mod.thrift` | 1 | 1 | spellcraft | -2500 | all forms | magnitude × 8500 |
| `glyph.mod.volatile` | 1 | 1 | evocation | +0 | stages with magnitude | magnitude × (5000 + v0), v0 from rng.magic (§3.9) |
| `glyph.mod.keen` | 1 | 2 | divination | +3000 | damaging stages | crit chance +1500 Bp; bolt and chain hits on precision parts use precision multiplier 15000 |
| `glyph.mod.siphon` | 2 | 3 | abjuration | +4000 | damaging stages | caster regains aether per HP hit (§6.6) |
| `glyph.mod.leech` | 2 | 3 | restoration | +4000 | damaging stages | caster heals per HP hit (§6.6) |
| `glyph.mod.sculpt` | 1 | 2 | abjuration | +3000 | orb, cone, aura, field, wall, rune, nova, chain | harmful area effects skip every `fac.errata` member |
| `glyph.mod.anchor` | 1 | 1 | alteration | +2000 | aura, field, wall, rune | aura stays at the cast position; duration × 15000 |
| `glyph.mod.heavy` | 1 | 1 | alteration | +3000 | damaging stages | poise damage × 20000; knockback +4000 mm/s; projectile speed × 7500 |
| `glyph.mod.twin` | 2 | 3 | spellcraft | +7000 | every form except beam, manifest and stages with polymorph | the stage resolves again 20 ticks later at magnitude × 6000 |

Modifier rules:

- Magnitude multipliers apply in glyph order after the form magnitude (§6.4). A modifier whose own Contradiction state is Strained or Contradicted contributes `10000 + mul_bp(mag_bp − 10000, its potency)`; a Disconnected modifier contributes nothing and its other effects are off (Canon §5.5 per-component scope). Its cost still counts.
- `glyph.mod.homing` locks at launch onto the hostile with the smallest angle to the aim inside an 8192 Bam cone within 15000 mm (ties to lower EntityId) and turns the velocity toward it by at most 136 Bam per tick in yaw and in pitch. With no lock it flies straight.
- `glyph.mod.split` projectiles each count toward `PROJECTILE_CAP` and each can carry the stage's trigger independently.
- `glyph.mod.twin` repeats the whole stage, including its trigger; it never repeats the cost, the failure roll or the cooldown.
- `glyph.mod.volatile` uses `v0` of §3.9: factor `5000 + v0`, so 5000 to 14999 Bp.
- Modifiers add 0 cast ticks and 0 cooldown; the only cast-tick changes are quicken's and charged's, given in the table.
- `glyph.mod.delay`: a bolt sticks to the entity or block it hits and the stage resolves 60 ticks later on that entity if it is alive, else at the stuck point; an orb bursts 60 ticks after impact at the impact point; a nova resolves 60 ticks after the cast completes, centered on the caster's position at that tick. Triggers of the stage fire from the delayed resolution. Its cost add was +0, which made +20% magnitude free on every single-target bolt; at +2000 its cost rises in step with its magnitude.
- `glyph.mod.siphon` and `glyph.mod.leech` are capped per root resolution (§6.6).

### 5.5 Triggers

| ID | Cx | Tier (spellcraft) | Flat cost (m-aether) | Fires (max) | Fire cost factor (Bp) | Host forms | Fires when | Payload origin and aim |
|---|---|---|---|---|---|---|---|---|
| `glyph.trig.on_impact` | 2 | 1 | 1000 | 1 | 10000 | bolt, orb, chain, rune | the host projectile or burst first touches an entity or a solid block; the last chain hit | impact point; aim = host travel direction |
| `glyph.trig.on_kill` | 2 | 2 | 1500 | 3 | 20000 | any damaging stage | a hit of the host stage kills an entity | the corpse origin; aim = host hit direction |
| `glyph.trig.on_expire` | 2 | 2 | 1000 | 1 | 10000 | aura, field, wall, rune, manifest | the host entity ends by expiry, dispel or death | host position; aim = caster aim at that tick |
| `glyph.trig.timer` | 2 | 1 | 1000 | 1 | 10000 | bolt, orb | 45 ticks after launch; the host projectile ends | projectile position; aim = travel direction |
| `glyph.trig.proximity` | 2 | 2 | 1500 | 1 | 10000 | bolt, orb, field, wall | a hostile origin comes within 2000 mm of the host | host position; aim = toward that hostile |
| `glyph.trig.on_struck` | 3 | 3 | 2000 | 1 | 10000 | self, touch with a helpful essence | the bearer takes a non-DoT hostile hit within 600 ticks | bearer position; aim = toward the attacker |
| `glyph.trig.on_status` | 2 | 3 | 1500 | 3 | 20000 | stages that apply a status | a hit of the host stage fills a buildup meter or applies a direct status | the afflicted entity origin; aim = host hit direction |
| `glyph.trig.pulse` | 3 | 3 | 2500 | 5 | 40000 | aura, field, wall, manifest | every 120 ticks while the host exists | host position; aim = caster aim at that tick |

Trigger rules:

1. The payload stage resolves on the tick after the event (event tick + 1), owned by the original caster, with no cast time, cost, cooldown or failure roll. Its aether was prepaid in the base cost (§6.1).
2. The payload uses the caster's **snapshot** taken at root resolution: ATN, school skills, Spellcraft, implement step-0 factors and tolerance. Local axioms are read at the caster's current position, or the last position if the caster died.
3. Caster-centered forms in a payload (nova, aura is forbidden, self) center on the payload origin, except `self`, which targets the caster.
4. A trigger fires at most `max_fires` times per host instance (+1 with Spellcraft 75, §7.3). A host that ends early fires nothing more.
5. **Subjects** for conditions: on_impact the entity hit, or the impact cell; on_kill the killed entity; on_expire, timer and pulse the host cell; proximity the approaching hostile; on_struck the attacker; on_status the afflicted entity.

### 5.6 Conditions

| ID | Cx | Tier (spellcraft) | Flat cost (m-aether) | Passes when |
|---|---|---|---|---|
| `glyph.cond.foe` | 1 | 1 | 500 | subject entity is hostile to the caster (§15 stance hostile, or `st.charmed` by the caster reversed) |
| `glyph.cond.ally` | 1 | 1 | 500 | subject entity is the caster or a `fac.errata` member |
| `glyph.cond.wounded` | 1 | 1 | 500 | subject entity HP is below 5000 Bp of its Max HP |
| `glyph.cond.native` | 1 | 2 | 500 | subject entity is native (Canon §5.6) |
| `glyph.cond.wet` | 1 | 1 | 500 | subject has `st.soaked` or Contact (Canon §12.1) with any liquid; a cell subject holds at least 100 ml of liquid |
| `glyph.cond.burning` | 1 | 1 | 500 | subject has `st.burning`; a cell subject is on fire (doc 03 fire grid) |
| `glyph.cond.dark` | 1 | 2 | 500 | `light_at(subject)` at most 4; a cell subject has effective light at most 4 |
| `glyph.cond.unaware` | 1 | 2 | 500 | subject is an NPC whose awareness of the caster is below `aware.alert`, or has `st.sleeping` |

Condition rules: entity tests (foe, ally, wounded, native, unaware) are false on a cell subject. Cell tests (wet, burning, dark) on an entity subject read the entity's eye cell and status. A failed condition consumes the fire of a single-fire trigger (on_impact, timer, on_expire). For proximity, on_struck, on_kill, on_status and pulse, a failed condition consumes nothing and the trigger keeps waiting until its host ends.

### 5.7 Skin names

Forms and essences:

| Glyph | Mythic Fantasy | Cyberpunk | Wuxia | Cosmic Horror | Steampunk |
|---|---|---|---|---|---|
| `glyph.form.bolt` | Bolt | Packet | Flying Needle | Sigil-Dart | Pneumatic Shot |
| `glyph.form.orb` | Orb | Payload | Rolling Pearl | Bloated Sphere | Aether Shell |
| `glyph.form.beam` | Ray | Stream | Unbroken Thread | Gaze | Arc-Jet |
| `glyph.form.cone` | Breath | Burst | Fan Strike | Exhalation | Blast Nozzle |
| `glyph.form.touch` | Touch | Jack-In | Palm | Laying-On | Contact Plate |
| `glyph.form.self` | Self | Localhost | Inner Gate | Within | Personal Rig |
| `glyph.form.aura` | Aura | Broadcast | Qi Mantle | Miasma of Being | Pressure Halo |
| `glyph.form.field` | Circle | Zone Lock | Formation | Unholy Ground | Vent Field |
| `glyph.form.wall` | Wall | Firewall | Jade Screen | Veil Between | Bulkhead |
| `glyph.form.rune` | Rune | Logic Bomb | Talisman Seal | Elder Mark | Pressure Mine |
| `glyph.form.nova` | Nova | Overload | Lion Roar | Revelation | Boiler Burst |
| `glyph.form.chain` | Chain | Daisy-Chain | Linked Lotus | Contagion | Tesla Relay |
| `glyph.form.manifest` | Summoning | Spawn Process | Spirit Calling | Calling | Automaton Assembly |

| Glyph | Mythic Fantasy | Cyberpunk | Wuxia | Cosmic Horror | Steampunk |
|---|---|---|---|---|---|
| `glyph.ess.flame` | Flame | Thermal | Crimson Phoenix | Black Fire | Firebox |
| `glyph.ess.frost` | Frost | Cryo | Winter Plum | Void Chill | Refrigerant |
| `glyph.ess.storm` | Lightning | Surge | Heaven Lightning | Crawling Spark | Galvanic |
| `glyph.ess.acid` | Acid | Corrosive | Rotting Dew | Dissolution | Vitriol |
| `glyph.ess.force` | Force | Kinetic | Mountain Force | Weight of Stars | Piston |
| `glyph.ess.edge` | Blades | Monofilament | Sword Qi | Flaying | Shrapnel |
| `glyph.ess.radiance` | Radiance | Holo-Glare | Golden Sun | Unbearable Light | Limelight |
| `glyph.ess.shadow` | Shadow | Blackout | Yin Night | Outer Dark | Soot |
| `glyph.ess.mind` | Mind | Neural Spike | Heart Demon | Madness | Mesmeric |
| `glyph.ess.arcane` | Arcana | Raw Data | Pure Qi | Gnosis | Aether |
| `glyph.ess.venom` | Venom | Nanotoxin | Five Poisons | Spore | Arsenic |
| `glyph.ess.thunder` | Thunder | Subsonic | Thunderclap | Piping Choir | Steam Whistle |
| `glyph.ess.mend` | Mending | Nanorepair | Meridian Healing | Flesh-Knitting | Patch Kit |
| `glyph.ess.ward` | Ward | Shield Proxy | Golden Bell | Circle of Salt | Aegis Plate |
| `glyph.ess.swiftness` | Haste | Overclock | Lightness Step | Hastening Hours | Governor Override |
| `glyph.ess.levitate` | Levitation | Grav-Null | Cloud Ladder | Unmooring | Balloon Lift |
| `glyph.ess.veil` | Invisibility | Optical Camo | Shadowless Step | Unseen | Refraction Cloak |
| `glyph.ess.bind` | Binding | Lockdown | Pressure Point | Clutching Hands | Clamp |
| `glyph.ess.dread` | Fear | Panic Feed | Killing Intent | Dread | Banshee Siren |
| `glyph.ess.dominion` | Domination | Override | Heart-Binding Art | Possession | Mesmer Coil |
| `glyph.ess.slumber` | Sleep | Sleep Mode | Dream Acupoint | Unending Dream | Laudanum Mist |
| `glyph.ess.hush` | Silence | Jammer | Sealing Acupoint | Unspeaking | Muffler |
| `glyph.ess.polymorph` | Polymorph | Avatar Swap | Body-Changing Art | Reshaping | Transmogrifier |
| `glyph.ess.unmaking` | Dispel | Purge | Meridian Severing | Banishment | Short Circuit |
| `glyph.ess.summon` | Conjure | Daemon | Spirit Beast | Thing Beyond | Automaton |
| `glyph.ess.shape` | Stoneshape | Fabricate | Mountain Moving | Non-Euclid | Excavator |
| `glyph.ess.tide` | Water | Coolant | Flowing River | Brine | Condenser |
| `glyph.ess.gale` | Wind | Fan Array | Wind Palm | Breath of Void | Bellows |
| `glyph.ess.lumen` | Light | Floodlight | Lantern Qi | Witchlight | Arc Lamp |
| `glyph.ess.sight` | Scrying | Scan | Heavenly Eye | Second Sight | Spectroscope |
| `glyph.ess.appraise` | Appraisal | Metadata Read | Discerning Eye | Unwelcome Knowledge | Assay |
| `glyph.ess.omen` | Omen | Threat Scan | Fortune Reading | Portent | Divining Rod |
| `glyph.ess.psychometry` | Psychometry | Data Recovery | Object Memory | Echoes of the Owner | Spectrograph |

Modifiers, triggers and conditions:

| Glyph | Mythic Fantasy | Cyberpunk | Wuxia | Cosmic Horror | Steampunk |
|---|---|---|---|---|---|
| `glyph.mod.empower` | Empowered | Amplified | Full Force | Swollen | High-Pressure |
| `glyph.mod.extend` | Extended | Persistent | Lingering Breath | Unending | Long-Burn |
| `glyph.mod.widen` | Widened | Wide-Band | Sweeping | Spreading | Flared |
| `glyph.mod.reach` | Far-Reaching | Long-Range | Thousand-Li | Distant | Rifled |
| `glyph.mod.quicken` | Quickened | Optimized | Flash | Hurried | Hair-Trigger |
| `glyph.mod.charged` | Charged | Spooled | Gathered | Gestating | Capacitor |
| `glyph.mod.split` | Split | Forked | Scattered Petals | Multiplying | Buckshot |
| `glyph.mod.homing` | Seeking | Target-Locked | Heart-Seeking | Hungry | Gyro-Guided |
| `glyph.mod.pierce` | Piercing | Penetrator | Through-Strike | Boring | Armor-Piercing |
| `glyph.mod.ricochet` | Ricochet | Bounce Protocol | Rebounding | Wandering | Deflector |
| `glyph.mod.linger` | Lingering | Residue | Afterimage | Staining | Slag |
| `glyph.mod.delay` | Delayed | Scheduled | Delayed Palm | Patient | Time-Fused |
| `glyph.mod.hushed` | Silent | Stealth Mode | Soundless | Whispered | Muffled |
| `glyph.mod.thrift` | Frugal | Lightweight | Economy of Motion | Starved | Low-Pressure |
| `glyph.mod.volatile` | Wild | Unstable Build | Reckless | Chaotic | Overpressurized |
| `glyph.mod.keen` | Keen | Precision | Vital-Point | Knowing | Calibrated |
| `glyph.mod.siphon` | Siphoning | Bandwidth Leech | Qi-Drinking | Feeding | Regenerative |
| `glyph.mod.leech` | Vampiric | Bio-Drain | Life-Stealing | Parasitic | Recuperative |
| `glyph.mod.sculpt` | Sculpted | IFF-Tagged | Discerning | Choosing | Safety-Valved |
| `glyph.mod.anchor` | Anchored | Static Host | Rooted Stance | Fixed | Bolted-Down |
| `glyph.mod.heavy` | Heavy | Hardened | Mountain-Crushing | Crushing | Steam-Hammer |
| `glyph.mod.twin` | Twinned | Duplicated | Twin Dragons | Echoing | Double-Action |

| Glyph | Mythic Fantasy | Cyberpunk | Wuxia | Cosmic Horror | Steampunk |
|---|---|---|---|---|---|
| `glyph.trig.on_impact` | upon Impact | on_hit() | Upon Touching | When It Arrives | Contact Fuse |
| `glyph.trig.on_kill` | upon Death | on_kill() | Upon Victory | When It Feeds | Kill Switch |
| `glyph.trig.on_expire` | upon Ending | on_exit() | When Breath Fails | When It Wanes | Run-Down Fuse |
| `glyph.trig.timer` | after a Breath | sleep() | After Three Heartbeats | At the Appointed Hour | Clockwork Fuse |
| `glyph.trig.proximity` | upon Approach | on_proximity() | Upon Intrusion | When It Is Near | Tripwire |
| `glyph.trig.on_struck` | Contingency | on_attacked() | Counter-Stance | When It Is Wounded | Recoil Spring |
| `glyph.trig.on_status` | upon Affliction | on_flag() | Upon Sealing | When It Takes Root | Indicator Fuse |
| `glyph.trig.pulse` | Recurring | while(true) | Breathing Cycle | Tidal | Escapement |

| Glyph | Mythic Fantasy | Cyberpunk | Wuxia | Cosmic Horror | Steampunk |
|---|---|---|---|---|---|
| `glyph.cond.foe` | if Foe | if(hostile) | Against Enemies | If It Is Other | Hostile Sensor |
| `glyph.cond.ally` | if Friend | if(friendly) | For Sworn Kin | If It Is Ours | Friendly Sensor |
| `glyph.cond.wounded` | if Wounded | if(hp<half) | Upon Weakness | If It Bleeds | Damage Gauge |
| `glyph.cond.native` | if Native | if(local) | Against the Rooted | If It Belongs Here | Provenance Gauge |
| `glyph.cond.wet` | if Wet | if(wet) | Upon Water | If It Drips | Humidity Gauge |
| `glyph.cond.burning` | if Burning | if(thermal) | Upon Fire | If It Burns | Thermometer |
| `glyph.cond.dark` | if Dark | if(lowlight) | In Shadow | If the Light Fails | Photometer |
| `glyph.cond.unaware` | if Unaware | if(undetected) | Against the Unwary | If It Does Not See | Blind-Side Gauge |

Skins in the other nine register genres and the Ur-Text. The Form rows of these nine genres and the whole Form column of the Ur-Text are doc 18 §7.5's names, adopted unchanged; this chapter adds every other class. Together with the five columns above, every glyph has a canonical skin in all 15 genres that carry a register (§2.2); only `genre.unwritten` has none. The skin is cosmetic text; its dependency comes from the glyph origin through §2.2.

Forms:

| Glyph | Gothic Horror | Space Opera | Post-Apocalypse | Weird West | Hardboiled Noir | Fairy Tale | Mythic Antiquity | Oneiric | The Redaction |
|---|---|---|---|---|---|---|---|---|---|
| `glyph.form.bolt` | Nail of Blood | Psi Dart | Zip Shot | Hex Bullet | Cheap Shot | Wishing Dart | Far-Darting Arrow | Thrown Thought | Memo |
| `glyph.form.orb` | Clotted Orb | Focus Sphere | Junk Bomb | Cursed Coin | Tossed Bottle | Glass Bead | Thunderstone | Soap Bubble | Packet |
| `glyph.form.beam` | Evil Eye | Mind Lance | Arc Torch | Witch's Stare | Hard Stare | Moonbeam | Sun-Chariot Ray | Staring Too Long | Directive |
| `glyph.form.cone` | Grave Breath | Psi Wave | Spray Rig | Spit Hex | Smoke Screen | Dandelion Breath | Breath of the Winds | Sigh | Circular |
| `glyph.form.touch` | Corpse Touch | Mind Touch | Jumper Clamp | Hand of Glory | Sucker Punch | Fairy Kiss | Anointing | Shoulder Tap | Stamp |
| `glyph.form.self` | Within the Vein | Centering | Self-Wire | Rabbit's Foot | Second Wind | Heart's Wish | Heroic Spirit | Pinch | Self-Assessment |
| `glyph.form.aura` | Mourning Shroud | Psi Field | Static Halo | Dust Devil Mantle | Bad Reputation | Rose Mantle | Aegis Mantle | Daydream | Clearance Badge |
| `glyph.form.field` | Circle of Candles | Stasis Zone | Junk Field | Salt Circle | Turf | Toadstool Circle | Sacred Precinct | Room That Shouldn't Be | Restricted Area |
| `glyph.form.wall` | Iron Gate | Barrier Plane | Scrap Wall | Bone Fence | Stonewall | Briar Hedge | Wall of Bronze | Locked Door | Cordon |
| `glyph.form.rune` | Hex Mark | Psi Mine | Tripwire Rig | Hex Sign | Setup | Witch's Knot | Votive Mark | Note Under the Pillow | Pending Form |
| `glyph.form.nova` | Bloodburst | Mind Flare | Overload Rig | Brimstone Burst | Blowup | Midnight Chime | Olympian Thunder | Sudden Waking | Mass Notice |
| `glyph.form.chain` | Thread of Kin | Resonance Link | Daisy Wire | Hanging Rope | Domino | Silken Thread | Thread of the Fates | Déjà Vu | Chain of Custody |
| `glyph.form.manifest` | Raising | Projection | Scrapbot Build | Calling Up | Calling a Favor | Summoned Helper | Invocation of Heroes | Imaginary Friend | Requisition |

Essences:

| Glyph | Gothic Horror | Space Opera | Post-Apocalypse | Weird West | Hardboiled Noir | Fairy Tale | Mythic Antiquity | Oneiric | The Redaction |
|---|---|---|---|---|---|---|---|---|---|
| `glyph.ess.flame` | Hellfire | Plasma | Napalm | Brimstone | Torch Job | Hearthfire | Promethean Fire | Fever Dream | Incineration |
| `glyph.ess.frost` | Grave Chill | Absolute Zero | Cold Snap | Blue Norther | Cold Shoulder | Jack Frost | Breath of Boreas | Cold Sweat | Freeze Order |
| `glyph.ess.storm` | Galvanism | Ion Storm | Live Current | Heat Lightning | Jolt | Will-o'-Spark | Bolt of Zeus | Static Dream | Power Surge |
| `glyph.ess.acid` | Embalming Fluid | Xeno-Acid | Battery Acid | Rotgut | Acid Bath | Bitter Brew | Hydra Bile | Melting | Solvent |
| `glyph.ess.force` | Crushing Weight | Telekinesis | Sledge | Mule Kick | Knuckle Sandwich | Giant's Hand | Strength of Heracles | Falling Dream | Blunt Instrument |
| `glyph.ess.edge` | Razor | Mind Blade | Scrap Shards | Bowie Hex | Shiv | Briar Thorns | Harpe | Paper Cuts | Red Pen |
| `glyph.ess.radiance` | Holy Light | Starfire | Saint's Glow | Preacher's Light | Spotlight | Dawn | Light of Apollo | Waking Light | Disclosure |
| `glyph.ess.shadow` | Grave Shadow | Dark Matter | Fallout Shadow | Gloaming | Dark Alley | Wicked Shadow | Shade of Erebus | Nightmare | Black Bar |
| `glyph.ess.mind` | Mesmerism | Psychic Lance | Rad Madness | Spook | Mind Games | Bewitchment | Madness of Lyssa | Night Terror | Cognitohazard |
| `glyph.ess.arcane` | Occult Force | Raw Psi | Weird Energy | Hoodoo | Moxie | Old Magic | Divine Spark | Dreamstuff | Anomaly |
| `glyph.ess.venom` | Belladonna | Bio-Toxin | Rad Sludge | Snake Oil | Mickey Finn | Poison Apple | Hydra Venom | Bad Taste | Toxic Memo |
| `glyph.ess.thunder` | Banshee Wail | Sonic Boom | Air Horn | Rebel Yell | Loud Mouth | Giant's Roar | Shout of Stentor | Alarm Clock | Public Address |
| `glyph.ess.mend` | Transfusion | Psi Healing | Duct Tape | Sawbones | Patch-Up | Mother's Remedy | Touch of Asclepius | Sweet Dream | Medical Leave |
| `glyph.ess.ward` | Holy Ward | Deflector Shield | Scrap Plating | Lucky Charm | Protection Racket | Silver Thread | Shield of Athena | Blanket | Indemnity |
| `glyph.ess.swiftness` | Quickening Blood | Hyperdrive | Adrenaline | Greased Lightning | Getaway | Seven-League Stride | Sandals of Hermes | Running Dream | Expedite |
| `glyph.ess.levitate` | Bat's Wing | Zero-G | Updraft Rig | Tumbleweed | Walking on Air | Thistledown | Wings of Icarus | Flying Dream | Elevated Clearance |
| `glyph.ess.veil` | Shroud | Cloaking | Camo Rags | Ghost Walk | Fade Out | Fern-Seed Cloak | Helm of Hades | Forgotten Face | Need-to-Know |
| `glyph.ess.bind` | Grasping Dead | Tractor Beam | Razorwire | Lasso | Cuffs | Spindle Thread | Chains of Hephaestus | Paralysis Dream | Hold Order |
| `glyph.ess.dread` | Terror | Psi Fear | Rad Panic | Haunting | Intimidation | Bogeyman | Phobos | Dread Dream | Threat Notice |
| `glyph.ess.dominion` | Thrall | Mind Control | Cult Brand | Snake Charm | Leverage | Love Spell | Will of Aphrodite | Puppet Dream | Reassignment |
| `glyph.ess.slumber` | Coffin Sleep | Stasis | Knockout Gas | Sandman Hex | Sap | Hundred-Year Sleep | Gift of Hypnos | Deep Sleep | Sedation Order |
| `glyph.ess.hush` | Vow of Silence | Psi Damper | Signal Killer | Tongue-Tie | Hush Money | Mute Curse | Oath of Harpocrates | Silent Scream | Gag Order |
| `glyph.ess.polymorph` | Beast Curse | Bio-Rewrite | Mutagen | Varmint Hex | Frame-Up | Frog Curse | Wrath of Circe | Shapeshift Dream | Reclassification |
| `glyph.ess.unmaking` | Exorcism | Psi Disruption | EMP | Cleansing | Cleanup | Unweaving | Atropos' Cut | Waking | Retraction |
| `glyph.ess.summon` | Raised Servant | Drone Call | Scrap Critter | Haint | Hired Gun | Helper | Servant of the Gods | Dream Companion | Field Agent |
| `glyph.ess.shape` | Grave Digging | Matter Shaping | Demolition | Prospecting | Construction Job | Giant's Spade | Hephaestian Craft | Clay Dream | Renovation |
| `glyph.ess.tide` | Black Rain | Hydro-Burst | Flash Flood | Rainmaker | Downpour | Wishing Well | Wrath of Poseidon | Drowning Dream | Sprinkler System |
| `glyph.ess.gale` | Ill Wind | Vacuum Breach | Dust Storm | Twister | Hot Air | North Wind | Bag of Aeolus | Dream Wind | Ventilation |
| `glyph.ess.lumen` | Candlelight | Lumen Drone | Glow Stick | Lantern Hex | Flashlight | Fairy Light | Torch of Hecate | Night-Light | Fluorescent Tube |
| `glyph.ess.sight` | Spirit Sight | Sensor Sweep | Geiger Sense | Tracker's Eye | Stakeout | Magic Mirror | Eye of Argus | Lucid Sight | Surveillance |
| `glyph.ess.appraise` | Provenance | Analysis Scan | Salvage Read | Assayer's Eye | Fence's Eye | Truth-Telling | Gift of Athena | Recognition | Audit |
| `glyph.ess.omen` | Tarot | Precog | Bad Feeling | Bone Reading | Hunch | Black Cat | Augury | Premonition | Risk Assessment |
| `glyph.ess.psychometry` | Touch of the Dead | Psi Imprint | Old World Memory | Object Reading | Fingerprints | Remembering Thing | Memory of Mnemosyne | Familiar Feeling | Provenance Check |

Modifiers:

| Glyph | Gothic Horror | Space Opera | Post-Apocalypse | Weird West | Hardboiled Noir | Fairy Tale | Mythic Antiquity | Oneiric | The Redaction |
|---|---|---|---|---|---|---|---|---|---|
| `glyph.mod.empower` | Engorged | Overcharged | Souped-Up | Double-Loaded | Heavy-Handed | Thrice-Spoken | Heroic | Vivid | Priority |
| `glyph.mod.extend` | Undying | Sustained | Long-Haul | Long-Running | Drawn-Out | Till Midnight | Eternal | Endless | Extended Term |
| `glyph.mod.widen` | Spreading | Wide-Field | Sprawling | Wide-Open | Citywide | Far and Wide | Titanic | Ballooning | Department-Wide |
| `glyph.mod.reach` | Far-Cast | Light-Year | Scoped | Long Rifle | Long-Distance | Over the Hills | Far-Darting | Faraway | Remote |
| `glyph.mod.quicken` | Hasty | Hyper | Quick-and-Dirty | Fast-Draw | Snap | Quick as a Wink | Swift-Footed | Instant | Expedited |
| `glyph.mod.charged` | Swelling | Charged-Up | Juiced | Cocked | Wound-Up | Gathering | Gathered Wrath | Building | Pending |
| `glyph.mod.split` | Splintered | Multi-Vector | Shotgunned | Fanned | Spread | Three Sisters | Many-Headed | Kaleidoscope | Carbon-Copied |
| `glyph.mod.homing` | Bloodhound | Lock-On | Heat-Seeking | Dead-Eye | Tailing | Finds Its Way | Unerring | Inevitable | Addressed |
| `glyph.mod.pierce` | Staking | Phase-Piercing | Rebar-Tipped | Through-and-Through | Clean Through | Needle-Sharp | Achillean | Passing-Through | Cross-Departmental |
| `glyph.mod.ricochet` | Restless | Deflecting | Pinballing | Caroming | Bank Shot | Skipping Stone | Resounding | Bouncing | Rerouted |
| `glyph.mod.linger` | Festering | Residual | Contaminating | Smoldering | Stale | Lasting | Ever-Burning | Aftertaste | Permanent Record |
| `glyph.mod.delay` | Midnight | Timed | Fused | Slow-Burning | Time Bomb | When the Clock Strikes | Fated | Later | Deferred |
| `glyph.mod.hushed` | Graveside | Silent-Running | Suppressed | Quiet | On the Q.T. | Mouse-Quiet | Unsung | Dreamquiet | Confidential |
| `glyph.mod.thrift` | Anemic | Efficient | Scavenged | Cheap | Cut-Rate | Humble | Ascetic | Faint | Budget |
| `glyph.mod.volatile` | Feverish | Overloaded | Haywire | Loose Cannon | Hot-Headed | Capricious | Dionysian | Surreal | Uncontrolled |
| `glyph.mod.keen` | Surgical | Targeted | Sharp-Eyed | Sure-Shot | Below-the-Belt | Clever | Far-Sighted | Lucid | Precise |
| `glyph.mod.siphon` | Draining | Energy-Siphoning | Scavenging | Soul-Taking | Skimming | Pilfering | Tithing | Dream-Eating | Garnishing |
| `glyph.mod.leech` | Sanguinary | Bio-Siphon | Blood-Bag | Bloodsucking | Bleeding Dry | Life-Borrowing | Lamia's | Sapping | Reimbursed |
| `glyph.mod.sculpt` | Sparing | Friendly-Lock | Careful | Sharpshooter's | Discreet | Kind-Hearted | Just | Selective | Authorized |
| `glyph.mod.anchor` | Consecrated | Stationary | Staked-Down | Planted | Stationed | Hearth-Bound | Founded | Recurring | On-Site |
| `glyph.mod.heavy` | Leaden | Gravitic | Sledgehammer | Hard-Hitting | Brass-Knuckled | Ogre's | Titan's | Weighted | Load-Bearing |
| `glyph.mod.twin` | Doppel | Mirrored | Doubled | Two-Gun | Double-Cross | Twin-Born | Dioscuri | Echoed | Duplicate |

Triggers:

| Glyph | Gothic Horror | Space Opera | Post-Apocalypse | Weird West | Hardboiled Noir | Fairy Tale | Mythic Antiquity | Oneiric | The Redaction |
|---|---|---|---|---|---|---|---|---|---|
| `glyph.trig.on_impact` | upon the Wound | On Contact | Impact Fuse | upon the Hit | When It Lands | When It Touches | When It Strikes | Upon Arrival | Upon Delivery |
| `glyph.trig.on_kill` | upon the Last Breath | On Kill Confirmed | Body Count | Boot Hill | When They Drop | When It Falls | Upon Triumph | When the Dream Ends | Upon Termination |
| `glyph.trig.on_expire` | upon the Fading | On Power-Down | Burn-Out | At Sundown | When Time's Up | At the Last Petal | When the Thread Is Cut | On Waking | Upon Expiry |
| `glyph.trig.timer` | at the Stroke | T-Minus | Egg Timer | Pocket-Watch | Count of Three | At the Third Chime | When Chronos Wills | In a Moment | Scheduled Release |
| `glyph.trig.proximity` | upon the Threshold | Proximity Alert | Motion Sensor | Rattlesnake | Too Close | Who Comes Near | Upon Trespass | When It Draws Near | Unauthorized Entry |
| `glyph.trig.on_struck` | Blood for Blood | Counter-Protocol | Payback Rig | Eye for an Eye | Payback | Tit for Tat | Nemesis | Startle | Incident Response |
| `glyph.trig.on_status` | upon the Taint | On Condition | Symptom Trigger | Hex-Mark | When It Sticks | When the Spell Takes | When the Curse Takes | When It Sinks In | Upon Flagging |
| `glyph.trig.pulse` | Heartbeat | Pulse Cycle | Generator Hum | Drumbeat | On the Hour | Round and Round | Eternal Return | Recurring Dream | Routine Review |

Conditions:

| Glyph | Gothic Horror | Space Opera | Post-Apocalypse | Weird West | Hardboiled Noir | Fairy Tale | Mythic Antiquity | Oneiric | The Redaction |
|---|---|---|---|---|---|---|---|---|---|
| `glyph.cond.foe` | if Enemy | If Hostile Signature | If Raider | If Outlaw | If Mark | If Wicked | If Foe of the Gods | If Nightmare | If Unauthorized |
| `glyph.cond.ally` | if Kin | If Crewmate | If Crew | If Posse | If Family | If Friend of the Forest | If Guest-Friend | If Dreamer | If Cleared |
| `glyph.cond.wounded` | if Weakened | If Hull Breached | If Busted | If Winged | If Hurting | If Pricked | If Laid Low | If Fading | If Damaged |
| `glyph.cond.native` | if of This Night | If Local Species | If Wastelander | If Homesteader | If From Around Here | If Born in the Wood | If Earth-Born | If Dreamt Here | If On File |
| `glyph.cond.wet` | if Rain-Soaked | If Moisture Detected | If Soaked | If Wet Behind the Ears | If Rained On | If Dewy | If Touched by Poseidon | If Dripping | If Water-Damaged |
| `glyph.cond.burning` | if Pyre | If Overheating | If On Fire | If Branded | If Smoking | If Kindled | If Ablaze | If Feverish | If Flagged Flammable |
| `glyph.cond.dark` | if Moonless | If Unlit Sector | If Pitch-Black | If After Dark | If In the Shadows | If Night | If Under Nyx | If Lights Out | If Off the Record |
| `glyph.cond.unaware` | if Unsuspecting | If Off Sensors | If Napping | If Bushwhacked | If Blindsided | If Unwitting | If Unforeseen | If Sleepwalking | If Uninformed |

The Ur-Text (register Word, doc 18 §7.3 vocabulary of word, first, beginning and page):

| Glyph | The Ur-Text |
|---|---|
| `glyph.form.bolt` | First Letter |
| `glyph.form.orb` | Round Hand |
| `glyph.form.beam` | Underline |
| `glyph.form.cone` | Flourish |
| `glyph.form.touch` | Signature |
| `glyph.form.self` | First Person |
| `glyph.form.aura` | Illumination |
| `glyph.form.field` | Paragraph |
| `glyph.form.wall` | Ruled Line |
| `glyph.form.rune` | Initial |
| `glyph.form.nova` | Exclamation |
| `glyph.form.chain` | Ligature |
| `glyph.form.manifest` | Character Sketch |
| `glyph.ess.flame` | Burning Word |
| `glyph.ess.frost` | Frozen Page |
| `glyph.ess.storm` | Lightning Stroke |
| `glyph.ess.acid` | Etching |
| `glyph.ess.force` | Heavy Type |
| `glyph.ess.edge` | Paper Edge |
| `glyph.ess.radiance` | Gilt Light |
| `glyph.ess.shadow` | Blot |
| `glyph.ess.mind` | Unreadable Line |
| `glyph.ess.arcane` | The Word |
| `glyph.ess.venom` | Bitter Gloss |
| `glyph.ess.thunder` | Shouted Line |
| `glyph.ess.mend` | Correction |
| `glyph.ess.ward` | Margin Guard |
| `glyph.ess.swiftness` | Shorthand |
| `glyph.ess.levitate` | Superscript |
| `glyph.ess.veil` | Blank Space |
| `glyph.ess.bind` | Bookbinding |
| `glyph.ess.dread` | Foreshadowing |
| `glyph.ess.dominion` | Authorial Will |
| `glyph.ess.slumber` | Dull Chapter |
| `glyph.ess.hush` | Ellipsis |
| `glyph.ess.polymorph` | Revision |
| `glyph.ess.unmaking` | Deletion |
| `glyph.ess.summon` | Character |
| `glyph.ess.shape` | Typesetting |
| `glyph.ess.tide` | Wash |
| `glyph.ess.gale` | Page Turn |
| `glyph.ess.lumen` | Lamp of the Scribe |
| `glyph.ess.sight` | Reading |
| `glyph.ess.appraise` | Proofreading |
| `glyph.ess.omen` | Foreword |
| `glyph.ess.psychometry` | Marginal History |
| `glyph.mod.empower` | Bold |
| `glyph.mod.extend` | Long Form |
| `glyph.mod.widen` | Wide Margin |
| `glyph.mod.reach` | Far Margin |
| `glyph.mod.quicken` | Quick Draft |
| `glyph.mod.charged` | Building Climax |
| `glyph.mod.split` | Three Columns |
| `glyph.mod.homing` | Cross-Referenced |
| `glyph.mod.pierce` | Through the Page |
| `glyph.mod.ricochet` | Rebounding Line |
| `glyph.mod.linger` | Lingering Stain |
| `glyph.mod.delay` | Postscript |
| `glyph.mod.hushed` | Silent Letter |
| `glyph.mod.thrift` | Terse |
| `glyph.mod.volatile` | Unedited |
| `glyph.mod.keen` | Italic |
| `glyph.mod.siphon` | Borrowed Word |
| `glyph.mod.leech` | Quoted |
| `glyph.mod.sculpt` | Careful Hand |
| `glyph.mod.anchor` | Fixed Type |
| `glyph.mod.heavy` | Heavy Stroke |
| `glyph.mod.twin` | Repeated |
| `glyph.trig.on_impact` | At the Full Stop |
| `glyph.trig.on_kill` | At the Last Word |
| `glyph.trig.on_expire` | At the End of the Chapter |
| `glyph.trig.timer` | After a Pause |
| `glyph.trig.proximity` | When It Is Read |
| `glyph.trig.on_struck` | In Rebuttal |
| `glyph.trig.on_status` | When It Is Annotated |
| `glyph.trig.pulse` | Refrain |
| `glyph.cond.foe` | If Antagonist |
| `glyph.cond.ally` | If Protagonist |
| `glyph.cond.wounded` | If Torn |
| `glyph.cond.native` | If In This Story |
| `glyph.cond.wet` | If Smudged |
| `glyph.cond.burning` | If Scorched |
| `glyph.cond.dark` | If Unlit Page |
| `glyph.cond.unaware` | If Unread |

## 6. Cost and potency

### 6.1 Base cost (skill-independent except three perks)

```
core(stage)       = form.cost + Σ_i mul_bp(essence_i.cost', 10000 + 2500 × i)       // i = 0 primary, 1 secondary
stage_cost(stage) = mul_bp(core, max(2500, 10000 + Σ modifier.cost_add_bp')) + trigger.cost + condition.cost
essence.cost'     = mul_bp(essence.cost, 5000) for swiftness and levitate with Alteration 100, else essence.cost
cost_add_bp'      = 0 for keen with Divination 50; mul_bp(cost_add_bp, 8000) for a positive add with Spellcraft 25; else cost_add_bp
base_cost(spell)  = Σ over stages s of mul_bp(mul_bp(stage_cost(s), DEPTH_BP[depth(s)]), fire_bp(s))
DEPTH_BP = [10000, 7500, 6000];  fire_bp(root) = 10000;  fire_bp(payload) = fires_bp of the trigger that opens it
```

For a beam the root `stage_cost` is a per-second drain. `base_cost` feeds skill XP (§7.4) and inscription (§8.3). Skill levels never enter it; only the perks Spellcraft 25, Divination 50 and Alteration 100 do (§7.3).

### 6.2 Cast cost

```
cost_skill = floor_div(3 × school_skill + skill.spellcraft, 4)
cast_cost  = base_cost → × (12500 − 50 × cost_skill) → × implement cost_bp (§3.8) → × register rider cost_bp (§10)
                       → × ench.thrift (§9.4) → × each [T] aether factor in ascending ID byte order   // one mul_bp each, this order
cast_cost  = max(1000, cast_cost) when base_cost > 0
cast_cost  = 0 for a free cast; the 1000 floor does not apply to it
```

The skill factor runs from 12500 at skill 0 to 7500 at skill 100.

- `[T]` aether factors: doc 04 talents `efficient_casting` (9000), `overchannel` (15000 while toggled on) and `grand_design` (7000, the Signature spell only); doc 14 Echo traits `echo.preacher_amos` (11500, restoration spells) and `echo.briar_thornwood` (11500, conjuration spells).
- **Free casts**: doc 04 talent `spell_echo` (every 5th effective cast); doc 11 `perk.arcane_surge` (every 5th cast through the item) and `item.palimpsestic.authors_quill` (once per 600 ticks). When several free-cast counters are due on one cast, the cast is free once and every due counter is consumed.
- Texts, wands, on-use enchantments and prayers have `cast_cost = 0`.
- Doc 04 §5.3 also states a school cost factor of `10000 − 25 × skill`. Spell cost belongs to this chapter, so this formula stands and doc 04 is asked to cite it (Interface assumptions).

### 6.3 Potency

```
row29      = 10000 + (ATN − 40) × 50 + school_skill × 30 + Σ [T] flat potency terms       // Canon §6.4 row 29
spell_pot  = row29 → × glyph_term(g) for each glyph g of the stage, in grammar order → × rider_pot (§10)
glyph_term = 10000 + (local_axiom(caster.pos, dep(g)) − 5) × 200                      // Canon §5.7 GLYPH
```

`[T]` flat potency terms are Bp added inside row 29, for example `echo.drowned_clerk` (+1500 while `st.soaked`; −2000 for stages whose damage type is `dmg.fire`). A bound glyph (§3.8) adds no glyph term. A stage holds at most 8 glyphs (9 with Spellcraft 100), so its product of glyph terms lies between `9000^8` and `11000^8` (×0.43 to ×2.14; ×0.39 to ×2.36 with 9); each added glyph also adds at least 1 complexity and its own cost (§6.1), so the home-genre bonus grows only with a larger and dearer spell.

Each stage of a spell computes its own `spell_pot` from its own glyphs (Canon gap G2). NPC casters never use row 29 (Canon §6.5), so NPC `spell_pot = 10000` (§6.5).

### 6.4 Magnitudes

`mag_chain` = the form magnitude, then each modifier `mag_bp` in glyph order, then `charge_bp`, then the volatile factor, then 20000 for Wild Growth (§11.3); one `mul_bp` each.

| Quantity | Formula |
|---|---|
| Damage base | `dmg_base = mag_chain(b_0 + mul_bp(b_1, 7500))`, where `b_i` is the base mHP of damage essence `i` |
| Type shares | one damage essence: 10000; two: `share_0 = floor_div(b_0 × 10000, b_0 + mul_bp(b_1, 7500))`, `share_1 = 10000 − share_0` |
| Step 0 (Canon §10.4) | `B = dmg_base → × P(src_lvl) → × source rarity power → × source quality → × (10000 + 600 × source eff_enh) → × source material dmg_bp`, with the source item of §3.1 (10000 for each factor with no implement) |
| Pipeline | Canon §10.5 from step 1; step 2.1 multiplies by `spell_pot`; step 2.5 by the spell's Contradiction potency; step 2.6 by `10000 + edge_bp` against natives |
| Status power | `p = essence power (× 7500 if secondary) → × mag_chain with p_form in place of the form magnitude → × spell_pot → × 12000 for damage essences of evocation-school stages with Evocation 25 → × 12500 for control-kind essences with Illusion 50 → × 20000 for summon attacks with Conjuration 100 (§12.1)`, with `p_form` the form magnitude, or `max(form magnitude, 2500)` for aura, field and wall; then Canon §11.2 from `S(src_lvl)` on (Canon gap G14) |
| Heal | `heal_bp = mag_chain(2000, or 1500 if secondary) → × spell_pot → × C potency → × 11000 with Restoration 25`; amount `mul_bp(recipient Max HP, heal_bp)`, then Canon §10.9 (row 3 after the Restoration 25 factor) |
| Ward | `ward_bp = mag_chain(2500, or 1875 if secondary) → × spell_pot → × C potency → × 12000 with Abjuration 25`; `st.shielded` pool `mul_bp(recipient Max HP, ward_bp)` for 600 ticks × `duration_bp` |
| `duration_bp` | 10000, × 20000 per extend, × 15000 per anchor (aura, field, wall, rune) |
| Helpful direct duration | `status base → × duration_bp → × min(spell_pot, 20000) → × C potency → × 12500 with Alteration 25 → × 15000 for veil's `st.invisible` with Illusion 25`, at least 1 tick |
| Harmful direct duration | Canon §11.4 with `base = status base → × duration_bp → × C potency` |
| Persistent entity duration | `form or template duration → × duration_bp → × C potency` |
| Radius, half-angle, jump range | form value × 15000 per widen → × doc 11 `perk.wide_conduit` for radii → × 12500 for orb, nova and rune radii with Evocation 100; half-angle capped at 16384 Bam |
| Range, speed | form value × 20000 range and × 15000 speed per reach; speed × 7500 per heavy |

### 6.5 NPC spells

An NPC casts authored `spell.` IDs from its template (doc 13). Its spell damage is an enemy source (Canon §10.4): `B = mul_bp(dmg_base, NPC_SPELL_SCALE_BP) → × T(d) → × rank damage`, with `NPC_SPELL_SCALE_BP = 4500`, so a standard Folio-1 bolt of flame hits for `mul_bp(24000, 4500) = 10,800` mHP, matching `REF_ENEMY_HIT_D1` (Canon §17.4). Step 2.1 is 10000. Status power is `p × p_form × modifier mags`, then Canon §11.2 with `S(d)`. NPC aether is the template's `aether_max_mae` and `aether_regen_mae_s` (default 2,000 per second, doc 13 §2), regenerated each tick through the `k = 60` accumulator with no AETHER term, because Canon §6.5 makes NPC aether a template constant. NPC cast cost is `base_cost` with no skill factor. Canon §6.4 row 30 applies to NPC interruption. Contradiction applies to foreign NPC casters (Canon §5.5) with NPC tolerance (Canon §5.4); a failed NPC roll always fizzles and never surges, as doc 13 §5 states. Doc 13 §5 writes NPC spell damage as `template_dmg_mhp × T(d) × rank`; for a spell, `template_dmg_mhp` is `mul_bp(dmg_base, NPC_SPELL_SCALE_BP)`.

### 6.6 Siphon and leech

```
siphon_maero = floor_div(mul_bp(hp_lost_mhp, 1500) × 10000, P(d_current))                 // per hit, to the caster
leech_bp     = min(2500, floor_div(hp_lost_mhp × 2500, target_max_hp_mhp))                // per hit
leech_mhp    = mul_bp(caster_max_hp_mhp, leech_bp)                                         // a heal, Canon §10.9
```

`hp_lost_mhp` is HP removed after ward (Canon §10.5 step 7). Example at `d_current = 3`: a 36,523 mHP hit siphons `floor_div(5478 × 10000, 11881) = 4610` m-aether; against a target of 80,000 mHP Max HP it leeches `floor_div(36523 × 2500, 80000) = 1141` Bp, which heals a 105 HP caster for 11,980 mHP.

Caps per root resolution, summed over every stage, hit and twin repeat of that cast:

```
siphon_total ≤ mul_bp(cast_cost, 5000)          // 0 for texts, wands, on-use enchantments, prayers and free casts
leech_total  ≤ 2500 Bp of the caster's Max HP
```

Only hits on hostile entities count; allies, the caster, own minions, entities tagged `summoned` (doc 13) and blocks give nothing. Without the caps, a caster at ATN 100 and Evocation and Spellcraft 100 in Mythic Fantasy, with a Common focus, Ley-Bound and `ench.thrift` III, `src_lvl = d_current` and unmitigated hits, cast `nova flame siphon` for 5315 m-aether (Spellcraft 25 lowers siphon's cost add to 3200) and got back 4801 per target: 90% of the cost from one target and 4.5 times the cost from five, so aether never ran out. The same caster's five-target `nova flame leech` healed 6665 Bp (67%) of Max HP per cast against standard Folio-1 enemies. Example: Ref-A's `bolt flame siphon` costs 7011 m-aether and its bolt hit of 38,713 mHP siphons `floor_div(mul_bp(38713, 1500) × 10000, 11881) = 4886`, capped at `mul_bp(7011, 5000) = 3505`.

### 6.7 Worked spells

**Reference caster Ref-A**: Origin Mythic Fantasy, level 5, ATN 50, FOC 45, LOG 50, MEM 40, other sub-attributes 40. Max HP 105, max aether 70, budget 9, 4 Grimoire slots, regeneration `mul_bp(1500, 13000) = 1950` m-aether per second in Mythic Fantasy. Skills: Evocation 30, Spellcraft 24, Conjuration 25, Abjuration 25 (only for the aura's tier-2 gate), Restoration 12. Doc 04 perks in play (§7.3): `skill.evocation.m25` (status power × 12000 for evocation damage essences) and `skill.conjuration.m25` (summon duration × 15000); `skill.abjuration.m25` affects only wards. Implement: Common Normal +0 focus of a material with `dmg_bp` 10000, `ilvl` 3 (`P(3) = 11881`), `cost_bp` 9500. Location `genre.mythic_fantasy` (ARC 8), `d = 3`. All glyphs have Mythic Fantasy origin: `Req[arcana] = 6 <= 8`, so `C = 0`, `E = 0`, glyph term `10000 + 3 × 200 = 10600`. Register rider Ley-Bound: cost × 9500 with an implement. Target: a standard Folio-3 NPC with 0 resistance and 0 AR to the types shown, nerve threshold `100 + 2 × 3 = 106`.

**W1 Searing Bolt** `bolt flame` (Evocation)

```
complexity 1 + 1 = 2;  base_cost 1000 + 4000 = 5000
cost_skill floor_div(90 + 24, 4) = 28 → 12500 − 1400 = 11100
cast_cost 5000 → 5550 → ×9500 5272 → ×9500 5008 m-aether;  cast ticks 24, recovery 20
row29 10000 + 500 + 900 = 11400 → ×10600 (bolt) 12084 → ×10600 (flame) 12809 = spell_pot
dmg_base 24000;  B = mul_bp(24000, 11881) = 28514;  step 2.1: mul_bp(28514, 12809) = 36523 mHP
st.burning p: 40 → ×10000 → ×12809 = 51 → ×12000 (Evocation 25) 61 → ×S(3)=10600 → 64 per hit; 2 hits fill 106
```

**W2 Widened Frost Orb** `orb frost widen`

```
complexity 2 + 1 + 1 = 4;  base (2000 + 4000) × (10000 + 4000) = 8400;  cast 8400 → 9324 → 8857 → 8414
spell_pot 11400 → 12084 → 12809 → ×10600 (widen) 13577;  radius 2000 × 15000 = 3000 mm
dmg_base 20000 → ×8000 16000 → ×9000 14400;  B 17108;  step 2.1: 23227 mHP to every hostile within 3000 mm
st.frozen p: 35 → ×8000 28 → ×9000 25 → ×13577 33 → ×12000 39 → ×S(3) 41;  every hit also applies st.chilled
```

**W3 Storm Ray** `beam storm` (channel)

```
complexity 3;  base 2000 + 4500 = 6500 per second;  cast 6511 per second
drain: accumulator k = 60, first Channeling tick q = floor_div(6511, 60) = 108; exactly 6511 per 60 ticks
spell_pot 12809;  per pulse dmg_base 22000 × 3000 = 6600;  B 7841;  step 2.1: 10043 mHP every 15 ticks = 40172 mHP per s
full 300-tick channel: 20 pulses, 32555 m-aether drained;  st.shocked p per pulse 35 → ×3000 10 → ×12809 12 → ×12000 14 → ×S(3) 14
```

**W4 Empowered Firestorm Bolt** `bolt flame storm empower` (two damage essences)

```
complexity 4;  core 1000 + 4000 + mul_bp(4500, 12500) = 10625;  base mul_bp(10625, 15000) = 15937;  cast 15964
spell_pot 11400 → 12084 → 12809 → 13577 (storm) → 14391 (empower)
dmg_base (24000 + mul_bp(22000, 7500) = 16500) = 40500 → ×13000 = 52650;  shares fire 5925, shock 4075
B 62553;  step 2.1: 90020;  step 3: fire 53336 + 1 remainder = 53337, shock 36683
p: burning 40 → ×13000 52 → ×14391 74 → ×12000 88 → ×S(3) 93;  shocked 35 × 7500 = 26 → ×13000 33 → ×14391 47 → ×12000 56 → ×S(3) 59
```

**W5 Cinder Bolt** `bolt flame on_impact ( nova frost )` (trigger, depth 1)

```
complexity 1 + 1 + 2 + (2 + 1) + 2 = 9 (budget 9: fits);  the variant with orb and cond.foe has complexity 11: E_COMPLEXITY
root stage_cost 5000 + 1000 = 6000;  payload stage_cost 3000 + 4000 = 7000 → ×7500 5250 → ×10000 5250
base 11250;  cast 11250 → 12487 → 11862 → 11268
root spell_pot 11400 → 12084 → 12809 → ×10600 (on_impact) 13577;  bolt hit: B 28514 → 38713 mHP
payload (tick after impact) spell_pot 12809;  nova dmg_base 20000 × 7000 = 14000;  B 16633 → 21305 mHP to every entity
  within 5000 mm of the impact point, allies at FF_DIRECT_BP;  st.frozen p 35 → ×7000 24 → ×12809 30 → ×12000 36 → ×S(3) 38
```

**W6 Flame Conjuration** `manifest summon flame` (Conjuration 25)

```
complexity 2 + 2 + 1 = 5;  base 3000 + 8000 + mul_bp(4000, 12500) = 16000
cost_skill floor_div(75 + 24, 4) = 24 → 11300;  cast 16000 → 18080 → 17176 → 16317;  cast ticks 60;  cooldown 600
row29 10000 + 500 + 750 = 11250 → 11925 → 12640 → 13398 = spell_pot (snapshot for the summon)
summon.elemental (§12.2): HP 60000 → ×P(3) 71286 → ×13398 95508 mHP;  attack 8000 → 9504 → 12733 mHP dmg.fire every 45 ticks
attack status p 30 → ×13398 40 → ×S(3) 42;  duration 3600 × (10000 + 50 × 25) = 4050 → ×15000 (Conjuration 25) 6075 ticks;  cap min(5, 1 + 1) = 2
```

**W7 Mending Aura, Extended** `aura mend extend` (Restoration 12)

```
complexity 4;  base (3000 + 5000) × 14000 = 11200;  cost_skill floor_div(36 + 24, 4) = 15 → 11750;  cast 11200 → 13160 → 12502 → 11876
row29 10000 + 500 + 360 = 10860 → 11511 → 12201 → 12933
heal per pulse: 2000 × 600 = 120 → ×10000 (extend has mag 10000) → ×12933 = 155 Bp;  duration 600 × 20000 = 1200 ticks = 40 pulses
on a 105 HP ally: mul_bp(105000, 155) = 1627 mHP per pulse, 6200 Bp (62% of Max HP) over the aura;  cooldown 900
```

**W8 Program in a fairy tale (Contradiction)**. Ref-B: Origin Cyberpunk, ATN 40, level 8, Evocation 35, Spellcraft 30. Cyberdeck focus Rare (11000), Good (11000), +1, material `dmg_bp` 10000, `ilvl` 8 (`P(8) = 18280`). Spell `bolt storm quicken`, all glyphs of Cyberpunk origin (`dep = techne`, `Req = 7`). Cast in `genre.fairy_tale` (TEC 1), `d = 8`, at a native target.

```
raw_gap 7 − 1 = 6;  Tol[techne] = 2 + clamp(floor_div(0, 30), 0, 3) = 2;  C = 4: potency 6000, failure 1000;  E = 6: edge 2400
base (1000 + 4500) × (10000 + mul_bp(3000, 8000) Spellcraft 25) = 5500 × 12400 = 6820;  cost_skill floor_div(105 + 30, 4) = 33 → 10850;  cast 6820 → 7399 → ×9500 7029 (no cost rider)
cast ticks 24 × 5000 = 12;  glyph term (1 − 5) × 200 + 10000 = 9200 per glyph
spell_pot 11050 → 10166 → 9352 → 8603
B: 22000 → ×18280 40216 → ×11000 44237 → ×11000 48660 → ×10600 51579
step 2.1: 44373;  step 2.5 ×6000: 26623;  step 2.6 ×12400: 33012 mHP;  first hit on a native non-boss: Shock of the New stagger
```

With run seed `0x0123456789ABCDEF`, Folio 8, actor `(8 << 40) | 7`, input tick 30016, seq 0: `rng.combat` draws 482 < 1000, so the cast fails. `rng.magic` draws `v0..v3 = 3992, 2458, 59936, 5793`; at C = 4, `v1 = 2458` selects entry 2, Backfire (§11.3): the bolt resolves on Ref-B at magnitude × 5000: `B = mul_bp(51579, 5000) = 25789`, step 2.1 22186, step 2.5 13311 mHP before Ref-B's own mitigation, with no friendly-fire factor (a self-hit).

## 7. Schools and skills

### 7.1 Schools

| School | Skill (Canon §7) | Glyphs of the school |
|---|---|---|
| `school.evocation` | `skill.evocation` (ATN) | forms bolt, orb, beam, cone, nova, chain; essences flame, frost, storm, arcane, thunder; modifiers widen, charged, split, pierce, volatile |
| `school.abjuration` | `skill.abjuration` (RSV) | forms aura, rune; essences ward, unmaking, hush; modifiers siphon, sculpt |
| `school.alteration` | `skill.alteration` (FOC) | forms touch, self, field, wall; essences acid, force, venom, swiftness, levitate, bind, polymorph, shape, gale; modifiers extend, anchor, heavy |
| `school.conjuration` | `skill.conjuration` (MEM) | form manifest; essences edge, shadow, summon, tide; modifier linger |
| `school.illusion` | `skill.illusion` (GUI) | essences mind, veil, dread, dominion, slumber; modifier hushed |
| `school.restoration` | `skill.restoration` (RES) | essences radiance, mend; modifier leech |
| `school.divination` | `skill.divination` (ITU) | essences lumen, sight, appraise, omen, psychometry; modifiers homing, keen |
| `school.spellcraft` | `skill.spellcraft` (LOG) | modifiers empower, reach, quicken, ricochet, delay, thrift, twin; every trigger and condition |

A glyph's school sets its tier gate. A spell's school (its primary essence's school) sets row 29, `cost_skill` and XP.

### 7.2 Skill effects

| Skill | Effect in this chapter |
|---|---|
| each school skill | row 29 `+30 Bp` per point; `cost_skill` (§6.2); tier gates |
| Spellcraft | 25% of `cost_skill` for every spell; tier gates of triggers, conditions and its modifiers |
| Abjuration | dispel count `2 + floor_div(abjuration, 25)` (§14.1) |
| Alteration | shape power `3 + floor_div(alteration, 25)` (§13.2) |
| Conjuration | summon duration × `(10000 + 50 × conjuration)`; minion cap (Canon §15.1) |
| Devotion | favor scaling (Canon §16.5); prayer potency via row 29; petition favor costs as losses (§16.4) |
| Inscription | enchantment tier gates 0, 25, 50 (§9.3); text and tome complexity limit `4 + floor_div(inscription, 10)`; inscription aether factor (§8.3) |
| Handling | commands minions (Canon §15.1) |

### 7.3 Mastery hooks for doc 04 perks

Doc 04 §5.3 owns the perk IDs `skill.<name>.m25` to `.m100` and grants them; its magic, faith and Inscription perk texts are this table, and this chapter owns the mechanics they modify. The hook column fixes where each perk enters this chapter's formulas, one `mul_bp` per factor.

| Perk | Effect | Hook |
|---|---|---|
| `skill.evocation.m25` | damage-essence status power × 12000 | §6.4 status power, after `spell_pot`, for damage essences of evocation-school stages |
| `skill.evocation.m50` | charge factor caps at 17500, reached at `c = 90` | §3.3: `c` runs `0..=90` past `ct`, `charge_bp(c) = 10000 + floor_div(7500 × c, 90)`, auto-release at `held = ct + 150` |
| `skill.evocation.m75` | chain +1 jump | §5.2 chain: up to 4 jumps |
| `skill.evocation.m100` | orb, nova and rune radius × 12500 | §6.4 radius, after widen and `perk.wide_conduit` |
| `skill.abjuration.m25` | ward magnitude × 12000 | §6.4 `ward_bp` chain |
| `skill.abjuration.m50` | dispel count +1 | §14.1 `n` |
| `skill.abjuration.m75` | counterspell silence 180 ticks instead of 60 | §14.3 base duration |
| `skill.abjuration.m100` | a ward cast also applies `st.fortified` for 300 ticks | every recipient of this caster's ward essence also gets `st.fortified` by direct application, base 300 ticks (unscaled, Canon §11.1) |
| `skill.alteration.m25` | helpful direct durations × 12500 | §6.4 helpful direct duration |
| `skill.alteration.m50` | shape power +2 | §13.2 |
| `skill.alteration.m75` | polymorph break threshold 4000 Bp instead of 2000 | §12.3, for polymorphs this caster applies |
| `skill.alteration.m100` | swiftness and levitate cost × 5000 | §6.1 `core`: those essences' `cost` × 5000 |
| `skill.conjuration.m25` | summon duration × 15000 | §12.1 duration |
| `skill.conjuration.m50` | summon Max HP × 12500 | §12.1 Max HP, a `[T]` factor |
| `skill.conjuration.m75` | summons resist every type +1500 Bp, within their caps | §12.1 resistances, NPC cap 9000 |
| `skill.conjuration.m100` | summon attacks carry status power × 20000 | §12.1 summon status power, after `pot` |
| `skill.illusion.m25` | veil duration × 15000 | §6.4 helpful direct duration of the `st.invisible` a veil essence applies |
| `skill.illusion.m50` | control status power × 12500 | §6.4 status power, after `spell_pot`, for control-kind essences (§5.3) |
| `skill.illusion.m75` | targets you charm deal damage × 12000 | +2000 Bp in Canon §10.5 step 1 for hits by an entity this caster charmed |
| `skill.illusion.m100` | phantasm spawns 2 copies | §12.2: a phantasm cast spawns 2 `summon.phantasm`, each counting toward the minion cap |
| `skill.restoration.m25` | heal magnitude × 11000, before row 3 | §6.4 heal chain |
| `skill.restoration.m50` | mend also ends one DoT status (lowest status ID) | each mend resolution removes the target's DoT-category status with the lowest ID (`ev.status_expired` reason 1) |
| `skill.restoration.m75` | healing beyond Max HP becomes `st.shielded` up to 1000 Bp of Max HP | a mend's amount above Max HP is added to the target's `st.shielded` pool, at most 1000 Bp of Max HP from this source, expiry 600 ticks |
| `skill.restoration.m100` | a mend that reaches a downed ally completes its revival at once (doc 16 §9.4), once per ally per Folio | a mend resolution on a `st.downed` ally completes doc 16's revival with its `revive_hp_bp`, then the heal applies |
| `skill.divination.m25` | sight radius × 15000 | the area radius of sight-essence stages and the 6000 mm self-sight radius (§13.1) |
| `skill.divination.m50` | keen costs 0; awareness icons through walls (doc 09 §8.3) | §6.1: `glyph.mod.keen` has `cost_add_bp` 0 for this caster |
| `skill.divination.m75` | condition glyphs cost 0 complexity | §4.4: `condition.cx = 0` for this caster |
| `skill.divination.m100` | see `st.invisible` entities within 12000 mm | doc 09 sight ignores `st.invisible` within 12000 mm for this Erratum |
| `skill.spellcraft.m25` | modifier cost addends × 8000 | §6.1: each positive `cost_add_bp` × 8000 |
| `skill.spellcraft.m50` | depth surcharge −1 | §4.4 `spellcraft_hook = 1` |
| `skill.spellcraft.m75` | trigger `max_fires` +1 | §5.5, with `fires_bp` unchanged |
| `skill.spellcraft.m100` | a stage holds 4 modifiers | §4.2 BNF and §4.3 check 1 |
| `skill.devotion.m25` | petition favor cost × 8000 | §16.4, before Devotion scaling |
| `skill.devotion.m50` | a supplication answered pleased resolves 2 needs | §16.3: the first two needs that apply |
| `skill.devotion.m75` | prayers to non-local patrons apply × 12500 | §16.3 boon and wrath durations and §16.4 magnitudes, in place of the local patron's 15000 |
| `skill.devotion.m100` | the first supplication each Folio ignores the prayer timer | that supplication takes the `timer == 0` branch of §16.3; its timer increment still applies |
| `skill.inscription.m25` | +1 enchantment capacity point per item | items this Erratum enchants (§9.1) |
| `skill.inscription.m50` | text inscription aether × 7500 | §8.3 |
| `skill.inscription.m75` | 4 enchantments per item | items this Erratum enchants (§9.1) |
| `skill.inscription.m100` | on-use enchantments +1 use per Folio | on-use enchantments of items this Erratum has equipped (§9.4) |

`base_cost` (§6.1) depends on the caster only through Spellcraft 25, Divination 50 and Alteration 100; the compiled-spell cache is rebuilt when those perks are gained (§4.5).

### 7.4 Skill XP

Doc 04 §5.2 owns the XP table; its magic, faith and Inscription rows are these:

| Action | Skill | Milli-XP |
|---|---|---|
| spell, wand, text or summon hit on a hostile | the stage school (summons: Conjuration) | Canon §7.1 damage-normalized formula, `min(1500, floor_div(1500 × dmg, max))`, per hit |
| effective non-damaging root resolution | spell school | `floor_div(base_cost, 20)` |
| any effective root resolution | Spellcraft | 100 per modifier, trigger and condition glyph in the spell, at most 500 |
| first effective cast this run of a newly compiled spell with `g` glyphs | Spellcraft | `2000 × g`, at most 3 per Folio (doc 04) |
| fizzle or surge | any | 0 |
| supplication answered pleased or helped | Devotion | 2000 |
| petition | Devotion | `20 × favor cost` before scaling |
| sacrifice | Devotion | `10 × favor gained` |
| text inscription | Inscription | `floor_div(base_cost, 10)` |
| enchantment applied | Inscription | `2000 × points` |

"Effective" is doc 04's definition: a resolution that restored HP, raised `res.ward`, applied or removed a status, spawned a summon, changed a block or object, or revealed an object; damage pays only through the first row. "Newly compiled" means a Grimoire or tome spell whose `spell_hash` has not resolved before in this run for this Erratum.

## 8. Learning, Grimoire, texts, wands and implements

### 8.1 Lexicon and learning sources

The **Lexicon** is a per-Erratum `BTreeSet<(glyph id32, origin id32)>` in run state. Learning emits `ev.glyph_learned` (Canon gap G3).

| Source | Teaches | Rules |
|---|---|---|
| Run start | `glyph.form.self`, `glyph.form.touch`, `glyph.form.bolt`, `glyph.ess.mend`, `glyph.ess.lumen` | origin = the Erratum's Origin genre (`genre.ur_text` for an Unwritten Origin); Calling and Echo kits (doc 04, doc 14) add more |
| Glyph tome (`cat.book`) | its 1 to 3 glyphs, origin = tome `origin_genre` | read for 600 ticks while not `in_combat`; not consumed, so every party member can read it. Doc 11's `item.book.primer_<school>` templates are glyph tomes: Calling kits fix their glyphs (doc 14), and loot fills them with `glyph_tome` below |
| Text study | the text's highest-tier unknown glyph that is in the loot pool (ties to lower ID), with that glyph's origin in the text | 300 ticks, not `in_combat`, Inscription ≥ 25; consumes the text |
| Trainer (shop service, doc 11) | one glyph from the loot pool, origin = local genre | `mul_bp(100 × tier, P(d_current))` Lucre |
| Scriptorium Rewrite (doc 12) | a known glyph re-originated to the local genre, added beside the old entry | `mul_bp(25 × tier, P(d_current))` Lucre; refused where the local axiom of the new dependency is 0 |

**Glyph loot pool.** The loot pool decides only what glyph tomes, trainers and text study can teach. Calling kits (doc 14), Echo kits, authored spells (wands, texts, NPCs) and inscribed texts use any glyph. The content set (Canon §13.6) holds `base` always and each other pool only when its unlock is owned; `unlock.daily_baseline` holds `base` only.

| Pool | Glyphs | Added by |
|---|---|---|
| `base` | the 52 glyphs not listed below: 11 forms, 23 essences, 11 modifiers, triggers on_impact, on_expire and timer, conditions foe, ally, wounded and native | always |
| `glyphs_2` | form rune, form wall, essences slumber and hush, modifiers homing and sculpt, trigger on_status, condition unaware | `unlock.loot.glyphs_2` (doc 14) |
| `family.ember` | modifiers linger and volatile, trigger on_kill, condition burning | `unlock.glyph.ember` (doc 15 Script) |
| `family.hollow` | essence shadow, modifiers siphon and leech, condition dark | `unlock.glyph.hollow` |
| `family.mirror` | essences veil and dominion, modifiers twin and split | `unlock.glyph.mirror` |
| `family.storm` | essences thunder and gale, modifier ricochet, trigger proximity | `unlock.glyph.storm` |
| `family.thorn` | essences venom and polymorph, modifier delay, trigger on_struck | `unlock.glyph.thorn` |
| `family.tide` | essence tide, condition wet, modifier anchor, trigger pulse | `unlock.glyph.tide` |
| `family.<exp_genre_suffix>` | the 4 glyphs a Folio Expansion adds | that expansion's Script (doc 15) |

```rust
/// Contents of a looted glyph tome. `rng` is the stream doc 11 uses for that drop
/// (rng.loot_place for placed tomes, keyed rng.loot for drops). Exactly n draws.
fn glyph_tome(school: SchoolId, d: i32, set: &ContentSet, rng: &mut Rng) -> Vec<GlyphId> {
    let max_tier = min(3, 1 + floor_div(d - 1, 5));          // tier 1 from Folio 1, tier 2 from 6, tier 3 from 11
    let mut pool: Vec<GlyphId> = set.loot_pool_glyphs()        // triggers and conditions count as school.spellcraft
        .filter(|g| g.school == school && g.tier <= max_tier).sorted_by_id();
    if pool.is_empty() { return glyph_tome(SCHOOL_EVOCATION, d, set, rng); }   // evocation tier 1 is never empty
    let n = min(2, pool.len());
    (0..n).map(|_| pool.remove(rng.range(pool.len() as u64) as usize)).collect()
}
```

### 8.2 Grimoire

- Slots: Canon §6.4 row 27. Memorizing a spell into a slot is a 120-tick channel while not `in_combat`; clearing a slot is instant.
- A slot stores glyph references only (§4.6). If slot count drops, the highest-index slots above the new count lock and keep their contents.
- The Grimoire is in the save and survives Folio transitions.

### 8.3 Texts

- **Reading** casts the text's spell at 0 aether. Read time is `max(1, ceil_div(30 × 10000, row17))` ticks (Canon §6.4 row 17) in the Winding state; it is interrupted like a cast. `st.blinded` or `st.silenced` forbids reading. The reader's row 29 applies; `src_lvl` is the text `ilvl`; the text's glyph origins set Contradiction. The text is consumed on resolution; an interrupted read keeps it. A blessed text multiplies its positive magnitudes (`mag_chain`, heal and ward) by 12500; a cursed text multiplies them by 7500 and carries `curse.misprint` (doc 10 §7.5).
- **Inscribing** scrapes one identified `cat.text` of any type, consuming it as a palimpsest, and writes onto it a spell the inscriber can compile, producing one `item.text.inscribed` (a doc 11 template, Interface assumptions) that carries the glyph references: 180 ticks, not `in_combat`, complexity at most `min(row 26 budget, 4 + floor_div(inscription, 10))`, aether `mul_bp(base_cost, 20000 − 100 × inscription)`, then × 7500 with Inscription 50 (§7.3). The new text has `ilvl = d_current`, item `origin_genre` = local genre, glyph origins from the inscriber's Lexicon, and starts `know.identified` for the party. Inscription refuses channel and charge spells (a `beam` form or `glyph.mod.charged`), because reading is a single activation. A text's 30-tick read and a wand's 12-tick zap replace the form's cast ticks; the form decides only the delivery. Example: Inscription 10 inscribing W5 (base 11250) costs `mul_bp(11250, 19000) = 21375` m-aether.

Effects of doc 11's text templates (doc 10 owns `item.text.appraisal`, `item.text.sanctify` and its five proposed texts). Each spell text is an authored `spell.` record whose glyph origins are the text's `origin_genre`; `use_id` is the doc 10 §5.1 rule.

| Template | Effect | Targeted | `use_id` |
|---|---|---|---|
| `item.text.mending` | `spell.text_mending` = `nova mend`: heals the reader and every ally within 5000 mm | no | `appr.use.on_effect` |
| `item.text.blink` | the reader teleports up to 8000 mm along the aim to the farthest standable cell with line of effect (the `ench.blink` rule) | no | `appr.use.always` |
| `item.text.mapping` | every room outline, door and exit of the current Folio appears on the party map (doc 17); secrets stay hidden | no | `appr.use.always` |
| `item.text.demolition` | `spell.text_demolition` = `nova force shape`: force damage to hostiles and shape against `destructible` blocks within the nova | no | `appr.use.on_effect` |
| `item.text.recharge` | the chosen wand: uncursed sets charges to max and lowers max by 1 (never below 1); blessed sets charges to max and keeps max; cursed sets charges to `mul_bp(old max, 7500)` and lowers max by 1 | yes, `cat.wand` | `appr.use.always` |
| `item.text.warding` | `spell.text_warding` = `nova ward`: a `st.shielded` pool on the reader and every ally within 5000 mm | no | `appr.use.on_effect` |
| `item.text.silence` | `spell.text_silence` = `field hush`: a null field at the aimed point (§14.4) | no | `appr.use.always` |
| `item.text.summoning` | `spell.text_summoning` = `manifest summon edge`: one `summon.hound` | no | `appr.use.always` |
| `item.text.inscribed` | the inscribed spell | as the spell | never type-hidden; created identified (doc 10 §2) |

### 8.4 Wands

| Rarity | Max charges |
|---|---|
| Common, Uncommon, Rare, Epic | 4, 5, 6, 7 |
| Legendary, Exotic | 8 |
| Palimpsestic | 10 |

- A wand holds one `spell.` of complexity at most 8, drawn by doc 11 from the wand pool below. All its glyph origins equal the wand's `origin_genre`.
- **Zap**: spends 1 charge at Winding start; Winding lasts `max(1, ceil_div(12 × 10000, row17))` ticks; the wielder's row 29 applies; `src_lvl` is the wand `ilvl`; step 0 uses the wand's rarity, quality, enhancement and material `dmg_bp`. No aether, no cooldown. Zapping an unknown wand identifies its type (doc 10).
- **Recharge**: +1 charge per 2400 ticks while carried. A recharge service or recharge text sets charges to max and lowers max by 1, never below 1.
- **Snap**: hold use for 60 ticks to destroy the wand; its spell's essences resolve as a nova of radius 3000 mm centered on the wielder, including the wielder, at magnitude × `(5000 + 2500 × charges)`. A wand with 0 charges cannot be snapped.
- A wand is zapped from the inventory through the quick-cast wheel or equipped in `slot.offhand` (doc 10 §4.3). Its Contradiction and misfire roll use `rng.combat` with the zap's key (§19.2); a failed zap is a fizzle and its charge is spent. Cursed wands carry one doc 10 §7.3 zap curse: `curse.misfire` adds its `m` to the failure chance (§3.9), `curse.backfire` makes the effect originate at and target the zapper on a successful `rng.ident` roll, `curse.leaking` spends an extra charge, `curse.bloodprice` costs HP.

**Wand pool.** These 15 authored spells carry the tag `wand_pool` (doc 10 §4.2 allows at most 16). Doc 11 draws a generated wand's spell with `weighted` over `wand_weight`; every glyph origin becomes the wand's `origin_genre`. All are complexity 8 or less.

| Spell | Glyphs | Complexity | `wand_weight` |
|---|---|---|---|
| `spell.searing_bolt` | bolt flame | 2 | 1000 |
| `spell.frost_orb` | orb frost widen | 4 | 900 |
| `spell.packet_surge` | bolt storm quicken | 3 | 900 |
| `spell.thorn_sleep` | bolt slumber | 2 | 700 |
| `spell.wand_striking` | bolt force | 2 | 800 |
| `spell.wand_digging` | bolt shape | 3 | 600 |
| `spell.wand_light` | bolt lumen | 2 | 700 |
| `spell.wand_haste` | self swiftness | 2 | 600 |
| `spell.wand_polymorph` | bolt polymorph | 4 | 300 |
| `spell.wand_dispel` | bolt unmaking | 3 | 600 |
| `spell.wand_fear` | cone dread | 2 | 600 |
| `spell.wand_lightning` | chain storm | 3 | 700 |
| `spell.wand_venom` | cone venom | 2 | 700 |
| `spell.wand_conjure` | manifest summon edge | 5 | 400 |
| `spell.wand_invisibility` | self veil | 2 | 500 |

The weights sum to 10000. Beams and channels never enter the pool, because a zap spends one charge per activation.

### 8.5 Implements and tome spells

- Implement fields are in §3.8. A tome holds 2 inscribed spells, inscribed as in §8.3 but not consumed by casting; tome spells cost aether and obey cooldowns like Grimoire spells, and are castable only while the tome is the active implement.
- Bound glyphs (§3.8) roll from: empower, extend, reach, hushed, keen, thrift, heavy, anchor (doc 11 rolls).
- Implement JSON fields (added by this chapter to the doc 05 archetype record): `{ "casting": { "cost_bp": 9500, "cast_speed_bp": 10000, "tome_spell_slots": 0 } }`; the instance carries `{ "bound_glyph": "glyph.mod.keen" }` or none.

## 9. Enchanting

### 9.1 Capacity

```
capacity_points = floor_div(slot_base × (4 + arc_aff), 4) + rarity_bonus      // arc_aff = primary material's arcane affinity 0..=10 (doc 07)
```

| `slot_base` | Value | `rarity_bonus` | Value |
|---|---|---|---|
| weapons, staves, `slot.offhand` items | 4 | Common, Uncommon | 0 |
| body armor | 4 | Rare, Epic | 1 |
| head, legs, ring, neck, trinket, `slot.ward` | 3 | Legendary, Exotic | 2 |
| hands, feet, back, belt, face | 2 | Palimpsestic | 3 |
| wands, texts, consumables | 0 (never enchantable) | | |

An item holds at most 3 enchantments with total points at most its capacity. On an item enchanted by an Erratum with Inscription 25 the capacity is +1 point, and with Inscription 75 the item holds up to 4 enchantments (§7.3). Examples: an iron longsword with `arc_aff` 2 has `floor_div(4 × 6, 4) = 6` points, 7 if Rare. A silver ring with `arc_aff` 8 has `floor_div(3 × 12, 4) = 9`.

### 9.2 Modes and dependency

| Mode | Applies |
|---|---|
| constant | while the item is equipped and not suppressed |
| on-hit | on every hit of the item's base function (weapon hits, staff bolts), inside Canon §10.5 where the entry says |
| on-use | activated from the quick-cast wheel; a fixed number of uses per Folio (+1 when the wearer has Inscription 100), restored on `ev.folio_entered` |

Each enchantment is a component with `dep = {arcana}` (Canon §5.3) and origin = the local genre where it was applied: `Req = max(1, ARC − 2)`. Numbers scale by its own potency (`mul_bp`, floor); Disconnected turns it off. A Rewrite of the item re-origins an enchantment to the new genre only when that genre's ARCANA gate (§9.3) allows the enchantment's tier; otherwise the enchantment keeps its origin and `Req`, so a Rewrite never turns a high-ARCANA tier III enchantment into a portable `Req` 1 one. Null fields suppress every mode (§14.4).

### 9.3 Procedure

At an enchanting station (doc 07), while not `in_combat`: a 300-tick channel; Inscription at least 0, 25 or 50 and local ARCANA at least 1, 4 or 7 for tier I, II or III; `5000 × points` m-aether; one arcane reagent material (doc 07) per point. Enchanting is deterministic and never fails. The station refuses where local ARCANA is 0.

Balance note on the ARCANA gate: an enchantment's `Req` comes from the genre where it was applied, and nothing else depended on that genre, so without the gate every player enchanted in the lowest-ARCANA genre that still allowed it (Cyberpunk, ARC 1, `Req` 1) and got Coherent enchantments everywhere at full strength. With the gate, tier III needs Mythic Fantasy, Fairy Tale or Oneiric (`Req` 6 to 8), tier II also allows ARCANA 4 to 6 genres (`Req` 2 to 4), and the portable `Req` 1 enchantments of ARCANA 1 to 3 genres are tier I only. Removing an enchantment takes 60 ticks and returns nothing. Instance record: `{ "ench": "ench.searing", "tier": 2, "origin": "genre.mythic_fantasy", "param": null, "uses_left": 0 }`.

### 9.4 Enchantment catalog

Points and values are listed for tiers I / II / III.

| ID | Mode | Slots | Points | Effect |
|---|---|---|---|---|
| `ench.focus` | constant | armor, accessories | 2 / 3 / 4 | effective FOC +2 / +4 / +6 |
| `ench.attunement` | constant | armor, accessories | 2 / 3 / 4 | effective ATN +2 / +4 / +6 |
| `ench.logic` | constant | armor, accessories | 2 / 3 / 4 | effective LOG +2 / +4 / +6 |
| `ench.memory` | constant | armor, accessories | 2 / 3 / 4 | effective MEM +2 / +4 / +6 |
| `ench.toughness` | constant | armor, accessories | 2 / 3 / 4 | effective TGH +2 / +4 / +6 |
| `ench.agility` | constant | armor, accessories | 2 / 3 / 4 | effective AGI +2 / +4 / +6 |
| `ench.resistance` | constant | armor, accessories, ward generators | 2 / 3 / 5 | `resist[param]` +1000 / +2000 / +3000 Bp; `param` is any `dmg.` except `dmg.true` |
| `ench.potency` | constant | implements, accessories | 2 / 4 / 6 | +800 / +1600 / +2400 Bp to spell damage in Canon §10.5 step 1 |
| `ench.valor` | constant | weapons | 2 / 4 / 6 | +800 / +1600 / +2400 Bp to weapon damage in step 1 |
| `ench.precision` | constant | any | 2 / 3 / 4 | crit chance gear term +300 / +600 / +900 Bp (Canon §6.4 row 13) |
| `ench.ruin` | constant | weapons, implements | 2 / 4 / 6 | crit damage gear term +1000 / +2000 / +3000 Bp (row 14) |
| `ench.thrift` | constant | implements | 2 / 4 / 6 | cast cost × 9500 / 9000 / 8500 (§6.2) |
| `ench.celerity` | constant | implements | 2 / 4 / 6 | cast speed × 10500 / 11000 / 11500 (§3.4) |
| `ench.lightbearer` | constant | any | 1 / 2 / 3 | the bearer emits `light_block` 8 / 11 / 14 |
| `ench.breath` | constant | face, head, neck | 3 / 5 / — | I: gases breathable as a respirator; II: `res.breath` never drains |
| `ench.surefoot` | constant | feet | 2 / — / — | ignores the `block.ice` and `liq.oil` friction multipliers |
| `ench.searing` | on-hit | weapons, staves | 2 / 4 / 6 | converts 1500 / 2500 / 3500 Bp of the hit's shares to `dmg.fire` and adds `st.burning` power 20 / 30 / 40 |
| `ench.frostbite` | on-hit | weapons, staves | 2 / 4 / 6 | same with `dmg.frost` and `st.frozen` |
| `ench.galvanic` | on-hit | weapons, staves | 2 / 4 / 6 | same with `dmg.shock` and `st.shocked` |
| `ench.corrosive` | on-hit | weapons, staves | 2 / 4 / 6 | same with `dmg.acid` and `st.corroded` |
| `ench.sanctified` | on-hit | weapons, staves | 2 / 4 / 6 | same with `dmg.radiant` and `st.blinded` |
| `ench.vampiric` | on-hit | weapons, staves | 3 / 5 / 7 | leech (§6.6) with factor 1000 / 1500 / 2000 instead of 2500, cap 1000 Bp per hit |
| `ench.siphoning` | on-hit | weapons, staves | 3 / 5 / 7 | siphon (§6.6) with 500 / 750 / 1000 Bp instead of 1500 |
| `ench.disruption` | on-hit | weapons, staves | 2 / 4 / 6 | `st.unraveled` power 20 / 35 / 50, carrier `dmg.arcane` |
| `ench.spellstore` | on-use | implements, accessories | 3 / 5 / 7 | holds one spell of complexity at most 4 / 6 / 8, fixed at enchanting; 1 / 2 / 3 uses per Folio at 0 aether, `src_lvl` = item `ilvl` |
| `ench.blink` | on-use | feet, back, neck | 3 / 4 / 5 | teleports the bearer up to 8000 mm along the aim to the farthest standable cell with line of effect; 2 / 3 / 4 uses |
| `ench.restoring` | on-use | any | 2 / 4 / 6 | heals 2500 Bp of Max HP; 1 / 2 / 3 uses |

Share conversion: `conv = conversion Bp`; every existing share becomes `share − mul_bp(share, conv)`; the new type gains `10000 − Σ reduced shares`. Example: a pure-slash hit with `ench.searing` II becomes slash 7500, fire 2500.

## 10. Genre magic flavors: registers and riders

A spell's register is the origin of its root Form (§4.2). Every rider is part of the genre skin (Canon §5.3), applies only to spells of that register, and never changes a canon formula.

| § | Register | Rider | Exact effect |
|---|---|---|---|
| 10.1 | Spell (Mythic Fantasy) | Ley-Bound | `rider cost_bp` 9500 while an implement is active |
| 10.2 | Sanguine Rite (Gothic Horror) | Blood Price | for a caster without `aug.blood_battery` (§2.3): when aether is below `cast_cost` at cast start, the shortfall `s` is paid as `mul_bp(s, 15000)` mHP removed directly from HP (no pipeline, no ward, `ev.damage_dealt` flag `self_cost`); allowed only if HP stays at least 1000 mHP; never triggers Death's Door |
| 10.3 | Gnostic Rite (Cosmic Horror) | Composure Toll | `rider_pot` 11500; each root resolution is a composure loss of `mul_bp(cast_cost, 5000)` m-composure, which doc 04 multiplies by the MYTHOS composure term (Canon §5.7) |
| 10.4 | Discipline (Space Opera) | Focused Channel | channel drain × 8000 |
| 10.5 | Program (Cyberpunk) | Silent Execution | ignores `st.silenced`; cannot start under `st.shocked` or `st.hacked`; gaining `st.shocked` hard-interrupts it; cast noise −20 dB |
| 10.6 | Rig (Post-Apocalypse) | Jury-Rigged | `rider cost_bp` 8000; cast noise +15 dB; each root resolution fills the caster's eye cell with `gas.smoke` at full density |
| 10.7 | Hex (Weird West) | Quick-Draw | cast ticks × 5000 when the caster changed weapon slot within the previous 30 ticks |
| 10.8 | Gambit (Hardboiled Noir) | Desperation | `rider cost_bp` 7500 while HP is below 5000 Bp of Max HP |
| 10.9 | Technique (Wuxia) | Flowing Body | no §3.6 restriction; jump and dodge never cancel; the no-implement `cost_bp` is 10000 instead of 11000 |
| 10.10 | Device (Steampunk) | Pressure Vessel | no soft-interrupt rolls (§3.7); each root resolution vents one cell of `gas.steam` at the caster's feet cell |
| 10.11 | Bargain (Fairy Tale) | Credit | with no outstanding Debt, the caster may start with aether below `cast_cost`: aether becomes 0 and `Debt = mul_bp(shortfall, 12500)`, refused if Debt would exceed max aether; Debt is repaid first from regeneration (§2.1); a Folio exit with Debt above 0 is a favor loss of `ceil_div(Debt, 1000)` with `patron.fairy_tale` and clears Debt |
| 10.12 | Invocation (Mythic Antiquity) | Heard Above | each root resolution is a favor gain of `floor_div(cast_cost, 10000)` with `patron.mythic_antiquity`, at most 30 per Folio before scaling |
| 10.13 | Lucid Dream (Oneiric) | Dream Logic | a successful root resolution with `v3 < 500` gets Wild Growth (magnitude × 20000) |
| 10.14 | Procedure (The Redaction) | Redundant Copy | the first soft interrupt of each cast restarts Winding from tick 0 instead of ending the cast |
| 10.15 | Word (The Ur-Text) | none | — |

Riders travel with the register: they apply wherever the spell is cast, and none is `law_bound` (Canon §5.10). No launch World Law (doc 12 §4) grants tolerance to spell glyphs, prayers or enchantments.

Worked flavor spells (each with `C = 0`, home genre, glyph term from §2.2):

| Spell | Caster and place | Cost | Potency | Result |
|---|---|---|---|---|
| **W9 Palm of Mountain Force** `touch force heavy`, a Wuxia Technique | Wuxia Origin, ATN 45, Alteration 30, Spellcraft 10, no implement, `d = 6` so `src_lvl = 3` | base `3500 × 13000 = 4550`; `cost_skill 25 → 11250`; 5118 m-aether (no-implement factor 10000 by Flowing Body) | `11150 → 12265 → 13491 → 14840` (glyph term 11000) | `dmg_base 22000 × 13000 = 28600`; `B = 33979`; 50424 mHP `dmg.blunt`, instant with 45 recovery ticks, full move speed; knockback 9000 mm/s; poise × 20000; `st.stunned` p 47, ×S(3) 49 |
| **W10 Gaze of Madness** `cone mind`, a Gnostic Rite | Cosmic Horror Origin, ATN 55, Illusion 40, Spellcraft 20, Rare focus `ilvl` 12, `d = 12` | base 6500; `cost_skill 35 → 10750`; 6987 → ×9500 = 6637 m-aether; composure toll 3318, × 15000 MYTHOS term = 4977 m-composure | `11950 → 13145 → 14459 → ×11500 = 16627` | `dmg_base 14000`; `B = 14000 → ×25804 36125 → ×11000 39737`; 66070 mHP `dmg.psychic` to every target in the cone; `st.dread` p `45 → ×7000 31 → ×16627 51 → ×S(12) 67` |
| **W11 Thorn-Sleep on credit** `bolt slumber`, a Bargain | Fairy Tale Origin, ATN 40, Illusion 25 (slumber's tier-2 gate), Spellcraft 10, focus `ilvl` 4, 2000 m-aether left | base 6000; `cost_skill 21 → 11450`; 6870 → ×9500 = 6526; shortfall 4526, `Debt = mul_bp(4526, 12500) = 5657` | `10750 → 11825 → 13007` | `st.sleeping` p `50 → ×13007 65 → ×S(4) 70`; regeneration 1950 m-aether per s repays the Debt in 175 ticks; leaving the Folio before then costs `ceil_div(5657, 1000) = 6` favor |
| **W12 Boiler Burst** `nova thunder`, a Device | Steampunk Origin, ATN 40, Evocation 25, Spellcraft 15, Common focus `ilvl` 10, `d = 10` | base 7500; `cost_skill 22 → 11400`; 8550 → ×9500 = 8122 m-aether | `10750 → 11180 → 11627` (glyph term 10400) | `dmg_base 14000`; `B = 30405`; 35351 mHP `dmg.sonic` within 5000 mm; `st.deafened` p `40 → ×7000 28 → ×11627 32 → ×12000 (Evocation 25) 38 → ×S(10) 48`; no interruption rolls; one `gas.steam` cell vents at the caster's feet |

## 11. Contradiction for spells

### 11.1 Components of a spell

Every glyph is a component (Canon §5.3) with `dep = {glyph-origin aether axis}`. The **base-function component** of a stage is its core: the Form and its essences. The spell's `C` is the maximum `C` over the root core glyphs; `E` is `min(6, max raw_gap)` over the same glyphs when `C <= 5` (Canon §5.6). Pipeline steps 2.5 and 2.6 use these. Modifiers, triggers and conditions are per-component (§5.4). Payload stages compute their own core `C` at resolution for their potency, with no failure roll. Any Disconnected core glyph makes the spell uncastable (greyed, torn-page icon). Contradiction is re-evaluated by the Canon §5.5 events; each change emits `ev.contradiction_changed`.

### 11.2 Fizzle (C = 3)

A failed roll at C = 3 is a **fizzle**: the full cast cost stays spent, nothing resolves, the cooldown keys are set, a 70 dB noise and a puff of the register's VFX occur at the caster, and `ev.spell_cast` is emitted with `fizzled = true` (doc 03 payload). The same fizzle follows a failed roll by an NPC at any C (doc 13 §5), a curse misfire (§3.9), and a resolution whose core glyph went Disconnected during the cast (no roll). Every failed roll at C ≥ 3 by any character, Erratum or NPC, counts as a Contradiction failure toward `st.unmoored` (Canon §5.11): the third within 600 ticks applies it for 300 ticks, and while it lasts a Strained spell (C 1 or 2) rolls as C = 3.

### 11.3 Wild surge (C >= 4)

A failed roll at C = 4 or C = 5 is a **wild surge**: the cost stays spent, the cooldown starts, and the entry chosen by `v1` (§3.9) with the weights below happens instead of the spell. `v2` is the entry parameter. Emits `ev.wild_surge`.

| # | Entry | Weight C=4 | Weight C=5 | Effect |
|---|---|---|---|---|
| 1 | Smoke and Noise | 2000 | 1000 | fizzle effects plus the caster's eye cell fills with `gas.smoke`; 90 dB |
| 2 | Backfire | 1200 | 1500 | the root stage resolves with the caster as target and origin, magnitude × 5000; harmful hits on the caster skip the friendly-fire step |
| 3 | Misfire | 1000 | 1000 | the root stage resolves with aim yaw `v2` (Bam) and pitch 0 |
| 4 | Inversion | 800 | 900 | essences swap to their opposed partners (§4.3 check 6), others unchanged; the result resolves if it passes §4.3.1, else Smoke and Noise |
| 5 | Wild Growth | 800 | 600 | resolves normally at magnitude × 20000 |
| 6 | Aether Burn | 1000 | 1000 | aether set to 0; the caster takes a `dmg.arcane` hit of 500 Bp of Max HP entering at Canon §10.5 step 4 |
| 7 | Tongue-Tied | 700 | 700 | `st.silenced` direct on the caster, base 240 ticks |
| 8 | Blink | 600 | 600 | the caster teleports 8000 mm toward yaw `v2` to the farthest standable cell with line of effect |
| 9 | Toad | 400 | 600 | the caster gains `st.polymorphed` as `summon.poly.toad` by direct application, base 300 ticks (Canon §11.4, arcane scaling) |
| 10 | Red Ink | 500 | 800 | the caster gains 1 stack of `st.erasure` |
| 11 | Misprint | 300 | 500 | a hostile `summon.misprint` spawns at the standable cell nearest to 3000 mm from the caster toward yaw `v2` |
| 12 | Flash | 700 | 800 | `st.blinded` direct on every entity within 6000 mm, the caster included |

Selection: cumulative weights in table order; entry `i` is chosen when `v1` falls in its interval. With `v1 = 2458` at C = 4 the cumulative bounds 2000 and 3200 select entry 2 (W8).

## 12. Summons, pets and polymorph

### 12.1 Minion contract

- A spell summon is a minion (Canon §15.1): `fac.errata`, `owner_slot`, `st.bound`, `ev.minion_bound` on spawn and `ev.minion_released` on death, release or despawn. It despawns on `ev.folio_exited`.
- Cap `MINION_CAP_PER_PLAYER = min(5, 1 + floor_div(max(conjuration, handling), 25))`. Summoning at the cap first releases the owner's lowest-EntityId spell summon. At `FOLIO_MINION_CAP = 16` the cast cannot start unless the owner has a spell summon to release.
- **Stats** with `pot` = the summoning stage's `spell_pot` snapshot: Max HP `template_hp → × P(src_lvl) → × pot → × each [T] summon HP factor in ascending ID byte order` (doc 04 Conjuration 50 12500, Leadership 25 11500, Oracle talent `spirit_guide` 12000; doc 14 `echo.briar_thornwood` 11500); AR `template_ar → × T(src_lvl)`; attack base `template_dmg → × P(src_lvl) → × pot`, entering Canon §10.5 at step 1 (no step 2.1); status power `template_p → × pot → × 20000 with Conjuration 100`, then Canon §11.2 with `S(src_lvl)`; effective sub-attributes `40 + floor_div(3 × (src_lvl − 1), 2)`, capped 150, for NPC-row formulas (Canon §6.5).
- **Duration** `3600 → × (10000 + 50 × conjuration) → × duration_bp → × C potency → × 15000 with Conjuration 25` ticks. The familiar lasts until Folio exit.
- **Resistances**: template resistances, +1500 Bp on every type with Conjuration 75, clamped to the NPC cap 9000 (Canon §10.2). Doc 04's Oracle talent `consecrated` converts 3000 Bp of each summon hit's shares to `dmg.radiant` by the §9.4 conversion rule.
- **Owner states** (doc 13 §6): owner downed, the summon guards the owner; owner dead or a ghost, every spell summon of that owner despawns (`ev.minion_released`, reason 0).
- **Contradiction**: the summon's components depend on the summon glyph's axis and origin, evaluated at the summon's position with the owner's tolerance. Step 2.5 of its hits uses that potency. Disconnected makes the summon inert (no attacks, no specials) until reconnected.
- **AI hooks** (doc 13 runs them): think every 6 ticks. Target priority: the `input.minion_attack` target; else the most recent attacker of the owner within 12000 mm; else the nearest hostile within 12000 mm of the summon (minimum `dist2`, ties to lower EntityId). Leash 16000 mm from the owner. `input.minion_hold` holds position; `input.minion_follow` follows at 3000 mm. Profiles: `support`, `artillery` (keeps 6000 to 8000 mm), `guardian` (stays within 4000 mm of the owner), `skirmisher`, `healer`, `decoy`, `bodyguard` (stays between the owner and the owner's nearest attacker).

### 12.2 Summon templates

Values are at `P = 10000` and `pot = 10000`. The summon template is chosen by the secondary essence of `glyph.ess.summon`.

| ID | Secondary essence | Body | HP (mHP) | AR | Attack (mHP, type, interval ticks, reach mm) | Move (mm/s) | Profile | Special |
|---|---|---|---|---|---|---|---|---|
| `summon.familiar` | none | avian | 30000 | 0 | 4000, arcane, 60, 8000 ranged | 6000, flies | support | the pet: one per Erratum, no duration; every 120 ticks applies `st.marked` (480 ticks) to each hostile it sees within 12000 mm; emits light 8 |
| `summon.elemental` | flame, frost, storm, acid, thunder, arcane | amorphous | 60000 | 0 | 8000, that essence's type, 45, 8000 ranged; status power 30 | 4000 | artillery | immune to its own type |
| `summon.servitor` | shadow | humanoid | 70000 | 10 | 10000, slash, 60, 2000 | 4300 | guardian | tagged undead; immune to `st.poisoned` and `st.diseased` |
| `summon.swarm` | venom | swarm | 40000 | 0 | 3000, toxin, 15, 1000; status power 20 | 5000, flies | skirmisher | — |
| `summon.wisp` | radiance, mend | amorphous | 35000 | 0 | none | 6000, flies | healer | every 120 ticks heals the lowest HP-ratio ally within 10000 mm by `mul_bp(400, pot)` Bp of Max HP; emits light 12 |
| `summon.golem` | shape | construct | 140000 | 20 | 14000, blunt, 90, 2000 | 3000 | guardian | every 300 ticks applies `st.taunted` direct to hostiles within 5000 mm |
| `summon.hound` | edge, force | quadruped | 60000 | 5 | 9000, slash (edge) or blunt (force), 45, 2000 | 7000 | skirmisher | — |
| `summon.phantasm` | mind | humanoid | 40000 | 0 | none | 4300 | decoy | 2 spawn per cast with Illusion 100, each counting toward the cap; on spawn `st.taunted` direct to hostiles within 6000 mm; on death `st.confused` power 60 (carrier psychic) within 3000 mm and 100 dB noise |
| `summon.sentinel` | ward | construct | 80000 | 30 | none | 6500, flies | bodyguard | a hostile projectile whose path passes within 1500 mm of it hits it instead |
| `summon.herald` | prayer only (§16.3) | humanoid | 150000 | 30 | 16000, radiant, 60, 2500 | 6000, flies | guardian | friendly: 3600 ticks; hostile: an NPC of `rank.elite` |
| `summon.misprint` | surge only (§11.3) | amorphous | 50000 | 0 | 10000, arcane, 60, 2000 | 5000 | hostile (doc 13 hunter behavior) | `fac.editors`, `rank.standard`; erases Errata-placed blocks like an Editor (Canon §11.6) |

A hostile herald or misprint is an enemy: HP `template → × H(d_current) → × rank HP → × party HP`, damage `template → × T(d_current) → × rank damage`.

### 12.3 Polymorph templates (`st.polymorphed`)

| ID | Registers | Body | Capsule r / h (mm) | Move (mm/s) | Jump (mm) | Attack (reach mm, Bp of reference, interval ticks) | Special |
|---|---|---|---|---|---|---|---|
| `summon.poly.toad` | fairy_tale, wuxia | quadruped | 200 / 400 | 2400 | 1500 | 1500, 1000, 60 | swims at 3000 |
| `summon.poly.sheep` | mythic_fantasy, mythic_antiquity | quadruped | 350 / 1000 | 3000 | 600 | none | — |
| `summon.poly.rat` | gothic_horror, post_apocalypse, hardboiled_noir | quadruped | 150 / 300 | 5000 | 500 | 600, 1500, 30 | passes 1-block gaps |
| `summon.poly.crow` | weird_west, oneiric | avian | 200 / 400 | 4000 | 600 | 800, 1000, 45 | gravity × 2000 while falling |
| `summon.poly.ooze` | cosmic_horror, space_opera | amorphous | 450 / 600 | 1500 | 0 | 1000, 2500, 60 | immune to `st.bleeding` |
| `summon.poly.paper` | cyberpunk, steampunk, redacted, ur_text | humanoid | 300 / 1800 | 4300 | 1100 | none | `resist[dmg.fire] = −10000` |
| `summon.poly.beast` | `afflict.beast` only | quadruped | 400 / 1200 | 7500 | 1700 | 2200, Errata `B = mul_bp(mul_bp(20000, P(d_current)), 15000)`, 40 | Canon §16.4 |

Rules:

- `glyph.ess.polymorph` applies the template of the spell's register by direct application: base `1800 → × duration_bp → × C potency`, then Canon §11.4 with the arcane threshold, then the rank control multiplier. Bosses, editors and hunters are immune (Canon §11.4).
- While polymorphed: equipment is stowed (no item component works, enchantments included); no casting, prayer, item use, block, sprint or dodge; movement, capsule, jump and attack come from the template; AR 0 on every part; Max HP is unchanged, so the HP ratio is kept; parts map to the template body plan with every pool at the current HP ratio. The **reference** attack B is the bearer's primary attack B (NPC: template damage × `T(d)` × rank; Erratum: `10000 × P(d_current)`).
- The status ends at expiry, by dispel, or when damage taken since application reaches 2000 Bp of Max HP (4000 when the applier has Alteration 75). On revert, original parts take the current HP ratio; severed parts stay severed.

## 13. Spells and the world

### 13.1 Interaction table

A spell hit or pulse applies its damage types to the cells it touches: the impact cell for bolts, beams, touch and chain; every cell of the area for orb, cone, nova, rune, aura, field and wall; at most 512 cells per resolution in ascending local block index. Canon §12.1 triggers fire for those cells (fire on oil ignites, frost freezes water, shock electrifies, radiant cleans red ink) and emit `ev.liquid_reaction`. Block damage per hit is `floor_div(block_power × form magnitude, 10000)` in doc 07 block-HP points and applies only to blocks tagged `destructible` (Canon §4.3); every other block takes none. Fire never removes a block that is not tagged `destructible`.

| Essence | Blocks | Liquids and gases | Light and sound |
|---|---|---|---|
| flame | ignites flammable blocks in touched cells (doc 03 fire grid) | Canon §12.1 fire rows; `linger` and field leave fire on flammable blocks | light 10 at impact for 6 ticks |
| frost | — | water, blood, alcohol become `block.ice` (Canon §12.1); fire cells in the area go out | — |
| storm | — | electrifies connected water, blood or alcohol (Canon §12.1) | — |
| acid | block power 3, × 2 on metal blocks | `linger` leaves 1000 ml `liq.acid` | — |
| force | block power 6 | — | — |
| edge | destroys `block.web` and `destructible` foliage blocks in touched cells | — | — |
| radiance | — | `liq.red_ink` becomes water (Canon §12.1) | light 15 at impact for 12 ticks |
| shadow | — | — | field and `linger` areas lower effective light by 8 (floor 0) for `light_at` inside |
| thunder | block power 4, × 3 on glass and ice | — | Canon §10.1 noise per hit; ends `st.frozen` |
| venom | — | field pulses and `linger` add one full-density `gas.miasma` cell at the center, at most 8 per instance | — |
| tide | — | adds `liq.water`: bolt 2000 ml, orb 6000, cone 3000, touch 1000, nova 4000, beam 1000 per s, field 1000 per pulse (at most 16000 per instance); hit entities gain `st.soaked`; ends `st.burning` | — |
| gale | — | moves every gas cell in the area up to 3 cells along the push direction (doc 08); pushes entities 8000 mm/s | — |
| lumen | — | — | light source: self 12 following the caster; bolt, orb, touch 14 at the impact block; aura and field 15 at the center; 3600 ticks × `duration_bp`; at most 8 per caster, oldest removed |
| shape | §13.2 | — | — |
| sight | reveals hidden traps and secrets in the area (doc 09 auto-detect); a self sight covers 6000 mm around the caster (× 15000 with Divination 25) | — | — |

Spell noise: every root resolution emits `ev.noise` at the caster with the form's cast noise (§5.2) adjusted by `glyph.mod.hushed` (−30) and riders. Each impact or burst emits `50 + 10 × essence count` dB at the impact point, after hushed; thunder adds its Canon §10.1 noise.

### 13.2 Shaping stone

`shape_power = 3 + floor_div(alteration, 25)`, +2 with Alteration 50. Bolt and touch remove the first collision block they meet if it is tagged `destructible` and its doc 07 hardness is at most `shape_power`; the block drops as if mined (doc 07). Nova removes every such block within 2000 mm, at most 27 in ascending local index. Wall places 15 **conjured** blocks of the local genre's stone (doc 07), a temporary barricade under Canon §4.3, in the wall's air and liquid cells; conjured blocks vanish at the wall's expiry, drop nothing, count as Errata-placed for Editor erasure (Canon §11.6) and emit `ev.block_changed`. Wall with bind places `block.web` cells; wall with frost places `block.ice` cells; both vanish at expiry. Damage walls of other essences place no blocks.

## 14. Dispel, silence, counterspell and anti-magic

### 14.1 Dispel

A status instance is **dispellable** when a spell, prayer, wand, text, enchantment or spell summon applied it (Canon gap G9). `st.erasure`, `st.downed`, `st.ghost`, need statuses, afflictions and indicators never are. An unmaking hit on an entity removes up to `n = 2 + floor_div(abjuration, 25)` dispellable instances (+1 with Abjuration 50; NPC casters use `n = 2`; minus 1, minimum 1, when unmaking is secondary): harmful ones from the caster and allies, helpful ones from hostiles, latest application tick first, ties to the higher status ID. It also ends hostile-owned spell entities in its target or area (fields, walls, runes, auras, lights, conjured blocks, spell projectiles), firing their on_expire triggers; deals a `dmg.true` hit of 3000 Bp of Max HP, entering Canon §10.5 at step 4, to each hostile spell summon; and suppresses the constant and on-hit enchantments of a hostile's equipment for 180 ticks.

### 14.2 Silence

`st.silenced` forbids Grimoire spells, tome spells, reading texts, supplication and petitions. Wands, on-use enchantments and summons keep working. Programs ignore it (§10.5). Sources: `glyph.ess.hush` buildup, counterspell, `counter.disruptor` (Canon §17.3a), the Tongue-Tied surge.

### 14.3 Counterspell

An unmaking hit on an entity in Winding, Channeling or Charging hard-interrupts the cast with no roll, the target's debited aether stays spent, and the target gains `st.silenced` by direct application with base 60 ticks (180 when the caster has Abjuration 75). Unmaking projectiles and areas also destroy hostile spell projectiles they touch. NPC casters counterspell Errata by the same rule.

### 14.4 Anti-magic

`glyph.ess.hush` in a field creates a **null field** instead of applying buildup. Inside its cylinder: no spell, text, wand, prayer or on-use enchantment can start, and casts in progress are hard-interrupted; spell projectiles that enter end with no effect; overlapping spell entities pause (their timers still run); constant and on-hit enchantments of carried items are suppressed; spell summons take a `dmg.true` hit of 200 Bp of Max HP per pulse, entering Canon §10.5 at step 4. It affects every faction, the caster included, unless `glyph.mod.sculpt` exempts `fac.errata`. Genre anti-magic is Contradiction itself: in `genre.hardboiled_noir` (ARC 0) an arcana glyph has `raw_gap = Req`.

## 15. Statuses owned by this chapter

Canon §11.1 fixes stacking, base duration and the canonical effect; these are the remaining rules.

| Status | Rules |
|---|---|
| `st.withering` | built by `dmg.umbral`; heal × 5000 and DoT as canon |
| `st.rooted` | cannot move, jump or dodge; can act and cast |
| `st.silenced` | §14.2 |
| `st.unraveled` | removes 10000 m-aether on application (floor 0); regeneration 0; NPCs: ends `elite.surging` healing (Canon §17.3a) |
| `st.sleeping` | canon and Canon §11.8; an NPC's awareness drops to its lowest state (doc 09) |
| `st.charmed` | the bearer's AI treats the applier's faction as its own; not a minion, outside every cap, its kills give no XP; ends on any damage from the applier's faction |
| `st.feared` | AI flees from the source position and cannot attack; players per Canon §11.8 |
| `st.confused` | every 30 active ticks, `keyed_rng("rng.status", folio, bearer, t, 0x3000).range(32768) − 16384` Bam rotates movement input until the next rotation; 1 draw |
| `st.berserk` | AI attacks the nearest entity of any faction (minimum `dist2`, ties lower EntityId) |
| `st.dread` | players: composure −15 on application, regeneration 0; NPCs have no composure: doc 13 raises flee utility, no numeric effect |
| `st.hasted` | includes cast speed (§3.4) |
| `st.regenerating` | 200 Bp of Max HP per s through the Canon §11.3 accumulator (`k = 4`) |
| `st.shielded` | reapplication sets the pool to `max(current, new)` and refreshes expiry; pool at most 5000 Bp of Max HP; a ward cast by a caster with Abjuration 100 also applies `st.fortified` for 300 ticks (§7.3) |
| `st.invisible` | a harmful root resolution by the bearer ends it (Canon §11.1 "ends on attack"); a veil cast by a caster with Illusion 25 lasts × 15000 (§6.4) |
| `st.empowered`, `st.fortified`, `st.blessed` | as canon |
| `st.levitating` | gravity 0; jump input rises and crouch input descends at 1500 mm/s; on expiry the fall peak is the expiry position |
| `st.polymorphed` | §12.3 |

## 16. Prayers and Devotion

### 16.1 Patrons

Each patron `patron.<genre_suffix>` is the Author of its genre. Prayers to a patron are components with `dep = {mythos}` and origin = the patron's genre, so `Req = max(1, MYT − 2)` of that genre (Canon §16.5).

| ID | Name | Signature boon (pleased, no need) | Wrath (displeased; angry doubles durations) | Taboo (favor loss) |
|---|---|---|---|---|
| `patron.mythic_fantasy` | Ysolde of the Thousand Roads | `st.blessed` 1800 ticks | `st.weakened` 1800 | leaving a Folio while an ally's Remains stay behind: 100 |
| `patron.gothic_horror` | The Vigil-Keeper | `st.regenerating` 1200 | `st.withering` 480 and composure −20 | each `summon.servitor` cast: 30 |
| `patron.cosmic_horror` | The Sleeper Under the Index | one unknown carried item type becomes known (doc 10); composure −10 | composure −30 | destroying a `cat.book`: 50 |
| `patron.space_opera` | The Admiral of the Last Fleet | `st.shielded` 4000 Bp, 1800 ticks | `st.slowed` 600 | a friendly-fire hit removing at least 1000 Bp of an ally's Max HP: 20 |
| `patron.cyberpunk` | The Sysop | `st.hasted` 1200 | `st.silenced` 1800 | killing a non-hostile NPC: 50 |
| `patron.post_apocalypse` | Mother Rust | satiation +600, toxicity −50000 | 3 stacks of `st.irradiated` | satiation reaching 0: 30, once per Folio |
| `patron.weird_west` | The Rider at the Crossroads | `st.focused` 1800 and `st.inspired` 600 | `st.marked` on the Erratum 1800 | killing a `st.feared` enemy: 5 |
| `patron.hardboiled_noir` | The Witness | `st.marked` 480 on every hostile within 30000 mm | lose 1000 Bp of carried Lucre | each successful pickpocket: 10 |
| `patron.wuxia` | The Sage of the Ninth Peak | `st.inspired` 1800 | `st.confused` 300 | each sneak-attack kill: 5 |
| `patron.steampunk` | The Grand Engineer | `st.fortified` 1800 | `st.burning` 240 | an equipped item reaching durability 0: 20 |
| `patron.fairy_tale` | The Godmother of Thorns | `st.blessed` 900 and Bargain Debt cleared | `st.polymorphed` as toad, direct, base 300 (Canon §11.4) | Folio exit with Debt: `ceil_div(Debt, 1000)` |
| `patron.mythic_antiquity` | The Laurel-Crowned | `st.empowered` 1200 | a `dmg.radiant` hit of 2000 Bp of Max HP entering Canon §10.5 at step 4 | 40 kills since the last sacrifice or shrine prayer to this patron: 100, counter resets (the per-Erratum mirror of doc 12's `law.mythic_antiquity.hubris`) |
| `patron.oneiric` | The Dreamer Who Writes Us | aether to max and `st.levitating` 600 | `st.sleeping` 240 | waking a `st.sleeping` creature with damage: 5 |
| `patron.redacted` | The Censor | removes 2 stacks of `st.erasure` | adds 2 stacks of `st.erasure` | reading a `cat.text`: 5 |
| `patron.unwritten` | The Blank Page | removes `st.unmoored`, clears the Canon §5.11 failure count, and `st.blessed` 900 | none | none |
| `patron.ur_text` | The First Author | removes every DoT, debuff, control and mental status; HP to max | `st.dread` | none |

An Erratum who breaks an altar block loses 200 favor with that altar's patron. The Blank Page's boon used to set the prayer timer to 0, which let a full-health Erratum pray to it every 60 ticks for +50 favor each time and reach 1000 favor in 20 prayers; its boon now leaves the timer alone.

### 16.2 Favor

- `res.favor` is per Erratum and per patron: 16 values, `−1000..=1000`, starting 0 each run (Canon gap G12).
- Gains × `(100 + devotion) / 100`, losses × `100 / (100 + devotion)`, integer floor (Canon §16.5). Devotion 20: a gain of 50 becomes 60, a loss of 100 becomes 83.

```
gain(x) = floor_div(x × (100 + devotion), 100) → × each [T] favor-gain factor in ascending ID byte order
          // doc 04 Oracle talent devout 12500; Origin trait favored 12500 (Mythic Antiquity Origin)
loss(x) = floor_div(x × 100, 100 + devotion)
favor   = clamp(favor ± result, −1000, 1000)
```

- **Shrines.** A supplication, petition or offering made within 3000 mm of an altar block of the chosen patron is made at a shrine. Doc 12's `law.mythic_antiquity.favor` adds +50 before scaling to each Erratum's first prayer per Folio at a `room.shrine`, and a `secret.room.shrine` altar adds +100 before scaling to its first prayer.
- **Prayer timer increments** (§16.3, §16.4) are multiplied by 7500 with the Oracle talent `divine_favor` (doc 04).
- Sources: sacrifice (§16.5); exiting a Folio whose genre is the patron's: +25; each `fac.editors` kill: +10 to the local patron; Heard Above (§10.12); prayer outcomes; taboos; altar breaking; Bargain Debt (§10.11). Each change emits `ev.favor_changed` (Canon gap G3).

### 16.3 Supplication

`prayer.supplication` asks the chosen patron (default: the local genre's) for help. It is a 60-tick Winding cast, interruptible by §3.7 and forbidden by `st.silenced` and `st.downed` (doc 16 forbids every cast while downed). Each Erratum has `prayer_timer: Ticks`, starting 0 and falling by 1 per tick to 0. Contradiction of the prayer is rolled like a spell (`rng.combat`, key `(erratum, input tick, 0)`); a failure is **Unheard**: no effect, timer +3600, favor unchanged, and it counts toward `st.unmoored` like a fizzle.

```
r  = keyed_rng("rng.magic", folio, erratum, input_tick, 0)                 // exactly 4 draws, always
r0 = r.range(500); r1 = r.range(180_000); r2 = r.range(10000); r3 = r.range(65536)
f  = favor before this prayer
if timer == 0:
    if f >= -(r0 as i32): PLEASED   timer += 18000; favor += gain(50); resolve one need
    else:                 DISPLEASED timer += 36000; favor -= loss(50); wrath
else:
    if f > r0 as i32 && (timer as u64) < r1: HELPED  timer += 90000; favor -= loss(75); resolve one need
    else: ANGRY  timer += 180000; favor -= loss(100); wrath with doubled durations;
          if f < -250 && r2 < 2000: a hostile summon.herald spawns 6000 mm away toward yaw r3
if PLEASED && f > 250 && r2 < 500:        a friendly summon.herald for 3600 ticks
else if PLEASED && f > 100 && 500 <= r2 && r2 < 1000:
    Gift: +1 enhancement to the active weapon or implement (within Canon §13.7 caps, ilvl unchanged, so a Gift is
          weaker than an Emendation); if f >= 500, also +1 to the lowest-uid other equipped item
// skill.devotion.m100: the Erratum's first supplication each Folio evaluates as if timer == 0 (its increment still applies)
// skill.devotion.m50: a PLEASED answer resolves the first two needs that apply instead of one
```

Balance note on `r1`: Canon §16.5 scales IVAN's timers by 3.6, which would make IVAN's `Timer < RAND_N(500000)` into `range(1_800_000)`. In real time that is 500 minutes, so an Erratum with 500 favor or more could pray again right after a PLEASED answer and be HELPED with 99% chance, then with 94%, then 89%, each answer resolving a need such as a full heal, until its favor fell below 500. With `range(180_000)` a second prayer right after PLEASED (timer 18000) is HELPED with 90% chance, a third (timer 108000) with 40%, and a fourth never.

Needs, resolved in this priority (the first that applies): (1) HP below 3333 Bp of Max HP: HP to max; (2) a severed part: every severed part reattached (Canon §9.1); (3) `st.madness` or composure below 2500 Bp: composure to max; (4) a cursed equipped item: the lowest-uid one uncursed (Canon §13.7 Emendation rule without the Emendation); (5) any DoT status: all removed; (6) satiation at most 600: set to 1800; (7) aether below 3333 Bp: aether to max; (8) otherwise the signature boon. Praying to the local genre's patron multiplies boon and wrath durations and petition magnitudes by 15000 (Canon §16.5); with Devotion 75 every other patron applies 12500 (§7.3).

Worked draws (run seed `0x0123456789ABCDEF`, Folio 5, Erratum `(5 << 40) | 2`, tick 18000, seq 0): `r0..r3 = 299, 125411, 2047, 7972`. Favor 120, timer 0: `120 >= −299`, PLEASED; `r2 = 2047` gives no herald and no gift. Favor −300, timer 0: `−300 < −299`, DISPLEASED. Favor 300, timer 20000: `300 > 299` and `20000 < 125411`, HELPED. At tick 18002 the draws are `491, 36979, 9278, 14839`; the same favor 300 and timer 20000 fail `300 > 491` and give ANGRY.

### 16.4 Petitions

A petition spends favor for a fixed effect, with no outcome roll. It needs favor at least the minimum and `prayer_timer <= 7200`; it is a 30-tick Winding cast; it costs its favor (× 8000 with Devotion 25) as a loss scaled by Devotion and adds 3600 ticks to `prayer_timer`, so from a timer of 0 three petitions chain and the fourth waits 60 s. The gate was `prayer_timer <= 36000`, which allowed ten petitions back to back: at Devotion 50 and ATN 40 that was ten local-patron heals of 60% of Max HP in 500 ticks, for 330 favor. A Contradiction failure is Unheard and spends no favor. Magnitudes multiply by row 29 with school skill = Devotion, then × 15000 for the local patron, then by Contradiction potency.

| ID | Favor cost | Minimum favor | Effect |
|---|---|---|---|
| `prayer.benediction` | 60 | 0 | `st.blessed` 1800 ticks on the petitioner and allies within 8000 mm |
| `prayer.smite` | 40 | 0 | a `dmg.radiant` hit on the aimed target within 20000 mm, `dmg_base` 30000 mHP, step 0 with the prayer `src_lvl` |
| `prayer.sanctuary` | 80 | 100 | a field of radius 4000 mm for 600 ticks; hostile NPCs inside gain `st.feared` (direct, base 60) every 30 ticks |
| `prayer.mending` | 50 | 0 | heals 2500 Bp of Max HP on the petitioner or a touched ally (4312 Bp at ATN 40, Devotion 50, local patron) |
| `prayer.reattachment` | 150 | 200 | reattaches every severed part of the petitioner or a touched ally |
| `prayer.absolution` | 200 | 300 | uncurses the lowest-uid cursed equipped item |
| `prayer.turning` | 50 | 0 | undead- and fiend-tagged hostiles within 10000 mm gain `st.feared` (direct, base 240) |
| `prayer.provision` | 30 | 0 | satiation +800, at most 2400 |
| `prayer.vigil` | 40 | 0 | composure +5000 Bp of max |
| `prayer.intercession` | 400 | 500 | a ghost ally whose Remains are within 3000 mm resurrects now at 2500 Bp of Max HP (Canon §17.6) |
| `prayer.lantern` | 10 | 0 | a light-15 source at the petitioner's position for 7200 ticks |
| `prayer.oath` | 50 | 50 | `st.empowered` 480 and `st.fortified` 600 on the petitioner |

Example: Devotion 50 pays `floor_div(60 × 100, 150) = 40` favor for `prayer.benediction`.

### 16.5 Sacrifice

`prayer.offering` places items on an altar block (doc 07, doc 12) of a patron. An Erratum-placed altar takes the patron chosen at placement. Each item gives a favor gain:

```
v      = item.value × stack                         // a stack is one offering
base   = isqrt(v) → × 2 if item.origin_genre is the patron's genre
favor += gain(base)                                  // §16.2
```

Items of `cat.currency`, `cat.key` and `cat.quest`, items of value 0 and Errata Remains cannot be offered. A `cat.corpse` of a killed NPC can, with `value = mul_bp(mul_bp(100, P(d_current)), rank_xp_bp)` (Canon §17.3) and `origin_genre` = the NPC's; a corpse tagged `summoned` (doc 13) cannot. A standard-rank corpse at Folio 10 has value 217 and gives `isqrt(217) = 14` before doubling and scaling. Example: value 2500 from the patron's genre gives `2 × 50 = 100`, and with Devotion 20 `floor_div(100 × 120, 100) = 120`. A value-99 item from another genre gives 9. Sacrifice resets the Laurel-Crowned hubris counter for that patron.

## 17. Spell name generator

Names are display text. They never enter simulation state, saves or packets.

```rust
fn spell_name(sp: &CompiledSpell, genre: GenreId) -> String {
    let f  = |g: &GlyphRef| loc_name(g, genre);             // Canon §1.10 resolution order
    let st = &sp.stages[0];
    let (form, ess, mods) = (f(&st.form), join(st.essences.iter().map(f), " and "), join(st.modifiers.iter().map(f), " "));
    let v  = (fnv1a32(&sp.canonical) % 2) as usize;
    let mut s = match (genre_suffix(genre), v) {
        ("cyberpunk", 0) => format!("{ess}.{form}{}", slash_join(&mods)),
        ("cyberpunk", _) => format!("{form}://{ess}{}", slash_join(&mods)),
        ("cosmic_horror", 0) => format!("The {mods} {form} of {ess}"),
        ("cosmic_horror", _) => format!("{ess} {form}, {mods}"),
        ("steampunk", 0) => format!("Patent {mods} {ess} {form}"),
        ("steampunk", _) => format!("{ess} {form}, Mk. {}", 1 + (fnv1a32(&sp.canonical) >> 8) % 9),
        (_, 0) => format!("{mods} {ess} {form}"),
        (_, _) => format!("{form} of {mods} {ess}"),
    };
    if let Some(t) = &st.trigger { s = format!("{s} {} → {}", f(t), spell_name(&sp.payload_spell(1), genre)); }
    normalize_spaces_and_separators(s)                        // collapse doubled spaces; drop dangling ", " and "/"
}
```

Examples: W1 in Mythic Fantasy, `fnv1a32 = 0x009418ee`, `v = 0`: "Flame Bolt". W4, `0x24cc5875`, `v = 1`: "Bolt of Empowered Flame and Lightning". W8 shown in Cyberpunk, `0xd2451055`, `v = 1`: "Packet://Surge/Optimized". W9 in Wuxia, `0xa38cb39d`, `v = 1`: "Palm of Mountain-Crushing Mountain Force". Players may rename a slot; the custom name is client profile data. The optional language model (DESIGN_BRIEF §5) may only replace the displayed string.

## 18. AI spell use hooks

- NPC templates (doc 13) list `spells: [spell ID]`, `aether_max_mae`, `aether_regen_mae_s`. NPC casts use §3 with key `(NPC EntityId, tick on which sys.actions starts the cast, 0)`.
- Pure queries for the doc 13 utility scorer, drawing no random numbers:

| Query | Returns |
|---|---|
| `estimate_damage(spell, caster, target)` | mHP after the full Canon §10.5 pipeline with no crit and volatile factor 10000 |
| `area_members(spell, caster, aim)` | sorted EntityIds the root stage would affect |
| `ally_risk(spell, caster, aim)` | count of the caster's allies in `area_members` |
| `cast_state(entity)` | `Option<{ spell_hash, state, ticks_remaining, form, aim_point }>` for any visible caster |
| `spell_tags(spell)` | the authored `ai.tags` (§4.6) |

- An NPC never casts a harmful area spell with `ally_risk > 0` unless its AI profile is `reckless`. NPCs keep target distance within the spell's `min_range_mm..=max_range_mm`.
- Interrupters (doc 13 profile) prefer targets whose `cast_state.ticks_remaining >= 12`. Counterspell-capable NPCs cast unmaking at Errata in Winding.
- Casting is perceptible: `ev.cast_started` (Canon gap G3) feeds doc 09 sight and the cast noise feeds `ev.noise`.

## 19. Events and random streams

### 19.1 Events

Consumed and emitted canon events: `ev.spell_cast` (root resolution, channel end, and fizzle with `fizzled = true`), `ev.damage_dealt`, `ev.healed`, `ev.status_applied`, `ev.status_expired`, `ev.noise`, `ev.ward_broken`, `ev.minion_bound`, `ev.minion_released`, `ev.entity_spawned`, `ev.entity_despawned`, `ev.contradiction_changed`, `ev.block_changed`, `ev.liquid_reaction`, `ev.item_used`, `ev.item_identified`, `ev.part_reattached`, `ev.player_resurrected`, `ev.folio_entered`, `ev.folio_exited`. In `ev.spell_cast`, `spell` holds the authored spell's `id32`, or 0 for a compiled Grimoire or tome spell, and doc 03 is asked to add `spell_hash: u64`.

Requested additions (Canon gap G3), with the payloads doc 03 is asked to add:

| Event | Payload |
|---|---|
| `ev.cast_started` | `caster: EntityId, spell_hash: u64, source: u8 /* 0 grimoire, 1 tome, 2 text, 3 wand, 4 on-use, 5 prayer */, form: Id32, ct: Ticks, aim_point: Fx3` |
| `ev.cast_interrupted` | `caster: EntityId, spell_hash: u64, cause: u8 /* 0 hard status, 1 soft hit, 2 counterspell, 3 null field, 4 voluntary cancel */, refund_maero: i64` |
| `ev.wild_surge` | `caster: EntityId, spell_hash: u64, c: u8, entry: u8 /* 1..=12 */, param: u16` |
| `ev.glyph_learned` | `erratum: EntityId, glyph: Id32, origin: Id32, source: u8 /* 0 run start, 1 tome, 2 text study, 3 trainer, 4 Rewrite, 5 kit */` |
| `ev.enchant_applied` | `erratum: EntityId, uid: u64, ench: Id32, tier: u8, origin: Id32` |
| `ev.prayer_resolved` | `erratum: EntityId, patron: Id32, prayer: Id32, outcome: u8 /* 0 pleased, 1 displeased, 2 helped, 3 angry, 4 unheard, 5 petition */, need: u8 /* 0 none, 1..=8 */` |
| `ev.favor_changed` | `erratum: EntityId, patron: Id32, old: i32, new: i32, cause: u8 /* 0 sacrifice, 1 prayer, 2 taboo, 3 Folio exit, 4 Editor kill, 5 rider, 6 altar, 7 shrine bonus */` |

### 19.2 Random draws

| Draw | Stream | Key `(actor, tick, seq)` | Count |
|---|---|---|---|
| root resolution values `v0..v3` (spells, texts, wands, on-use enchantments) | `rng.magic` | caster, cast input tick, 0 | 4, always |
| Contradiction and misfire failure of a spell, text, wand, on-use enchantment, supplication or petition | `rng.combat` | caster, input tick, 0 (the doc 05 §1.4 primary-action slot) | 1 when the failure chance > 0, else 0 |
| payload `v0` | `rng.magic` | caster, event tick, `0x4000 \| stage_index << 8 \| fire_index` | 1 |
| interruption | `rng.magic` | caster, hit tick, `0x2000 \| k` | 1 per soft-interrupt hit |
| supplication `r0..r3` | `rng.magic` | Erratum, input tick, 0 | 4, always |
| `st.confused` rotation | `rng.status` | bearer, rotation tick, `0x3000` | 1 per rotation |
| crit per spell hit | `rng.combat` | doc 05 hit key: `source` = the spell projectile for bolt and orb hits, else the caster; seq `16 + i` | 1 per hit (Canon §3.4) |
| psychometry (§5.3) and `curse.backfire` on a zap (§8.4) | `rng.ident` | doc 10 §5.12 action key | 1 each |
| glyph tome contents (§8.1) | the stream doc 11 uses for that drop | doc 11 | 2, or the pool size when it holds fewer |
| wand spell (§8.4) | the stream doc 11 uses for that item | doc 11 | 1 `weighted` |

A supplication and a spell never share a key, because both are casts and a caster starts at most one cast per tick (§3.5).

Compilation, cost, potency, buildup, enchanting, favor, summon AI target choice and naming draw nothing.

## Exports

All IDs below are in namespaces Canon §19 assigns to doc 06.

| Namespace | IDs |
|---|---|
| `glyph.form.` (13) | bolt, orb, beam, cone, touch, self, aura, field, wall, rune, nova, chain, manifest |
| `glyph.ess.` (33) | flame, frost, storm, acid, force, edge, radiance, shadow, mind, arcane, venom, thunder, mend, ward, swiftness, levitate, veil, bind, dread, dominion, slumber, hush, polymorph, unmaking, summon, shape, tide, gale, lumen, sight, appraise, omen, psychometry |
| `glyph.mod.` (22) | empower, extend, widen, reach, quicken, charged, split, homing, pierce, ricochet, linger, delay, hushed, thrift, volatile, keen, siphon, leech, sculpt, anchor, heavy, twin |
| `glyph.trig.` (8) | on_impact, on_kill, on_expire, timer, proximity, on_struck, on_status, pulse |
| `glyph.cond.` (8) | foe, ally, wounded, native, wet, burning, dark, unaware |
| `school.` (8) | evocation, abjuration, alteration, conjuration, illusion, restoration, divination, spellcraft |
| `spell.` (28 authored) | `spell.searing_bolt` = W1, `spell.frost_orb` = W2, `spell.storm_ray` = W3, `spell.firestorm_bolt` = W4, `spell.cinder_bolt` = W5, `spell.flame_conjuration` = W6, `spell.mending_aura` = W7 (origin `genre.mythic_fantasy`); `spell.packet_surge` = W8 (`genre.cyberpunk`); `spell.mountain_palm` = W9 (`genre.wuxia`); `spell.gaze_of_madness` = W10 (`genre.cosmic_horror`); `spell.thorn_sleep` = W11 (`genre.fairy_tale`); `spell.boiler_burst` = W12 (`genre.steampunk`); wand pool `spell.wand_striking`, `.wand_digging`, `.wand_light`, `.wand_haste`, `.wand_polymorph`, `.wand_dispel`, `.wand_fear`, `.wand_lightning`, `.wand_venom`, `.wand_conjure`, `.wand_invisibility` (§8.4); text spells `spell.text_mending`, `.text_demolition`, `.text_warding`, `.text_silence`, `.text_summoning` (§8.3) |
| `summon.` (18) | familiar, elemental, servitor, swarm, wisp, golem, hound, phantasm, sentinel, herald, misprint; `summon.poly.` toad, sheep, rat, crow, ooze, paper, beast |
| `patron.` (16) | one per genre suffix of Canon §5.2 |
| `prayer.` (14) | supplication, offering, benediction, smite, sanctuary, mending, reattachment, absolution, turning, provision, vigil, intercession, lantern, oath |
| `ench.` (27) | focus, attunement, logic, memory, toughness, agility, resistance, potency, valor, precision, ruin, thrift, celerity, lightbearer, breath, surefoot, searing, frostbite, galvanic, corrosive, sanctified, vampiric, siphoning, disruption, spellstore, blink, restoring |

Constants: `CAST_RECOVERY_TICKS = 20`, `INSTANT_RECOVERY_TICKS = 45`, interrupt lockout 30 ticks, charge `charge_bp(c) = 10000 + floor_div(5000 × c, 60)` with `c` in `0..=60` past `ct` and auto-release at `ct + 120` (Evocation 50: `10000 + floor_div(7500 × c, 90)`, `c` in `0..=90`, auto-release at `ct + 150`), `DEPTH_BP = [10000, 7500, 6000]`, `NPC_SPELL_SCALE_BP = 4500`, max trigger depth 2, casting base speeds 2150 and 1720 mm/s, siphon cap 5000 Bp of `cast_cost` and leech cap 2500 Bp of Max HP per root resolution, enchanting ARCANA gates 1, 4 and 7, supplication `r1 = range(180_000)`, petition gate `prayer_timer <= 7200`. Enumerations (not content IDs): glyph pools `base`, `glyphs_2`, `family.<name>`; essence kinds `damage`, `heal`, `buff`, `control`, `poly`, `dispel`, `summon`, `world`, `lore`; implement profiles `focus`, `tome`, `staff`. Behavior of the Canon §11.1 statuses owned by 06 (§15) and of `res.aether`, `res.ward` (spell side) and `res.favor`.

## Interface assumptions

| Doc | This chapter assumes |
|---|---|
| 03 | `aether_tick` runs first in `sys.actions` for each entity, then its cast machine; spell projectiles join `sys.projectiles` and spell hits the `sys.damage` queue; `ev.spell_cast` gains `spell_hash: u64`; the seven §19.1 events with their payloads; an `ev.damage_dealt` flag `self_cost`; `raycast_blocks` for line of effect; fire-grid `ignite(cell)`; temporary block-light sources `add_light(pos, level, expiry)` and darkness volumes (−8 effective light) for shadow; block place and remove with a `conjured` flag |
| 04 | keeps its §5.3 magic, faith and Inscription perk texts identical to §7.3 and its §5.2 magic rows identical to §7.4; cites the §6.2 cost factor; grants `RowFlat(26, ·)` and `RowFlat(27, ·)` talents that §4.3 check 13 and §8.2 read; applies composure loss with the MYTHOS term (§10.3); lists starting glyphs in Calling and Echo kits; gives an Unwritten Origin glyph origin `genre.ur_text` |
| 05 | exports `arch.ranged.staff`, `arch.ranged.wand` and `arch.offhand.tome` with the §8.5 `casting` block (doc 11 already uses the first two); `HitRecord` gains a spell extension `{ spell_pot, c_pot, edge_bp, school }` and carries `move_id` = the stage's form glyph `id32`; spell crit keys as §19.2; casting stances join §1.6; knockback impulse; poise damage; the friendly-fire classes of gap G11 |
| 07 | (present) material `arcane_affinity` and `dmg_bp` (§2), `hardness` 0..=10 with `hardness_milli = -1` for non-destructible blocks, tags `flammable`, `metal`, `stone`, `arcane_reagent`; `station.enchanting`; `placed_meta` with patron and `conjured` flag; the casting-material tendency `C = −150 × (arcane_affinity − 4)` folded into implement `cost_bp`; staff, wand and tome recipes; each genre's stone block |
| 08 | `add_liquid(cell, liq, ml)`, `place_gas(cell, gas, density)`, `move_gas(cell, dir, n)`; aether potions and `liq.mana` |
| 09 | (present) `noise.spell.<school>`, fizzle 70 dB and surge 90 dB noise IDs; shadow-area darkness (−8); `st.invisible` detection; the awareness-state icons of Divination 50; hidden-object reveal for sight; runes as trap-detection objects |
| 10 | the effects of `appraise`, `omen`, `psychometry` and sight's aura; `observable` per the §8.3 `use_id`s; the 15 `wand_pool` spells; zap curses; type identification on zap and read; an API that makes one unknown carried type known (Cosmic Horror boon) |
| 11 | templates `item.text.inscribed` (an inscribed text holding glyph references) and `item.book.primer_<school>` for alteration, conjuration, illusion, restoration and spellcraft; wand spells drawn by the §8.4 weights; glyph tomes filled by `glyph_tome` (§8.1); trainer service from the loot pool; bound-glyph roll on Legendary and Exotic implements from the §8.5 list; recharge service |
| 12 | Scriptorium glyph Rewrite; `room.shrine` and `secret.room.shrine` altars with their patron; Colophon tome stock; campfire aether refill; bleed and seam zones for local axioms |
| 13 | utility AI consuming the §18 queries; NPC spell lists, aether pools and the fizzle-only rule (§6.5); NPC spell `template_dmg_mhp` = `mul_bp(dmg_base, 4500)`; flee behavior for `st.feared` and `st.dread`; hostile herald and misprint behavior; the §12.1 minion profiles; `enemy.editors.invader` casts no spells and is a `rank.hunter` target, immune to charm, sleep, confusion, hacking and polymorph (Canon §11.4) |
| 14 | kit primers as §8.1 states; `unlock.loot.glyphs_2` adds the `glyphs_2` pool; Echo traits enter the `[T]` slots of §3.4, §6.2, §6.3 and §12.1 |
| 15 | each `unlock.glyph.<family>` adds that §8.1 family pool; each Folio Expansion supplies 4 glyphs for its family |
| 16 | prediction of cast start, Winding and spell projectile stand-ins with the §19.2 keys; downed rules; the Restoration 100 revival hook |
| 17 | (present) `input.cast`, `input.cast_wheel`, `input.pray`, `input.cast_cancel` and the bit layout of §3.2; the lore-essence target list; the spell composer over `compile`; the party map reveal of `item.text.mapping`. Doc 17's composer example lists W1 burning power 54; with the Evocation 25 hook it is 64 (§6.7) |
| 18 | (present) §7.5 Form skins, adopted unchanged in §5.7; localization of every §5.7 skin and of patron, prayer and enchantment names; register VFX and audio (`mag` cast, release and fizzle per register) |

## Canon gaps

| # | Gap or conflict | What this chapter assumed |
|---|---|---|
| G1 | Resolved in this revision. Canon §10.4 already gives `material_bp` for Errata item sources | spells take their source item's material `dmg_bp` (§3.1, §6.4), 10000 with no implement and for texts |
| G2 | Canon §6.4 row 29 multiplies "the GLYPH term for each glyph" without saying which glyphs when a spell has payloads | each stage multiplies the terms of its own glyphs only; the literal product per glyph spans ×0.43 to ×2.14 for an 8-glyph stage (§6.3) |
| G3 | Canon §18 lacks casting, prayer and learning events | requests `ev.cast_started`, `ev.cast_interrupted`, `ev.wild_surge`, `ev.glyph_learned`, `ev.enchant_applied`, `ev.prayer_resolved`, `ev.favor_changed` with the §19.1 payloads; a fizzle uses doc 03's `ev.spell_cast { fizzled: true }` |
| G4 | Canon §19 gives no namespace for polymorph templates | uses `summon.poly.` inside the owned `summon.` namespace |
| G5 | Canon §13.1 and §13.6 name staves, wands and tomes, but doc 05 exports only `arch.offhand.focus` | uses `arch.ranged.staff`, `arch.ranged.wand` (as doc 11 does) and `arch.offhand.tome`, and asks doc 05 to export all three |
| G6 | Canon §4.2 has no casting movement modes | adds base speeds 2150 mm/s (Winding, Charging) and 1720 mm/s (Channeling) as doc 05 §1.6 stances |
| G7 | Canon §16.5 scales IVAN's prayer timers by 3.6 but says nothing of IVAN's `Timer < RAND_N(500000)` | uses `range(180_000)` (§16.3 balance note); favor deltas +50, −50, −75, −100 come from IVAN |
| G8 | Canon §3.3 defines `seq` as a per-actor counter that resets each tick; doc 05 §1.4 allocates fixed slots instead | a cast, zap, read or prayer draws at seq 0, doc 05's primary-action slot, because it never starts on the same tick as a weapon action (§3.5); seq 0x2000 to 0x2FFF (interruption), 0x3000 (`st.confused`) and 0x4000 to 0x4FFF (payloads) are reserved for doc 06, and other docs' per-tick counters stay below 0x2000 |
| G9 | status instances have no source tag | adds a `dispellable: bool` field set at application (§14.1) |
| G10 | `st.dread` acts on composure, which NPCs lack | NPC dread has no numeric effect; doc 13 reads it as flee pressure |
| G11 | Canon §17.5 does not classify spell bursts | orb, nova, rune, cone and chain hits are spell hits (`FF_DIRECT_BP`); spell-made fire, surfaces and liquids are area (`FF_AREA_BP`) |
| G12 | Canon §12 does not say whether `res.favor` is per Erratum or per party | per Erratum |
| G13 | Canon §10.4 has no scale for NPC use of player-scaled spell bases | `NPC_SPELL_SCALE_BP = 4500` at step 0 |
| G14 | Canon §11.2 starts from a hit's status power `p` | for spells, `p` already includes `spell_pot` (§6.4); the canon chain then runs unchanged |
| G15 | Resolved by canon-1.4: limited building (§4.3) and invasions (§17.9) are canon | spells break only `destructible` blocks; conjured walls, webs and ice walls are temporary barricades; the invader casts no spells (doc 13) |
| G16 | Canon §13.8.1 lists the casting implement as "focus, wand, staff or tome" and omits texts, on-use enchantment items and holy symbols | a text, the enchanted item and an equipped `item.trinket.holy_symbol` (for prayers) are the casting implement of their own activations (§3.1) |

## Acceptance tests

1. **Aether regeneration.** ATN 40 in `genre.cyberpunk` gains exactly 1,820 m-aether over 60 ticks; ATN 50 in `genre.mythic_fantasy` gains 1,950. With `st.unraveled` it gains 0 and the accumulator is 0.
2. **Glyph dependency.** A glyph learned in Cyberpunk: `dep = techne`, `Req = 7`; Mythic Fantasy: `arcana`, 6; Hardboiled Noir: `verve`, 1; Ur-Text: `arcana`, 3. Worldgen never produces a glyph with origin `genre.unwritten`.
3. **Complexity.** `bolt flame` = 2. `bolt flame on_impact ( nova frost )` = 9 and compiles at LOG 50. `orb flame on_impact foe ( nova frost )` = 11: `E_COMPLEXITY` at LOG 50, success at LOG 70.
4. **Validation order.** `bolt flame frost` → `E_OPPOSED`; `bolt mend flame` → `E_POLARITY`; `self flame` → `E_FORM_ESSENCE`; `bolt flame quicken charged` → `E_MODIFIER`; `manifest summon bind` → `E_SUMMON_PAIR`; `manifest summon flame` compiles; `bolt flame on_expire ( nova frost )` → `E_TRIGGER_HOST`; a trigger inside a depth-2 stage → `E_DEPTH`.
5. **Costs (Ref-A).** W1 base 5000, cast 5008; W2 8400, 8414; W3 6500 and 6511 per second; W4 15937, 15964; W5 11250, 11268; W6 16000, 16317; W7 11200, 11876.
6. **Potency.** W1 `spell_pot` 12809; W4 14391; W5 root 13577, payload 12809; W8 8603; W10 16627.
7. **Damage.** W1 `B = 28514`, after step 2.1 36523 mHP. W4 after step 2.1 90020, split fire 53337, shock 36683. W5 payload 21305 per target.
8. **Channel drain.** W3 drains 108 m-aether on the first Channeling tick and exactly 6511 per 60 ticks; a full channel fires 20 pulses of 10043 mHP.
9. **Charge.** `charge_bp(0) = 10000`, `(30) = 12500`, `(60) = 15000`. A charged bolt (`ct = 24`) released at `held = 10` cancels with no debit and a 30-tick lockout; released at `held = 54` it resolves at 12500 and debits `mul_bp(cast_cost, 12500)`; held on, it auto-releases at `held = 144` at 15000.
10. **Timing.** Orb with quicken, `st.hasted`, focus cast speed 10000: cast 16 ticks, recovery 18.
11. **Interruption.** Keep chance FOC 40: 2500 Bp; FOC 60: 4000; FOC 150: 9500. A hit fully absorbed by ward rolls nothing. A Device rolls nothing.
12. **Contradiction.** W8: C 4, potency 6000, failure 1000, E 6; native-target damage after step 2.6 is 33012 mHP.
13. **Keyed draws.** Run seed `0x0123456789ABCDEF`, Folio 8, actor `(8 << 40) | 7`, tick 30016, seq 0: `rng.combat` first `range(10000)` = 482; `rng.magic` four draws = 3992, 2458, 59936, 5793; at C = 4 the surge is entry 2, Backfire, dealing 13311 mHP before Ref-B's mitigation.
14. **Draw count.** Every root resolution makes exactly 4 `rng.magic` calls whatever the outcome; the `rng.combat` roll happens only when the failure chance is above 0 (C ≥ 3 after the Unmoored rule, or a cursed wand's misfire).
15. **Surge table.** Both weight columns sum to 10000.
16. **Summon.** W6 elemental: HP 95508 mHP, attack 12733 mHP every 45 ticks, status power 42 after `S(3)`, duration 6075 ticks (4050 before the Conjuration 25 factor), cap 2; a third summon releases the lower-EntityId one with `ev.minion_released` reason 0.
17. **Heal aura.** W7: 155 Bp per pulse, 1627 mHP on a 105 HP ally, 40 pulses, 6200 Bp total.
18. **Siphon and leech.** A 36523 mHP hit at `d_current = 3` siphons 4610 m-aether; against an 80,000 mHP target it leeches 1141 Bp, healing a 105 HP caster by 11980 mHP.
19. **Bargain.** W11: cast 6526, shortfall 4526, Debt 5657, repaid in 175 ticks at 1950 m-aether per second; a Folio exit before then costs 6 favor at Devotion 0.
20. **Composure toll.** W10 costs 3318 m-composure before the MYTHOS term, 4977 after × 15000.
21. **Supplication.** The §16.3 worked draws are `299, 125411, 2047, 7972` at tick 18000 and `491, 36979, 9278, 14839` at tick 18002, and give PLEASED, DISPLEASED, HELPED and ANGRY exactly as stated; PLEASED adds 18000 to the timer and 50 favor at Devotion 0.
22. **Devotion scaling.** Devotion 20: gain 50 → 60, loss 100 → 83. Devotion 50: `prayer.benediction` costs 40.
23. **Sacrifice.** Value 2500, matching genre, Devotion 20: +120 favor. Value 99, other genre, Devotion 0: +9.
24. **Enchanting.** Capacity: `arc_aff` 2 longsword 6 (Rare 7); `arc_aff` 8 ring 9. `ench.searing` II on a pure-slash hit gives slash 7500, fire 2500. The station refuses where local ARC is 0. A tier III enchantment is refused in Space Opera (ARC 6) and allowed in Mythic Fantasy (ARC 8); tier II is allowed in Weird West (ARC 4); tier I is the only tier in Cyberpunk (ARC 1).
25. **Inscription.** Inscribing W5 at Inscription 10 costs 21375 m-aether.
26. **Hashes and names.** `spell_hash` of W1's canonical text = `0x9d32104fc5e81a6e`. Names: W1 "Flame Bolt"; W4 "Bolt of Empowered Flame and Lightning"; W8 in Cyberpunk "Packet://Surge/Optimized"; W9 in Wuxia "Palm of Mountain-Crushing Mountain Force".
27. **Silence.** With `st.silenced`, a Mythic Fantasy Grimoire spell fails with `Silenced`; a Program casts; a wand zaps. A Program under `st.shocked` fails with `Jammed`.
28. **Counterspell and null field.** An unmaking bolt hitting an NPC in Winding ends its cast and applies `st.silenced` with base 60 ticks through Canon §11.4. Inside a null field no cast starts and an entering bolt ends with no effect.
29. **Polymorph.** A standard NPC polymorphed by a Fairy Tale register becomes `summon.poly.toad`; the status ends once it has taken 2000 Bp of its Max HP; a `rank.boss` target is immune.
30. **Determinism.** Compiling the same glyph references with the same caster twice yields byte-identical `CompiledSpell` values and draws nothing; a save at any tick during Channeling reloads to the same drain accumulator and pulse schedule.
31. **Limited building.** A force bolt (block power 6) on a stone block not tagged `destructible` deals 0 block damage. A shape bolt at Alteration 25 (the tier-2 gate; `shape_power = 3 + 1 = 4`) removes a `destructible` block of hardness 4 and fails on hardness 5. A shape wall's 15 conjured blocks are gone on the tick after its 600-tick duration ends.
32. **Instant recovery.** `touch flame` in Mythic Fantasy at Folio 1 (skill 20, ATN 40, no implement) resolves for 37,159 mHP before mitigation; holding cast input, consecutive resolutions are 46 ticks apart (recovery 45 at `speed_bp` 10000). `bolt flame` under the same conditions resolves 28,584 mHP every 44 ticks.
33. **Cooldown keys.** After `aura mend` resolves on tick `t`, `aura mend extend` (a different `spell_hash`) fails with `Cooldown` until `t + 900`. After `self veil`, `bolt veil` fails with `Cooldown` until 600 ticks later. `bolt flame` never has a key.
34. **Siphon and leech caps.** Ref-A's `bolt flame siphon` costs 7011 m-aether; its 38,713 mHP hit computes 4886 m-aether and grants 3505. A wand zap with siphon grants 0. A five-target `nova leech` heals at most 2500 Bp of the caster's Max HP in total.
35. **Unmoored.** An Erratum whose rolls fail at C ≥ 3 three times within 600 ticks gains `st.unmoored` for 300 ticks; a Strained (C = 1) spell cast during it rolls `rng.combat` at 500 Bp; a fizzle caused only by `curse.misfire` at C = 0 does not count toward the three.
36. **Disconnected mid-cast.** A cast that starts at C = 5 and resolves after the caster enters a zone where C = 6 fizzles: 4 `rng.magic` draws, 0 `rng.combat` draws, full cost spent, no Unmoored count.
37. **NPC failures.** An NPC caster at C = 4 whose roll fails fizzles; no surge entry is chosen and no `summon.misprint` spawns.
38. **Skins.** Each of the 84 glyphs has a non-empty skin in all 15 genres except `genre.unwritten` (1260 strings), no two glyphs share a skin within one genre, and the Form rows equal doc 18 §7.5. In `genre.ur_text` every bolt displays "First Letter"; in `genre.unwritten` a Wuxia-origin bolt displays "Flying Needle" and an Ur-Text-origin bolt "First Letter".
39. **Wand pool.** Exactly 15 authored spells carry `wand_pool`; their `wand_weight` values sum to 10000; each has complexity 8 or less and no beam form.
40. **Glyph pools.** `base` holds 52 glyphs: 11 forms, 23 essences, 11 modifiers, 3 triggers, 4 conditions. With `unlock.daily_baseline` a Folio-1 evocation tome holds 2 tier-1 `base` evocation glyphs and makes exactly 2 draws; `glyph.form.rune` never appears without `unlock.loot.glyphs_2`.
41. **Petitions.** With `prayer_timer` set to 0, 3600, 7200 and 7201 at petition start, the first three petitions start and the fourth fails; each resolved petition adds 3600 (2700 with the Oracle talent `divine_favor`). With Devotion 25 `prayer.benediction` costs `mul_bp(60, 8000) = 48` before Devotion scaling. `prayer.mending` for a petitioner with ATN 40 and Devotion 50 at the local patron heals 4312 Bp of Max HP.
42. **Blank Page.** A PLEASED supplication to `patron.unwritten` at full health leaves `prayer_timer` at 18000 and grants `st.blessed` for 900 ticks.
43. **Downed.** A supplication input while `st.downed` fails with `Disabled` and draws nothing.
44. **Sacrifice.** A standard-rank NPC corpse at Folio 10 has value 217 and gives 14 favor at Devotion 0 from another genre; a stack of 20 items of value 1 gives 4; a `cat.currency` pile is refused.
45. **Perk hooks.** W1's `st.burning` power is 64 per hit; with the `skill.evocation.m25` flag cleared and every other input unchanged it is 54. W8's cast cost is 7029 with `skill.spellcraft.m25` and 7369 without it. W6's summon lasts 6075 ticks with `skill.conjuration.m25` and 4050 without it.
46. **Recharge text.** On an uncursed wand at 3 of 6 charges: uncursed text gives 5 of 5, blessed gives 6 of 6, cursed gives 4 of 5.
47. **Evocation 50 charge.** With `skill.evocation.m50`, `charge_bp(45) = 13750` and `charge_bp(90) = 17500`, and a charged bolt (`ct = 24`) auto-releases at `held = 174`; without the perk it auto-releases at `held = 144` at 15000.
48. **Spellcraft hooks.** `bolt flame empower widen quicken` fails `E_MODIFIER` (widen does not apply to bolt); `orb flame empower widen quicken reach` fails `E_GRAMMAR` at Spellcraft 99 and compiles past check 1 with `skill.spellcraft.m100`. With `skill.spellcraft.m50`, `orb flame on_impact foe ( nova frost )` has complexity 10 instead of 11.

## Invariants annex

```
Σ doc06: aether ⊗ glyph grammar ⊗ cost ⊗ potency ⊗ contradiction ⊗ summons ⊗ enchanting ⊗ prayer
W! glyph.dep := {origin.aether_axiom} | {arcana} if none        N! glyph origin = genre.unwritten
W! Req(glyph) = max(1, origin.axiom[dep] − 2)                     W! register := origin(root form)
W! stage = 1 form ∧ 1..=2 essences ∧ 0..=3 modifiers (0..=4 with Spellcraft 100) ∧ 0..=1 trigger ∧ (condition ⇒ trigger)
W! trigger depth ≤ 2                                              W! complexity ≤ row 26 budget (4 + floor_div(LOG, 10) + RowFlat grants)
W! validation order = §4.3 order; first failure reported          N! opposed pair ∈ one stage
N! helpful ∧ harmful essence ∈ one stage                          W! compile pure ∧ draws = 0
W! cost: base → skill → implement → rider → thrift → [T], one mul_bp each    W! cast_cost ≥ 1000 if base > 0 ∧ not free
W! spell_pot = row29 × Π glyph_term(stage glyphs)                 N! NPC uses row 29
W! spell dmg enters Canon §10.5 at step 0 with the source item's material dmg_bp    W! heal|ward in Bp of recipient Max HP
W! root resolution: rng.magic draws = 4 exactly                   W! failure roll ⇔ fail_bp > 0 (C ≥ 3 or wand misfire), stream rng.combat
W! C = 3 ∧ fail ⇒ fizzle, full cost spent                         W! C ∈ {4,5} ∧ fail ∧ Erratum ⇒ wild surge
N! cast while any core glyph Disconnected                         N! payload cost|roll|cooldown at fire time
W! summon ∈ fac.errata ∧ owner_slot ∧ cap per Canon §15.1         W! spell summon despawns on ev.folio_exited
W! summon stats ∝ P(src_lvl) × pot snapshot; AR ∝ T(src_lvl)      N! summon hit uses step 2.1
N! st.polymorphed on boss|editor|hunter                           W! polymorph keeps HP ratio
W! enchant dep = {arcana}; capacity by arc_aff                    N! enchant where local ARC = 0
W! favor ∈ −1000..=1000 per (Erratum, patron)                     W! gains ×(100+dev)/100, losses ×100/(100+dev)
W! sacrifice = isqrt(value × stack) × (2 if origin = patron genre)  W! local patron × 15000; non-local × 12500 with Devotion 75
W! supplication draws = 4 exactly                                 N! names ∈ sim state
N! HashMap|HashSet ∈ magic state                                  N! float ∈ magic state | content
N! spell removes block ∉ destructible                             W! conjured blocks vanish at expiry ∧ drop nothing
W! cooldown keys := root form(cd>0) ∪ root essences|modifiers(cd_add>0)    N! cooldown keyed by spell_hash
W! ct = 0 ⇒ recovery base 45; ct ≥ 1 ⇒ 20                         W! charge resolves only if held ≥ ct; charge_bp ∈ 10000..=15000 (..=17500 with Evocation 50)
W! siphon per root resolution ≤ cast_cost/2                       W! leech per root resolution ≤ 2500 Bp caster Max HP
W! failed roll ∧ C ≥ 3 ⇒ Unmoored count                           W! C becomes 6 mid-cast ⇒ fizzle, rng.combat draws = 0
W! NPC failed roll ⇒ fizzle                                        N! NPC wild surge
W! cast|zap|read|prayer draws at seq 0                             N! cast ∧ weapon action start on one tick
N! cast ∨ prayer while st.downed                                   W! perk effects := doc 04; perk hooks := §7.3
W! enchant tier I|II|III ⇒ local ARC ≥ 1|4|7                       W! |wand_pool| = 15 ≤ 16 ∧ Σ wand_weight = 10000
W! prayer r1 = range(180000)                                       W! petition ⇒ prayer_timer ≤ 7200
W! ∀ glyph ∀ genre ≠ unwritten: skin ≠ ∅                          W! unwritten skin := glyph-origin skin
W! loot-pool gating ⊆ {tomes, trainers, text study}                N! loot pool gates kits | authored spells
```
