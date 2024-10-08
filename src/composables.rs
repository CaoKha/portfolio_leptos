// use leptos_use::use_window;
// use woothee::parser::{Parser, WootheeResult};

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
      content: "Automating data analysis processes for numerous legacy projects. \
            Creating automation scripts and a CLI tool. Converting legacy PLM documents into Git history. Developing REST API and pipeline for data analysis.",
      tech_stack: &["Rust", "Python", "C", "Batch File", "Polars", "Makefile", ]
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

pub const PROJECTS: [Project; 5] = [
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
        link: "/Projects/Trimble",
        description:
            "Aligned IMU & camera data by detecting IMU readings before each image capture, marking image timestamps, & creating synchronized IMU data.",
        img_src: "/images/trimble_robot.png",
        tech_stack: &["C++", "CMake", "Boost", "OpenCV", "Python", "FPGA"],
    },
    Project {
        title: "Building Web Applications",
        link: "/Projects/FFF",
        description:
            "Just a classical fullstack developer. I have developed and maintained multiple applications and tools for internal FFF employees",
        img_src: "/images/fff.png",
        tech_stack: &["React", "Angular", "HTML", "CSS", "Python", "FastAPI", "Javascript", "Typescript",
        "PrimeNG", "MaterialUI", "Bootstrap", "TailwindCSS", "Traefik", "Azure", "MongoDB", "Redis"],
    },
    Project {
        title: "Rewrite git history",
        link: "/Projects/Allianz",
        description:
            "Migrating version control from Excel to Git",
        img_src: "/images/legacy_git.png",
        tech_stack: &["Rust", "Python", "C", "C++", "Batch File", "Polars", "Makefile", "Cmake"],
    },
    Project {
        title: "Rewrite ETL pipeline from Python to Rust",
        link: "/Projects/Allianz2",
        description:
            "This work is an attempt to benchmark Rust performance against existing Python code base.",
        img_src: "/images/polarsvsspark.png",
        tech_stack: &["Rust", "Python", "Polars", "Kafka", "Makefile", "Docker Compose"],
    }
];
