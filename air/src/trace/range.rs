use super::{RANGE_CHECK_AUX_TRACE_OFFSET, RANGE_CHECK_TRACE_OFFSET};

// CONSTANTS
// ================================================================================================

// --- Column accessors in the main trace ---------------------------------------------------------

/// A binary selector column to help specify whether or not the value should be included in the
/// running product.
pub const S0_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET;
/// A binary selector column to help specify whether or not the value should be included in the
/// running product.
pub const S1_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 1;
/// A column to hold the values being range-checked.
pub const V_COL_IDX: usize = RANGE_CHECK_TRACE_OFFSET + 2;

// --- Column accessors in the auxiliary columns --------------------------------------------------

/// The running product column used for verifying that the range check lookups performed in the
/// Stack and the Memory chiplet match the values checked in the Range Checker.
pub const B_RANGE_COL_IDX: usize = RANGE_CHECK_AUX_TRACE_OFFSET;

/// An auxiliary trace column of intermediate values used to enforce AIR constraints on `b_range`.
/// It contains the product of the lookups performed by the Stack processor at each cycle.
pub const Q_COL_IDX: usize = B_RANGE_COL_IDX + 1;