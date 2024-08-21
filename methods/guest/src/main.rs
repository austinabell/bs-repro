use risc0_zkvm::guest::env;

fn main() {
    let input: Option<(Vec<u8>, [u32; 8])> = env::read();

    if let Some((journal, image_id)) = input {
        env::verify(image_id, &journal).unwrap();
    }

    env::commit_slice("test".as_bytes());
}
