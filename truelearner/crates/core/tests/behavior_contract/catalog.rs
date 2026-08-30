//! The shared catalog stays in `truelearner-behavior-contract`.
//!
//! This module records only which old implementation profile establishes each
//! family. The scenario itself never sees this detail.

use super::legacy::LegacyProfile;

pub const CORE_STORY: LegacyProfile = LegacyProfile::Physical;
