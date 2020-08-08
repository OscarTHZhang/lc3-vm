pub mod memory;
pub mod register;
pub mod instruction;

fn main() -> std::io::Result<()> {
    let path = std::env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}