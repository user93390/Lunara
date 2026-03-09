[![CodeQL Advanced](https://github.com/user93390/lunara/actions/workflows/codeql.yml/badge.svg)](https://github.com/user93390/lunara/actions/workflows/codeql.yml)
[![Rust CI](https://github.com/user93390/lunara/actions/workflows/rust.yml/badge.svg)](https://github.com/user93390/lunara/actions/workflows/rust.yml)
[![Docker Image CI](https://github.com/user93390/lunara/actions/workflows/docker-image.yml/badge.svg)](https://github.com/user93390/lunara/actions/workflows/docker-image.yml)
[![Dependabot Updates](https://github.com/user93390/lunara/actions/workflows/dependabot/dependabot-updates/badge.svg)](https://github.com/user93390/lunara/actions/workflows/dependabot/dependabot-updates)

![image](https://github.com/user93390/Lunara/blob/master/Lunara.svg)

> Note that Lunara is still in beta development.

# Technologies Used

![Codecov](https://img.shields.io/badge/Codecov-F01F7A?logo=codecov&logoColor=fff) \
![Docker](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white) \
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) \
![Licence](https://img.shields.io/badge/Apache--2.0-green?style=for-the-badge) \
![Git](https://img.shields.io/badge/Git-F05032?logo=git&logoColor=fff)

# Overview

<p> Lunara is aimed to make local Minecraft server hosting easier, faster, and simpler.</p>
<p> I aim to make Lunara performance-based, this means that I will not be using shit frameworks.</p>
<p> Lunara is written in rust, with an asynchronous structure with the Axum framework.</p>
<p> If you are having any issues with building or running Lunara make sure you are running the containerized version.</p>

# Ways Of Supporting

<p> There's no shame in opening and issue or PR on my GitHub repo. Don't be scared to do so!</p>
<p> Make sure to follow the coding standards.</p>
<p> Use your own fingers to type, don't give it to the AI</p>
<p> Even if you just changed one line, it still makes a big difference.</p>

# The Idea

<p> The main goal of this project is to make everyone able to have atleast a starting point for making a Minecraft server</p>
<p> While Lunara is primarly for my usages, it may fit yours too.</p>
<p> I want everyone to be able to use this piece of software. In todays world, not everyone's device supports your software.</p>
<p> By using just plain docker, we can support all 3 major operating systems: Windows, Mac, Linux.</p>
<p> Utilizing rust's memory-safe features and small its utilization footprint, even the smallest devices can run Lunara.</p>

# Compiling
>  You must have
>  - git
>  - cargo
>  - rust
>  - make
>  - bun
>  - Configured ssh with git

<p>Open your terminal of choice and enter:</p>

```bash
git clone git@github.com:user93390/lunara.git && cd lunara

cargo build
```

# Docker Guide
> [!NOTE]
> Use the above steps to build, you must build the application before using docker.

<p> Build docker by using make with the following:</p>

```bash
make dock_init
```
<p> Then run the following command to initilize the containers:</p>

```bash
make dock_compose
```
