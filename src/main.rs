//  TO-DO
//  
//  test if 2 clients sending a 1080p screen simultaniously can be handled
//  literally write thefucking program
//  save "lcs" last corrrect screen for when recieving broken screen packets
//  use multiple threads for encoding and decoding
//


use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    // USERS:
    //  [ [user1,ip1,uservolume1,ssvolume1,lastpacket1],
    //  [u2,i2,uv2,ssv2,lp2],
    //  [...], ... ]
    static ref USERS: Mutex<Vec<Vec<String>>> = Mutex::new(Vec::new));
}

// username that others use to refer to you
static MYUSER: &str = "myun";
// ip address for others to send packets to
// currently working with port 6996, but you and your group could use a different one
//static MYIP: &str = "myip:6996"; // replaced by lookieg in users.saved

fn send_packet(puser: String, ptype: String, pvoice: String, pscreen: String, psaudio: String) {
//  to pusers ip
//  ptype, len of MYUSER, MYUSER, len of pvoice, [pvoice], len of psceen, [pscreen]
}

fn finduserdata(tuser: String) -> Vec<String> {
//  n+=1
//
//  if n>14
//      print error
//      return garbage data
//
//  find user in users.saved
//
//  if found
//      return Vec<ip, vol, ssvol>
//      n=0
//
//  else 
//      print save user to file and try again
//      wait for input
//      return finduserdata(tuser)
}

fn handle_packet() {
    //read first byte
    //determine ptype to type and username lenght to userlen
    //read userlen bytes to user
    //match type to the types
    //call handle_<type> based on match
}

fn 
