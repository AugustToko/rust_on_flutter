mod lib;

use lib::socket_test_1;
use std::net::TcpStream;
use std::io::{Write};

/*

int arrToDec(List<int> m) {
  int sum = 0;
  int a = 1; //2的0次方
  m.asMap().forEach((i, value) {
    if (i > 0) {
      a = a * 2;
    }
    sum += a * value;
  });
  // print(sum);
  return sum;
  // 12.toRadixString(16);
}

*/

fn arr_to_dec(m: &[u8; 8]) -> u8 {
    let mut sum: u8 = 0;
    let mut a: u8 = 1;  // 2的0次方

    let mut index = 1;
    while index != m.len() {
        if index > 0 {
            a = a * 2;
        }
        sum += a * m[index];
        index += 1;
    }

    return sum;
}

// modbus 8DO DEMO
fn main() -> () {
    if let Ok(mut stream) = TcpStream::connect("192.168.100.10:502") {
        println!("Connected to the server!");
        let dec = arr_to_dec(&[
            0,
            0,
            1,
            0,
            0,
            0,
            0,
            0,
        ]);
        println!("{}", dec);
        let list = [0, 0, 0, 0, 0, 8, 1, 15, 0, 0, 0, 8, 1, dec];
        stream.write_all(&list).unwrap();
        stream.flush().unwrap();
    } else {
        println!("Couldn't connect to server...");
    }
}