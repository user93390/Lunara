![image](https://github.com/user93390/Lunara/blob/master/Lunara.svg)

> Note that Lunara is still in heavy development.

# Technologies Used

![Docker](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white) \
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) \
![Licence](https://img.shields.io/badge/Apache--2.0-green?style=for-the-badge) \
![React](https://img.shields.io/badge/React-%2320232a.svg?logo=react&logoColor=%2361DAFB) \
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

# Goals
- <p>Clean frontend with minimal resource consumption.</p>
- <p>Improved authentication.</p>
- <p>God tier level dashboard</p>

# Building Locally.
>  You must have
>  - git
>  - cargo
>  - rust
>  - make
>  - bun
>  - Configured ssh with git

<p>Open your terminal of choice and enter</p>

`git clone git@github.com:user93390/Lunara.git` \
`cd Lunara` \
`make build_all`

# Docker configuration
Build docker image using `make dock_init` 
Run docker by using `make dock_compose` \

Automake all the above by using `make dock_auto`
