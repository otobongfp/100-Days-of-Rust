

fn main() {

    // enum Iptype {
    //     v4(String),
    //     v6(String),
    // }


    // let home = Iptype::v4(String::from("127:0:0:1"));

    // let loopback = Iptype::v6(String::from("0.0.1"));

    enum Message{
        Quit,
        Move {x:i32, y:i32},
        Write(String),
    }

    impl Message{
        fn call(&self){

        }
    }

    let m = Message::Write(String::from("Hello Enum"));
    m.call();

    fn main();
}