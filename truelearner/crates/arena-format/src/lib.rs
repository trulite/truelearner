#![forbid(unsafe_code)]
//! Canonical, machine-independent durable arena bytes.
//!
//! This crate owns stable identities and persistence representation. It does
//! not execute organism transitions.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

const MAGIC: &[u8; 8] = b"TLARNA01";
const FORMAT_VERSION: u16 = 1;
const HEADER_LEN: usize = 156;
const CELL_RECORD_LEN: usize = 35;
const ARROW_RECORD_LEN: usize = 74;
const LIVE_FLAG: u8 = 1;
const BODY_MAGIC: &[u8; 8] = b"TLBODY01";

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct ArenaId(pub u64);

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct CellId(pub u64);

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct ArrowId(pub u64);

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Generation(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CellRef {
    pub arena: ArenaId,
    pub id: CellId,
    pub generation: Generation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ArrowRef {
    pub arena: ArenaId,
    pub id: ArrowId,
    pub generation: Generation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableCell {
    pub id: CellId,
    pub generation: Generation,
    pub physical_id: u64,
    pub position: i32,
    pub region: i16,
    pub threshold: i32,
    pub resistance: u32,
    pub live: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableArrow {
    pub id: ArrowId,
    pub generation: Generation,
    pub from: CellRef,
    pub to: CellRef,
    pub delay: i64,
    pub phase: i32,
    pub coupling: i32,
    pub resistance: u32,
    pub transmission_mode: u8,
    pub live: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaBody {
    pub arena: ArenaId,
    pub version: u64,
    pub minimum_position: i32,
    pub maximum_position: i32,
    pub cell_capacity: u32,
    pub arrow_capacity: u32,
    pub cells: Vec<DurableCell>,
    pub arrows: Vec<DurableArrow>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ContentHash(pub [u8; 32]);

impl ContentHash {
    pub fn of(bytes: &[u8]) -> Self {
        Self(Sha256::digest(bytes).into())
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArenaVersion {
    pub arena: ArenaId,
    pub block: ContentHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyVersion {
    pub version: u64,
    pub parent: Option<ContentHash>,
    pub arenas: Vec<ArenaVersion>,
}

impl BodyVersion {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, FormatError> {
        let mut arenas = self.arenas.clone();
        arenas.sort_by_key(|entry| entry.arena);
        if let Some(pair) = arenas
            .windows(2)
            .find(|pair| pair[0].arena == pair[1].arena)
        {
            return Err(FormatError::DuplicateArenaId(pair[0].arena));
        }
        let mut bytes = Vec::with_capacity(63 + arenas.len() * 40);
        bytes.extend_from_slice(BODY_MAGIC);
        put_u16(&mut bytes, FORMAT_VERSION);
        put_u64(&mut bytes, self.version);
        bytes.push(u8::from(self.parent.is_some()));
        bytes.extend_from_slice(self.parent.unwrap_or(ContentHash([0; 32])).as_bytes());
        put_u32(
            &mut bytes,
            u32::try_from(arenas.len()).map_err(|_| FormatError::CapacityExceeded)?,
        );
        put_u64(
            &mut bytes,
            u64::try_from(63 + arenas.len() * 40).map_err(|_| FormatError::InvalidHeader)?,
        );
        for entry in arenas {
            put_u64(&mut bytes, entry.arena.0);
            bytes.extend_from_slice(entry.block.as_bytes());
        }
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, FormatError> {
        if bytes.len() < 63 {
            return Err(FormatError::Truncated);
        }
        if &bytes[..BODY_MAGIC.len()] != BODY_MAGIC {
            return Err(FormatError::WrongMagic);
        }
        let mut cursor = Cursor::new(bytes, BODY_MAGIC.len());
        let format = cursor.u16()?;
        if format != FORMAT_VERSION {
            return Err(FormatError::UnsupportedVersion(format));
        }
        let version = cursor.u64()?;
        let parent_present = cursor.u8()?;
        if parent_present > 1 {
            return Err(FormatError::InvalidFlags(parent_present));
        }
        let parent_hash = ContentHash(cursor.array_32()?);
        let count = usize::try_from(cursor.u32()?).map_err(|_| FormatError::InvalidHeader)?;
        let total = usize::try_from(cursor.u64()?).map_err(|_| FormatError::InvalidHeader)?;
        let expected = 63usize
            .checked_add(count.checked_mul(40).ok_or(FormatError::InvalidHeader)?)
            .ok_or(FormatError::InvalidHeader)?;
        if total != expected {
            return Err(FormatError::InvalidHeader);
        }
        if bytes.len() < expected {
            return Err(FormatError::Truncated);
        }
        if bytes.len() > expected {
            return Err(FormatError::TrailingBytes);
        }
        let mut arenas = Vec::with_capacity(count);
        let mut seen = BTreeSet::new();
        for _ in 0..count {
            let arena = ArenaId(cursor.u64()?);
            if !seen.insert(arena) {
                return Err(FormatError::DuplicateArenaId(arena));
            }
            arenas.push(ArenaVersion {
                arena,
                block: ContentHash(cursor.array_32()?),
            });
        }
        Ok(Self {
            version,
            parent: (parent_present == 1).then_some(parent_hash),
            arenas,
        })
    }

    pub fn content_hash(&self) -> Result<ContentHash, FormatError> {
        Ok(ContentHash::of(&self.canonical_bytes()?))
    }
}

impl fmt::Display for ContentHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormatError {
    Truncated,
    WrongMagic,
    UnsupportedVersion(u16),
    InvalidHeader,
    InvalidBounds,
    CapacityExceeded,
    DuplicateCellId(CellId),
    DuplicateArrowId(ArrowId),
    DuplicatePhysicalId(u64),
    DuplicateArenaId(ArenaId),
    InvalidFlags(u8),
    SectionChecksum,
    TrailingBytes,
}

impl fmt::Display for FormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for FormatError {}

impl ArenaBody {
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, FormatError> {
        self.validate()?;
        let mut cells = self.cells.clone();
        cells.sort_by_key(|cell| cell.id);
        let mut arrows = self.arrows.clone();
        arrows.sort_by_key(|arrow| arrow.id);

        let mut cell_bytes = Vec::with_capacity(cells.len() * CELL_RECORD_LEN);
        for cell in &cells {
            put_u64(&mut cell_bytes, cell.id.0);
        }
        for cell in &cells {
            put_u32(&mut cell_bytes, cell.generation.0);
        }
        for cell in &cells {
            put_u64(&mut cell_bytes, cell.physical_id);
        }
        for cell in &cells {
            put_i32(&mut cell_bytes, cell.position);
        }
        for cell in &cells {
            put_i16(&mut cell_bytes, cell.region);
        }
        for cell in &cells {
            put_i32(&mut cell_bytes, cell.threshold);
        }
        for cell in &cells {
            put_u32(&mut cell_bytes, cell.resistance);
        }
        for cell in &cells {
            cell_bytes.push(u8::from(cell.live));
        }

        let mut arrow_bytes = Vec::with_capacity(arrows.len() * ARROW_RECORD_LEN);
        for arrow in &arrows {
            put_u64(&mut arrow_bytes, arrow.id.0);
        }
        for arrow in &arrows {
            put_u32(&mut arrow_bytes, arrow.generation.0);
        }
        for arrow in &arrows {
            put_cell_ref(&mut arrow_bytes, arrow.from);
        }
        for arrow in &arrows {
            put_cell_ref(&mut arrow_bytes, arrow.to);
        }
        for arrow in &arrows {
            put_i64(&mut arrow_bytes, arrow.delay);
        }
        for arrow in &arrows {
            put_i32(&mut arrow_bytes, arrow.phase);
        }
        for arrow in &arrows {
            put_i32(&mut arrow_bytes, arrow.coupling);
        }
        for arrow in &arrows {
            put_u32(&mut arrow_bytes, arrow.resistance);
        }
        for arrow in &arrows {
            arrow_bytes.push(arrow.transmission_mode);
        }
        for arrow in &arrows {
            arrow_bytes.push(u8::from(arrow.live));
        }

        let cell_offset = HEADER_LEN;
        let arrow_offset = cell_offset + cell_bytes.len();
        let total_len = arrow_offset + arrow_bytes.len();
        let cell_hash = ContentHash::of(&cell_bytes);
        let arrow_hash = ContentHash::of(&arrow_bytes);

        let mut bytes = Vec::with_capacity(total_len);
        bytes.extend_from_slice(MAGIC);
        put_u16(&mut bytes, FORMAT_VERSION);
        put_u16(
            &mut bytes,
            u16::try_from(HEADER_LEN).expect("fixed header fits u16"),
        );
        put_u64(&mut bytes, self.arena.0);
        put_u64(&mut bytes, self.version);
        put_i32(&mut bytes, self.minimum_position);
        put_i32(&mut bytes, self.maximum_position);
        put_u32(&mut bytes, self.cell_capacity);
        put_u32(&mut bytes, self.arrow_capacity);
        put_u32(
            &mut bytes,
            u32::try_from(cells.len()).map_err(|_| FormatError::CapacityExceeded)?,
        );
        put_u32(
            &mut bytes,
            u32::try_from(arrows.len()).map_err(|_| FormatError::CapacityExceeded)?,
        );
        put_u64(
            &mut bytes,
            u64::try_from(cell_offset).map_err(|_| FormatError::InvalidHeader)?,
        );
        put_u64(
            &mut bytes,
            u64::try_from(cell_bytes.len()).map_err(|_| FormatError::InvalidHeader)?,
        );
        put_u64(
            &mut bytes,
            u64::try_from(arrow_offset).map_err(|_| FormatError::InvalidHeader)?,
        );
        put_u64(
            &mut bytes,
            u64::try_from(arrow_bytes.len()).map_err(|_| FormatError::InvalidHeader)?,
        );
        put_u64(
            &mut bytes,
            u64::try_from(total_len).map_err(|_| FormatError::InvalidHeader)?,
        );
        bytes.extend_from_slice(cell_hash.as_bytes());
        bytes.extend_from_slice(arrow_hash.as_bytes());
        debug_assert_eq!(bytes.len(), HEADER_LEN);
        bytes.extend_from_slice(&cell_bytes);
        bytes.extend_from_slice(&arrow_bytes);
        Ok(bytes)
    }

    pub fn content_hash(&self) -> Result<ContentHash, FormatError> {
        Ok(ContentHash::of(&self.canonical_bytes()?))
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, FormatError> {
        if bytes.len() < HEADER_LEN {
            return Err(FormatError::Truncated);
        }
        if &bytes[..MAGIC.len()] != MAGIC {
            return Err(FormatError::WrongMagic);
        }
        let mut cursor = Cursor::new(bytes, MAGIC.len());
        let version = cursor.u16()?;
        if version != FORMAT_VERSION {
            return Err(FormatError::UnsupportedVersion(version));
        }
        if usize::from(cursor.u16()?) != HEADER_LEN {
            return Err(FormatError::InvalidHeader);
        }
        let arena = ArenaId(cursor.u64()?);
        let body_version = cursor.u64()?;
        let minimum_position = cursor.i32()?;
        let maximum_position = cursor.i32()?;
        let cell_capacity = cursor.u32()?;
        let arrow_capacity = cursor.u32()?;
        let cell_count = usize::try_from(cursor.u32()?).map_err(|_| FormatError::InvalidHeader)?;
        let arrow_count = usize::try_from(cursor.u32()?).map_err(|_| FormatError::InvalidHeader)?;
        let cell_offset = usize::try_from(cursor.u64()?).map_err(|_| FormatError::InvalidHeader)?;
        let cell_len = usize::try_from(cursor.u64()?).map_err(|_| FormatError::InvalidHeader)?;
        let arrow_offset =
            usize::try_from(cursor.u64()?).map_err(|_| FormatError::InvalidHeader)?;
        let arrow_len = usize::try_from(cursor.u64()?).map_err(|_| FormatError::InvalidHeader)?;
        let total_len = usize::try_from(cursor.u64()?).map_err(|_| FormatError::InvalidHeader)?;
        let cell_hash = ContentHash(cursor.array_32()?);
        let arrow_hash = ContentHash(cursor.array_32()?);

        let expected_cell_len = cell_count
            .checked_mul(CELL_RECORD_LEN)
            .ok_or(FormatError::InvalidHeader)?;
        let expected_arrow_len = arrow_count
            .checked_mul(ARROW_RECORD_LEN)
            .ok_or(FormatError::InvalidHeader)?;
        if cell_offset != HEADER_LEN
            || cell_len != expected_cell_len
            || arrow_offset
                != cell_offset
                    .checked_add(cell_len)
                    .ok_or(FormatError::InvalidHeader)?
            || arrow_len != expected_arrow_len
            || total_len
                != arrow_offset
                    .checked_add(arrow_len)
                    .ok_or(FormatError::InvalidHeader)?
        {
            return Err(FormatError::InvalidHeader);
        }
        if bytes.len() < total_len {
            return Err(FormatError::Truncated);
        }
        if bytes.len() > total_len {
            return Err(FormatError::TrailingBytes);
        }
        let cell_section = &bytes[cell_offset..arrow_offset];
        let arrow_section = &bytes[arrow_offset..total_len];
        if ContentHash::of(cell_section) != cell_hash
            || ContentHash::of(arrow_section) != arrow_hash
        {
            return Err(FormatError::SectionChecksum);
        }

        let mut cell_cursor = Cursor::new(cell_section, 0);
        let cell_ids = collect_values(cell_count, || cell_cursor.u64().map(CellId))?;
        let cell_generations = collect_values(cell_count, || cell_cursor.u32().map(Generation))?;
        let physical_ids = collect_values(cell_count, || cell_cursor.u64())?;
        let positions = collect_values(cell_count, || cell_cursor.i32())?;
        let regions = collect_values(cell_count, || cell_cursor.i16())?;
        let thresholds = collect_values(cell_count, || cell_cursor.i32())?;
        let cell_resistances = collect_values(cell_count, || cell_cursor.u32())?;
        let cell_flags = collect_values(cell_count, || cell_cursor.u8())?;
        for flags in &cell_flags {
            validate_flags(*flags)?;
        }
        let mut cells = Vec::with_capacity(cell_count);
        for index in 0..cell_count {
            cells.push(DurableCell {
                id: cell_ids[index],
                generation: cell_generations[index],
                physical_id: physical_ids[index],
                position: positions[index],
                region: regions[index],
                threshold: thresholds[index],
                resistance: cell_resistances[index],
                live: cell_flags[index] & LIVE_FLAG != 0,
            });
        }

        let mut arrow_cursor = Cursor::new(arrow_section, 0);
        let arrow_ids = collect_values(arrow_count, || arrow_cursor.u64().map(ArrowId))?;
        let arrow_generations = collect_values(arrow_count, || arrow_cursor.u32().map(Generation))?;
        let from = collect_values(arrow_count, || arrow_cursor.cell_ref())?;
        let to = collect_values(arrow_count, || arrow_cursor.cell_ref())?;
        let delays = collect_values(arrow_count, || arrow_cursor.i64())?;
        let phases = collect_values(arrow_count, || arrow_cursor.i32())?;
        let couplings = collect_values(arrow_count, || arrow_cursor.i32())?;
        let arrow_resistances = collect_values(arrow_count, || arrow_cursor.u32())?;
        let transmission_modes = collect_values(arrow_count, || arrow_cursor.u8())?;
        let arrow_flags = collect_values(arrow_count, || arrow_cursor.u8())?;
        for flags in &arrow_flags {
            validate_flags(*flags)?;
        }
        let mut arrows = Vec::with_capacity(arrow_count);
        for index in 0..arrow_count {
            arrows.push(DurableArrow {
                id: arrow_ids[index],
                generation: arrow_generations[index],
                from: from[index],
                to: to[index],
                delay: delays[index],
                phase: phases[index],
                coupling: couplings[index],
                resistance: arrow_resistances[index],
                transmission_mode: transmission_modes[index],
                live: arrow_flags[index] & LIVE_FLAG != 0,
            });
        }

        let body = Self {
            arena,
            version: body_version,
            minimum_position,
            maximum_position,
            cell_capacity,
            arrow_capacity,
            cells,
            arrows,
        };
        body.validate()?;
        Ok(body)
    }

    pub fn validate(&self) -> Result<(), FormatError> {
        if self.minimum_position > self.maximum_position {
            return Err(FormatError::InvalidBounds);
        }
        if self.cells.len() > self.cell_capacity as usize
            || self.arrows.len() > self.arrow_capacity as usize
        {
            return Err(FormatError::CapacityExceeded);
        }
        let mut cell_ids = BTreeSet::new();
        let mut physical_ids = BTreeSet::new();
        for cell in &self.cells {
            if !cell_ids.insert(cell.id) {
                return Err(FormatError::DuplicateCellId(cell.id));
            }
            if !physical_ids.insert(cell.physical_id) {
                return Err(FormatError::DuplicatePhysicalId(cell.physical_id));
            }
        }
        let mut arrow_ids = BTreeSet::new();
        for arrow in &self.arrows {
            if !arrow_ids.insert(arrow.id) {
                return Err(FormatError::DuplicateArrowId(arrow.id));
            }
        }
        Ok(())
    }
}

fn validate_flags(flags: u8) -> Result<(), FormatError> {
    if flags & !LIVE_FLAG != 0 {
        return Err(FormatError::InvalidFlags(flags));
    }
    Ok(())
}

fn put_cell_ref(bytes: &mut Vec<u8>, reference: CellRef) {
    put_u64(bytes, reference.arena.0);
    put_u64(bytes, reference.id.0);
    put_u32(bytes, reference.generation.0);
}

fn put_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn put_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn put_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn put_i32(bytes: &mut Vec<u8>, value: i32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn put_u64(bytes: &mut Vec<u8>, value: u64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn put_i64(bytes: &mut Vec<u8>, value: i64) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn collect_values<T, F>(count: usize, mut read: F) -> Result<Vec<T>, FormatError>
where
    F: FnMut() -> Result<T, FormatError>,
{
    let mut values = Vec::with_capacity(count);
    for _ in 0..count {
        values.push(read()?);
    }
    Ok(values)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8], offset: usize) -> Self {
        Self { bytes, offset }
    }

    fn take<const N: usize>(&mut self) -> Result<[u8; N], FormatError> {
        let end = self.offset.checked_add(N).ok_or(FormatError::Truncated)?;
        let slice = self
            .bytes
            .get(self.offset..end)
            .ok_or(FormatError::Truncated)?;
        self.offset = end;
        slice.try_into().map_err(|_| FormatError::Truncated)
    }

    fn u8(&mut self) -> Result<u8, FormatError> {
        Ok(self.take::<1>()?[0])
    }

    fn u16(&mut self) -> Result<u16, FormatError> {
        Ok(u16::from_le_bytes(self.take()?))
    }

    fn i16(&mut self) -> Result<i16, FormatError> {
        Ok(i16::from_le_bytes(self.take()?))
    }

    fn u32(&mut self) -> Result<u32, FormatError> {
        Ok(u32::from_le_bytes(self.take()?))
    }

    fn i32(&mut self) -> Result<i32, FormatError> {
        Ok(i32::from_le_bytes(self.take()?))
    }

    fn u64(&mut self) -> Result<u64, FormatError> {
        Ok(u64::from_le_bytes(self.take()?))
    }

    fn i64(&mut self) -> Result<i64, FormatError> {
        Ok(i64::from_le_bytes(self.take()?))
    }

    fn array_32(&mut self) -> Result<[u8; 32], FormatError> {
        self.take()
    }

    fn cell_ref(&mut self) -> Result<CellRef, FormatError> {
        Ok(CellRef {
            arena: ArenaId(self.u64()?),
            id: CellId(self.u64()?),
            generation: Generation(self.u32()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn body() -> ArenaBody {
        let arena = ArenaId(42);
        ArenaBody {
            arena,
            version: 7,
            minimum_position: -8,
            maximum_position: 8,
            cell_capacity: 8,
            arrow_capacity: 8,
            cells: vec![
                DurableCell {
                    id: CellId(9),
                    generation: Generation(2),
                    physical_id: 900,
                    position: 3,
                    region: 1,
                    threshold: 2,
                    resistance: 4,
                    live: true,
                },
                DurableCell {
                    id: CellId(2),
                    generation: Generation(1),
                    physical_id: 200,
                    position: -3,
                    region: 0,
                    threshold: 1,
                    resistance: 3,
                    live: true,
                },
            ],
            arrows: vec![DurableArrow {
                id: ArrowId(4),
                generation: Generation(3),
                from: CellRef {
                    arena,
                    id: CellId(2),
                    generation: Generation(1),
                },
                to: CellRef {
                    arena,
                    id: CellId(9),
                    generation: Generation(2),
                },
                delay: 3,
                phase: 2,
                coupling: 1,
                resistance: 5,
                transmission_mode: 0,
                live: true,
            }],
        }
    }

    #[test]
    fn canonical_round_trip_and_order() {
        let original = body();
        let bytes = original.canonical_bytes().unwrap();
        let decoded = ArenaBody::decode(&bytes).unwrap();
        let encoded_again = decoded.canonical_bytes().unwrap();
        assert_eq!(bytes, encoded_again);
        assert_eq!(decoded.cells[0].id, CellId(2));
        assert_eq!(decoded.content_hash().unwrap(), ContentHash::of(&bytes));
    }

    #[test]
    fn corruption_truncation_and_trailing_bytes_fail_closed() {
        let bytes = body().canonical_bytes().unwrap();
        let mut corrupt = bytes.clone();
        *corrupt.last_mut().unwrap() ^= 1;
        assert_eq!(
            ArenaBody::decode(&corrupt),
            Err(FormatError::SectionChecksum)
        );
        assert_eq!(
            ArenaBody::decode(&bytes[..bytes.len() - 1]),
            Err(FormatError::Truncated)
        );
        let mut trailing = bytes;
        trailing.push(0);
        assert_eq!(
            ArenaBody::decode(&trailing),
            Err(FormatError::TrailingBytes)
        );

        let mut overlapping = body().canonical_bytes().unwrap();
        overlapping[68..76].copy_from_slice(&(HEADER_LEN as u64).to_le_bytes());
        assert_eq!(
            ArenaBody::decode(&overlapping),
            Err(FormatError::InvalidHeader)
        );
    }

    #[test]
    fn duplicate_identity_is_rejected() {
        let mut invalid = body();
        invalid.cells.push(invalid.cells[0]);
        assert_eq!(
            invalid.validate(),
            Err(FormatError::DuplicateCellId(CellId(9)))
        );
    }

    #[test]
    fn body_manifest_is_canonical_and_hashed() {
        let first = ArenaVersion {
            arena: ArenaId(9),
            block: ContentHash([9; 32]),
        };
        let second = ArenaVersion {
            arena: ArenaId(2),
            block: ContentHash([2; 32]),
        };
        let manifest = BodyVersion {
            version: 12,
            parent: Some(ContentHash([1; 32])),
            arenas: vec![first, second],
        };
        let bytes = manifest.canonical_bytes().unwrap();
        let decoded = BodyVersion::decode(&bytes).unwrap();
        assert_eq!(decoded.arenas, vec![second, first]);
        assert_eq!(decoded.canonical_bytes().unwrap(), bytes);
        assert_eq!(decoded.content_hash().unwrap(), ContentHash::of(&bytes));
    }
}
