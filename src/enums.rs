

fn main() {

    enum Iptype {
        v4(String),
        v6(String),
    }


    struct Ipaddress{
        kind : Iptype,
        address : String,
    }


    let home = Ipaddress{
        kind: Iptype::v4,
        address: String::from("127.0.0.1"),
    };

}