trait TurbineTrait {
    fn send_command(&self, cmd: &str) -> String;
    fn get_turbine_number(&self) -> i32;
}

struct Turbine {
    turbine_number: i32,
}

impl TurbineTrait for Turbine {
    fn send_command(&self, cmd: &str) -> String {
        format!("The turbine {} get command: {}", self.turbine_number, cmd)
    }
    fn get_turbine_number(&self) -> i32 {
        self.turbine_number
    }
}

struct CentralController {
    turbines: Vec<Box<dyn TurbineTrait>>,
}

// #[macro_use]
impl CentralController {
    // #[warn(dead_code)]
    // fn attach_turbine(&mut self, turbine: Box<dyn TurbineTrait>) {
    //     if self.turbines.iter().all(|x| x.get_turbine_number() != turbine.get_turbine_number()) {
    //         self.turbines.push(turbine);
    //     }
    // }

    fn shutdown_turbine(&mut self, turbine_number: i32) -> String {
        if let Some(idx) = self.turbines.iter().position(|x| x.get_turbine_number() == turbine_number) {
            self.turbines.remove(idx);
            format!("Turbine {} is OFF", turbine_number)
        } else {
            format!("Turbine {} NOT Found!", turbine_number) // no exception
        }
    }

    fn notify_turbine(&self, cmd: &str) -> Vec<String> {
        // mapping iterable object
        // self.turbines.iter().map(|x| x.send_command(cmd)).collect()
        let mut messages = vec![];
        for turbine in self.turbines.iter(){
            messages.push(turbine.send_command(cmd))
        }
        messages
    }
}

fn main() {
    let mut controller = CentralController {
        turbines: vec![
            Box::new(Turbine { turbine_number: 1 }),
            Box::new(Turbine { turbine_number: 2 }),
            Box::new(Turbine { turbine_number: 3 }),
            Box::new(Turbine { turbine_number: 4 }),
        ],
    };
    // All turbines are on operating
    let received_messages = controller.notify_turbine("[On Operation]");
    for msg in received_messages {
        println!("{}", msg);
    }
    // The central controller shuts down several turbines
    println!("{}", controller.shutdown_turbine(2));
    println!("{}", controller.shutdown_turbine(4));

    // After shutting down some turbines
    let received_messages = controller.notify_turbine("[On Operation]");
    for msg in received_messages {
        println!("{}", msg);
    }
}