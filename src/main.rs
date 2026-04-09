//Generics project

#[derive(Debug)]
enum DigitalContent{
    AudioFile,
    VideoFile
}

#[derive(Debug)]
struct ChatMessage <T>{
    Content: T,
    Time: String,
}

impl ChatMessage<DigitalContent> {

    fn consume_entertainment(&self){
        match self.Content{
            DigitalContent::AudioFile => println!("Playing audio file"),
            DigitalContent::VideoFile => println!("Playing video file"),
            _=> println!("Invalid content"),
        }
    }
}

impl<T> ChatMessage<T>{
    fn retriveTime(&self)->String{
        println!("The time is {}", self.Time);
        self.Time.clone()
    }
}
fn main(){

    let chat_message = ChatMessage{
        Content: "hello",
        Time: String::from("12:00"),
    };

    

    let chat_message_2 = ChatMessage{
        Content: String::from("hello james"),
        Time: String::from("12:45"),
    };

    let chat_message_3 = ChatMessage{
        Content: DigitalContent::AudioFile,
        Time: String::from("12:40"),
    };

    let chat_message_4 = ChatMessage{
        Content: DigitalContent::VideoFile,
        Time: String::from("12:50"),
    };

    chat_message_3.consume_entertainment();
    chat_message_4.consume_entertainment();

    chat_message.retriveTime();
    chat_message_2.retriveTime();
    chat_message_3.retriveTime();
}