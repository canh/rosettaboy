use std::time::{Duration, SystemTime};

pub struct Clock {
    start: SystemTime,
    last_frame_start: SystemTime,
    last_report: SystemTime,
    time_per_frame: Duration,
    cycle: u32,
    frame: u32,
    profile: u32,
    turbo: bool,
    fps: bool,
}

impl Clock {
    pub fn init(profile: u32, turbo: bool, fps: bool) -> Clock {
        let start = SystemTime::now();
        let last_frame_start = SystemTime::now();
        let time_per_frame = Duration::from_millis((1000.0 / 60.0) as u64);
        let last_report = SystemTime::now();

        Clock {
            start,
            last_frame_start,
            last_report,
            time_per_frame,
            cycle: 0,
            frame: 0,
            profile,
            turbo,
            fps,
        }
    }

    pub fn tick(&mut self) -> bool {
        self.cycle += 1;

        // Do a whole frame's worth of sleeping at the start of each frame
        if self.cycle % 17556 == 20 {
            // Sleep if we have time left over
            let time_spent = SystemTime::now()
                .duration_since(self.last_frame_start)
                .expect("time");
            if !self.turbo && time_spent < self.time_per_frame {
                ::std::thread::sleep(self.time_per_frame - time_spent);
            }
            self.last_frame_start = SystemTime::now();

            // Print FPS once per frame
            if self.fps && self.frame % 60 == 0 {
                let t = SystemTime::now();
                let fps =
                    60000.0 / (t.duration_since(self.last_report).unwrap().as_millis()) as f32;
                println!("{:.1}fps", fps);
                self.last_report = t;
            }

            // Exit if we've hit the frame limit
            if self.profile != 0 && self.frame > self.profile {
                println!(
                    "Hit frame limit after {:?}",
                    SystemTime::now().duration_since(self.start).expect("time")
                );
                return false;
            }

            self.frame += 1;
        }

        return true;
    }
}
