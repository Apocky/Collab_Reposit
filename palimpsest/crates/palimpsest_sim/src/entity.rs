//! SPEC: canon §1.11 EntityId and uid allocation
//!
//! `EntityId` and item `uid` share one layout,
//! `((folio_index as u64) << 40) | counter` (canon §1.11, doc 03 §3.1). Each
//! has its own per-Folio counter that starts at 1 at Folio entry, only grows,
//! and is never reused. Folio indices only grow within a run, so every value
//! is unique for the whole run, and an ID allocated later is always larger.
//! Ordering is plain `u64` order, which is `(folio_index, counter)`
//! lexicographic order: the ascending iteration order of canon §1.11.
//!
//! The shift in the layout is a checked multiplication by `2^40` (canon §1.3
//! "Shifts"): a `folio_index` above 24 bits panics instead of losing bits.

use core::fmt;

/// SPEC: canon §1.11. Number of low bits that hold the per-Folio counter.
pub const COUNTER_BITS: u32 = 40;

/// SPEC: canon §1.11. Mask of the counter bits, `0xFF_FFFF_FFFF`; also the
/// tick-divider phase-key mask.
pub const COUNTER_MASK: u64 = 0xFF_FFFF_FFFF;

/// SPEC: canon §1.11, doc 03 §3.1 "Counter range". The largest counter a
/// Folio can hand out, `2^40 - 1`.
pub const MAX_COUNTER: u64 = COUNTER_MASK;

/// SPEC: canon §1.11, doc 03 §3.1. The largest `folio_index` the layout can
/// hold: bits 40 to 63 give 24 bits.
pub const MAX_FOLIO_INDEX: u32 = (1 << (64 - COUNTER_BITS)) - 1;

/// SPEC: canon §1.11. The first counter value of every Folio.
pub const FIRST_COUNTER: u64 = 1;

/// SPEC: canon §1.11 `((folio_index as u64) << 40) | counter`, with the shift
/// done as a checked multiply (canon §1.3). Panics if `folio_index` exceeds
/// [`MAX_FOLIO_INDEX`] or `counter` is outside `1..=MAX_COUNTER`.
#[must_use]
#[track_caller]
pub const fn pack_id(folio_index: u32, counter: u64) -> u64 {
    assert!(
        folio_index <= MAX_FOLIO_INDEX,
        "pack_id: folio_index exceeds 24 bits"
    );
    assert!(
        counter >= FIRST_COUNTER && counter <= MAX_COUNTER,
        "pack_id: counter outside 1..=2^40-1"
    );
    let hi = match (folio_index as u64).checked_mul(1 << COUNTER_BITS) {
        Some(v) => v,
        None => panic!("pack_id: folio_index << 40 overflows"),
    };
    hi | counter
}

/// SPEC: canon §1.11 and §1.2. A simulation entity's identity, a `u64` in the
/// canon layout. Ordered by raw value, which is ascending
/// `(folio_index, counter)`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct EntityId(u64);

impl EntityId {
    /// SPEC: canon §1.11. Build from a Folio index and a counter in
    /// `1..=2^40-1`; panics outside those ranges (see [`pack_id`]).
    #[must_use]
    #[track_caller]
    pub const fn new(folio_index: u32, counter: u64) -> EntityId {
        EntityId(pack_id(folio_index, counter))
    }

    /// SPEC: canon §1.11. Wrap a raw `u64` as stored in saves, events and
    /// packets. No validation: a raw value with counter 0 never comes from an
    /// allocator but is representable.
    #[inline]
    #[must_use]
    pub const fn from_raw(raw: u64) -> EntityId {
        EntityId(raw)
    }

    /// SPEC: canon §1.11. The raw `u64`.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }

    /// SPEC: canon §1.11. The Folio the entity was created in (bits 40 to 63).
    #[inline]
    #[must_use]
    pub const fn folio_index(self) -> u32 {
        // Bits 40..64 are 24 bits, so the narrowing is lossless.
        (self.0 >> COUNTER_BITS) as u32
    }

