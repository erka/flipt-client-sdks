use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub enum FlagType {
    #[default]
    #[serde(rename = "VARIANT_FLAG_TYPE")]
    Variant,
    #[serde(rename = "BOOLEAN_FLAG_TYPE")]
    Boolean,
}

impl From<i32> for FlagType {
    fn from(n: i32) -> FlagType {
        match n {
            1 => FlagType::Boolean,
            0 => FlagType::Variant,
            _ => FlagType::Variant,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub enum SegmentOperator {
    #[default]
    #[serde(rename = "OR_SEGMENT_OPERATOR")]
    Or,
    #[serde(rename = "AND_SEGMENT_OPERATOR")]
    And,
}

impl From<i32> for SegmentOperator {
    fn from(n: i32) -> SegmentOperator {
        match n {
            1 => SegmentOperator::And,
            0 => SegmentOperator::Or,
            _ => SegmentOperator::Or,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub enum SegmentMatchType {
    #[default]
    #[serde(rename = "ANY_SEGMENT_MATCH_TYPE")]
    Any,
    #[serde(rename = "ALL_SEGMENT_MATCH_TYPE")]
    All,
}
impl From<i32> for SegmentMatchType {
    fn from(n: i32) -> SegmentMatchType {
        match n {
            1 => SegmentMatchType::Any,
            0 => SegmentMatchType::All,
            _ => SegmentMatchType::All,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub enum ConstraintComparisonType {
    #[default]
    #[serde(rename = "UNKNOWN_CONSTRAINT_COMPARISON_TYPE")]
    Unknown,
    #[serde(rename = "STRING_CONSTRAINT_COMPARISON_TYPE")]
    String,
    #[serde(rename = "NUMBER_CONSTRAINT_COMPARISON_TYPE")]
    Number,
    #[serde(rename = "BOOLEAN_CONSTRAINT_COMPARISON_TYPE")]
    Boolean,
    #[serde(rename = "DATETIME_CONSTRAINT_COMPARISON_TYPE")]
    DateTime,
}

impl From<i32> for ConstraintComparisonType {
    fn from(n: i32) -> ConstraintComparisonType {
        match n {
            0 => ConstraintComparisonType::Unknown,
            1 => ConstraintComparisonType::String,
            2 => ConstraintComparisonType::Number,
            3 => ConstraintComparisonType::Boolean,
            4 => ConstraintComparisonType::DateTime,
            _ => ConstraintComparisonType::Unknown,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub enum RolloutType {
    #[default]
    #[serde(rename = "UNKNOWN_ROLLOUT_TYPE")]
    Unknown,
    #[serde(rename = "SEGMENT_ROLLOUT_TYPE")]
    Segment,
    #[serde(rename = "THRESHOLD_ROLLOUT_TYPE")]
    Threshold,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum EvaluationReason {
    #[default]
    #[serde(rename = "UNKNOWN_EVALUATION_REASON")]
    Unknown,
    #[serde(rename = "FLAG_DISABLED_EVALUATION_REASON")]
    FlagDisabled,
    #[serde(rename = "MATCH_EVALUATION_REASON")]
    Match,
    #[serde(rename = "DEFAULT_EVALUATION_REASON")]
    Default,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum ErrorEvaluationReason {
    #[serde(rename = "UNKNOWN_ERROR_EVALUATION_REASON")]
    Unknown,
    #[serde(rename = "NOT_FOUND_ERROR_EVALUATION_REASON")]
    NotFound,
}
