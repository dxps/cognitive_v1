# Cognitive

A space of knowledge and with cognitive functions.

Currently, the focus is on the knowledge: the structure (of entities with their attributes) and links (between entities).<br/>
Later, the cognitive functions will be introduced through a pluggable mechanism. Most probably, a WASM based server side implementation.

---

Initially, it was considered for fulfilling these three needs:

-   Service Registry
    -   exposing API for apps and services to self register
    -   and for service discovery, so that clients can know where to reach apps and services
-   Service Catalog
    -   that shows the list of all the apps and services that exist in your infrastructure
    -   including various (and relevant) details about them, based on their self published metadata
    -   their dependencies
    -   their deployments
-   Centralized Configuration
    -   each service can get its configuration from a central place
    -   as per service versions, configuration would also need to be versioned

<br/>

## Prerequisites & Setup

The followings are the required tools and steps to have the proper setup for the app to run.

### Front-end related

Note: These are needed during development. In other words, if you don't change any code in the components (within `rsx` blocks), then there is no need to run the Tailwind CSS compiler in parallel with the back-end. However, updates to `tailwindcss` npm package generate updates the final `main.css` file.

1. Install [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm).
2. Install the Tailwind CSS [CLI](https://tailwindcss.com/docs/installation).

### Back-end related

1. Have `rust` installed using [rustup.rs](https://rustup.rs/).
2. Install Dioxus CLI ver. 0.6.3 using `cargo install dioxus-cli@0.6.3`.

<br/>

### Run

-   Start the (Tailwind related) CSS compiler using `./run_css.sh` script.
-   Start the back-end (that includes the front-end pages/views) using `./run-dev.sh`.

In case of compilation error that remain hidden behind Dioxus CLI, run `cargo check --features server,web` to reveal them.

<br/>

### The Domain Model

![](./docs/db_model_erd.png)
