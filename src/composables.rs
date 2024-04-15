use leptos_use::use_window;
use woothee::parser::{Parser, WootheeResult};

pub struct Experience<'a> {
    pub date: &'a str,
    pub job: &'a str,
    pub company_name: &'a str,
    pub link: &'a str,
    pub content: &'a str,
    pub tech_stack: &'a [&'a str],
}

pub struct Project<'a> {
    pub title: &'a str,
    pub link: &'a str,
    pub description: &'a str,
    pub img_src: &'a str,
    pub tech_stack: &'a [&'a str],
}

pub const NAV_ITEMS: [&str; 2] = ["Portfolio", "Projects"];
pub const HEADERS: [&str; 3] = ["About", "Experiences", "Projects"];
pub const EXPERIENCES: [Experience; 4] = [
    Experience {
      date: "Sep 2023 — Present",
      job: "Software Engineer",
      company_name: "Allianz Trade",
      link: "https://www.allianz-trade.fr/",
      content: "Automating data analysis processes for numerous legacy C/C++ projects. \
            Creating automation scripts and a CLI tool. Converting legacy PLM documents into Git history for improved tracking systems.",
      tech_stack: &["Rust", "Python", "C", "C++", "Batch File", "Polars", "Makefile", "Cmake"]
    },
    Experience {
      date: "Sep 2021 — Sep 2023",
      job: "Fullstack Engineer",
      company_name: "Federation Francaise de Football",
      link: "https://www.fff.fr/",
      content: "Developed and maintained multiple applications and tools for internal FFF employees \
              such as payment app, event organizers app, document management app, etc",
      tech_stack: &["React", "Angular", "HTML", "CSS", "Python", "FastAPI", "Javascript", "Typescript",
        "PrimeNG", "MaterialUI", "Bootstrap", "TailwindCSS", "Traefik", "Azure", "MongoDB", "Redis"]
    },
    Experience {
      date: "Sep 2020 — Mars 2021",
      job: "Deep Learning Engineer",
      company_name: "Fintricity",
      link: "https://www.fintricity.com/",
      content: "Researched and developed a Deep Learning Model for Weed Detection App. \
              The Deep Learning Model can differentiate different growth stage of the plant \
              Worked with a researcher to implement his solution into Amazon Web Service",
      tech_stack: &["Python", "Numpy", "FastAI", "OpenCV", "AWS"]
    },
    Experience {
      date: "Mars 2019 — Sep 2019",
      job: "Robotics Engineer",
      company_name: "Trimble Inc",
      link: "https://www.trimble.com/",
      content: "Researched and developed a synchronization method between motion sensors and camera on a mobile robot. \
              Worked with a researcher to implement his solution onto navigation system of the company",
      tech_stack: &["C++", "CMake", "Boost", "OpenCV", "Python", "FPGA"]
    }
  ];

pub const PROJECTS: [Project; 2] = [
    Project {
        title: "Finite State Machine Mobile Robot",
        link: "/Projects/SumoRobot",
        description:
            "A robot that can push any object using luminosity sensor and infrared sensor. \
                  Implementation of finite state machine methodology",
        img_src: "/images/sumo_robot.png",
        tech_stack: &["C", "Arduino", "Blender"],
    },
    Project {
        title: "Sensor Synchronization",
        link: "/Projects/TrimbleIntern",
        description:
            "Aligned IMU & camera data by detecting IMU readings before each image capture, marking image timestamps, & creating synchronized IMU data.",
        img_src: "/images/trimble_robot.png",
        tech_stack: &["C++", "CMake", "Boost", "OpenCV", "Python", "FPGA"],
    },
];

pub fn get_ua<'a>() -> Option<WootheeResult<'a>> {
    let parser = Parser::new();
    match use_window()
        .navigator()
        .and_then(|nav| nav.user_agent().ok())
    {
        Some(ua_str) => {
            let ua_str_heap_ref: &'a str = Box::leak(ua_str.into_boxed_str());
            // ua_str is a local variable to the closure,
            // in order to pass in a reference we need to allocate a new string
            // on the heap and take a reference to it
            parser.parse(ua_str_heap_ref)
        }
        None => None,
    }
}
