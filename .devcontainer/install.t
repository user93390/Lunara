use warnings FATAL => 'all';
use strict;

my @packages = (
    "make",
    "docker",
    "git",
    "rustup",
    "unzip"
);

my @optional_packages = (
    "fish",
    "helix",
);


# Change if you want.
my $optional = 1;

sub install_required {
    print("Installing packages...\n");


    system("curl -fsSL https://bun.com/install | bash");

    for my $package (@packages) {
        install($package);
    }

    if (!$optional) {
        return;
    }

    install_optional();
}

sub install_optional {
    print("installing option dependencies... \n");

    system("cargo install ripgrep");
    system("cargo install fd-find");
    system("cargo install bottom --locked");

    system("echo '[charm]
    name=Charm
    baseurl=https://repo.charm.sh/yum/
    enabled=1
    gpgcheck=1
    gpgkey=https://repo.charm.sh/yum/gpg.key' | sudo tee /etc/yum.repos.d/charm.repo
    sudo yum install glow"
    );

    for my $optional_package (@optional_packages) {
        install($optional_package);
    }
}

sub install {
    my ($arg) = @_;
    system("sudo dnf install " . $arg);
    print("Installed " . $arg . "\n");
}
