extern crate iron;
extern crate router;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use router::Router;

use std::io::prelude::*;
use std::fs::File;
use std::io::BufWriter;

fn main(){

    MainLoop();
}

fn MainLoop(){
    let mut router = Router::new();
    router.post("/", catchIP, "catchIP");

    Iron::new(router).http("localhost:80").unwrap();

    fn catchIP(req: &mut Request) -> IronResult<Response>{
        let mut body = String::new();
        req.body.read_to_string(&mut body)
            .expect("Failed to read line");

        println!("{}", body);
        //FileOutPut(body);
        
        let res = "catch ".to_string() + &body;
        Ok(Response::with((status::Ok, res)))
    }
}
/*
fn FileOutPut(ipaddress: String){
    let file = File::open("database.txt").unwrap();
    let mut w = BufWriter::new(file);
    write!(w, "ipaddress").unwrap();
    w.flush().unwrap();
}
*/