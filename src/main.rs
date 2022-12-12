#![windows_subsystem = "windows"]

use std::ffi::CString;
use user32::MessageBoxW;
use winapi::winuser::{MB_OK, MB_ICONINFORMATION};

use iced::{
    alignment,
    window,
    Sandbox,
    Element,
    Settings,

    Column,
    Text,
    Length,
    Container,
    Space,
    Row,
    Rule,
    TextInput,
    Button,
    Color
};
use iced::widget::image::Image;

use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;

mod style;
mod download;

fn show_message(title: &mut String, content: &mut String) {
    title.push_str("\0");
    content.push_str("\0");

    let lp_caption: Vec<u16> = title.as_str().encode_utf16().collect();
    let lp_text: Vec<u16> = content.as_str().encode_utf16().collect();

    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            MB_OK
        );
    }
}

async fn download_webtoon(title: String, start: i32, end: i32) -> Result<(), ()> {
    let html = reqwest::get(String::from("https://comic.naver.com/search?keyword=".to_string() + title.as_str()).as_str()).await.unwrap().text().await.unwrap();
    let base = html.split("<ul class=\"resultList\">").collect::<Vec<&str>>()[1];
    if String::from(base).contains("검색 결과가 없습니다.") {
        return Err(());
    } else {
        let title = base.split("<a href=\"").collect::<Vec<&str>>()[1].to_string()
            .to_string().split("\">").collect::<Vec<&str>>()[1]
            .to_string().split("<").collect::<Vec<&str>>()[0].to_string();
        let link = base.split("<a href=\"").collect::<Vec<&str>>()[1].to_string()
            .to_string().split("\"").collect::<Vec<&str>>()[0]
            .to_string().split("titleId=").collect::<Vec<&str>>()[1].to_string();
        println!("{}", link);
        //title = title.split("\">").collect::<Vec<&str>>()[1].to_string();
        println!("{}", title);

        //https://comic.naver.com/webtoon/detail?titleId={id}&no={no}

        std::fs::create_dir("./".to_owned() + title.as_str()).unwrap_or(());

        for i in start..end+1 {
            std::fs::create_dir("./".to_owned() + title.as_str() + "/" + i.to_string().as_str() + "화").unwrap_or(());

            let webtoon: String = "https://comic.naver.com/webtoon/detail?titleId=".to_string() + link.as_str() + "&no=" + i.to_string().as_str();
            let res = reqwest::get(webtoon).await.unwrap().text().await.unwrap();
            let img_base = res.split("<div class=\"wt_viewer\"").collect::<Vec<&str>>()[1]
                .to_string().split("</div>").collect::<Vec<&str>>()[0]
                .to_string();

            let splits = img_base.split("<img src=\"").collect::<Vec<&str>>();
            for j in 1..splits.len() {
                let link = splits[j].to_string().split("\"").collect::<Vec<&str>>()[0]
                    .to_string();
                let copy = title.clone();
                let func = async move {
                    download::fetch_url(link.to_string(), "./".to_owned() + copy.as_str() + "/" + i.to_string().as_str() + "화/" + j.to_string().as_str() + ".jpg").await.unwrap_or(());
                };
                tokio::task::spawn(func);
            }
            // let rst = download::fetch_url("https://image-comic.pstatic.net/webtoon/769209/70/20220628184630_0f3f6844ad6f07a8d2b1b3d404ce6d31_IMAG01_1.jpg".to_string(), "test.jpg".to_string());
            // if let Ok(r) = rst.await {
            //     println!("HI");
            // }
        }
    }
    show_message(&mut String::from(""), &mut String::from(title + " 웹툰의 다운로드가 완료되었습니다!"));
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //download_webtoon("연애혁명".to_string(), 1, 1).await.unwrap();

    // Window custom settings
    let settings = Settings {
        window: window::Settings {
            size: (800,500),
            position: window::Position::default(),
            resizable: false,
            decorations: true,
           ..Default::default()
        },
        default_font: Some(include_bytes!("font.ttf")),
        antialiasing: true,
        ..Default::default()
    };

    Application::run(settings);

    Ok(())
}

#[derive(Default)]
struct Application {
    name: String,
    start: String,
    end: String,
    input: iced::text_input::State,
    inputStart: iced::text_input::State,
    inputEnd: iced::text_input::State,
    btn: iced::button::State,

    message: Arc<Mutex<String>>,
}

#[derive(Debug, Clone)]
enum Message {
    NameChanged(String),
    StartChanged(String),
    EndChanged(String),
    ButtonPressed
}

