use rppal::spi;
fn main() {
    println!("Hello, world!");
    let val=analog_read(0,spi::Bus::Spi0);
    println!("val={}",val);
}

// fn analog_read(ch:u8,bus_no:rppal::spi::Bus)->Result<i32,String>{
fn analog_read(ch:u8,bus_no:spi::Bus)->i32{
    println!("ok");
    let mut r_buf:[u8;3]=[0;3];
    let w_buf:[u8;3]=[0x01,(0x01<<7)|(ch&0x07)<<4,0x00];
    println!("{:?}",w_buf);
    let spi=spi::Spi::new(bus_no, spi::SlaveSelect::Ss0, 50_000, spi::Mode::Mode0).unwrap();
    spi::Spi::transfer(&spi,&mut r_buf,&w_buf).unwrap();
    println!("{:?}",r_buf);
    ((r_buf[1]as i32 & 0x03)<<8)|r_buf[2] as i32
}
