use std::io::File;
use std::io::timer::sleep;

fn main() {
    (write!(File::create(&Path::new("/sys/class/gpio/export")), "{}", 18))
        .ok().expect("failed to export pin");
    
    let mut pin_file = File::create(&Path::new("/sys/class/gpio/gpio18/value"));
    
    for i in range(1,21) {
        (write!(pin_file, "{}", i%2))
            .ok().expect("failed to write pin value");
        sleep(500);
    }
    
    (write!(File::create(&Path::new("/sys/class/gpio/unexport")), "{}", 18))
        .ok().expect("failed to unexport pin");
}
