use crate::{AleError, Result};
use async_trait::async_trait;
use std::path::Path;

/// 语音识别trait
#[async_trait]
pub trait SpeechRecognizer: Send + Sync {
    /// 识别音频数据
    async fn transcribe(&self, audio_data: &[u8]) -> Result<String>;

    /// 获取支持的语言
    fn supported_languages(&self) -> Vec<String>;

    /// 获取模型信息
    fn model_info(&self) -> crate::ModelInfo;
}

/// Whisper语音识别器
pub struct WhisperRecognizer {
    model_path: std::path::PathBuf,
    model: Option<whisper_rs::WhisperContext>,
}

impl WhisperRecognizer {
    pub async fn new(model_path: &Path) -> Result<Self> {
        let model_path = model_path.to_path_buf();

        // 检查模型文件是否存在
        if !model_path.exists() {
            return Err(AleError::AsrError(format!(
                "Model file not found: {}",
                model_path.display()
            )));
        }

        Ok(Self {
            model_path,
            model: None,
        })
    }

    fn load_model(&mut self) -> Result<()> {
        if self.model.is_some() {
            return Ok(());
        }

        let ctx = whisper_rs::WhisperContext::new_with_params(
            self.model_path.to_str().unwrap(),
            whisper_rs::WhisperContextParameters::default(),
        )
        .map_err(|e| AleError::AsrError(format!("Failed to load whisper model: {}", e)))?;

        self.model = Some(ctx);
        Ok(())
    }
}

#[async_trait]
impl SpeechRecognizer for WhisperRecognizer {
    async fn transcribe(&self, audio_data: &[u8]) -> Result<String> {
        // 这里需要实现实际的转录逻辑
        // 由于whisper-rs API需要特定格式，这里简化处理
        Err(AleError::AsrError("Not implemented yet".to_string()))
    }

    fn supported_languages(&self) -> Vec<String> {
        vec![
            "en".to_string(),
            "zh".to_string(),
            "ja".to_string(),
            "ko".to_string(),
            "fr".to_string(),
            "de".to_string(),
            "es".to_string(),
            "ru".to_string(),
        ]
    }

    fn model_info(&self) -> crate::ModelInfo {
        crate::ModelInfo {
            name: "whisper".to_string(),
            version: "1.0".to_string(),
            device: "cpu".to_string(), // 可以检测是否有GPU
            loaded: self.model.is_some(),
        }
    }
}
