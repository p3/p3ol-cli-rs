pub mod atom_stream;

use crate::atom_stream::Atom;
use crate::atom_stream::AtomStream;

fn main() {
    let chat_room_packet: &str = "5a3cbf009b1c1420415403e5000100010904130000020f130101010a04000001010b0106477565737435100b0400000fa0020201020b0200011d00010a01000201020082011d00023504140000250f130102010a04000001000114297f4f6e6c696e65486f73743a094775657374352068617320656e74657265642074686520726f6f6d2e011d00000701020111000007010101180400000102011d000012000d5a99ce00031c14240d";

    let mut parser: AtomStream = AtomStream::new();

    parser.parse(chat_room_packet);

    let test: Atom = parser
        .atoms
        .into_iter()
        .find(|atom: &Atom| atom.name == "chat_add_user")
        .unwrap();

    println!(
        "The user that joined the chat from this packet: {:?}",
        test.data
    );
}
