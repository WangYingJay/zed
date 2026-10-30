//! B档第四刀：原 `language_model_core` 里 settings 层真正用到的三个枚举。
//! 内联到本 crate，避免 settings_content → language_model_core → cloud_llm_client 旁路加重。

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Default, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Speed {
    #[default]
    Standard,
    Fast,
}

impl Speed {
    pub fn toggle(self) -> Self {
        match self {
            Speed::Standard => Speed::Fast,
            Speed::Fast => Speed::Standard,
        }
    }
}

/// Settings-layer reasoning-effort enum（原 language_model_core::ReasoningEffort）。
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, strum::EnumString,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ReasoningEffort {
    None,
    Minimal,
    Low,
    Medium,
    High,
    XHigh,
    Max,
}

impl ReasoningEffort {
    pub const OPENAI_COMPATIBLE_SELECTABLE: [Self; 6] = [
        Self::Minimal,
        Self::Low,
        Self::Medium,
        Self::High,
        Self::XHigh,
        Self::Max,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Minimal => "Minimal",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::XHigh => "Extra High",
            Self::Max => "Max",
        }
    }

    pub fn value(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Minimal => "minimal",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::XHigh => "xhigh",
            Self::Max => "max",
        }
    }
}

/// Settings-layer model mode（原 language_model_core::ModelMode）。
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ModelMode {
    #[default]
    Default,
    Thinking {
        budget_tokens: Option<u32>,
    },
    Adaptive,
}
