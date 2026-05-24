use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ale-cli")]
#[command(about = "CLI tool for Ale, My Eyes!")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 语音识别
    Transcribe {
        /// 音频文件路径
        #[arg(short, long)]
        audio: PathBuf,

        /// 输出文本文件路径
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// 语音合成
    Synthesize {
        /// 要合成的文本
        #[arg(short, long)]
        text: String,

        /// 输出音频文件路径
        #[arg(short, long)]
        output: PathBuf,

        /// 语音
        #[arg(long)]
        voice: Option<String>,
    },

    /// 图像描述
    Describe {
        /// 图像文件路径
        #[arg(short, long)]
        image: PathBuf,

        /// 输出文本文件路径
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// 显示状态
    Status,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Transcribe { audio, output: _ } => {
            println!("Transcribing audio: {}", audio.display());
            // TODO: 实现转录功能
        }
        Commands::Synthesize {
            text,
            output: _,
            voice: _,
        } => {
            println!("Synthesizing text: {}", text);
            // TODO: 实现合成功能
        }
        Commands::Describe { image, output: _ } => {
            println!("Describing image: {}", image.display());
            // TODO: 实现描述功能
        }
        Commands::Status => {
            println!("Ale, My Eyes! CLI");
            println!("Version: 0.1.0");
            // TODO: 显示引擎状态
        }
    }

    Ok(())
}
