use atmospheric_utilities::standard_atmosphere::standard_atmosphere::*;


fn main() {
    let elev: f32 = 5355.0;
    let slp: f32 = 1005.9;
    let std_temp = standard_temperature_deg_kelvin(elev);
    let std_pressure = standard_pressure_milibars(elev);
    let std_sos = standard_speed_of_sound_meters_per_second(elev);
    let std_dens = standard_density_kilograms_per_cubic_meter(elev);
    let press_alt = standard_pressure_altitude(elev, slp);
    println!("Standard Temp x={elev}  y={std_temp}");
    println!("Standard Pressure x={elev}  y={std_pressure}");
    println!("Standard Speed of Sound x={elev}  y={std_sos}");
    println!("Standard Density x={elev}  y={std_dens}");
    println!("Standard Pressure Altitude {press_alt}");
}