    /// SPEC: canon §1.11. The per-Folio counter (bits 0 to 39).
    #[inline]
    #[must_use]
    pub const fn counter(self) -> u64 {
        self.0 & COUNTER_MASK
    }

    /// SPEC: canon §1.11 "Tick-divider phases use `(EntityId & 0xFF_FFFF_FFFF)
    /// % period`". This is the key before the `% period`.
    #[inline]
    #[must_use]
    pub const fn phase_key(self) -> u64 {
        self.0 & COUNTER_MASK
    }

    /// SPEC: canon §1.11, §2.2. The entity's phase for a subsystem of period
    /// `period` ticks. Panics if `period == 0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn phase(self, period: u32) -> u64 {
        assert!(period > 0, "EntityId::phase: period must be > 0");
        self.phase_key() % period as u64
    }

    /// SPEC: canon §2.5 canonical serialization: little-endian bytes.
    #[inline]
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }
}

impl fmt::Debug for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EntityId(f{}#{} = {:#018x})",
            self.folio_index(),
            self.counter(),
            self.0
        )
    }
}

/// SPEC: canon §1.11. An item instance's `uid`: the [`EntityId`] layout with
/// its own per-Folio counter. Ordered by raw value.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ItemUid(u64);

impl ItemUid {
    /// SPEC: canon §1.11. Build from a Folio index and a counter in
    /// `1..=2^40-1`; panics outside those ranges (see [`pack_id`]).
    #[must_use]
    #[track_caller]
    pub const fn new(folio_index: u32, counter: u64) -> ItemUid {
        ItemUid(pack_id(folio_index, counter))
    }

    /// SPEC: canon §1.11. Wrap a raw `u64` as stored in saves and packets.
    #[inline]
    #[must_use]
    pub const fn from_raw(raw: u64) -> ItemUid {
        ItemUid(raw)
    }

    /// SPEC: canon §1.11. The raw `u64`.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }

    /// SPEC: canon §1.11. The Folio the item was created in.
    #[inline]
    #[must_use]
    pub const fn folio_index(self) -> u32 {
        (self.0 >> COUNTER_BITS) as u32
    }

    /// SPEC: canon §1.11. The per-Folio uid counter.
    #[inline]
    #[must_use]
    pub const fn counter(self) -> u64 {
        self.0 & COUNTER_MASK
    }

    /// SPEC: canon §2.5 canonical serialization: little-endian bytes.
    #[inline]
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }
}

impl fmt::Debug for ItemUid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ItemUid(f{}#{} = {:#018x})",
            self.folio_index(),
            self.counter(),
            self.0
        )
    }
}

/// SPEC: canon §1.11. Why a saved counter state was rejected.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CounterError {
    /// `folio_index` is above [`MAX_FOLIO_INDEX`].
    FolioIndexTooLarge,
    /// The next counter is 0 or above `2^40`.
    NextOutOfRange,
}

/// SPEC: canon §1.11, doc 03 §3.1. One per-Folio counter: the current Folio
/// index and the next counter to hand out. Starts at 1 at each Folio entry
/// and only grows, so no value repeats within a run.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IdCounter {
    folio_index: u32,
    next: u64,
}

impl IdCounter {
    /// SPEC: canon §1.11. A counter for `folio_index` whose first value is 1.
    /// `folio_index = 0` covers run start, the hub and anything created before
    /// Folio 1. Panics if `folio_index` exceeds [`MAX_FOLIO_INDEX`].
    #[must_use]
    #[track_caller]
    pub const fn new(folio_index: u32) -> IdCounter {
        assert!(
            folio_index <= MAX_FOLIO_INDEX,
            "IdCounter::new: folio_index exceeds 24 bits"
        );
        IdCounter {
            folio_index,
            next: FIRST_COUNTER,
        }
    }

