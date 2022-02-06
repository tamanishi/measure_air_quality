mod data;
mod scd41;

fn main() {
    let mut scd41 = scd41::Scd41::new();
    match scd41.read_single_shot() {
        Ok(data) => {
            println!("{{{}}}", data);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
