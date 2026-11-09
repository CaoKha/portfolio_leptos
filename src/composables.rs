use leptos::document;
use std::sync::LazyLock;

/// Root path the site is served from, e.g. `""` at a domain root or
/// `"/portfolio_leptos"` on a GitHub Pages project site.
///
/// Trunk writes the public URL into the document's `<base>` tag, so the same
/// build works locally, on a custom domain and on `*.github.io/<repo>/`.
pub fn base_path() -> &'static str {
    static BASE: LazyLock<String> = LazyLock::new(|| {
        document()
            .base_uri()
            .ok()
            .flatten()
            .map(|uri| path_from_uri(&uri))
            .unwrap_or_default()
    });
    BASE.as_str()
}

/// Prefix a root-relative site path with the deployment base path.
pub fn site_path(path: &str) -> String {
    let base = base_path();
    format!("{}/{}", base, path.trim_start_matches('/'))
}

fn path_from_uri(uri: &str) -> String {
    let after_scheme = uri.split_once("://").map(|(_, rest)| rest).unwrap_or(uri);
    let path = after_scheme
        .find('/')
        .map(|i| &after_scheme[i..])
        .unwrap_or("/");
    let path = path.split(['?', '#']).next().unwrap_or("/");
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() {
        String::new()
    } else {
        trimmed.to_string()
    }
}

pub struct Experience<'a> {
    pub date: &'a str,
    pub job: &'a str,
    pub company_name: &'a str,
    pub link: &'a str,
    pub content: &'a str,
    pub highlights: &'a [&'a str],
    pub tech_stack: &'a [&'a str],
}

pub struct Project<'a> {
    pub title: &'a str,
    pub link: &'a str,
    pub description: &'a str,
    pub img_src: &'a str,
    pub tech_stack: &'a [&'a str],
}

pub struct SkillGroup<'a> {
    pub label: &'a str,
    pub skills: &'a [&'a str],
}

pub const NAV_ITEMS: [&str; 2] = ["Portfolio", "Projects"];
pub const HEADERS: [&str; 3] = ["About", "Experiences", "Projects"];

pub const SKILL_GROUPS: [SkillGroup; 9] = [
    SkillGroup {
        label: "Languages",
        skills: &[
            "Rust",
            "Python",
            "C++",
            "C",
            "TypeScript",
            "JavaScript",
            "SQL",
            "HTML",
            "CSS",
        ],
    },
    SkillGroup {
        label: "Rust / Bitcoin",
        skills: &[
            "rust-bitcoin",
            "secp256k1",
            "Tokio",
            "Serde",
            "Axum",
            "Tonic",
            "Polars",
            "Git2",
            "Leptos",
            "Tauri",
            "DLC",
            "adaptor signatures",
            "PSBT",
            "Taproot",
            "Schnorr",
            "Bitcoin Core",
        ],
    },
    SkillGroup {
        label: "Python / data",
        skills: &[
            "FastAPI",
            "Django",
            "Pandas",
            "PySpark",
            "SQLAlchemy",
            "Pytest",
            "Kafka",
            "Sentence Transformers",
        ],
    },
    SkillGroup {
        label: "TypeScript / frontend",
        skills: &[
            "React",
            "Angular",
            "Nuxt",
            "Vite",
            "oRPC",
            "Tailwind CSS",
            "ExpressJS",
            "RxJS",
        ],
    },
    SkillGroup {
        label: "Databases",
        skills: &["PostgreSQL", "MongoDB", "MySQL", "SQLite"],
    },
    SkillGroup {
        label: "Testing",
        skills: &[
            "cargo test",
            "Vitest",
            "Playwright",
            "Pytest",
            "Tox",
            "Postman",
            "Swagger",
        ],
    },
    SkillGroup {
        label: "Cloud / CI/CD",
        skills: &[
            "AWS",
            "Azure",
            "Docker",
            "Kubernetes",
            "GitHub Actions",
            "GitLab CI",
            "Azure DevOps",
            "Ansible",
        ],
    },
    SkillGroup {
        label: "Observability / build",
        skills: &[
            "Prometheus",
            "Grafana",
            "Pyroscope",
            "Cargo",
            "Make",
            "Bazel",
            "mdBook",
        ],
    },
    SkillGroup {
        label: "Spoken",
        skills: &["French", "English"],
    },
];

