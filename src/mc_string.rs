use std::io::net::tcp::TcpStream;
use std::io::MemReader;

pub trait MCString{
    fn read_mc_string(&mut self) -> String;
    fn write_mc_string(&mut self, mc_str: String);
}

impl MCString for TcpStream{
    fn read_mc_string(&mut self) -> String{
        let mut bytes: Vec<u8> = self.read_exact(64).unwrap();
        let mut length: uint = 0;
        bytes.reverse();
        for i in range(0u, bytes.len()){
            if *bytes.get_mut(i).unwrap() != 0x20{
                length = i;
                break;
            }
        }
        let mut splitted = bytes.slice_from(length).to_vec();
        splitted.reverse();
        return String::from_utf8(splitted.slice_from(0).to_vec()).unwrap();
    }
    
    fn write_mc_string(&mut self, mc_str: String){
        self.write(mc_str.as_bytes());
        for i in range(0, 64 - mc_str.as_bytes().len()){
            self.write_u8(0x20);
        }
    }
}

impl MCString for MemReader{
    fn read_mc_string(&mut self) -> String{
        let mut bytes: Vec<u8> = self.read_exact(64).unwrap();
        let mut length: uint = 0;
        bytes.reverse();
        for i in range(0u, bytes.len()){
            if *bytes.get_mut(i).unwrap() != 0x20{
                length = i;
                break;
            }
        }
        let mut splitted = bytes.slice_from(length).to_vec();
        splitted.reverse();
        return String::from_utf8(splitted.slice_from(0).to_vec()).unwrap();
    }
    
    fn write_mc_string(&mut self, mc_str: String){
    }
}
