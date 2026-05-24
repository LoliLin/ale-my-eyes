use ale_core::{AleEngine, AleEngineFactory};
use iced::widget::{button, column, container, text};
use iced::{Alignment, Element, Length, Subscription, Task};
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn main() -> iced::Result {
    iced::application(AleApp::new, AleApp::update, AleApp::view)
        .title("Ale, My Eyes!")
        .subscription(AleApp::subscription)
        .run()
}

struct AleApp {
    engine: Option<Arc<Mutex<AleEngine>>>,
    status: String,
    result: String,
    is_recording: bool,
}

#[derive(Clone)]
enum Message {
    ToggleRecording,
    DescribeImage,
    ClearResult,
    EngineReady(Result<Arc<Mutex<AleEngine>>, String>),
}

impl AleApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                engine: None,
                status: "正在初始化...".to_string(),
                result: String::new(),
                is_recording: false,
            },
            Task::perform(
                async {
                    AleEngineFactory::create_default()
                        .await
                        .map(|engine| Arc::new(Mutex::new(engine)))
                        .map_err(|error| error.to_string())
                },
                Message::EngineReady,
            ),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleRecording => {
                self.is_recording = !self.is_recording;
                if self.is_recording {
                    self.status = "正在聆听...".to_string();
                } else {
                    self.status = "处理中...".to_string();
                    // 模拟处理
                    self.result = "这是一条语音识别结果示例".to_string();
                    self.status = "就绪".to_string();
                }
            }
            Message::DescribeImage => {
                self.status = "处理中...".to_string();
                // 模拟处理
                self.result = "这是一张示例图片的描述".to_string();
                self.status = "就绪".to_string();
            }
            Message::ClearResult => {
                self.result.clear();
            }
            Message::EngineReady(Ok(engine)) => {
                self.engine = Some(engine);
                self.status = "就绪".to_string();
            }
            Message::EngineReady(Err(error)) => {
                self.status = format!("初始化失败: {error}");
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let status_card =
            container(column![text("系统状态").size(20), text(&self.status).size(16),].spacing(8))
                .padding(16)
                .style(container::rounded_box);

        let voice_button = button(
            text(if self.is_recording {
                "停止"
            } else {
                "开始录音"
            })
            .size(16),
        )
        .padding(20)
        .style(if self.is_recording {
            button::danger
        } else {
            button::primary
        })
        .on_press(Message::ToggleRecording);

        let describe_button = button(text("上传图像并描述").size(16))
            .padding(16)
            .style(button::secondary)
            .on_press(Message::DescribeImage);

        let result_card: Element<Message> = if !self.result.is_empty() {
            container(
                column![
                    text("识别结果").size(20),
                    text(&self.result).size(16),
                    button(text("清除结果").size(14))
                        .padding(8)
                        .style(button::text)
                        .on_press(Message::ClearResult),
                ]
                .spacing(8),
            )
            .padding(16)
            .style(container::rounded_box)
            .into()
        } else {
            container(text("")).into()
        };

        let content = column![
            status_card,
            column![
                text("语音交互").size(20),
                voice_button,
                text("支持中英文语音识别").size(14),
            ]
            .spacing(8)
            .align_x(Alignment::Center),
            column![text("图像描述").size(20), describe_button,]
                .spacing(8)
                .align_x(Alignment::Center),
            result_card,
        ]
        .spacing(24)
        .padding(24)
        .align_x(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
