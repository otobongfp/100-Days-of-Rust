

fn main() {

    enum Iptype {
        v4,
        v6
    }


    struct Ipaddress{
        kind : Iptype,
        address : String;
    }


    let home = Ipaddress{
        kind: Iptype::v4,
        address: String::from("127.0.0.1"),
    };

}