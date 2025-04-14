#![allow(non_snake_case)]

use nalgebra::{Vector3, Matrix3, Quaternion, UnitQuaternion, Unit};
use serde::Serialize;
use serde::Deserialize;
mod logger; // Import the csv_writer module
use logger::write_to_csv; // Import the function

#[derive(Serialize)]
struct StateRow {
    time: f32,
    wx: f32,
    wy: f32,
    wz: f32,
    qi: f32,
    qj: f32,
    qk: f32,
    qw: f32
}

#[derive(Debug, Deserialize)]
struct Config {
    sim: Sim,
    vehicle_props: VehicleProperties,
    initial_conditions: InitialConditions,
    initial_state: InitialState
}

#[derive(Debug, Deserialize)]
struct Sim {
    dt: f32,
    t_f: f32,
    log_path: String,
    log_name: String
}

#[derive(Debug, Deserialize)]
struct VehicleProperties {
    moi: [[f32; 3]; 3]
}

#[derive(Debug, Deserialize)]
struct InitialConditions {
    tau: [f32; 3]
}

#[derive(Debug, Deserialize)]
struct InitialState {
    w_0: [f32; 3],
    q_i2b_0: [f32; 4]
}

// Main function
fn main() -> Result<(), config::ConfigError> {
    // Parse config file
    let config: Config = config::Config::builder()
        .add_source(config::File::with_name("config/input"))
        .build()?
        .try_deserialize()?;

    // Simulation configuration
    // Time step of simulation (s)
    let dt: f32 = config.sim.dt;
    // End time of simulation (s)
    let t_f: f32 = config.sim.t_f;
    // Number of iterations in simulation
    let num_iter = (t_f / dt).floor() as usize;
    // Data buffer for writing to file
    let mut data_buffer = Vec::with_capacity(num_iter);
    // Logs path
    let log_path = config.sim.log_path;
    // Log file name
    let log_name = config.sim.log_name;

    // Vehicle configuration
    // Moment of inertia tensor matrix (kg*m^2)
    let moi = Matrix3::from(config.vehicle_props.moi);
    // Compute the inverse of moi
    let moi_inv = moi.try_inverse().unwrap();
    // Torque in the body frame (N*m) -- constant for now
    let tau = Vector3::from(config.initial_conditions.tau);

    // Initial state
    // Initial angular velocity in the body frame (rad/s)
    let mut w = Vector3::from(config.initial_state.w_0);
    // Initial attitude quaternion (inertial to body)
    let q_in: Quaternion<f32> = Quaternion::from_vector(config.initial_state.q_i2b_0.into());
    let mut q_i2b: UnitQuaternion<f32> = UnitQuaternion::from_quaternion(q_in);

    // Main loop
    for ii in 0..=num_iter {
        // Calculate angular acceleration (rad/s^2)
        let w_dot = moi_inv * (tau - w.cross(&(moi * w)));
        // Integrate angular accelaration to get angular velocity
        w = w + w_dot * dt;
        // Integrate angular velocity to get delta-angle vector (rad)
        let theta: Vector3<f32> = w * dt;
        // Convert to eigen axis-angle pair
        let eigen_axis = Unit::new_normalize(theta);
        let eigen_angle = theta.norm();
        // Convert axis-angle to delta quaternion
        let dq = UnitQuaternion::from_axis_angle(&eigen_axis, eigen_angle);
        // Rotate attitude with dq
        q_i2b = UnitQuaternion::from_quaternion(*q_i2b * *dq);
        // Compute current time
        let t = dt * ii as f32;
        // Push current angular velocity to buffer
        data_buffer.push(StateRow {
            time: t,
            wx: w.x,
            wy: w.y,
            wz: w.z,
            qi: q_i2b.i,
            qj: q_i2b.j,
            qk: q_i2b.k,
            qw: q_i2b.w
        });
    }
    
    // Write data buffer to csv
    if let Err(e) = write_to_csv(&data_buffer, &(log_path + &log_name)) {
        eprintln!("Error writing to CSV: {}", e);
    }

    // Return status
    Ok(())
}