    /// SPEC: canon §2.5 restore of a saved counter. `next` is the next counter
    /// to hand out, in `1..=2^40` (`2^40` means the Folio is exhausted).
    pub const fn from_parts(folio_index: u32, next: u64) -> Result<IdCounter, CounterError> {
        if folio_index > MAX_FOLIO_INDEX {
            return Err(CounterError::FolioIndexTooLarge);
        }
        if next < FIRST_COUNTER || next > MAX_COUNTER + 1 {
            return Err(CounterError::NextOutOfRange);
        }
        Ok(IdCounter { folio_index, next })
    }

    /// SPEC: canon §1.11. The Folio whose IDs this counter hands out.
    #[inline]
    #[must_use]
    pub const fn folio_index(&self) -> u32 {
        self.folio_index
    }

    /// SPEC: canon §1.11, §2.5. The next counter value (saved in section 0).
    #[inline]
    #[must_use]
    pub const fn next_counter(&self) -> u64 {
        self.next
    }

    /// SPEC: canon §1.11, doc 03 §3.1 `spawn`. Hand out the next raw ID and
    /// advance. Panics with `entity_counter_overflow` past `2^40 - 1`.
    #[track_caller]
    pub fn alloc_raw(&mut self) -> u64 {
        assert!(self.next <= MAX_COUNTER, "entity_counter_overflow");
        let id = pack_id(self.folio_index, self.next);
        self.next += 1;
        id
    }

    /// SPEC: canon §1.11 "counter starts at 1 at each Folio entry". Switch to
    /// `folio_index` and restart the counter at 1. Folio indices only grow
    /// within a run, so a new index must exceed the current one; anything else
    /// panics, because restarting a counter for a Folio that already handed
    /// out IDs would reuse them.
    #[track_caller]
    pub fn enter_folio(&mut self, folio_index: u32) {
        assert!(
            folio_index > self.folio_index,
            "IdCounter::enter_folio: folio_index must increase"
        );
        *self = IdCounter::new(folio_index);
    }
}

/// SPEC: canon §1.11. The run's [`EntityId`] allocator.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityAllocator(IdCounter);

impl EntityAllocator {
    /// SPEC: canon §1.11. Allocator for `folio_index`, first ID counter 1.
    #[must_use]
    #[track_caller]
    pub const fn new(folio_index: u32) -> EntityAllocator {
        EntityAllocator(IdCounter::new(folio_index))
    }

    /// SPEC: canon §2.5. Rebuild from a saved counter.
    #[must_use]
    pub const fn from_counter(c: IdCounter) -> EntityAllocator {
        EntityAllocator(c)
    }

    /// SPEC: canon §1.11. The underlying counter (for saves).
    #[inline]
    #[must_use]
    pub const fn counter(&self) -> IdCounter {
        self.0
    }

    /// SPEC: canon §1.11, doc 03 §3.1. Allocate the next `EntityId`.
    #[track_caller]
    pub fn alloc(&mut self) -> EntityId {
        EntityId(self.0.alloc_raw())
    }

    /// SPEC: canon §1.11. Restart at counter 1 for a later Folio.
    #[track_caller]
    pub fn enter_folio(&mut self, folio_index: u32) {
        self.0.enter_folio(folio_index);
    }
}

/// SPEC: canon §1.11. The run's item [`ItemUid`] allocator, separate from the
/// entity counter, so items created in Folio 7 get the same uids whether or
/// not Folios 1 to 6 were played.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UidAllocator(IdCounter);

impl UidAllocator {
    /// SPEC: canon §1.11. Allocator for `folio_index`, first uid counter 1.
    #[must_use]
    #[track_caller]
    pub const fn new(folio_index: u32) -> UidAllocator {
        UidAllocator(IdCounter::new(folio_index))
    }

    /// SPEC: canon §2.5. Rebuild from a saved counter.
    #[must_use]
    pub const fn from_counter(c: IdCounter) -> UidAllocator {
        UidAllocator(c)
    }

