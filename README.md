![image](https://github.com/user93390/Lunara/blob/master/Lunara.svg)

# Overview
<p> Lunara is aimed to make local Minecraft server hosting easier, faster, and simpler.</p> 

<p> I aim to make Lunara performance-based, this means that I will not be using shit frameworks.</p> 

<p> Lunara is written in rust, with an asynchronous structure with the Axum framework.</p> 

<p> If you are having any issues with building or running Lunara make sure you are running the containerized version.</p>

# Ways Of Supporting

<p> There's no shame in opening and issue or PR on my GitHub repo. Don't be scared to do so!</p>

<p> Make sure to follow the coding standards.</p>

<p> Use your own fingers to type, don't vibe code. A lille bit is good, eh?</p>

<p> Even if you just changed one line, it still makes a big difference.</p>

# Goals
- <p>Clean frontend with minimal resource consumption.</p>
- <p>Improved authentication.</p>
- <p>God tier level dashboard</p>

# Building.
>  You must have
>  - git
>  - cargo 
>  - rust
>  - make
>  - Configured ssh with git

<p>Open your terminal of choice and enter</p>

`git clone git@github.com:user93390/Lunara.git` \
`cd Lunara` \
`make build_all`

# Docker configuration
Build docker image using `make dock_init` \
Run docker by using `make dock_compose` \
Automate building and docker by using `make dock_auto`
