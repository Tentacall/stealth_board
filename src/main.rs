use stealth_board::server::{Connection, IP};

fn main() {
    let my_ip: IP = IP::new([127, 0, 0, 1], 8000);
    let conn: Connection = Connection::new();
    conn.serve(my_ip);
}