    /// SPEC: canon §1.11. The underlying counter (for saves).
    #[inline]
    #[must_use]
    pub const fn counter(&self) -> IdCounter {
        self.0
    }

    /// SPEC: canon §1.11. Allocate the next item uid.
    #[track_caller]
    pub fn alloc(&mut self) -> ItemUid {
        ItemUid(self.0.alloc_raw())
    }

    /// SPEC: canon §1.11. Restart at counter 1 for a later Folio.
    #[track_caller]
    pub fn enter_folio(&mut self, folio_index: u32) {
        self.0.enter_folio(folio_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn layout_matches_doc03_example() {
        // Doc 03 §3.1: in Folio 3 the first spawn is (3 << 40) | 1.
        let mut a = EntityAllocator::new(0);
        a.enter_folio(3);
        let e = a.alloc();
        assert_eq!(e.raw(), 0x0000_0300_0000_0001);
        assert_eq!(e, EntityId::new(3, 1));
        assert_eq!(e.folio_index(), 3);
        assert_eq!(e.counter(), 1);
        assert_eq!(a.alloc().raw(), 0x0000_0300_0000_0002);
    }

    #[test]
    fn phase_key_matches_doc02_example() {
        // Doc 02 §5.3: entity (3 << 40) | 5 with period 6 has phase key 5.
        let e = EntityId::from_raw((3 << 40) | 5);
        assert_eq!(e.phase_key(), 5);
        assert_eq!(e.phase(6), 5);
        assert_eq!(e.phase(3), 2);
    }

    #[test]
    fn counters_start_at_one_and_are_independent() {
        let mut ents = EntityAllocator::new(0);
        let mut uids = UidAllocator::new(0);
        assert_eq!(ents.alloc(), EntityId::new(0, 1));
        assert_eq!(ents.alloc(), EntityId::new(0, 2));
        assert_eq!(uids.alloc(), ItemUid::new(0, 1));
        ents.enter_folio(7);
        uids.enter_folio(7);
        assert_eq!(ents.alloc(), EntityId::new(7, 1));
        assert_eq!(uids.alloc(), ItemUid::new(7, 1));
        assert_eq!(uids.alloc().raw(), (7 << 40) | 2);
    }

    #[test]
    fn folio7_uids_do_not_depend_on_earlier_folios() {
        // Canon §1.11: items created in Folio 7 get the same uids whether or
        // not Folios 1 to 6 were played.
        let mut played = UidAllocator::new(0);
        for f in 1..=6 {
            played.enter_folio(f);
            for _ in 0..(f * 13) {
                let _ = played.alloc();
            }
        }
        played.enter_folio(7);
        let mut skipped = UidAllocator::new(0);
        skipped.enter_folio(7);
        for _ in 0..50 {
            assert_eq!(played.alloc(), skipped.alloc());
        }
    }

    #[test]
    #[should_panic(expected = "folio_index must increase")]
    fn reentering_a_folio_panics() {
        let mut a = EntityAllocator::new(0);
        a.enter_folio(2);
        a.enter_folio(2);
    }

    #[test]
    #[should_panic(expected = "folio_index must increase")]
    fn returning_to_an_earlier_folio_panics() {
        let mut a = UidAllocator::new(0);
        a.enter_folio(5);
        a.enter_folio(4);
    }

    #[test]
    #[should_panic(expected = "entity_counter_overflow")]
    fn counter_overflow_panics() {
        let c = IdCounter::from_parts(1, MAX_COUNTER).unwrap();
        let mut a = EntityAllocator::from_counter(c);
        assert_eq!(a.alloc().counter(), MAX_COUNTER);
        let _ = a.alloc();
    }

    #[test]
    #[should_panic(expected = "exceeds 24 bits")]
    fn folio_index_above_24_bits_panics() {
        let _ = EntityId::new(MAX_FOLIO_INDEX + 1, 1);
    }

    #[test]
    #[should_panic(expected = "counter outside")]
    fn counter_zero_panics() {
        let _ = EntityId::new(1, 0);
    }

    #[test]
    fn extremes_pack_exactly() {
        let e = EntityId::new(MAX_FOLIO_INDEX, MAX_COUNTER);
        assert_eq!(e.raw(), u64::MAX);
        assert_eq!(e.folio_index(), MAX_FOLIO_INDEX);
        assert_eq!(e.counter(), MAX_COUNTER);
        assert_eq!(e.phase_key(), MAX_COUNTER);
    }

    #[test]
    fn from_parts_validates() {
        assert_eq!(
            IdCounter::from_parts(MAX_FOLIO_INDEX + 1, 1),
            Err(CounterError::FolioIndexTooLarge)
        );
        assert_eq!(
            IdCounter::from_parts(1, 0),
            Err(CounterError::NextOutOfRange)
        );
        assert_eq!(
            IdCounter::from_parts(1, MAX_COUNTER + 2),
            Err(CounterError::NextOutOfRange)
        );
        assert!(IdCounter::from_parts(1, MAX_COUNTER + 1).is_ok());
    }

    #[test]
    fn carried_entity_sorts_before_new_folio_entities() {
        // Doc 02 §5.1: a carried entity has a smaller folio_index, so a spawn
        // in the new Folio is always an append.
        let carried = EntityId::new(2, 900_000);
        let fresh = EntityId::new(3, 1);
        assert!(carried < fresh);
    }

    #[test]
    fn debug_format_shows_parts() {
        let s = format!("{:?}", EntityId::new(3, 5));
        assert_eq!(s, "EntityId(f3#5 = 0x0000030000000005)");
        let s = format!("{:?}", ItemUid::new(7, 2));
        assert_eq!(s, "ItemUid(f7#2 = 0x0000070000000002)");
    }

    proptest! {
        #[test]
        fn pack_unpack_round_trip(f in 0u32..=MAX_FOLIO_INDEX, c in 1u64..=MAX_COUNTER) {
            let e = EntityId::new(f, c);
            prop_assert_eq!(e.folio_index(), f);
            prop_assert_eq!(e.counter(), c);
            prop_assert_eq!(EntityId::from_raw(e.raw()), e);
            let u = ItemUid::new(f, c);
            prop_assert_eq!(u.raw(), e.raw());
            prop_assert_eq!(u.folio_index(), f);
            prop_assert_eq!(u.counter(), c);
        }

        #[test]
        fn order_is_folio_then_counter(
            f1 in 0u32..=MAX_FOLIO_INDEX, c1 in 1u64..=MAX_COUNTER,
            f2 in 0u32..=MAX_FOLIO_INDEX, c2 in 1u64..=MAX_COUNTER,
        ) {
            let a = EntityId::new(f1, c1);
            let b = EntityId::new(f2, c2);
            prop_assert_eq!(a.cmp(&b), (f1, c1).cmp(&(f2, c2)));
        }

        #[test]
        fn allocation_is_strictly_increasing_across_folios(
            steps in proptest::collection::vec((1u32..4, 0usize..20), 1..12)
        ) {
            let mut a = EntityAllocator::new(0);
            let mut folio = 0u32;
            let mut last: Option<EntityId> = None;
            for (gap, n) in steps {
                folio += gap;
                a.enter_folio(folio);
                for i in 0..n {
                    let id = a.alloc();
                    prop_assert_eq!(id.folio_index(), folio);
                    prop_assert_eq!(id.counter(), i as u64 + 1);
                    if let Some(prev) = last {
                        prop_assert!(id > prev);
                    }
                    last = Some(id);
                }
            }
        }

        #[test]
        fn phase_is_counter_mod_period(f in 0u32..=MAX_FOLIO_INDEX, c in 1u64..=MAX_COUNTER, p in 1u32..=60) {
            let e = EntityId::new(f, c);
            prop_assert_eq!(e.phase(p), c % u64::from(p));
        }
    }
}