impl Sandbox for Application {
    type Message = Message;

    fn new() -> Self {
        Application::default()
    }

    fn title(&self) -> String {
        "Webtoon Downloader".into()
    }

    fn update(&'_ mut self, message: Message) {
        match message {
            Message::NameChanged(content) => {
                self.name = content
            },
            Message::StartChanged(content) => {
                self.start = content
            },
            Message::EndChanged(content) => {
                self.end = content
            },
            Message::ButtonPressed => {
                let ok: Result<(), std::num::ParseIntError> = (|| -> Result<(), std::num::ParseIntError> {
                    self.start.parse::<i32>()?;
                    Ok(())
                })();

                if let Err(res) = ok {
                    //show_message(&mut String::from(""), &mut String::from("시작 화를 제대로 입력해주세요!"));
                    self.message = Arc::new(Mutex::new("시작 화를 제대로 입력해주세요!".into()));
                    // let clone = Arc::clone(&self.message);
                    // // FUCK YOU BITCH ERROR FUCKING RUST. I DON't KNOW ANYTHING ABOUT THIS ERROR.
                    // // ㅋㅋ 해결했죠 미친이찬욱
                    // // 아니었네 씨발
                    // let test = async move {
                    //     tokio::time::sleep(Duration::from_millis(1000)).await;
                    //     //thread::sleep(Duration::from_millis(1000));
                    //     *clone.lock().unwrap() = "teasdasdst".into();
                    // };
                    // tokio::task::spawn(test);
                    return
                }

                let ok: Result<(), std::num::ParseIntError> = (|| -> Result<(), std::num::ParseIntError> {
                    self.end.parse::<i32>()?;
                    Ok(())
                })();

                if let Err(res) = ok {
                    //show_message(&mut String::from(""), &mut String::from("끝 화를 제대로 입력해주세요!"));
                    self.message = Arc::new(Mutex::new("끝 화를 제대로 입력해주세요!".into()));
                    return
                }

                // TODO
                tokio::task::spawn(download_webtoon(self.name.clone(), self.start.parse::<i32>().unwrap(), self.end.parse::<i32>().unwrap()));
                show_message(&mut String::from(""), &mut String::from(self.name.clone() + " 웹툰의 다운로드가 시작됩니다!"));
            },
        }
    }
    
    fn view(&mut self) -> Element<Message> {
        let image_stream: &[u8] = include_bytes!("../resources/logo.png");

        let image = iced::widget::image::Handle::from_memory(image_stream.to_vec());

        let top: Row<Message> = Row::new()
        .height(Length::from(60))
        .push(Image::new(image).width(Length::from(60)).height(Length::from(60)))
        .push(Rule::vertical(16))
        .push(Space::with_width(Length::from(15)))
        .push(Column::new()
            .push(Space::with_height(Length::from(15)))
            .push(Text::new("네이버 웹툰 다운로더 v1.0 by Devil's Husband").size(30))
        );

        let text_input = TextInput::new(
            &mut self.input,
            "네이버 웹툰 이름을 입력해주세요.",
            &self.name,
            Message::NameChanged
        )
        .padding(7)
        .width(Length::from(500));

        let center: Row<Message> = Row::new().push(
            TextInput::new(
                &mut self.inputStart,
                "시작 화",
                &self.start,
                Message::StartChanged
            ).padding(6).width(Length::from(300))
        )
            .push(Space::with_width(Length::from(100)))
            .push(TextInput::new(
            &mut self.inputEnd,
            "끝 화",
            &self.end,
            Message::EndChanged
        ).padding(6).width(Length::from(300)));

        let bottom: Row<Message> = Row::new().push(
        Column::new()
            .push(Space::with_height(Length::from(5)))
            .push(Text::new("웹툰 이름").size(20))
        )
        .push(Space::with_width(Length::from(10)))
        .push(text_input)
        .push(Space::with_width(Length::from(10)))
        .push(Button::new(&mut self.btn, Text::new("다운로드"))
            .padding(8)
            .on_press(Message::ButtonPressed)
            .style(style::Theme))
        ;

        let column: Column<Message> = Column::new()
            .push(Space::with_width(Length::from(800 - 20)))
            .push(top)
            .push(Space::with_height(Length::from(40)))
            .push(center)
            .push(Space::with_height(Length::from(40)))
            .push(bottom)
            .push(Text::new(self.message.lock().unwrap().as_str()).color(Color::from_rgb(0.91, 0.5, 0.4)));


        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(alignment::Horizontal::Right)
            //.style(self.theme)
            .into()
    }
}