pub const EXPERIENCES: [Experience; 5] = [
    Experience {
        date: "Mar 2025 — Jan 2026",
        job: "Rust Engineer",
        company_name: "Ito",
        link: "https://ito.am/",
        content: "DLC Markets — protocol and engine in Rust.",
        highlights: &[
            "Refactored the oracle engine: event announcement, attestation and signing paths.",
            "Implemented the DLC contract lifecycle: offer, accept and sign message exchange, funding transaction, and Contract Execution Transactions (CETs).",
            "Settled contracts on-chain with adaptor signatures and oracle attestations, built on rust-bitcoin with secp256k1, PSBT, Taproot and Schnorr signatures.",
            "Wrote the wallet, key management and transaction-building logic, and end-to-end integration tests against Bitcoin Core in regtest and signet.",
            "Split the codebase into a cargo workspace: one crate per domain (oracle, contract lifecycle, wallet), with the domain crates free of I/O.",
            "Turned a PhD researcher's protocol research into production Rust code; kept the workspace building on Linux and macOS.",
        ],
        tech_stack: &[
            "Rust",
            "rust-bitcoin",
            "secp256k1",
            "Tokio",
            "Serde",
            "DLC",
            "PSBT",
            "Taproot",
            "Schnorr",
            "Bitcoin Core",
        ],
    },
    Experience {
        date: "Jan 2026 — Present",
        job: "Rust / TypeScript Engineer",
        company_name: "Ito",
        link: "https://ito.am/",
        content: "LN Markets — backend and full stack on the Bitcoin derivatives platform.",
        highlights: &[
            "Built REST APIs in TypeScript and Python; delivered the announcement banner flow and implemented trusted-device and session management.",
            "Building the referral programme for qAssets: gold, stocks, an index and oil, margined and settled in sats.",
            "Built the backend with oRPC: end-to-end type-safe API contracts shared by server and client, removing a separate API schema layer.",
            "Built the frontend in React and TypeScript with Tailwind CSS; tested with Vitest for unit tests and Playwright for end-to-end tests.",
        ],
        tech_stack: &[
            "TypeScript",
            "Rust",
            "Python",
            "React",
            "oRPC",
            "Tailwind CSS",
            "Vite",
            "Vitest",
            "Playwright",
            "GitHub Actions",
            "Docker",
        ],
    },
    Experience {
        date: "Sep 2023 — Mar 2025",
        job: "Backend Software Engineer",
        company_name: "Allianz Trade",
        link: "https://www.allianz-trade.fr/",
        content: "Data and platform work: moving version control from Excel to Git, and building data pipelines, backend services and monitoring tools for the data science team.",
        highlights: &[
            "Data pipelines (Python): extracted and analysed Excel and PDF documents with open-source LLMs from Hugging Face, tuned a Sentence Transformers model, and built PySpark and Kafka pipelines exposed through FastAPI.",
            "Data infrastructure and tooling (Rust): converted Excel tables to Parquet with Polars, profiled the infrastructure with Grafana Pyroscope, and built CLI tools that removed manual steps from the data scientists' workflow.",
            "Legacy modernisation (Rust and Git): built a linking algorithm that reconstructs a Git history graph, automated it across many legacy projects, and migrated version control from Excel to Git across the team.",
            "Backend services (Rust and Python): HTTP and gRPC services with Axum and Tonic, Kafka consumers, FastAPI REST APIs and an event-driven architecture deployed on-premises with Ansible and GitLab CI.",
        ],
        tech_stack: &[
            "Rust",
            "Python",
            "Polars",
            "Git2",
            "Axum",
            "Tonic",
            "Tokio",
            "FastAPI",
            "PySpark",
            "Kafka",
            "PostgreSQL",
            "Prometheus",
            "Grafana",
            "Pyroscope",
            "Docker",
            "Ansible",
            "GitLab CI",
        ],
    },
    Experience {
        date: "Sep 2021 — Sep 2023",
        job: "Full-stack Developer",
        company_name: "French Football Federation",
        link: "https://www.fff.fr/",
        content: "Club management application, online membership payment, and a user dashboard shared by several internal applications.",
        highlights: &[
            "Front end (Angular): built responsive UIs in Angular, TypeScript and SCSS with Angular Material and Bootstrap; route resolvers, route guards and RxJS state with Observables and BehaviorSubject.",
            "Front end (React): built UIs in React and TypeScript with MUI and Tailwind CSS, consumed REST APIs with Axios and lazy-loaded heavy modules.",
            "Back end (Python / FastAPI): developed and documented APIs with FastAPI and Swagger, mapped the database with SQLAlchemy, optimised slow queries and reduced server memory usage.",
            "Testing and delivery: API tests with Postman, end-to-end tests with Playwright, back-end tests with Pytest; CI/CD on Azure DevOps, Docker and Kubernetes.",
        ],
        tech_stack: &[
            "TypeScript",
            "JavaScript",
            "Python",
            "Angular",
            "React",
            "FastAPI",
            "RxJS",
            "PostgreSQL",
            "MongoDB",
            "Playwright",
            "Pytest",
            "Docker",
            "Kubernetes",
            "Azure DevOps",
            "mdBook",
        ],
    },
    Experience {
        date: "Sep 2020 — Jun 2021",
        job: "Data Engineer / Data Scientist",
        company_name: "Fintricity",
        link: "https://www.fintricity.com/",
        content: "Weed classification — a proof of concept for a deep-learning model.",
        highlights: &[
            "Built the proof of concept with Jupyter Notebook, fast.ai and OpenCV; compared MobileNetV3 and ResNet50 for the classification task.",
            "Deployed the model on AWS with Boto3 and Lambda; containerised the work with Docker and documented it in Confluence.",
        ],
        tech_stack: &[
            "Python",
            "NumPy",
            "Pandas",
            "fast.ai",
            "TensorFlow",
            "Keras",
            "PyTorch",
            "OpenCV",
            "AWS Lambda",
            "Boto3",
            "Docker",
        ],
    },
];

