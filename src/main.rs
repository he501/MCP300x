use rppal::spi;
fn main() {
    println!("Hello, world!");
    analog_read(3,spi::Bus::Spi1);
    // let val=analog_read(0,spi::Bus::Spi1);
}

// fn analog_read(ch:u8,bus_no:rppal::spi::Bus)->Result<i32,String>{
fn analog_read(ch:u8,bus_no:spi::Bus){

    println!("ok");
    let mut r_buf:[u8;2]=[0;2];
    let w_buf:[u8;3]=[0x01,(0x01<<7)|(ch&0x07)<<4,0x00];
    println!("{:?}",w_buf);
    let spi=spi::Spi::new(bus_no, spi::SlaveSelect::Ss0, 100_000, spi::Mode::Mode0).unwrap();
    spi::Spi::transfer(&spi,&mut r_buf,&w_buf).unwrap();
    println!("{:?}",r_buf);
}