pub const PROJECTS: [Project; 8] = [
    Project {
        title: "DLC Markets — DLC protocol and engine",
        link: "/Projects/DlcMarkets",
        description:
            "Oracle engine, DLC contract lifecycle and on-chain settlement with adaptor signatures, PSBT, Taproot and Schnorr signatures.",
        img_src: "",
        tech_stack: &[
            "Rust",
            "rust-bitcoin",
            "secp256k1",
            "PSBT",
            "Taproot",
            "Schnorr",
            "Bitcoin Core",
        ],
    },
    Project {
        title: "LN Markets — Bitcoin derivatives platform",
        link: "/Projects/LnMarkets",
        description:
            "Backend and full-stack features with TypeScript, Rust and Python: REST APIs, sessions, qAssets referrals and type-safe oRPC contracts.",
        img_src: "",
        tech_stack: &[
            "TypeScript",
            "Rust",
            "Python",
            "React",
            "oRPC",
            "Tailwind CSS",
            "Vitest",
            "Playwright",
        ],
    },
    Project {
        title: "Data infrastructure and tooling in Rust",
        link: "/Projects/Etl",
        description:
            "Excel to Parquet with Polars, profiling with Grafana Pyroscope, and Rust CLI tools that removed manual steps for the data science team.",
        img_src: "/images/polarsvsspark.png",
        tech_stack: &["Rust", "Polars", "Pyroscope", "Python", "PySpark", "Kafka"],
    },
    Project {
        title: "Rewrite Git history",
        link: "/Projects/Allianz",
        description:
            "A linking algorithm that reconstructs a Git history graph from legacy Excel projects, automated across the team's version control migration.",
        img_src: "/images/legacy_git.png",
        tech_stack: &["Rust", "Polars", "Git2", "Python", "Makefile"],
    },
    Project {
        title: "Building web applications",
        link: "/Projects/FFF",
        description:
            "Club management, online membership payment and a user dashboard shared by several internal applications at the French Football Federation.",
        img_src: "/images/fff.png",
        tech_stack: &[
            "Angular",
            "React",
            "TypeScript",
            "Python",
            "FastAPI",
            "Tailwind CSS",
        ],
    },
    Project {
        title: "Sensor synchronization",
        link: "/Projects/Trimble",
        description:
            "Aligned RealSense D345 camera and DMU11 IMU data in C++, interpolated with SLERP and published the fused stream as a ROS topic.",
        img_src: "/images/trimble_robot.png",
        tech_stack: &["C++", "SLERP", "ROS", "OpenCV", "Python"],
    },
    Project {
        title: "Finite state machine mobile robot",
        link: "/Projects/SumoRobot",
        description:
            "A robot that pushes objects out of a ring using luminosity and infrared sensors, implemented with finite state machine methodology.",
        img_src: "/images/sumo_robot.png",
        tech_stack: &["C", "Arduino", "Blender"],
    },
    Project {
        title: "E2E encryption",
        link: "/Projects/E2ee",
        description: "A small Rust CLI for end-to-end encryption.",
        img_src: "/images/lock.jpeg",
        tech_stack: &["Rust", "CLI", "Cargo", "Makefile"],
    },
];

#[cfg(test)]
mod tests {
    use super::path_from_uri;
    use leptos_router::matching::Matcher;

    #[test]
    fn matches_root_route_under_base_path() {
        let matcher = Matcher::new("/portfolio_leptos/");
        assert!(matcher.test("/portfolio_leptos/").is_some());
        let project = Matcher::new("/portfolio_leptos/Projects");
        assert!(project.test("/portfolio_leptos/Projects").is_some());
    }

    #[test]
    fn extracts_deployment_paths() {
        assert_eq!(
            path_from_uri("https://caokha.github.io/portfolio_leptos/"),
            "/portfolio_leptos"
        );
        assert_eq!(
            path_from_uri("https://caokha.github.io/portfolio_leptos"),
            "/portfolio_leptos"
        );
        assert_eq!(path_from_uri("http://127.0.0.1:8080/"), "");
        assert_eq!(path_from_uri("https://example.com"), "");
        assert_eq!(
            path_from_uri("/portfolio_leptos/?x=1#top"),
            "/portfolio_leptos"
        );
    }
}
